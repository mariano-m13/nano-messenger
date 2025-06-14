/// Session 11: File Deduplication System
/// 
/// Provides content-based deduplication to save storage space and reduce transfer times
/// by identifying duplicate files and chunks with quantum-resistant hashing.

use crate::error::{NanoError, Result};
use crate::media::{
    storage::{FileStorage, FileId, StorageLocation},
    metadata::UserId,
};
use blake2::{Blake2b512, Blake2s256, Digest as Blake2Digest};
use sha3::{Sha3_512, Digest};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::RwLock;

/// Content hash for deduplication
pub type ContentHash = Vec<u8>;

/// Chunk reference for partial deduplication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkReference {
    pub chunk_hash: ContentHash,
    pub storage_location: StorageLocation,
    pub size: u64,
    pub reference_count: u32,
    pub created_at: SystemTime,
}

/// File chunk for deduplication processing
#[derive(Debug, Clone)]
pub struct FileChunk {
    pub index: u32,
    pub data: Vec<u8>,
    pub hash: ContentHash,
}

impl FileChunk {
    /// Create a new file chunk
    pub fn new(index: u32, data: Vec<u8>) -> Self {
        let hash = Self::calculate_hash(&data);
        Self { index, data, hash }
    }

    /// Calculate Blake2b hash for chunk
    fn calculate_hash(data: &[u8]) -> ContentHash {
        let mut hasher = Blake2b512::new();
        Blake2Digest::update(&mut hasher, data);
        Blake2Digest::finalize(hasher).to_vec()
    }

    /// Get chunk size
    pub fn size(&self) -> u64 {
        self.data.len() as u64
    }
}

/// Deduplication result for a complete file
#[derive(Debug, Clone)]
pub enum DeduplicationResult {
    /// File is completely new, stored at given location
    NewFile(StorageLocation),
    /// File already exists, reference to existing file
    ExistingFile(FileReference),
    /// Partial match with some new chunks and some existing chunk references
    PartialMatch {
        new_chunks: Vec<FileChunk>,
        existing_refs: Vec<ChunkReference>,
    },
}

/// File reference for existing deduplicated files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileReference {
    pub file_id: FileId,
    pub content_hash: ContentHash,
    pub storage_location: StorageLocation,
    pub size: u64,
    pub reference_count: u32,
    pub users: Vec<UserId>,
}

/// Chunk deduplication result
#[derive(Debug, Clone)]
pub struct ChunkDeduplicationResult {
    pub new_chunks: Vec<FileChunk>,
    pub existing_chunks: Vec<ChunkReference>,
    pub space_saved: u64,
    pub deduplication_ratio: f64,
}

/// Hash algorithm selection
#[derive(Debug, Clone, Copy)]
pub enum HashAlgorithm {
    Blake2b512,
    Blake2b256,
    Sha3_512,
}

impl Default for HashAlgorithm {
    fn default() -> Self {
        HashAlgorithm::Blake2b512
    }
}

/// File deduplication manager
pub struct FileDeduplication {
    storage: Arc<dyn FileStorage>,
    hash_algorithm: HashAlgorithm,
    chunk_dedup: bool,
    file_index: Arc<RwLock<HashMap<ContentHash, FileReference>>>,
    chunk_index: Arc<RwLock<HashMap<ContentHash, ChunkReference>>>,
    stats: Arc<RwLock<DeduplicationStats>>,
}

/// Deduplication statistics
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeduplicationStats {
    pub total_files_processed: u64,
    pub duplicate_files_found: u64,
    pub total_chunks_processed: u64,
    pub duplicate_chunks_found: u64,
    pub space_saved_bytes: u64,
    pub deduplication_ratio: f64,
}

impl FileDeduplication {
    /// Create a new file deduplication manager
    pub fn new(storage: Arc<dyn FileStorage>, hash_algorithm: HashAlgorithm, chunk_dedup: bool) -> Self {
        Self {
            storage,
            hash_algorithm,
            chunk_dedup,
            file_index: Arc::new(RwLock::new(HashMap::new())),
            chunk_index: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(DeduplicationStats::default())),
        }
    }

    /// Perform content-based deduplication for a file
    pub async fn deduplicate_file(&self, content: &[u8], user_id: &UserId) -> Result<DeduplicationResult> {
        let content_hash = self.calculate_file_hash(content);
        
        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.total_files_processed += 1;
        }

        // Check if file already exists
        {
            let file_index = self.file_index.read().await;
            if let Some(existing_ref) = file_index.get(&content_hash) {
                // File already exists, increment reference count
                let mut updated_ref = existing_ref.clone();
                updated_ref.reference_count += 1;
                if !updated_ref.users.contains(user_id) {
                    updated_ref.users.push(user_id.clone());
                }

                // Update index
                drop(file_index);
                {
                    let mut file_index = self.file_index.write().await;
                    file_index.insert(content_hash.clone(), updated_ref.clone());
                }

                // Update stats
                {
                    let mut stats = self.stats.write().await;
                    stats.duplicate_files_found += 1;
                    stats.space_saved_bytes += content.len() as u64;
                    stats.deduplication_ratio = 
                        stats.space_saved_bytes as f64 / 
                        (stats.total_files_processed * content.len() as u64) as f64;
                }

                return Ok(DeduplicationResult::ExistingFile(updated_ref));
            }
        }

        // File is new, store it
        let file_id = FileId::new_v4();
        let storage_location = self.storage.store_file(file_id, content).await?;
        
        // Create file reference
        let file_ref = FileReference {
            file_id,
            content_hash: content_hash.clone(),
            storage_location: storage_location.clone(),
            size: content.len() as u64,
            reference_count: 1,
            users: vec![user_id.clone()],
        };

        // Update index
        {
            let mut file_index = self.file_index.write().await;
            file_index.insert(content_hash, file_ref);
        }

        Ok(DeduplicationResult::NewFile(storage_location))
    }

    /// Perform chunk-level deduplication for large files
    pub async fn deduplicate_chunks(&self, chunks: &[FileChunk]) -> Result<ChunkDeduplicationResult> {
        if !self.chunk_dedup {
            return Err(NanoError::Media("Chunk deduplication is disabled".to_string()));
        }

        let mut new_chunks = Vec::new();
        let mut existing_chunks = Vec::new();
        let mut space_saved = 0u64;

        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.total_chunks_processed += chunks.len() as u64;
        }

        for chunk in chunks {
            // Check if chunk already exists
            {
                let chunk_index = self.chunk_index.read().await;
                if let Some(existing_ref) = chunk_index.get(&chunk.hash) {
                    // Chunk exists, increment reference count
                    let mut updated_ref = existing_ref.clone();
                    updated_ref.reference_count += 1;
                    existing_chunks.push(updated_ref.clone());
                    
                    // Update index
                    drop(chunk_index);
                    {
                        let mut chunk_index = self.chunk_index.write().await;
                        chunk_index.insert(chunk.hash.clone(), updated_ref);
                    }

                    space_saved += chunk.size();

                    // Update stats
                    {
                        let mut stats = self.stats.write().await;
                        stats.duplicate_chunks_found += 1;
                    }

                    continue;
                }
            }

            // Chunk is new, need to store it
            let chunk_file_id = FileId::new_v4();
            let storage_location = self.storage.store_file(chunk_file_id, &chunk.data).await?;
            
            // Create chunk reference
            let chunk_ref = ChunkReference {
                chunk_hash: chunk.hash.clone(),
                storage_location,
                size: chunk.size(),
                reference_count: 1,
                created_at: SystemTime::now(),
            };

            // Update index
            {
                let mut chunk_index = self.chunk_index.write().await;
                chunk_index.insert(chunk.hash.clone(), chunk_ref);
            }

            new_chunks.push(chunk.clone());
        }

        let total_size = chunks.iter().map(|c| c.size()).sum::<u64>();
        let deduplication_ratio = if total_size > 0 {
            space_saved as f64 / total_size as f64
        } else {
            0.0
        };

        // Update global stats
        {
            let mut stats = self.stats.write().await;
            stats.space_saved_bytes += space_saved;
            // Recalculate global deduplication ratio
            let total_processed = stats.total_files_processed + stats.total_chunks_processed;
            if total_processed > 0 {
                stats.deduplication_ratio = stats.space_saved_bytes as f64 / 
                    (total_processed * 1024 * 1024) as f64; // Rough estimate
            }
        }

        Ok(ChunkDeduplicationResult {
            new_chunks,
            existing_chunks,
            space_saved,
            deduplication_ratio,
        })
    }

    /// Add a file reference for an existing file
    pub async fn add_file_reference(&self, content_hash: &ContentHash, user_id: &UserId) -> Result<()> {
        let mut file_index = self.file_index.write().await;
        
        if let Some(file_ref) = file_index.get_mut(content_hash) {
            file_ref.reference_count += 1;
            if !file_ref.users.contains(user_id) {
                file_ref.users.push(user_id.clone());
            }
            Ok(())
        } else {
            Err(NanoError::Media("File not found in deduplication index".to_string()))
        }
    }

    /// Remove a file reference and clean up if no more references
    pub async fn remove_file_reference(&self, content_hash: &ContentHash, user_id: &UserId) -> Result<bool> {
        let mut file_index = self.file_index.write().await;
        
        if let Some(file_ref) = file_index.get_mut(content_hash) {
            // Remove user from list
            file_ref.users.retain(|u| u != user_id);
            
            if file_ref.reference_count > 0 {
                file_ref.reference_count -= 1;
            }

            // If no more references, delete the file
            if file_ref.reference_count == 0 || file_ref.users.is_empty() {
                let storage_location = file_ref.storage_location.clone();
                file_index.remove(content_hash);
                
                // Delete from storage
                drop(file_index);
                self.storage.delete_file(&storage_location).await?;
                Ok(true) // File was deleted
            } else {
                Ok(false) // File still has references
            }
        } else {
            Err(NanoError::Media("File not found in deduplication index".to_string()))
        }
    }

    /// Get deduplication statistics
    pub async fn get_stats(&self) -> DeduplicationStats {
        let stats = self.stats.read().await;
        stats.clone()
    }

    /// Perform garbage collection to clean up unreferenced chunks
    pub async fn garbage_collect(&self) -> Result<GarbageCollectionResult> {
        let mut chunks_deleted = 0u64;
        let mut bytes_freed = 0u64;

        // Clean up chunks with zero references
        {
            let mut chunk_index = self.chunk_index.write().await;
            let mut to_remove = Vec::new();

            for (hash, chunk_ref) in chunk_index.iter() {
                if chunk_ref.reference_count == 0 {
                    to_remove.push((hash.clone(), chunk_ref.storage_location.clone(), chunk_ref.size));
                }
            }

            for (hash, storage_location, size) in to_remove {
                chunk_index.remove(&hash);
                
                // Delete from storage
                if let Err(e) = self.storage.delete_file(&storage_location).await {
                    log::warn!("Failed to delete chunk during garbage collection: {}", e);
                } else {
                    chunks_deleted += 1;
                    bytes_freed += size;
                }
            }
        }

        // Clean up files with zero references
        {
            let mut file_index = self.file_index.write().await;
            let mut to_remove = Vec::new();

            for (hash, file_ref) in file_index.iter() {
                if file_ref.reference_count == 0 || file_ref.users.is_empty() {
                    to_remove.push((hash.clone(), file_ref.storage_location.clone(), file_ref.size));
                }
            }

            for (hash, storage_location, size) in to_remove {
                file_index.remove(&hash);
                
                // Delete from storage
                if let Err(e) = self.storage.delete_file(&storage_location).await {
                    log::warn!("Failed to delete file during garbage collection: {}", e);
                } else {
                    bytes_freed += size;
                }
            }
        }

        Ok(GarbageCollectionResult {
            chunks_deleted,
            bytes_freed,
        })
    }

    /// Calculate hash for file content
    fn calculate_file_hash(&self, content: &[u8]) -> ContentHash {
        match self.hash_algorithm {
            HashAlgorithm::Blake2b512 => {
                let mut hasher = Blake2b512::new();
                Blake2Digest::update(&mut hasher, content);
                Blake2Digest::finalize(hasher).to_vec()
            }
            HashAlgorithm::Blake2b256 => {
                let mut hasher = Blake2s256::new();
                Blake2Digest::update(&mut hasher, content);
                Blake2Digest::finalize(hasher).to_vec()
            }
            HashAlgorithm::Sha3_512 => {
                let mut hasher = Sha3_512::new();
                Digest::update(&mut hasher, content);
                Digest::finalize(hasher).to_vec()
            }
        }
    }

    /// Find similar files based on content similarity
    pub async fn find_similar_files(&self, content_hash: &ContentHash, _similarity_threshold: f64) -> Vec<FileReference> {
        // This would implement fuzzy hashing for similarity detection
        // For now, we'll return exact matches only
        let file_index = self.file_index.read().await;
        
        if let Some(file_ref) = file_index.get(content_hash) {
            vec![file_ref.clone()]
        } else {
            Vec::new()
        }
    }

    /// Get storage efficiency metrics
    pub async fn get_efficiency_metrics(&self) -> EfficiencyMetrics {
        let stats = self.stats.read().await;
        let file_index = self.file_index.read().await;
        let chunk_index = self.chunk_index.read().await;

        let total_unique_files = file_index.len() as u64;
        let total_unique_chunks = chunk_index.len() as u64;
        
        let total_file_refs = file_index.values()
            .map(|f| f.reference_count as u64)
            .sum::<u64>();
        
        let total_chunk_refs = chunk_index.values()
            .map(|c| c.reference_count as u64)
            .sum::<u64>();

        let storage_efficiency = if stats.total_files_processed > 0 {
            total_unique_files as f64 / stats.total_files_processed as f64
        } else {
            1.0
        };

        EfficiencyMetrics {
            storage_efficiency,
            total_unique_files,
            total_file_references: total_file_refs,
            total_unique_chunks,
            total_chunk_references: total_chunk_refs,
            space_saved_percentage: stats.deduplication_ratio * 100.0,
            space_saved_bytes: stats.space_saved_bytes,
        }
    }
}

/// Garbage collection result
#[derive(Debug, Clone, Serialize)]
pub struct GarbageCollectionResult {
    pub chunks_deleted: u64,
    pub bytes_freed: u64,
}

/// Storage efficiency metrics
#[derive(Debug, Clone, Serialize)]
pub struct EfficiencyMetrics {
    pub storage_efficiency: f64,
    pub total_unique_files: u64,
    pub total_file_references: u64,
    pub total_unique_chunks: u64,
    pub total_chunk_references: u64,
    pub space_saved_percentage: f64,
    pub space_saved_bytes: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::media::storage::LocalFileStorage;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_file_deduplication() {
        let temp_dir = TempDir::new().unwrap();
        let storage = LocalFileStorage::new(temp_dir.path().to_path_buf()).await.unwrap();
        let storage_arc = Arc::new(storage) as Arc<dyn FileStorage>;
        
        let dedup = FileDeduplication::new(storage_arc, HashAlgorithm::Blake2b512, true);
        
        let content = b"test file content for deduplication";
        let user1 = "user1".to_string();
        let user2 = "user2".to_string();

        // First upload should create new file
        let result1 = dedup.deduplicate_file(content, &user1).await.unwrap();
        assert!(matches!(result1, DeduplicationResult::NewFile(_)));

        // Second upload of same content should find existing file
        let result2 = dedup.deduplicate_file(content, &user2).await.unwrap();
        assert!(matches!(result2, DeduplicationResult::ExistingFile(_)));

        let stats = dedup.get_stats().await;
        assert_eq!(stats.total_files_processed, 2);
        assert_eq!(stats.duplicate_files_found, 1);
        assert!(stats.space_saved_bytes > 0);
    }

    #[tokio::test]
    async fn test_chunk_deduplication() {
        let temp_dir = TempDir::new().unwrap();
        let storage = LocalFileStorage::new(temp_dir.path().to_path_buf()).await.unwrap();
        let storage_arc = Arc::new(storage) as Arc<dyn FileStorage>;
        
        let dedup = FileDeduplication::new(storage_arc, HashAlgorithm::Blake2b512, true);

        let chunk1 = FileChunk::new(0, vec![1, 2, 3, 4, 5]);
        let chunk2 = FileChunk::new(1, vec![1, 2, 3, 4, 5]); // Same content
        let chunk3 = FileChunk::new(2, vec![6, 7, 8, 9, 10]); // Different content

        let chunks = vec![chunk1, chunk2, chunk3];
        let result = dedup.deduplicate_chunks(&chunks).await.unwrap();

        assert_eq!(result.new_chunks.len(), 2); // chunk1 and chunk3
        assert_eq!(result.existing_chunks.len(), 1); // chunk2 matches chunk1
        assert!(result.space_saved > 0);
        assert!(result.deduplication_ratio > 0.0);
    }

    #[test]
    fn test_file_chunk_creation() {
        let data = vec![1, 2, 3, 4, 5];
        let chunk = FileChunk::new(0, data.clone());
        
        assert_eq!(chunk.index, 0);
        assert_eq!(chunk.data, data);
        assert_eq!(chunk.size(), 5);
        assert_eq!(chunk.hash.len(), 64); // Blake2b512 produces 64-byte hash
    }

    #[tokio::test]
    async fn test_garbage_collection() {
        let temp_dir = TempDir::new().unwrap();
        let storage = LocalFileStorage::new(temp_dir.path().to_path_buf()).await.unwrap();
        let storage_arc = Arc::new(storage) as Arc<dyn FileStorage>;
        
        let dedup = FileDeduplication::new(storage_arc, HashAlgorithm::Blake2b512, true);

        // Add some files and then remove references
        let content = b"test content for gc";
        let user = "test_user".to_string();
        
        let _result = dedup.deduplicate_file(content, &user).await.unwrap();
        
        // Run garbage collection (should find nothing to clean since file is referenced)
        let gc_result = dedup.garbage_collect().await.unwrap();
        assert_eq!(gc_result.chunks_deleted, 0);
        assert_eq!(gc_result.bytes_freed, 0);
    }

    #[tokio::test]
    async fn test_efficiency_metrics() {
        let temp_dir = TempDir::new().unwrap();
        let storage = LocalFileStorage::new(temp_dir.path().to_path_buf()).await.unwrap();
        let storage_arc = Arc::new(storage) as Arc<dyn FileStorage>;
        
        let dedup = FileDeduplication::new(storage_arc, HashAlgorithm::Blake2b512, true);
        
        let metrics = dedup.get_efficiency_metrics().await;
        assert_eq!(metrics.total_unique_files, 0);
        assert_eq!(metrics.total_file_references, 0);
        assert_eq!(metrics.storage_efficiency, 1.0);
    }
}
