/// File Storage Abstraction
/// 
/// Provides a unified interface for storing and retrieving files
/// across different storage backends (local, S3, distributed)

use crate::error::{NanoError, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::Uuid;

/// Unique identifier for files
pub type FileId = Uuid;

/// Storage location information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageLocation {
    pub backend_type: String,
    pub path: String,
    pub metadata: HashMap<String, String>,
}

impl StorageLocation {
    pub fn new(backend_type: String, path: String) -> Self {
        Self {
            backend_type,
            path,
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Storage-specific errors
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("File not found: {path}")]
    FileNotFound { path: String },

    #[error("Permission denied: {path}")]
    PermissionDenied { path: String },

    #[error("Storage full")]
    StorageFull,

    #[error("Invalid path: {path}")]
    InvalidPath { path: String },

    #[error("IO error: {source}")]
    Io {
        #[from]
        source: std::io::Error,
    },

    #[error("Backend error: {message}")]
    Backend { message: String },
}

impl From<StorageError> for NanoError {
    fn from(err: StorageError) -> Self {
        NanoError::Storage(err.to_string())
    }
}

/// File storage trait that all backends must implement
#[async_trait]
pub trait FileStorage: Send + Sync {
    /// Store a file and return its storage location
    async fn store_file(&self, file_id: FileId, content: &[u8]) -> Result<StorageLocation>;

    /// Retrieve a file's content from its storage location
    async fn retrieve_file(&self, location: &StorageLocation) -> Result<Vec<u8>>;

    /// Delete a file from storage
    async fn delete_file(&self, location: &StorageLocation) -> Result<()>;

    /// Check if a file exists at the given location
    async fn file_exists(&self, location: &StorageLocation) -> Result<bool>;

    /// Get file size without retrieving content
    async fn get_file_size(&self, location: &StorageLocation) -> Result<u64>;

    /// Store a file in chunks for large files
    async fn store_file_chunked(
        &self,
        file_id: FileId,
        chunks: Vec<Vec<u8>>,
    ) -> Result<Vec<StorageLocation>>;

    /// Retrieve file chunks
    async fn retrieve_file_chunked(
        &self,
        locations: &[StorageLocation],
    ) -> Result<Vec<Vec<u8>>>;

    /// Health check for the storage backend
    async fn health_check(&self) -> Result<()>;

    /// Get storage statistics
    async fn get_stats(&self) -> Result<StorageStats>;
}

/// Storage statistics
#[derive(Debug, Serialize)]
pub struct StorageStats {
    pub total_files: u64,
    pub total_size_bytes: u64,
    pub available_space_bytes: Option<u64>,
    pub backend_specific: HashMap<String, String>,
}

/// Local filesystem storage implementation
pub struct LocalFileStorage {
    base_path: PathBuf,
    chunk_dir: PathBuf,
}

impl LocalFileStorage {
    /// Create a new local file storage instance
    pub async fn new(base_path: PathBuf) -> Result<Self> {
        // Ensure the base directory exists
        fs::create_dir_all(&base_path).await.map_err(|e| {
            NanoError::Storage(format!("Failed to create storage directory: {}", e))
        })?;

        let chunk_dir = base_path.join("chunks");
        fs::create_dir_all(&chunk_dir).await.map_err(|e| {
            NanoError::Storage(format!("Failed to create chunk directory: {}", e))
        })?;

        Ok(Self {
            base_path,
            chunk_dir,
        })
    }

    /// Get the full path for a file ID
    fn get_file_path(&self, file_id: FileId) -> PathBuf {
        let file_id_str = file_id.to_string();
        // Create subdirectories based on first two characters for better distribution
        let subdir = &file_id_str[..2];
        self.base_path.join(subdir).join(&file_id_str)
    }

    /// Get the chunk path for a file ID and chunk index
    fn get_chunk_path(&self, file_id: FileId, chunk_index: usize) -> PathBuf {
        let file_id_str = file_id.to_string();
        let subdir = &file_id_str[..2];
        self.chunk_dir
            .join(subdir)
            .join(format!("{}_chunk_{}", file_id_str, chunk_index))
    }

    /// Ensure subdirectory exists for a path
    async fn ensure_subdir(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await.map_err(|e| {
                NanoError::Storage(format!("Failed to create subdirectory: {}", e))
            })?;
        }
        Ok(())
    }
}

#[async_trait]
impl FileStorage for LocalFileStorage {
    async fn store_file(&self, file_id: FileId, content: &[u8]) -> Result<StorageLocation> {
        let file_path = self.get_file_path(file_id);
        self.ensure_subdir(&file_path).await?;

        let mut file = fs::File::create(&file_path).await.map_err(|e| {
            StorageError::Backend {
                message: format!("Failed to create file: {}", e),
            }
        })?;

        file.write_all(content).await.map_err(|e| {
            StorageError::Backend {
                message: format!("Failed to write file: {}", e),
            }
        })?;

        file.sync_all().await.map_err(|e| {
            StorageError::Backend {
                message: format!("Failed to sync file: {}", e),
            }
        })?;

        Ok(StorageLocation::new(
            "local".to_string(),
            file_path.to_string_lossy().to_string(),
        ))
    }

    async fn retrieve_file(&self, location: &StorageLocation) -> Result<Vec<u8>> {
        if location.backend_type != "local" {
            return Err(StorageError::Backend {
                message: "Invalid backend type for local storage".to_string(),
            }
            .into());
        }

        let path = Path::new(&location.path);
        
        let mut file = fs::File::open(path).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                StorageError::FileNotFound {
                    path: location.path.clone(),
                }
            } else {
                StorageError::Backend {
                    message: format!("Failed to open file: {}", e),
                }
            }
        })?;

        let mut content = Vec::new();
        file.read_to_end(&mut content).await.map_err(|e| {
            StorageError::Backend {
                message: format!("Failed to read file: {}", e),
            }
        })?;

        Ok(content)
    }

    async fn delete_file(&self, location: &StorageLocation) -> Result<()> {
        if location.backend_type != "local" {
            return Err(StorageError::Backend {
                message: "Invalid backend type for local storage".to_string(),
            }
            .into());
        }

        let path = Path::new(&location.path);
        
        fs::remove_file(path).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                StorageError::FileNotFound {
                    path: location.path.clone(),
                }
            } else {
                StorageError::Backend {
                    message: format!("Failed to delete file: {}", e),
                }
            }
        })?;

        Ok(())
    }

    async fn file_exists(&self, location: &StorageLocation) -> Result<bool> {
        if location.backend_type != "local" {
            return Err(StorageError::Backend {
                message: "Invalid backend type for local storage".to_string(),
            }
            .into());
        }

        let path = Path::new(&location.path);
        Ok(path.exists())
    }

    async fn get_file_size(&self, location: &StorageLocation) -> Result<u64> {
        if location.backend_type != "local" {
            return Err(StorageError::Backend {
                message: "Invalid backend type for local storage".to_string(),
            }
            .into());
        }

        let path = Path::new(&location.path);
        
        let metadata = fs::metadata(path).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                StorageError::FileNotFound {
                    path: location.path.clone(),
                }
            } else {
                StorageError::Backend {
                    message: format!("Failed to get file metadata: {}", e),
                }
            }
        })?;

        Ok(metadata.len())
    }

    async fn store_file_chunked(
        &self,
        file_id: FileId,
        chunks: Vec<Vec<u8>>,
    ) -> Result<Vec<StorageLocation>> {
        let mut locations = Vec::new();

        for (index, chunk) in chunks.iter().enumerate() {
            let chunk_path = self.get_chunk_path(file_id, index);
            self.ensure_subdir(&chunk_path).await?;

            let mut file = fs::File::create(&chunk_path).await.map_err(|e| {
                StorageError::Backend {
                    message: format!("Failed to create chunk file: {}", e),
                }
            })?;

            file.write_all(chunk).await.map_err(|e| {
                StorageError::Backend {
                    message: format!("Failed to write chunk: {}", e),
                }
            })?;

            file.sync_all().await.map_err(|e| {
                StorageError::Backend {
                    message: format!("Failed to sync chunk: {}", e),
                }
            })?;

            locations.push(
                StorageLocation::new(
                    "local".to_string(),
                    chunk_path.to_string_lossy().to_string(),
                )
                .with_metadata("chunk_index".to_string(), index.to_string())
                .with_metadata("file_id".to_string(), file_id.to_string()),
            );
        }

        Ok(locations)
    }

    async fn retrieve_file_chunked(
        &self,
        locations: &[StorageLocation],
    ) -> Result<Vec<Vec<u8>>> {
        let mut chunks = Vec::new();

        for location in locations {
            let chunk_data = self.retrieve_file(location).await?;
            chunks.push(chunk_data);
        }

        Ok(chunks)
    }

    async fn health_check(&self) -> Result<()> {
        // Check if base directory is accessible
        if !self.base_path.exists() {
            return Err(StorageError::Backend {
                message: "Base storage directory does not exist".to_string(),
            }
            .into());
        }

        // Try to write and read a test file
        let test_file_id = FileId::new_v4();
        let test_content = b"health_check_test";
        
        let location = self.store_file(test_file_id, test_content).await?;
        let retrieved = self.retrieve_file(&location).await?;
        self.delete_file(&location).await?;

        if retrieved != test_content {
            return Err(StorageError::Backend {
                message: "Health check failed: data integrity issue".to_string(),
            }
            .into());
        }

        Ok(())
    }

    async fn get_stats(&self) -> Result<StorageStats> {
        let mut total_files = 0u64;
        let mut total_size_bytes = 0u64;

        // Walk through all files in the storage directory
        let mut dir_stack = vec![self.base_path.clone()];
        
        while let Some(current_dir) = dir_stack.pop() {
            let mut entries = fs::read_dir(&current_dir).await.map_err(|e| {
                StorageError::Backend {
                    message: format!("Failed to read directory: {}", e),
                }
            })?;

            while let Some(entry) = entries.next_entry().await.map_err(|e| {
                StorageError::Backend {
                    message: format!("Failed to read directory entry: {}", e),
                }
            })? {
                let metadata = entry.metadata().await.map_err(|e| {
                    StorageError::Backend {
                        message: format!("Failed to get metadata: {}", e),
                    }
                })?;

                if metadata.is_file() {
                    total_files += 1;
                    total_size_bytes += metadata.len();
                } else if metadata.is_dir() {
                    dir_stack.push(entry.path());
                }
            }
        }

        // Get available space (this is a simplified implementation)
        let available_space_bytes = match fs::metadata(&self.base_path).await {
            Ok(_) => {
                // In a real implementation, you'd use statvfs/GetDiskFreeSpace
                // For now, return None to indicate unknown
                None
            }
            Err(_) => None,
        };

        let mut backend_specific = HashMap::new();
        backend_specific.insert(
            "base_path".to_string(),
            self.base_path.to_string_lossy().to_string(),
        );
        backend_specific.insert(
            "chunk_path".to_string(),
            self.chunk_dir.to_string_lossy().to_string(),
        );

        Ok(StorageStats {
            total_files,
            total_size_bytes,
            available_space_bytes,
            backend_specific,
        })
    }
}

#[cfg(feature = "s3-storage")]
pub mod s3_storage {
    use super::*;
    use aws_sdk_s3::Client;
    use bytes::Bytes;

    /// S3 storage implementation
    pub struct S3FileStorage {
        client: Client,
        bucket: String,
        prefix: String,
    }

    impl S3FileStorage {
        pub async fn new(
            bucket: String,
            region: String,
            endpoint: Option<String>,
        ) -> Result<Self> {
            let config = aws_config::load_from_env().await;
            let mut s3_config = aws_sdk_s3::config::Builder::from(&config)
                .region(aws_sdk_s3::types::Region::new(region));

            if let Some(endpoint) = endpoint {
                s3_config = s3_config.endpoint_url(endpoint);
            }

            let client = Client::from_conf(s3_config.build());

            Ok(Self {
                client,
                bucket,
                prefix: "nano-messenger/files/".to_string(),
            })
        }

        fn get_object_key(&self, file_id: FileId) -> String {
            format!("{}{}", self.prefix, file_id)
        }
    }

    #[async_trait]
    impl FileStorage for S3FileStorage {
        async fn store_file(&self, file_id: FileId, content: &[u8]) -> Result<StorageLocation> {
            let key = self.get_object_key(file_id);

            self.client
                .put_object()
                .bucket(&self.bucket)
                .key(&key)
                .body(Bytes::from(content.to_vec()).into())
                .send()
                .await
                .map_err(|e| StorageError::Backend {
                    message: format!("S3 upload failed: {}", e),
                })?;

            Ok(StorageLocation::new("s3".to_string(), key)
                .with_metadata("bucket".to_string(), self.bucket.clone()))
        }

        async fn retrieve_file(&self, location: &StorageLocation) -> Result<Vec<u8>> {
            if location.backend_type != "s3" {
                return Err(StorageError::Backend {
                    message: "Invalid backend type for S3 storage".to_string(),
                }
                .into());
            }

            let response = self
                .client
                .get_object()
                .bucket(&self.bucket)
                .key(&location.path)
                .send()
                .await
                .map_err(|e| StorageError::Backend {
                    message: format!("S3 download failed: {}", e),
                })?;

            let bytes = response
                .body
                .collect()
                .await
                .map_err(|e| StorageError::Backend {
                    message: format!("Failed to read S3 response body: {}", e),
                })?;

            Ok(bytes.to_vec())
        }

        async fn delete_file(&self, location: &StorageLocation) -> Result<()> {
            if location.backend_type != "s3" {
                return Err(StorageError::Backend {
                    message: "Invalid backend type for S3 storage".to_string(),
                }
                .into());
            }

            self.client
                .delete_object()
                .bucket(&self.bucket)
                .key(&location.path)
                .send()
                .await
                .map_err(|e| StorageError::Backend {
                    message: format!("S3 delete failed: {}", e),
                })?;

            Ok(())
        }

        async fn file_exists(&self, location: &StorageLocation) -> Result<bool> {
            if location.backend_type != "s3" {
                return Err(StorageError::Backend {
                    message: "Invalid backend type for S3 storage".to_string(),
                }
                .into());
            }

            match self
                .client
                .head_object()
                .bucket(&self.bucket)
                .key(&location.path)
                .send()
                .await
            {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            }
        }

        async fn get_file_size(&self, location: &StorageLocation) -> Result<u64> {
            if location.backend_type != "s3" {
                return Err(StorageError::Backend {
                    message: "Invalid backend type for S3 storage".to_string(),
                }
                .into());
            }

            let response = self
                .client
                .head_object()
                .bucket(&self.bucket)
                .key(&location.path)
                .send()
                .await
                .map_err(|e| StorageError::Backend {
                    message: format!("S3 head object failed: {}", e),
                })?;

            Ok(response.content_length().unwrap_or(0) as u64)
        }

        async fn store_file_chunked(
            &self,
            file_id: FileId,
            chunks: Vec<Vec<u8>>,
        ) -> Result<Vec<StorageLocation>> {
            // For S3, we could use multipart upload here
            // For simplicity, store each chunk as a separate object
            let mut locations = Vec::new();

            for (index, chunk) in chunks.iter().enumerate() {
                let chunk_key = format!("{}{}_chunk_{}", self.prefix, file_id, index);

                self.client
                    .put_object()
                    .bucket(&self.bucket)
                    .key(&chunk_key)
                    .body(Bytes::from(chunk.clone()).into())
                    .send()
                    .await
                    .map_err(|e| StorageError::Backend {
                        message: format!("S3 chunk upload failed: {}", e),
                    })?;

                locations.push(
                    StorageLocation::new("s3".to_string(), chunk_key)
                        .with_metadata("bucket".to_string(), self.bucket.clone())
                        .with_metadata("chunk_index".to_string(), index.to_string())
                        .with_metadata("file_id".to_string(), file_id.to_string()),
                );
            }

            Ok(locations)
        }

        async fn retrieve_file_chunked(
            &self,
            locations: &[StorageLocation],
        ) -> Result<Vec<Vec<u8>>> {
            let mut chunks = Vec::new();

            for location in locations {
                let chunk_data = self.retrieve_file(location).await?;
                chunks.push(chunk_data);
            }

            Ok(chunks)
        }

        async fn health_check(&self) -> Result<()> {
            // Try to list bucket to verify connectivity
            self.client
                .head_bucket()
                .bucket(&self.bucket)
                .send()
                .await
                .map_err(|e| StorageError::Backend {
                    message: format!("S3 health check failed: {}", e),
                })?;

            Ok(())
        }

        async fn get_stats(&self) -> Result<StorageStats> {
            // This would require listing all objects in the bucket
            // For now, return basic stats
            let mut backend_specific = HashMap::new();
            backend_specific.insert("bucket".to_string(), self.bucket.clone());
            backend_specific.insert("prefix".to_string(), self.prefix.clone());

            Ok(StorageStats {
                total_files: 0, // Would need to count objects
                total_size_bytes: 0, // Would need to sum object sizes
                available_space_bytes: None, // S3 has no practical limit
                backend_specific,
            })
        }
    }
}

#[cfg(feature = "s3-storage")]
pub use s3_storage::S3FileStorage;

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_local_storage_basic_operations() {
        let temp_dir = TempDir::new().unwrap();
        let storage = LocalFileStorage::new(temp_dir.path().to_path_buf())
            .await
            .unwrap();

        let file_id = FileId::new_v4();
        let content = b"test file content";

        // Store file
        let location = storage.store_file(file_id, content).await.unwrap();
        assert_eq!(location.backend_type, "local");

        // Check file exists
        assert!(storage.file_exists(&location).await.unwrap());

        // Get file size
        let size = storage.get_file_size(&location).await.unwrap();
        assert_eq!(size, content.len() as u64);

        // Retrieve file
        let retrieved = storage.retrieve_file(&location).await.unwrap();
        assert_eq!(retrieved, content);

        // Delete file
        storage.delete_file(&location).await.unwrap();
        assert!(!storage.file_exists(&location).await.unwrap());
    }

    #[tokio::test]
    async fn test_local_storage_chunked_operations() {
        let temp_dir = TempDir::new().unwrap();
        let storage = LocalFileStorage::new(temp_dir.path().to_path_buf())
            .await
            .unwrap();

        let file_id = FileId::new_v4();
        let chunks = vec![
            b"chunk 1 content".to_vec(),
            b"chunk 2 content".to_vec(),
            b"chunk 3 content".to_vec(),
        ];

        // Store chunks
        let locations = storage.store_file_chunked(file_id, chunks.clone()).await.unwrap();
        assert_eq!(locations.len(), 3);

        // Retrieve chunks
        let retrieved_chunks = storage.retrieve_file_chunked(&locations).await.unwrap();
        assert_eq!(retrieved_chunks, chunks);

        // Clean up
        for location in &locations {
            storage.delete_file(location).await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_local_storage_health_check() {
        let temp_dir = TempDir::new().unwrap();
        let storage = LocalFileStorage::new(temp_dir.path().to_path_buf())
            .await
            .unwrap();

        // Health check should pass
        assert!(storage.health_check().await.is_ok());
    }

    #[tokio::test]
    async fn test_local_storage_stats() {
        let temp_dir = TempDir::new().unwrap();
        let storage = LocalFileStorage::new(temp_dir.path().to_path_buf())
            .await
            .unwrap();

        // Get initial stats
        let stats = storage.get_stats().await.unwrap();
        assert_eq!(stats.total_files, 0);
        assert_eq!(stats.total_size_bytes, 0);

        // Store a file
        let file_id = FileId::new_v4();
        let content = b"test content for stats";
        let _location = storage.store_file(file_id, content).await.unwrap();

        // Get updated stats
        let stats = storage.get_stats().await.unwrap();
        assert_eq!(stats.total_files, 1);
        assert_eq!(stats.total_size_bytes, content.len() as u64);
    }
}
