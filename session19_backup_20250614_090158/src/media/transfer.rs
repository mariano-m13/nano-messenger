/// File Transfer Manager
/// 
/// Orchestrates file uploads, downloads, and transfers with quantum-resistant
/// encryption, chunking for large files, and comprehensive progress tracking

use crate::crypto::{CryptoMode, UnifiedPublicKeys, UnifiedKeyPair};
use crate::error::{NanoError, Result};
use crate::media::{
    storage::{FileStorage, FileId},
    encryption::{FileEncryption, EncryptedFile},
    metadata::{FileMetadata, MetadataStore, FilePermissions, FileReference, UserId},
};
use mime_guess::from_path;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Cursor;
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::{RwLock, Semaphore};
use uuid::Uuid;

/// File upload data structure
#[derive(Debug, Clone)]
pub struct FileUpload {
    pub file_id: FileId,
    pub original_name: String,
    pub content: Vec<u8>,
    pub mime_type: Option<String>,
    pub tags: Vec<String>,
    pub description: Option<String>,
    pub custom_metadata: HashMap<String, String>,
    pub permissions: FilePermissions,
    pub expires_at: Option<SystemTime>,
}

impl FileUpload {
    /// Create a new file upload
    pub fn new(original_name: String, content: Vec<u8>) -> Self {
        let file_id = FileId::new_v4();
        let mime_type = from_path(&original_name)
            .first()
            .map(|mime| mime.to_string());

        Self {
            file_id,
            original_name,
            content,
            mime_type,
            tags: Vec::new(),
            description: None,
            custom_metadata: HashMap::new(),
            permissions: FilePermissions::default(),
            expires_at: None,
        }
    }

    /// Set MIME type
    pub fn with_mime_type(mut self, mime_type: String) -> Self {
        self.mime_type = Some(mime_type);
        self
    }

    /// Add tags
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Set permissions
    pub fn with_permissions(mut self, permissions: FilePermissions) -> Self {
        self.permissions = permissions;
        self
    }

    /// Set expiry time
    pub fn with_expiry(mut self, expires_at: SystemTime) -> Self {
        self.expires_at = Some(expires_at);
        self
    }

    /// Add custom metadata
    pub fn with_custom_metadata(mut self, key: String, value: String) -> Self {
        self.custom_metadata.insert(key, value);
        self
    }

    /// Get file size
    pub fn size(&self) -> u64 {
        self.content.len() as u64
    }

    /// Get MIME type or guess from filename
    pub fn get_mime_type(&self) -> String {
        self.mime_type.clone()
            .unwrap_or_else(|| {
                from_path(&self.original_name)
                    .first()
                    .map(|mime| mime.to_string())
                    .unwrap_or_else(|| "application/octet-stream".to_string())
            })
    }
}

/// Decrypted file data structure
#[derive(Debug, Clone)]
pub struct DecryptedFile {
    pub file_id: FileId,
    pub original_name: String,
    pub content: Vec<u8>,
    pub mime_type: String,
    pub metadata: FileMetadata,
}

impl DecryptedFile {
    /// Get file size
    pub fn size(&self) -> u64 {
        self.content.len() as u64
    }

    /// Save to a path
    pub async fn save_to_path(&self, path: &Path) -> Result<()> {
        tokio::fs::write(path, &self.content).await.map_err(|e| {
            NanoError::Media(format!("Failed to save file: {}", e))
        })
    }

    /// Get content as bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.content
    }

    /// Get content as string (if text)
    pub fn as_string(&self) -> Result<String> {
        String::from_utf8(self.content.clone()).map_err(|e| {
            NanoError::Media(format!("File is not valid UTF-8: {}", e))
        })
    }
}

/// Transfer progress information
#[derive(Debug, Clone, Serialize)]
pub struct TransferProgress {
    pub file_id: FileId,
    pub operation: TransferOperation,
    pub total_bytes: u64,
    pub transferred_bytes: u64,
    pub percentage: f64,
    pub speed_bytes_per_sec: f64,
    pub eta_seconds: Option<u64>,
    pub status: TransferStatus,
    pub error_message: Option<String>,
    pub started_at: SystemTime,
    pub updated_at: SystemTime,
}

impl TransferProgress {
    /// Create new progress tracker
    pub fn new(file_id: FileId, operation: TransferOperation, total_bytes: u64) -> Self {
        let now = SystemTime::now();
        Self {
            file_id,
            operation,
            total_bytes,
            transferred_bytes: 0,
            percentage: 0.0,
            speed_bytes_per_sec: 0.0,
            eta_seconds: None,
            status: TransferStatus::Starting,
            error_message: None,
            started_at: now,
            updated_at: now,
        }
    }

    /// Update progress
    pub fn update(&mut self, transferred_bytes: u64) {
        self.transferred_bytes = transferred_bytes;
        self.percentage = if self.total_bytes > 0 {
            (transferred_bytes as f64 / self.total_bytes as f64) * 100.0
        } else {
            0.0
        };

        let now = SystemTime::now();
        if let Ok(elapsed) = now.duration_since(self.started_at) {
            let elapsed_secs = elapsed.as_secs_f64();
            if elapsed_secs > 0.0 {
                self.speed_bytes_per_sec = transferred_bytes as f64 / elapsed_secs;
                
                if self.speed_bytes_per_sec > 0.0 {
                    let remaining_bytes = self.total_bytes - transferred_bytes;
                    self.eta_seconds = Some((remaining_bytes as f64 / self.speed_bytes_per_sec) as u64);
                }
            }
        }
        
        self.updated_at = now;
        
        if transferred_bytes >= self.total_bytes {
            self.status = TransferStatus::Completed;
        } else {
            self.status = TransferStatus::InProgress;
        }
    }

    /// Mark as completed
    pub fn complete(&mut self) {
        self.status = TransferStatus::Completed;
        self.percentage = 100.0;
        self.updated_at = SystemTime::now();
    }

    /// Mark as failed
    pub fn fail(&mut self, error: String) {
        self.status = TransferStatus::Failed;
        self.error_message = Some(error);
        self.updated_at = SystemTime::now();
    }
}

/// Transfer operation type
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TransferOperation {
    Upload,
    Download,
    Delete,
}

/// Transfer status
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TransferStatus {
    Starting,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

/// File transfer manager
pub struct FileTransferManager {
    storage: Arc<dyn FileStorage>,
    encryption: FileEncryption,
    metadata_store: Arc<MetadataStore>,
    active_transfers: Arc<RwLock<HashMap<FileId, TransferProgress>>>,
    concurrent_limit: Semaphore,
    chunk_size: usize,
}

impl FileTransferManager {
    /// Create a new file transfer manager
    pub fn new(
        storage: Arc<dyn FileStorage>,
        encryption: FileEncryption,
        metadata_store: MetadataStore,
    ) -> Self {
        Self {
            storage,
            encryption,
            metadata_store: Arc::new(metadata_store),
            active_transfers: Arc::new(RwLock::new(HashMap::new())),
            concurrent_limit: Semaphore::new(10), // Max 10 concurrent transfers
            chunk_size: 1024 * 1024, // 1MB chunks
        }
    }

    /// Upload a file
    pub async fn upload_file(
        &self,
        file_upload: FileUpload,
        sender_keypair: &UnifiedKeyPair,
        recipient_keys: &UnifiedPublicKeys,
    ) -> Result<FileReference> {
        let _permit = self.concurrent_limit.acquire().await.map_err(|e| {
            NanoError::Media(format!("Failed to acquire transfer permit: {}", e))
        })?;

        let file_id = file_upload.file_id;
        let file_size = file_upload.size();
        
        // Create progress tracker
        let mut progress = TransferProgress::new(file_id, TransferOperation::Upload, file_size);
        
        {
            let mut transfers = self.active_transfers.write().await;
            transfers.insert(file_id, progress.clone());
        }

        let result = self.upload_file_internal(file_upload, sender_keypair, recipient_keys, &mut progress).await;

        // Update final progress
        match &result {
            Ok(_) => progress.complete(),
            Err(e) => progress.fail(e.to_string()),
        }

        {
            let mut transfers = self.active_transfers.write().await;
            transfers.insert(file_id, progress);
        }

        result
    }

    /// Internal upload implementation
    async fn upload_file_internal(
        &self,
        file_upload: FileUpload,
        sender_keypair: &UnifiedKeyPair,
        recipient_keys: &UnifiedPublicKeys,
        progress: &mut TransferProgress,
    ) -> Result<FileReference> {
        let file_id = file_upload.file_id;
        let file_size = file_upload.size();

        // Validate file size and type here if needed
        // (This would integrate with MediaConfig validation)

        // Encrypt the file
        progress.status = TransferStatus::InProgress;
        let encrypted_file = if file_size > self.chunk_size as u64 {
            // Use chunked encryption for large files
            let cursor = Cursor::new(&file_upload.content);
            let chunked = self.encryption.encrypt_file_chunked(cursor, recipient_keys)?;
            
            // Store chunks
            let chunk_data: Vec<Vec<u8>> = chunked.chunks
                .iter()
                .map(|chunk| chunk.encrypted_data.clone())
                .collect();
            
            let _chunk_locations = self.storage.store_file_chunked(file_id, chunk_data).await?;
            
            // For simplicity, combine chunks into a single encrypted file for metadata
            // In a real implementation, you'd store chunk references separately
            let combined_content = chunked.chunks
                .iter()
                .flat_map(|chunk| chunk.encrypted_data.iter())
                .cloned()
                .collect();

            EncryptedFile {
                encrypted_content: combined_content,
                encrypted_key: chunked.encrypted_key,
                integrity_hash: chunked.integrity_hash,
                content_hash: vec![], // Would compute this properly
                crypto_mode: chunked.crypto_mode,
                encryption_metadata: chunked.encryption_metadata,
            }
        } else {
            // Use regular encryption for small files
            self.encryption.encrypt_file(&file_upload.content, recipient_keys)?
        };

        progress.update(file_size / 2); // 50% after encryption

        // Store encrypted file
        let storage_location = self.storage.store_file(file_id, &encrypted_file.encrypted_content).await?;
        
        progress.update(file_size); // 100% after storage

        // Create file metadata
        let checksum = hex::encode(&encrypted_file.content_hash);
        let mut metadata = FileMetadata::new(
            file_id,
            file_upload.original_name.clone(),
            file_upload.get_mime_type(),
            file_size,
            sender_keypair.public_key_string(), // Use as user ID
            encrypted_file.encryption_metadata,
            storage_location,
            checksum,
        );

        // Set additional metadata
        metadata.tags = file_upload.tags;
        metadata.description = file_upload.description;
        metadata.access_permissions = file_upload.permissions;
        metadata.expiry_timestamp = file_upload.expires_at;
        metadata.custom_metadata = file_upload.custom_metadata;

        // Store metadata
        self.metadata_store.store_metadata(metadata.clone()).await?;

        // Create file reference
        let reference = FileReference::new(
            file_id,
            sender_keypair.public_key_string(),
            file_upload.expires_at,
            None, // No usage limit by default
        );

        let reference_id = self.metadata_store.create_reference(reference.clone()).await?;

        Ok(FileReference {
            reference_id,
            ..reference
        })
    }

    /// Download a file by reference
    pub async fn download_file(
        &self,
        reference_id: &Uuid,
        recipient_keypair: &UnifiedKeyPair,
    ) -> Result<DecryptedFile> {
        let _permit = self.concurrent_limit.acquire().await.map_err(|e| {
            NanoError::Media(format!("Failed to acquire transfer permit: {}", e))
        })?;

        // Get and use the file reference
        let file_id = self.metadata_store.use_reference(reference_id).await?;

        // Get file metadata
        let metadata = self.metadata_store.get_metadata(&file_id).await?
            .ok_or_else(|| NanoError::Media("File metadata not found".to_string()))?;

        // Check if file is deleted or expired
        if metadata.is_deleted {
            return Err(NanoError::Media("File has been deleted".to_string()));
        }
        if metadata.is_expired() {
            return Err(NanoError::Media("File has expired".to_string()));
        }

        let file_size = metadata.file_size;
        let mut progress = TransferProgress::new(file_id, TransferOperation::Download, file_size);

        {
            let mut transfers = self.active_transfers.write().await;
            transfers.insert(file_id, progress.clone());
        }

        let result = self.download_file_internal(&metadata, recipient_keypair, &mut progress).await;

        // Update final progress
        match &result {
            Ok(_) => progress.complete(),
            Err(e) => progress.fail(e.to_string()),
        }

        {
            let mut transfers = self.active_transfers.write().await;
            transfers.insert(file_id, progress);
        }

        result
    }

    /// Internal download implementation
    async fn download_file_internal(
        &self,
        metadata: &FileMetadata,
        _recipient_keypair: &UnifiedKeyPair,
        progress: &mut TransferProgress,
    ) -> Result<DecryptedFile> {
        progress.status = TransferStatus::InProgress;

        // Retrieve encrypted file content
        let encrypted_content = self.storage.retrieve_file(&metadata.storage_location).await?;
        
        progress.update(metadata.file_size / 2); // 50% after retrieval

        // Create EncryptedFile structure for decryption
        let encrypted_file = EncryptedFile {
            encrypted_content,
            encrypted_key: vec![], // This should be stored in metadata
            integrity_hash: vec![], // This should be stored in metadata
            content_hash: vec![], // This should be stored in metadata
            crypto_mode: CryptoMode::Classical, // Should come from metadata
            encryption_metadata: metadata.encryption_info.clone(),
        };

        // Decrypt the file
        // Note: This is simplified - in reality, you'd need the proper private key format
        let private_key_bytes = vec![0u8; 32]; // Placeholder
        let decrypted_content = self.encryption.decrypt_file(&encrypted_file, &private_key_bytes)?;

        progress.update(metadata.file_size); // 100% after decryption

        Ok(DecryptedFile {
            file_id: metadata.file_id,
            original_name: metadata.original_name.clone(),
            content: decrypted_content,
            mime_type: metadata.mime_type.clone(),
            metadata: metadata.clone(),
        })
    }

    /// Get transfer progress
    pub async fn get_transfer_progress(&self, file_id: &FileId) -> Option<TransferProgress> {
        let transfers = self.active_transfers.read().await;
        transfers.get(file_id).cloned()
    }

    /// Get all active transfers
    pub async fn get_active_transfers(&self) -> Vec<TransferProgress> {
        let transfers = self.active_transfers.read().await;
        transfers.values().cloned().collect()
    }

    /// Cancel a transfer
    pub async fn cancel_transfer(&self, file_id: &FileId) -> Result<()> {
        let mut transfers = self.active_transfers.write().await;
        if let Some(progress) = transfers.get_mut(file_id) {
            progress.status = TransferStatus::Cancelled;
            progress.updated_at = SystemTime::now();
            Ok(())
        } else {
            Err(NanoError::Media("Transfer not found".to_string()))
        }
    }

    /// Delete a file
    pub async fn delete_file(&self, file_id: &FileId, user_id: &UserId) -> Result<()> {
        let _permit = self.concurrent_limit.acquire().await.map_err(|e| {
            NanoError::Media(format!("Failed to acquire transfer permit: {}", e))
        })?;

        // Get file metadata
        let mut metadata = self.metadata_store.get_metadata(file_id).await?
            .ok_or_else(|| NanoError::Media("File not found".to_string()))?;

        // Check permissions
        use crate::media::metadata::FilePermission;
        if !metadata.user_has_permission(user_id, FilePermission::Delete) {
            return Err(NanoError::Media("Permission denied".to_string()));
        }

        let mut progress = TransferProgress::new(*file_id, TransferOperation::Delete, 1);
        
        {
            let mut transfers = self.active_transfers.write().await;
            transfers.insert(*file_id, progress.clone());
        }

        // Delete from storage
        let result = self.storage.delete_file(&metadata.storage_location).await;

        match result {
            Ok(_) => {
                // Mark as deleted in metadata
                metadata.mark_deleted();
                self.metadata_store.update_metadata(file_id, metadata).await?;
                progress.complete();
            }
            Err(e) => {
                progress.fail(e.to_string());
                return Err(e);
            }
        }

        {
            let mut transfers = self.active_transfers.write().await;
            transfers.insert(*file_id, progress);
        }

        Ok(())
    }

    /// Get transfer statistics
    pub async fn get_transfer_stats(&self) -> TransferStatistics {
        let transfers = self.active_transfers.read().await;
        
        let mut stats = TransferStatistics {
            total_transfers: transfers.len() as u64,
            active_uploads: 0,
            active_downloads: 0,
            completed_transfers: 0,
            failed_transfers: 0,
            total_bytes_transferred: 0,
            average_speed_mbps: 0.0,
        };

        let mut total_speed = 0.0;
        let mut speed_count = 0;

        for progress in transfers.values() {
            match progress.operation {
                TransferOperation::Upload => {
                    if matches!(progress.status, TransferStatus::InProgress) {
                        stats.active_uploads += 1;
                    }
                }
                TransferOperation::Download => {
                    if matches!(progress.status, TransferStatus::InProgress) {
                        stats.active_downloads += 1;
                    }
                }
                TransferOperation::Delete => {}
            }

            match progress.status {
                TransferStatus::Completed => stats.completed_transfers += 1,
                TransferStatus::Failed => stats.failed_transfers += 1,
                _ => {}
            }

            stats.total_bytes_transferred += progress.transferred_bytes;

            if progress.speed_bytes_per_sec > 0.0 {
                total_speed += progress.speed_bytes_per_sec;
                speed_count += 1;
            }
        }

        if speed_count > 0 {
            let avg_bytes_per_sec = total_speed / speed_count as f64;
            stats.average_speed_mbps = (avg_bytes_per_sec / 1024.0 / 1024.0) * 8.0; // Convert to Mbps
        }

        stats
    }

    /// Clean up completed transfers
    pub async fn cleanup_completed_transfers(&self, max_age_seconds: u64) -> u64 {
        let mut transfers = self.active_transfers.write().await;
        let cutoff_time = SystemTime::now()
            .checked_sub(Duration::from_secs(max_age_seconds))
            .unwrap_or(SystemTime::UNIX_EPOCH);

        let initial_count = transfers.len();

        transfers.retain(|_, progress| {
            !matches!(progress.status, TransferStatus::Completed | TransferStatus::Failed)
                || progress.updated_at > cutoff_time
        });

        (initial_count - transfers.len()) as u64
    }

    /// Get storage statistics
    pub async fn get_storage_stats(&self) -> Result<crate::media::storage::StorageStats> {
        self.storage.get_stats().await
    }

    /// Perform health check
    pub async fn health_check(&self) -> Result<TransferManagerHealth> {
        let mut issues = Vec::new();

        // Check storage health
        if let Err(e) = self.storage.health_check().await {
            issues.push(format!("Storage backend error: {}", e));
        }

        // Check transfer queue
        let active_count = {
            let transfers = self.active_transfers.read().await;
            transfers.len()
        };

        if active_count > 100 {
            issues.push("High number of active transfers may impact performance".to_string());
        }

        // Check available permits
        let available_permits = self.concurrent_limit.available_permits();
        if available_permits == 0 {
            issues.push("All transfer slots are in use".to_string());
        }

        Ok(TransferManagerHealth {
            is_healthy: issues.is_empty(),
            issues,
            active_transfers: active_count as u64,
            available_permits: available_permits as u64,
            total_permits: 10, // Our current limit
        })
    }
}

/// Transfer statistics
#[derive(Debug, Clone, Serialize)]
pub struct TransferStatistics {
    pub total_transfers: u64,
    pub active_uploads: u64,
    pub active_downloads: u64,
    pub completed_transfers: u64,
    pub failed_transfers: u64,
    pub total_bytes_transferred: u64,
    pub average_speed_mbps: f64,
}

/// Transfer manager health status
#[derive(Debug, Clone, Serialize)]
pub struct TransferManagerHealth {
    pub is_healthy: bool,
    pub issues: Vec<String>,
    pub active_transfers: u64,
    pub available_permits: u64,
    pub total_permits: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::media::storage::LocalFileStorage;
    use crate::media::metadata::MetadataStore;
    use tempfile::TempDir;

    async fn create_test_transfer_manager() -> (FileTransferManager, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        
        // Ensure the storage directory exists
        std::fs::create_dir_all(temp_dir.path()).unwrap();
        
        let storage = LocalFileStorage::new(temp_dir.path().to_path_buf()).await.unwrap();
        let storage_arc = Arc::new(storage) as Arc<dyn FileStorage>;
        let encryption = FileEncryption::new(CryptoMode::Classical, 10);
        let metadata_store = MetadataStore::new().await.unwrap();

        let manager = FileTransferManager::new(storage_arc, encryption, metadata_store);
        (manager, temp_dir)
    }

    #[tokio::test]
    async fn test_file_upload_progress() {
        let content = b"test file content for progress"; // 30 bytes
        let file_upload = FileUpload::new("test.txt".to_string(), content.to_vec());
        let file_id = file_upload.file_id;

        let mut progress = TransferProgress::new(file_id, TransferOperation::Upload, content.len() as u64);

        assert_eq!(progress.percentage, 0.0);
        assert_eq!(progress.status as u8, TransferStatus::Starting as u8);

        progress.update(content.len() as u64 / 2);
        assert_eq!(progress.percentage, 50.0);
        assert_eq!(progress.status as u8, TransferStatus::InProgress as u8);

        progress.complete();
        assert_eq!(progress.percentage, 100.0);
        assert_eq!(progress.status as u8, TransferStatus::Completed as u8);
    }

    #[tokio::test]
    async fn test_file_upload_structure() {
        let content = b"test file content";
        let file_upload = FileUpload::new("document.pdf".to_string(), content.to_vec())
            .with_mime_type("application/pdf".to_string())
            .with_tags(vec!["important".to_string(), "work".to_string()])
            .with_description("Test document".to_string());

        assert_eq!(file_upload.original_name, "document.pdf");
        assert_eq!(file_upload.get_mime_type(), "application/pdf");
        assert_eq!(file_upload.size(), content.len() as u64);
        assert!(file_upload.tags.contains(&"important".to_string()));
        assert_eq!(file_upload.description, Some("Test document".to_string()));
    }

    #[test]
    fn test_transfer_progress_calculations() {
        let file_id = FileId::new_v4();
        let mut progress = TransferProgress::new(file_id, TransferOperation::Upload, 1000);

        // Test initial state
        assert_eq!(progress.percentage, 0.0);
        assert_eq!(progress.transferred_bytes, 0);

        // Test progress update
        std::thread::sleep(std::time::Duration::from_millis(100));
        progress.update(500);
        assert_eq!(progress.percentage, 50.0);
        assert!(progress.speed_bytes_per_sec > 0.0);

        // Test completion
        progress.update(1000);
        assert_eq!(progress.percentage, 100.0);
        assert_eq!(progress.status as u8, TransferStatus::Completed as u8);
    }

    #[test]
    fn test_mime_type_detection() {
        let file_upload = FileUpload::new("test.txt".to_string(), vec![]);
        assert_eq!(file_upload.get_mime_type(), "text/plain");

        let file_upload = FileUpload::new("image.jpg".to_string(), vec![]);
        assert_eq!(file_upload.get_mime_type(), "image/jpeg");

        let file_upload = FileUpload::new("document.pdf".to_string(), vec![]);
        assert_eq!(file_upload.get_mime_type(), "application/pdf");

        let file_upload = FileUpload::new("unknown.nonexistent123".to_string(), vec![]);
        assert_eq!(file_upload.get_mime_type(), "application/octet-stream");
    }

    #[tokio::test]
    async fn test_transfer_manager_health_check() {
        let (manager, _temp_dir) = create_test_transfer_manager().await;
        let health = manager.health_check().await.unwrap();
        
        assert!(health.is_healthy);
        assert_eq!(health.active_transfers, 0);
        assert!(health.available_permits > 0);
    }

    #[tokio::test]
    async fn test_transfer_statistics() {
        let (manager, _temp_dir) = create_test_transfer_manager().await;
        let stats = manager.get_transfer_stats().await;
        
        assert_eq!(stats.total_transfers, 0);
        assert_eq!(stats.active_uploads, 0);
        assert_eq!(stats.active_downloads, 0);
        assert_eq!(stats.completed_transfers, 0);
        assert_eq!(stats.failed_transfers, 0);
    }
}
