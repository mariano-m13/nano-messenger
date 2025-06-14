/// File Metadata Management
/// 
/// Handles file metadata storage, retrieval, and management
/// including permissions, access control, and file lifecycle

use crate::error::{NanoError, Result};
pub use crate::media::storage::{FileId, StorageLocation};
use crate::media::encryption::EncryptionMetadata;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;
use tokio::fs;
use tokio::sync::RwLock;
use uuid::Uuid;

/// User identifier type
pub type UserId = String;

/// File metadata containing all information about a stored file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub file_id: FileId,
    pub original_name: String,
    pub mime_type: String,
    pub file_size: u64,
    pub upload_timestamp: SystemTime,
    pub last_accessed: SystemTime,
    pub uploader_id: UserId,
    pub encryption_info: EncryptionMetadata,
    pub storage_location: StorageLocation,
    pub access_permissions: FilePermissions,
    pub tags: Vec<String>,
    pub description: Option<String>,
    pub checksum: String,
    pub version: u32,
    pub is_deleted: bool,
    pub deletion_timestamp: Option<SystemTime>,
    pub expiry_timestamp: Option<SystemTime>,
    pub download_count: u64,
    pub custom_metadata: HashMap<String, String>,
}

impl Default for FileMetadata {
    fn default() -> Self {
        use crate::media::encryption::EncryptionMetadata;
        use crate::media::storage::StorageLocation;
        
        Self {
            file_id: uuid::Uuid::new_v4(),
            original_name: String::new(),
            mime_type: String::new(),
            file_size: 0,
            upload_timestamp: SystemTime::now(),
            last_accessed: SystemTime::now(),
            uploader_id: String::new(),
            encryption_info: EncryptionMetadata::default(),
            storage_location: StorageLocation::new("local".to_string(), String::new()),
            access_permissions: FilePermissions::default(),
            tags: Vec::new(),
            description: None,
            checksum: String::new(),
            version: 1,
            is_deleted: false,
            deletion_timestamp: None,
            expiry_timestamp: None,
            download_count: 0,
            custom_metadata: HashMap::new(),
        }
    }
}

impl FileMetadata {
    /// Create new file metadata
    pub fn new(
        file_id: FileId,
        original_name: String,
        mime_type: String,
        file_size: u64,
        uploader_id: UserId,
        encryption_info: EncryptionMetadata,
        storage_location: StorageLocation,
        checksum: String,
    ) -> Self {
        let now = SystemTime::now();
        
        Self {
            file_id,
            original_name,
            mime_type,
            file_size,
            upload_timestamp: now,
            last_accessed: now,
            uploader_id,
            encryption_info,
            storage_location,
            access_permissions: FilePermissions::default(),
            tags: Vec::new(),
            description: None,
            checksum,
            version: 1,
            is_deleted: false,
            deletion_timestamp: None,
            expiry_timestamp: None,
            download_count: 0,
            custom_metadata: HashMap::new(),
        }
    }

    /// Update last accessed timestamp
    pub fn mark_accessed(&mut self) {
        self.last_accessed = SystemTime::now();
    }

    /// Increment download count
    pub fn increment_download_count(&mut self) {
        self.download_count += 1;
        self.mark_accessed();
    }

    /// Mark file as deleted
    pub fn mark_deleted(&mut self) {
        self.is_deleted = true;
        self.deletion_timestamp = Some(SystemTime::now());
    }

    /// Check if file has expired
    pub fn is_expired(&self) -> bool {
        if let Some(expiry) = self.expiry_timestamp {
            return SystemTime::now() > expiry;
        }
        false
    }

    /// Get file age in seconds
    pub fn age_seconds(&self) -> u64 {
        self.upload_timestamp
            .elapsed()
            .unwrap_or_default()
            .as_secs()
    }

    /// Get time since last access in seconds
    pub fn seconds_since_last_access(&self) -> u64 {
        self.last_accessed
            .elapsed()
            .unwrap_or_default()
            .as_secs()
    }

    /// Check if user has specific permission
    pub fn user_has_permission(&self, user_id: &UserId, permission: FilePermission) -> bool {
        // Owner always has all permissions
        if *user_id == self.uploader_id {
            return true;
        }

        self.access_permissions.user_has_permission(user_id, permission)
    }

    /// Add a tag to the file
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    /// Remove a tag from the file
    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
    }

    /// Set custom metadata
    pub fn set_custom_metadata(&mut self, key: String, value: String) {
        self.custom_metadata.insert(key, value);
    }

    /// Get custom metadata
    pub fn get_custom_metadata(&self, key: &str) -> Option<&String> {
        self.custom_metadata.get(key)
    }
}

/// File access permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilePermissions {
    pub is_public: bool,
    pub allowed_users: Vec<UserId>,
    pub allowed_groups: Vec<String>,
    pub permissions: HashMap<UserId, Vec<FilePermission>>,
    pub default_permissions: Vec<FilePermission>,
    pub require_authentication: bool,
    pub max_downloads: Option<u64>,
    pub download_expiry: Option<SystemTime>,
}

impl Default for FilePermissions {
    fn default() -> Self {
        Self {
            is_public: false,
            allowed_users: Vec::new(),
            allowed_groups: Vec::new(),
            permissions: HashMap::new(),
            default_permissions: vec![FilePermission::Read],
            require_authentication: true,
            max_downloads: None,
            download_expiry: None,
        }
    }
}

impl FilePermissions {
    /// Check if user has specific permission
    pub fn user_has_permission(&self, user_id: &UserId, permission: FilePermission) -> bool {
        // Check if user is explicitly allowed
        if !self.allowed_users.contains(user_id) && !self.is_public {
            return false;
        }

        // Check specific user permissions
        if let Some(user_perms) = self.permissions.get(user_id) {
            return user_perms.contains(&permission);
        }

        // Fall back to default permissions
        self.default_permissions.contains(&permission)
    }

    /// Grant permission to a user
    pub fn grant_permission(&mut self, user_id: UserId, permission: FilePermission) {
        self.allowed_users.push(user_id.clone());
        self.permissions
            .entry(user_id)
            .or_insert_with(Vec::new)
            .push(permission);
    }

    /// Revoke permission from a user
    pub fn revoke_permission(&mut self, user_id: &UserId, permission: FilePermission) {
        if let Some(user_perms) = self.permissions.get_mut(user_id) {
            user_perms.retain(|p| *p != permission);
        }
    }

    /// Set permissions for a user
    pub fn set_user_permissions(&mut self, user_id: UserId, permissions: Vec<FilePermission>) {
        if !self.allowed_users.contains(&user_id) {
            self.allowed_users.push(user_id.clone());
        }
        self.permissions.insert(user_id, permissions);
    }

    /// Remove user access entirely
    pub fn remove_user_access(&mut self, user_id: &UserId) {
        self.allowed_users.retain(|u| u != user_id);
        self.permissions.remove(user_id);
    }
}

/// File permission types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FilePermission {
    Read,
    Download,
    Share,
    Delete,
    Modify,
    ViewMetadata,
    ChangePermissions,
}

/// File reference for sharing and linking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileReference {
    pub file_id: FileId,
    pub reference_id: Uuid,
    pub creator_id: UserId,
    pub created_at: SystemTime,
    pub expires_at: Option<SystemTime>,
    pub access_permissions: FilePermissions,
    pub max_uses: Option<u64>,
    pub current_uses: u64,
    pub description: Option<String>,
    pub is_active: bool,
}

impl FileReference {
    /// Create a new file reference
    pub fn new(
        file_id: FileId,
        creator_id: UserId,
        expires_at: Option<SystemTime>,
        max_uses: Option<u64>,
    ) -> Self {
        Self {
            file_id,
            reference_id: Uuid::new_v4(),
            creator_id,
            created_at: SystemTime::now(),
            expires_at,
            access_permissions: FilePermissions::default(),
            max_uses,
            current_uses: 0,
            description: None,
            is_active: true,
        }
    }

    /// Check if reference is valid for use
    pub fn is_valid(&self) -> bool {
        if !self.is_active {
            return false;
        }

        // Check expiry
        if let Some(expires_at) = self.expires_at {
            if SystemTime::now() > expires_at {
                return false;
            }
        }

        // Check usage limits
        if let Some(max_uses) = self.max_uses {
            if self.current_uses >= max_uses {
                return false;
            }
        }

        true
    }

    /// Use the reference (increment use count)
    pub fn use_reference(&mut self) -> Result<()> {
        if !self.is_valid() {
            return Err(NanoError::Media("File reference is no longer valid".to_string()));
        }

        self.current_uses += 1;
        Ok(())
    }

    /// Deactivate the reference
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
}

/// Metadata store for managing file metadata
pub struct MetadataStore {
    metadata_db: RwLock<HashMap<FileId, FileMetadata>>,
    references_db: RwLock<HashMap<Uuid, FileReference>>,
    user_files_index: RwLock<HashMap<UserId, Vec<FileId>>>,
    persistence_path: Option<PathBuf>,
}

impl MetadataStore {
    /// Create a new metadata store
    pub async fn new() -> Result<Self> {
        Ok(Self {
            metadata_db: RwLock::new(HashMap::new()),
            references_db: RwLock::new(HashMap::new()),
            user_files_index: RwLock::new(HashMap::new()),
            persistence_path: None,
        })
    }

    /// Create a metadata store with persistence
    pub async fn with_persistence(persistence_path: PathBuf) -> Result<Self> {
        let mut store = Self::new().await?;
        store.persistence_path = Some(persistence_path);
        
        // Load existing data if available
        store.load_from_disk().await?;
        
        Ok(store)
    }

    /// Store file metadata
    pub async fn store_metadata(&self, metadata: FileMetadata) -> Result<()> {
        let file_id = metadata.file_id;
        let user_id = metadata.uploader_id.clone();

        // Store metadata
        {
            let mut db = self.metadata_db.write().await;
            db.insert(file_id, metadata);
        }

        // Update user files index
        {
            let mut index = self.user_files_index.write().await;
            index.entry(user_id).or_insert_with(Vec::new).push(file_id);
        }

        self.persist_to_disk().await?;
        Ok(())
    }

    /// Retrieve file metadata
    pub async fn get_metadata(&self, file_id: &FileId) -> Result<Option<FileMetadata>> {
        let db = self.metadata_db.read().await;
        Ok(db.get(file_id).cloned())
    }

    /// Update file metadata
    pub async fn update_metadata(&self, file_id: &FileId, metadata: FileMetadata) -> Result<()> {
        {
            let mut db = self.metadata_db.write().await;
            if db.contains_key(file_id) {
                db.insert(*file_id, metadata);
            } else {
                return Err(NanoError::Media("File not found".to_string()));
            }
        }

        self.persist_to_disk().await?;
        Ok(())
    }

    /// Delete file metadata
    pub async fn delete_metadata(&self, file_id: &FileId) -> Result<()> {
        let user_id = {
            let mut db = self.metadata_db.write().await;
            if let Some(metadata) = db.remove(file_id) {
                metadata.uploader_id
            } else {
                return Err(NanoError::Media("File not found".to_string()));
            }
        };

        // Update user files index
        {
            let mut index = self.user_files_index.write().await;
            if let Some(user_files) = index.get_mut(&user_id) {
                user_files.retain(|id| id != file_id);
            }
        }

        self.persist_to_disk().await?;
        Ok(())
    }

    /// Get files for a user
    pub async fn get_user_files(&self, user_id: &UserId) -> Result<Vec<FileMetadata>> {
        let file_ids = {
            let index = self.user_files_index.read().await;
            index.get(user_id).cloned().unwrap_or_default()
        };

        let mut files = Vec::new();
        {
            let db = self.metadata_db.read().await;
            for file_id in file_ids {
                if let Some(metadata) = db.get(&file_id) {
                    if !metadata.is_deleted {
                        files.push(metadata.clone());
                    }
                }
            }
        }

        Ok(files)
    }

    /// Search files by criteria
    pub async fn search_files(&self, criteria: &SearchCriteria) -> Result<Vec<FileMetadata>> {
        let db = self.metadata_db.read().await;
        let mut results = Vec::new();

        for (_, metadata) in db.iter() {
            if metadata.is_deleted {
                continue;
            }

            if self.matches_criteria(metadata, criteria) {
                results.push(metadata.clone());
            }
        }

        // Sort results
        match criteria.sort_by {
            SortBy::UploadDate => {
                results.sort_by(|a, b| b.upload_timestamp.cmp(&a.upload_timestamp));
            }
            SortBy::Size => {
                results.sort_by(|a, b| b.file_size.cmp(&a.file_size));
            }
            SortBy::Name => {
                results.sort_by(|a, b| a.original_name.cmp(&b.original_name));
            }
            SortBy::LastAccessed => {
                results.sort_by(|a, b| b.last_accessed.cmp(&a.last_accessed));
            }
        }

        // Apply pagination
        let start = criteria.offset.unwrap_or(0) as usize;
        let end = start + criteria.limit.unwrap_or(results.len() as u64) as usize;
        
        if start < results.len() {
            results = results[start..end.min(results.len())].to_vec();
        } else {
            results.clear();
        }

        Ok(results)
    }

    /// Create a file reference
    pub async fn create_reference(&self, reference: FileReference) -> Result<Uuid> {
        let reference_id = reference.reference_id;
        
        {
            let mut db = self.references_db.write().await;
            db.insert(reference_id, reference);
        }

        self.persist_to_disk().await?;
        Ok(reference_id)
    }

    /// Get file reference
    pub async fn get_reference(&self, reference_id: &Uuid) -> Result<Option<FileReference>> {
        let db = self.references_db.read().await;
        Ok(db.get(reference_id).cloned())
    }

    /// Use a file reference
    pub async fn use_reference(&self, reference_id: &Uuid) -> Result<FileId> {
        {
            let mut db = self.references_db.write().await;
            if let Some(reference) = db.get_mut(reference_id) {
                reference.use_reference()?;
                let file_id = reference.file_id;
                
                // Mark file as accessed
                drop(db);
                if let Ok(Some(mut metadata)) = self.get_metadata(&file_id).await {
                    metadata.mark_accessed();
                    self.update_metadata(&file_id, metadata).await?;
                }
                
                return Ok(file_id);
            }
        }

        Err(NanoError::Media("File reference not found".to_string()))
    }

    /// Get statistics
    pub async fn get_statistics(&self) -> Result<MetadataStatistics> {
        let db = self.metadata_db.read().await;
        let references_db = self.references_db.read().await;
        
        let mut total_files = 0;
        let mut total_size = 0;
        let mut active_files = 0;
        let mut deleted_files = 0;
        let mut mime_type_counts = HashMap::new();

        for (_, metadata) in db.iter() {
            total_files += 1;
            total_size += metadata.file_size;

            if metadata.is_deleted {
                deleted_files += 1;
            } else {
                active_files += 1;
            }

            *mime_type_counts.entry(metadata.mime_type.clone()).or_insert(0) += 1;
        }

        Ok(MetadataStatistics {
            total_files,
            active_files,
            deleted_files,
            total_size_bytes: total_size,
            total_references: references_db.len() as u64,
            mime_type_distribution: mime_type_counts,
        })
    }

    /// Clean up expired data
    pub async fn cleanup_expired(&self) -> Result<u64> {
        let mut cleaned_count = 0;

        // Clean up expired files
        {
            let mut db = self.metadata_db.write().await;
            let expired_files: Vec<FileId> = db
                .iter()
                .filter(|(_, metadata)| metadata.is_expired())
                .map(|(id, _)| *id)
                .collect();

            for file_id in expired_files {
                if let Some(metadata) = db.get_mut(&file_id) {
                    metadata.mark_deleted();
                    cleaned_count += 1;
                }
            }
        }

        // Clean up expired references
        {
            let mut references_db = self.references_db.write().await;
            let expired_refs: Vec<Uuid> = references_db
                .iter()
                .filter(|(_, reference)| !reference.is_valid())
                .map(|(id, _)| *id)
                .collect();

            for ref_id in expired_refs {
                references_db.remove(&ref_id);
                cleaned_count += 1;
            }
        }

        if cleaned_count > 0 {
            self.persist_to_disk().await?;
        }

        Ok(cleaned_count)
    }

    /// Check if criteria matches metadata
    fn matches_criteria(&self, metadata: &FileMetadata, criteria: &SearchCriteria) -> bool {
        // Check user filter
        if let Some(ref user_id) = criteria.user_id {
            if metadata.uploader_id != *user_id {
                return false;
            }
        }

        // Check MIME type filter
        if let Some(ref mime_type) = criteria.mime_type {
            if !metadata.mime_type.starts_with(mime_type) {
                return false;
            }
        }

        // Check tags filter
        if !criteria.tags.is_empty() {
            let has_any_tag = criteria.tags.iter().any(|tag| metadata.tags.contains(tag));
            if !has_any_tag {
                return false;
            }
        }

        // Check filename filter
        if let Some(ref filename_pattern) = criteria.filename_pattern {
            if !metadata.original_name.contains(filename_pattern) {
                return false;
            }
        }

        // Check size range
        if let Some(min_size) = criteria.min_size {
            if metadata.file_size < min_size {
                return false;
            }
        }
        if let Some(max_size) = criteria.max_size {
            if metadata.file_size > max_size {
                return false;
            }
        }

        // Check date range
        if let Some(after) = criteria.uploaded_after {
            if metadata.upload_timestamp < after {
                return false;
            }
        }
        if let Some(before) = criteria.uploaded_before {
            if metadata.upload_timestamp > before {
                return false;
            }
        }

        true
    }

    /// Load metadata from disk
    async fn load_from_disk(&self) -> Result<()> {
        if let Some(ref path) = self.persistence_path {
            if path.exists() {
                let data = fs::read_to_string(path).await.map_err(|e| {
                    NanoError::Storage(format!("Failed to read metadata file: {}", e))
                })?;

                let stored_data: StoredMetadata = serde_json::from_str(&data).map_err(|e| {
                    NanoError::Storage(format!("Failed to parse metadata file: {}", e))
                })?;

                {
                    let mut db = self.metadata_db.write().await;
                    *db = stored_data.metadata;
                }

                {
                    let mut references_db = self.references_db.write().await;
                    *references_db = stored_data.references;
                }

                {
                    let mut index = self.user_files_index.write().await;
                    *index = stored_data.user_files_index;
                }
            }
        }
        Ok(())
    }

    /// Persist metadata to disk
    async fn persist_to_disk(&self) -> Result<()> {
        if let Some(ref path) = self.persistence_path {
            let stored_data = {
                let db = self.metadata_db.read().await;
                let references_db = self.references_db.read().await;
                let index = self.user_files_index.read().await;

                StoredMetadata {
                    metadata: db.clone(),
                    references: references_db.clone(),
                    user_files_index: index.clone(),
                }
            };

            let json = serde_json::to_string_pretty(&stored_data).map_err(|e| {
                NanoError::Storage(format!("Failed to serialize metadata: {}", e))
            })?;

            // Ensure parent directory exists
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).await.map_err(|e| {
                    NanoError::Storage(format!("Failed to create metadata directory: {}", e))
                })?;
            }

            fs::write(path, json).await.map_err(|e| {
                NanoError::Storage(format!("Failed to write metadata file: {}", e))
            })?;
        }
        Ok(())
    }
}

/// Search criteria for file queries
#[derive(Debug, Clone)]
pub struct SearchCriteria {
    pub user_id: Option<UserId>,
    pub mime_type: Option<String>,
    pub tags: Vec<String>,
    pub filename_pattern: Option<String>,
    pub min_size: Option<u64>,
    pub max_size: Option<u64>,
    pub uploaded_after: Option<SystemTime>,
    pub uploaded_before: Option<SystemTime>,
    pub sort_by: SortBy,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

impl Default for SearchCriteria {
    fn default() -> Self {
        Self {
            user_id: None,
            mime_type: None,
            tags: Vec::new(),
            filename_pattern: None,
            min_size: None,
            max_size: None,
            uploaded_after: None,
            uploaded_before: None,
            sort_by: SortBy::UploadDate,
            limit: Some(50),
            offset: None,
        }
    }
}

/// Sorting options for search results
#[derive(Debug, Clone, Copy)]
pub enum SortBy {
    UploadDate,
    Size,
    Name,
    LastAccessed,
}

/// Metadata statistics
#[derive(Debug, Clone, Serialize)]
pub struct MetadataStatistics {
    pub total_files: u64,
    pub active_files: u64,
    pub deleted_files: u64,
    pub total_size_bytes: u64,
    pub total_references: u64,
    pub mime_type_distribution: HashMap<String, u64>,
}

/// Stored metadata structure for persistence
#[derive(Serialize, Deserialize)]
struct StoredMetadata {
    metadata: HashMap<FileId, FileMetadata>,
    references: HashMap<Uuid, FileReference>,
    user_files_index: HashMap<UserId, Vec<FileId>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::media::storage::StorageLocation;
    use crate::media::encryption::EncryptionMetadata;
    use tempfile::TempDir;

    fn create_test_metadata() -> FileMetadata {
        FileMetadata::new(
            Uuid::new_v4(),
            "test.txt".to_string(),
            "text/plain".to_string(),
            1024,
            "user123".to_string(),
            EncryptionMetadata {
                algorithm: "ChaCha20Poly1305".to_string(),
                key_algorithm: "X25519".to_string(),
                nonce_size: 12,
                tag_size: 16,
                original_size: 1024,
                encrypted_size: 1040,
                compression_used: false,
                custom_params: HashMap::new(),
            },
            StorageLocation::new("local".to_string(), "/path/to/file".to_string()),
            "checksum123".to_string(),
        )
    }

    #[tokio::test]
    async fn test_metadata_store_basic_operations() {
        let store = MetadataStore::new().await.unwrap();
        let metadata = create_test_metadata();
        let file_id = metadata.file_id;

        // Store metadata
        store.store_metadata(metadata.clone()).await.unwrap();

        // Retrieve metadata
        let retrieved = store.get_metadata(&file_id).await.unwrap().unwrap();
        assert_eq!(retrieved.file_id, file_id);
        assert_eq!(retrieved.original_name, "test.txt");

        // Update metadata
        let mut updated = retrieved.clone();
        updated.description = Some("Updated description".to_string());
        store.update_metadata(&file_id, updated.clone()).await.unwrap();

        let updated_retrieved = store.get_metadata(&file_id).await.unwrap().unwrap();
        assert_eq!(updated_retrieved.description, Some("Updated description".to_string()));

        // Delete metadata
        store.delete_metadata(&file_id).await.unwrap();
        assert!(store.get_metadata(&file_id).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_file_permissions() {
        let mut permissions = FilePermissions::default();
        let user_id = "user123".to_string();

        // Initially, user should not have access
        assert!(!permissions.user_has_permission(&user_id, FilePermission::Read));

        // Grant access
        permissions.grant_permission(user_id.clone(), FilePermission::Read);
        assert!(permissions.user_has_permission(&user_id, FilePermission::Read));

        // Revoke access
        permissions.revoke_permission(&user_id, FilePermission::Read);
        assert!(!permissions.user_has_permission(&user_id, FilePermission::Read));
    }

    #[tokio::test]
    async fn test_file_reference() {
        let file_id = Uuid::new_v4();
        let creator_id = "creator123".to_string();
        let mut reference = FileReference::new(file_id, creator_id, None, Some(3));

        assert!(reference.is_valid());
        assert_eq!(reference.current_uses, 0);

        // Use reference
        reference.use_reference().unwrap();
        assert_eq!(reference.current_uses, 1);
        assert!(reference.is_valid());

        // Use reference multiple times
        reference.use_reference().unwrap();
        reference.use_reference().unwrap();
        assert_eq!(reference.current_uses, 3);
        assert!(!reference.is_valid()); // Should be invalid due to usage limit

        // Should not be able to use again
        assert!(reference.use_reference().is_err());
    }

    #[tokio::test]
    async fn test_search_functionality() {
        let store = MetadataStore::new().await.unwrap();

        // Create test files
        let mut metadata1 = create_test_metadata();
        metadata1.original_name = "document.pdf".to_string();
        metadata1.mime_type = "application/pdf".to_string();
        metadata1.tags = vec!["important".to_string(), "work".to_string()];

        let mut metadata2 = create_test_metadata();
        metadata2.original_name = "image.jpg".to_string();
        metadata2.mime_type = "image/jpeg".to_string();
        metadata2.tags = vec!["personal".to_string()];

        store.store_metadata(metadata1.clone()).await.unwrap();
        store.store_metadata(metadata2.clone()).await.unwrap();

        // Search by MIME type
        let criteria = SearchCriteria {
            mime_type: Some("application/".to_string()),
            ..SearchCriteria::default()
        };
        let results = store.search_files(&criteria).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].original_name, "document.pdf");

        // Search by tags
        let criteria = SearchCriteria {
            tags: vec!["work".to_string()],
            ..SearchCriteria::default()
        };
        let results = store.search_files(&criteria).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].original_name, "document.pdf");
    }

    #[tokio::test]
    async fn test_persistence() {
        let temp_dir = TempDir::new().unwrap();
        let persistence_path = temp_dir.path().join("metadata.json");

        {
            // Create store with persistence
            let store = MetadataStore::with_persistence(persistence_path.clone()).await.unwrap();
            let metadata = create_test_metadata();
            store.store_metadata(metadata).await.unwrap();
        }

        {
            // Load from persistence
            let store = MetadataStore::with_persistence(persistence_path).await.unwrap();
            let files = store.get_user_files(&"user123".to_string()).await.unwrap();
            assert_eq!(files.len(), 1);
            assert_eq!(files[0].original_name, "test.txt");
        }
    }

    #[test]
    fn test_file_metadata_operations() {
        let mut metadata = create_test_metadata();
        let initial_access_time = metadata.last_accessed;

        // Test marking as accessed
        std::thread::sleep(std::time::Duration::from_millis(10));
        metadata.mark_accessed();
        assert!(metadata.last_accessed > initial_access_time);

        // Test download count
        assert_eq!(metadata.download_count, 0);
        metadata.increment_download_count();
        assert_eq!(metadata.download_count, 1);

        // Test tags
        metadata.add_tag("important".to_string());
        assert!(metadata.tags.contains(&"important".to_string()));
        
        metadata.remove_tag("important");
        assert!(!metadata.tags.contains(&"important".to_string()));

        // Test custom metadata
        metadata.set_custom_metadata("key1".to_string(), "value1".to_string());
        assert_eq!(metadata.get_custom_metadata("key1").unwrap(), "value1");
    }
}
