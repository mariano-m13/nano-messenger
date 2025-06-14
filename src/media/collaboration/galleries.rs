/// Shared Media Galleries
/// 
/// Provides collaborative media galleries with quantum-encrypted sharing,
/// permissions management, and real-time synchronization.

use crate::crypto::CryptoMode;
use crate::error::{NanoError, Result};
use crate::media::{
    metadata::{FileReference, UserId},
    storage::FileId,
    collaboration::interactions::MediaAnnotation,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

/// Gallery ID for tracking shared galleries
pub type GalleryId = Uuid;

/// Shared gallery key for quantum-resistant encryption
#[derive(Debug, Clone)]
pub struct SharedGalleryKey {
    pub key_data: Vec<u8>,
    pub crypto_mode: CryptoMode,
    pub participants: Vec<UserId>,
    pub created_at: SystemTime,
    pub rotated_at: SystemTime,
}

impl SharedGalleryKey {
    /// Create a new shared gallery key
    pub fn new(crypto_mode: CryptoMode, participants: Vec<UserId>) -> Self {
        use rand::RngCore;
        let mut key_data = vec![0u8; 32]; // 256-bit key
        rand::thread_rng().fill_bytes(&mut key_data);

        let now = SystemTime::now();
        Self {
            key_data,
            crypto_mode,
            participants,
            created_at: now,
            rotated_at: now,
        }
    }

    /// Rotate the gallery key for forward secrecy
    pub fn rotate(&mut self) -> Result<()> {
        use rand::RngCore;
        rand::thread_rng().fill_bytes(&mut self.key_data);
        self.rotated_at = SystemTime::now();
        Ok(())
    }

    /// Check if key needs rotation (every 30 days)
    pub fn needs_rotation(&self) -> bool {
        if let Ok(elapsed) = SystemTime::now().duration_since(self.rotated_at) {
            elapsed.as_secs() > 30 * 24 * 60 * 60 // 30 days
        } else {
            true
        }
    }
}

/// Gallery permissions for different users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalleryPermissions {
    pub can_upload: HashSet<UserId>,
    pub can_download: HashSet<UserId>,
    pub can_delete: HashSet<UserId>,
    pub can_annotate: HashSet<UserId>,
    pub can_invite: HashSet<UserId>,
    pub can_moderate: HashSet<UserId>,
}

impl GalleryPermissions {
    /// Create permissions with owner having all rights
    pub fn owner_permissions(owner: &UserId) -> Self {
        let mut permissions = Self {
            can_upload: HashSet::new(),
            can_download: HashSet::new(),
            can_delete: HashSet::new(),
            can_annotate: HashSet::new(),
            can_invite: HashSet::new(),
            can_moderate: HashSet::new(),
        };

        permissions.can_upload.insert(owner.clone());
        permissions.can_download.insert(owner.clone());
        permissions.can_delete.insert(owner.clone());
        permissions.can_annotate.insert(owner.clone());
        permissions.can_invite.insert(owner.clone());
        permissions.can_moderate.insert(owner.clone());

        permissions
    }

    /// Grant all permissions to a user
    pub fn grant_all_permissions(&mut self, user_id: &UserId) {
        self.can_upload.insert(user_id.clone());
        self.can_download.insert(user_id.clone());
        self.can_delete.insert(user_id.clone());
        self.can_annotate.insert(user_id.clone());
        self.can_invite.insert(user_id.clone());
        self.can_moderate.insert(user_id.clone());
    }

    /// Grant read-only permissions
    pub fn grant_read_only(&mut self, user_id: &UserId) {
        self.can_download.insert(user_id.clone());
    }

    /// Grant contributor permissions (read, upload, annotate)
    pub fn grant_contributor(&mut self, user_id: &UserId) {
        self.can_upload.insert(user_id.clone());
        self.can_download.insert(user_id.clone());
        self.can_annotate.insert(user_id.clone());
    }

    /// Check if user has specific permission
    pub fn can_upload(&self, user_id: &UserId) -> bool {
        self.can_upload.contains(user_id)
    }

    pub fn can_download(&self, user_id: &UserId) -> bool {
        self.can_download.contains(user_id)
    }

    pub fn can_delete(&self, user_id: &UserId) -> bool {
        self.can_delete.contains(user_id)
    }

    pub fn can_annotate(&self, user_id: &UserId) -> bool {
        self.can_annotate.contains(user_id)
    }

    pub fn can_invite(&self, user_id: &UserId) -> bool {
        self.can_invite.contains(user_id)
    }

    pub fn can_moderate(&self, user_id: &UserId) -> bool {
        self.can_moderate.contains(user_id)
    }
}

/// Gallery media item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalleryItem {
    pub file_id: FileId,
    pub file_reference: FileReference,
    pub uploader: UserId,
    pub uploaded_at: SystemTime,
    pub title: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub annotations: Vec<MediaAnnotation>,
    pub view_count: u64,
    pub download_count: u64,
}

impl GalleryItem {
    /// Create a new gallery item
    pub fn new(file_reference: FileReference, uploader: UserId) -> Self {
        Self {
            file_id: file_reference.file_id,
            file_reference,
            uploader,
            uploaded_at: SystemTime::now(),
            title: None,
            description: None,
            tags: Vec::new(),
            annotations: Vec::new(),
            view_count: 0,
            download_count: 0,
        }
    }

    /// Add an annotation
    pub fn add_annotation(&mut self, annotation: MediaAnnotation) {
        self.annotations.push(annotation);
    }

    /// Increment view count
    pub fn increment_views(&mut self) {
        self.view_count += 1;
    }

    /// Increment download count
    pub fn increment_downloads(&mut self) {
        self.download_count += 1;
    }
}

/// Shared media gallery
pub struct SharedGallery {
    pub gallery_id: GalleryId,
    pub title: String,
    pub description: Option<String>,
    pub owner: UserId,
    pub participants: HashSet<UserId>,
    pub permissions: GalleryPermissions,
    pub encryption_key: SharedGalleryKey,
    pub items: Arc<RwLock<HashMap<FileId, GalleryItem>>>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub event_broadcaster: broadcast::Sender<GalleryEvent>,
    pub is_public: bool,
    pub max_items: Option<u32>,
    pub max_size_bytes: Option<u64>,
}

impl SharedGallery {
    /// Create a new shared gallery
    pub async fn create_shared_gallery(
        title: String,
        owner: UserId,
        participants: &[UserId],
        is_public: bool,
    ) -> Result<Self> {
        let gallery_id = Uuid::new_v4();
        let mut all_participants = participants.to_vec();
        all_participants.push(owner.clone());

        let encryption_key = SharedGalleryKey::new(CryptoMode::Hybrid, all_participants.clone());
        let permissions = GalleryPermissions::owner_permissions(&owner);
        let (event_broadcaster, _) = broadcast::channel(1000);

        let now = SystemTime::now();
        Ok(Self {
            gallery_id,
            title,
            description: None,
            owner,
            participants: all_participants.into_iter().collect(),
            permissions,
            encryption_key,
            items: Arc::new(RwLock::new(HashMap::new())),
            created_at: now,
            updated_at: now,
            event_broadcaster,
            is_public,
            max_items: None,
            max_size_bytes: None,
        })
    }

    /// Add media to the gallery
    pub async fn add_media(&mut self, file_ref: FileReference, uploader: &UserId) -> Result<()> {
        // Check permissions
        if !self.permissions.can_upload(uploader) {
            return Err(NanoError::Media("User not authorized to upload to this gallery".to_string()));
        }

        // Check limits
        let items = self.items.read().await;
        if let Some(max_items) = self.max_items {
            if items.len() >= max_items as usize {
                return Err(NanoError::Media("Gallery item limit reached".to_string()));
            }
        }

        // Check size limit
        if let Some(max_size) = self.max_size_bytes {
            let current_size: u64 = items.values()
                .map(|_item| {
                    // Since FileReference doesn't have file_size, we'll use a placeholder
                    // In a real implementation, this would query the metadata store
                    1024u64 // Placeholder size
                })
                .sum();
            let new_file_size = 1024u64; // Placeholder for new file size
            
            if current_size + new_file_size > max_size {
                return Err(NanoError::Media("Gallery size limit would be exceeded".to_string()));
            }
        }

        drop(items);

        // Create gallery item
        let gallery_item = GalleryItem::new(file_ref.clone(), uploader.clone());
        let file_id = gallery_item.file_id;

        // Add to gallery
        {
            let mut items = self.items.write().await;
            items.insert(file_id, gallery_item);
        }

        // Update gallery timestamp
        self.updated_at = SystemTime::now();

        // Broadcast event
        let event = GalleryEvent::MediaAdded {
            gallery_id: self.gallery_id,
            file_id,
            uploader: uploader.clone(),
            timestamp: SystemTime::now(),
        };

        let _ = self.event_broadcaster.send(event);

        Ok(())
    }

    /// Remove media from the gallery
    pub async fn remove_media(&mut self, file_id: &FileId, remover: &UserId) -> Result<()> {
        // Check permissions
        let can_delete = {
            let items = self.items.read().await;
            if let Some(item) = items.get(file_id) {
                // Owner can delete their own items, or users with delete permission
                item.uploader == *remover || self.permissions.can_delete(remover)
            } else {
                return Err(NanoError::Media("Media item not found in gallery".to_string()));
            }
        };

        if !can_delete {
            return Err(NanoError::Media("User not authorized to delete this media".to_string()));
        }

        // Remove from gallery
        {
            let mut items = self.items.write().await;
            items.remove(file_id);
        }

        // Update gallery timestamp
        self.updated_at = SystemTime::now();

        // Broadcast event
        let event = GalleryEvent::MediaRemoved {
            gallery_id: self.gallery_id,
            file_id: *file_id,
            remover: remover.clone(),
            timestamp: SystemTime::now(),
        };

        let _ = self.event_broadcaster.send(event);

        Ok(())
    }

    /// Add annotation to media
    pub async fn add_annotation(&mut self, file_id: &FileId, annotation: MediaAnnotation, annotator: &UserId) -> Result<()> {
        // Check permissions
        if !self.permissions.can_annotate(annotator) {
            return Err(NanoError::Media("User not authorized to annotate in this gallery".to_string()));
        }

        // Add annotation
        {
            let mut items = self.items.write().await;
            if let Some(item) = items.get_mut(file_id) {
                item.add_annotation(annotation.clone());
            } else {
                return Err(NanoError::Media("Media item not found in gallery".to_string()));
            }
        }

        // Update gallery timestamp
        self.updated_at = SystemTime::now();

        // Broadcast event
        let event = GalleryEvent::AnnotationAdded {
            gallery_id: self.gallery_id,
            file_id: *file_id,
            annotation,
            annotator: annotator.clone(),
            timestamp: SystemTime::now(),
        };

        let _ = self.event_broadcaster.send(event);

        Ok(())
    }

    /// Add participant to gallery
    pub async fn add_participant(&mut self, user_id: &UserId, inviter: &UserId, role: ParticipantRole) -> Result<()> {
        // Check permissions
        if !self.permissions.can_invite(inviter) {
            return Err(NanoError::Media("User not authorized to invite to this gallery".to_string()));
        }

        // Add participant
        self.participants.insert(user_id.clone());

        // Grant permissions based on role
        match role {
            ParticipantRole::Viewer => self.permissions.grant_read_only(user_id),
            ParticipantRole::Contributor => self.permissions.grant_contributor(user_id),
            ParticipantRole::Moderator => self.permissions.grant_all_permissions(user_id),
        }

        // Update encryption key participants
        self.encryption_key.participants.push(user_id.clone());

        // Update gallery timestamp
        self.updated_at = SystemTime::now();

        // Broadcast event
        let event = GalleryEvent::ParticipantAdded {
            gallery_id: self.gallery_id,
            user_id: user_id.clone(),
            inviter: inviter.clone(),
            role,
            timestamp: SystemTime::now(),
        };

        let _ = self.event_broadcaster.send(event);

        Ok(())
    }

    /// Remove participant from gallery
    pub async fn remove_participant(&mut self, user_id: &UserId, remover: &UserId) -> Result<()> {
        // Check permissions (only moderators and owner can remove participants)
        if !self.permissions.can_moderate(remover) && *remover != self.owner {
            return Err(NanoError::Media("User not authorized to remove participants".to_string()));
        }

        // Cannot remove owner
        if *user_id == self.owner {
            return Err(NanoError::Media("Cannot remove gallery owner".to_string()));
        }

        // Remove participant
        self.participants.remove(user_id);

        // Remove all permissions
        self.permissions.can_upload.remove(user_id);
        self.permissions.can_download.remove(user_id);
        self.permissions.can_delete.remove(user_id);
        self.permissions.can_annotate.remove(user_id);
        self.permissions.can_invite.remove(user_id);
        self.permissions.can_moderate.remove(user_id);

        // Update encryption key (rotate for security)
        self.encryption_key.participants.retain(|p| p != user_id);
        self.encryption_key.rotate()?;

        // Update gallery timestamp
        self.updated_at = SystemTime::now();

        // Broadcast event
        let event = GalleryEvent::ParticipantRemoved {
            gallery_id: self.gallery_id,
            user_id: user_id.clone(),
            remover: remover.clone(),
            timestamp: SystemTime::now(),
        };

        let _ = self.event_broadcaster.send(event);

        Ok(())
    }

    /// Get gallery statistics
    pub async fn get_stats(&self) -> GalleryStats {
        let items = self.items.read().await;
        
        let total_items = items.len() as u32;
        let total_size = items.values()
            .map(|_item| {
                    // Since FileReference doesn't have file_size field, use placeholder
                    // In a real implementation, this would query the metadata store
                    1024u64 // Placeholder size
                })
            .sum();
        let total_views = items.values().map(|item| item.view_count).sum();
        let total_downloads = items.values().map(|item| item.download_count).sum();
        let total_annotations = items.values()
            .map(|item| item.annotations.len() as u64)
            .sum();

        GalleryStats {
            gallery_id: self.gallery_id,
            total_items,
            total_size_bytes: total_size,
            total_participants: self.participants.len() as u32,
            total_views,
            total_downloads,
            total_annotations,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }

    /// Subscribe to gallery events
    pub fn subscribe_to_events(&self) -> broadcast::Receiver<GalleryEvent> {
        self.event_broadcaster.subscribe()
    }

    /// Get gallery items with pagination
    pub async fn get_items(&self, offset: usize, limit: usize) -> Vec<GalleryItem> {
        let items = self.items.read().await;
        items.values()
            .skip(offset)
            .take(limit)
            .cloned()
            .collect()
    }

    /// Search gallery items
    pub async fn search_items(&self, query: &str) -> Vec<GalleryItem> {
        let items = self.items.read().await;
        let query_lower = query.to_lowercase();
        
        items.values()
            .filter(|item| {
                // Search in title, description, and tags
                item.title.as_ref().map_or(false, |t| t.to_lowercase().contains(&query_lower)) ||
                item.description.as_ref().map_or(false, |d| d.to_lowercase().contains(&query_lower)) ||
                item.tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .cloned()
            .collect()
    }
}

/// Participant role in gallery
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ParticipantRole {
    Viewer,      // Can only view and download
    Contributor, // Can view, download, upload, and annotate
    Moderator,   // Full permissions except ownership
}

/// Gallery events for real-time updates
#[derive(Debug, Clone, Serialize)]
pub enum GalleryEvent {
    MediaAdded {
        gallery_id: GalleryId,
        file_id: FileId,
        uploader: UserId,
        timestamp: SystemTime,
    },
    MediaRemoved {
        gallery_id: GalleryId,
        file_id: FileId,
        remover: UserId,
        timestamp: SystemTime,
    },
    AnnotationAdded {
        gallery_id: GalleryId,
        file_id: FileId,
        annotation: MediaAnnotation,
        annotator: UserId,
        timestamp: SystemTime,
    },
    ParticipantAdded {
        gallery_id: GalleryId,
        user_id: UserId,
        inviter: UserId,
        role: ParticipantRole,
        timestamp: SystemTime,
    },
    ParticipantRemoved {
        gallery_id: GalleryId,
        user_id: UserId,
        remover: UserId,
        timestamp: SystemTime,
    },
    PermissionsChanged {
        gallery_id: GalleryId,
        user_id: UserId,
        new_permissions: Vec<String>,
        modifier: UserId,
        timestamp: SystemTime,
    },
}

/// Gallery statistics
#[derive(Debug, Clone, Serialize)]
pub struct GalleryStats {
    pub gallery_id: GalleryId,
    pub total_items: u32,
    pub total_size_bytes: u64,
    pub total_participants: u32,
    pub total_views: u64,
    pub total_downloads: u64,
    pub total_annotations: u64,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

/// Gallery information for listing
#[derive(Debug, Clone, Serialize)]
pub struct GalleryInfo {
    pub gallery_id: GalleryId,
    pub title: String,
    pub description: Option<String>,
    pub owner: UserId,
    pub participant_count: u32,
    pub item_count: u32,
    pub is_public: bool,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

/// Gallery manager for handling multiple galleries
pub struct GalleryManager {
    galleries: Arc<RwLock<HashMap<GalleryId, SharedGallery>>>,
    user_galleries: Arc<RwLock<HashMap<UserId, HashSet<GalleryId>>>>,
}

impl GalleryManager {
    /// Create a new gallery manager
    pub fn new() -> Self {
        Self {
            galleries: Arc::new(RwLock::new(HashMap::new())),
            user_galleries: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a new gallery
    pub async fn create_gallery(
        &self,
        title: String,
        owner: UserId,
        participants: &[UserId],
        is_public: bool,
    ) -> Result<GalleryId> {
        let gallery = SharedGallery::create_shared_gallery(title, owner.clone(), participants, is_public).await?;
        let gallery_id = gallery.gallery_id;

        // Store gallery
        {
            let mut galleries = self.galleries.write().await;
            galleries.insert(gallery_id, gallery);
        }

        // Update user galleries index
        {
            let mut user_galleries = self.user_galleries.write().await;
            
            // Add for owner
            user_galleries.entry(owner.clone()).or_insert_with(HashSet::new).insert(gallery_id);
            
            // Add for participants
            for participant in participants {
                user_galleries.entry(participant.clone()).or_insert_with(HashSet::new).insert(gallery_id);
            }
        }

        Ok(gallery_id)
    }

    /// Get galleries for a user
    pub async fn get_user_galleries(&self, user_id: &UserId) -> Vec<GalleryInfo> {
        let user_gallery_ids = {
            let user_galleries = self.user_galleries.read().await;
            user_galleries.get(user_id).cloned().unwrap_or_default()
        };

        let galleries = self.galleries.read().await;
        let mut gallery_infos = Vec::new();

        for gallery_id in user_gallery_ids {
            if let Some(gallery) = galleries.get(&gallery_id) {
                let items = gallery.items.read().await;
                gallery_infos.push(GalleryInfo {
                    gallery_id,
                    title: gallery.title.clone(),
                    description: gallery.description.clone(),
                    owner: gallery.owner.clone(),
                    participant_count: gallery.participants.len() as u32,
                    item_count: items.len() as u32,
                    is_public: gallery.is_public,
                    created_at: gallery.created_at,
                    updated_at: gallery.updated_at,
                });
            }
        }

        gallery_infos
    }

    /// Get a specific gallery
    pub async fn get_gallery(&self, gallery_id: &GalleryId, user_id: &UserId) -> Result<Option<SharedGallery>> {
        let galleries = self.galleries.read().await;
        
        if let Some(gallery) = galleries.get(gallery_id) {
            // Check if user has access
            if gallery.participants.contains(user_id) || gallery.is_public {
                // Note: In a real implementation, you'd clone or return a reference
                // For now, we'll return None to indicate access without copying the entire structure
                Ok(None) // Placeholder - would return appropriate access
            } else {
                Err(NanoError::Media("Access denied to gallery".to_string()))
            }
        } else {
            Ok(None)
        }
    }

    /// Delete a gallery
    pub async fn delete_gallery(&self, gallery_id: &GalleryId, user_id: &UserId) -> Result<()> {
        // Check if user is owner
        let is_owner = {
            let galleries = self.galleries.read().await;
            galleries.get(gallery_id)
                .map(|g| g.owner == *user_id)
                .unwrap_or(false)
        };

        if !is_owner {
            return Err(NanoError::Media("Only gallery owner can delete gallery".to_string()));
        }

        // Remove gallery
        let removed_gallery = {
            let mut galleries = self.galleries.write().await;
            galleries.remove(gallery_id)
        };

        if let Some(gallery) = removed_gallery {
            // Update user galleries index
            let mut user_galleries = self.user_galleries.write().await;
            for participant in &gallery.participants {
                if let Some(user_gallery_set) = user_galleries.get_mut(participant) {
                    user_gallery_set.remove(gallery_id);
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_gallery_creation() {
        let owner = "owner_user".to_string();
        let participants = vec!["user1".to_string(), "user2".to_string()];
        
        let gallery = SharedGallery::create_shared_gallery(
            "Test Gallery".to_string(),
            owner.clone(),
            &participants,
            false,
        ).await.unwrap();

        assert_eq!(gallery.title, "Test Gallery");
        assert_eq!(gallery.owner, owner);
        assert_eq!(gallery.participants.len(), 3); // owner + 2 participants
        assert!(!gallery.is_public);
    }

    #[test]
    fn test_gallery_permissions() {
        let owner = "owner".to_string();
        let user = "user".to_string();
        
        let mut permissions = GalleryPermissions::owner_permissions(&owner);
        assert!(permissions.can_upload(&owner));
        assert!(!permissions.can_upload(&user));

        permissions.grant_contributor(&user);
        assert!(permissions.can_upload(&user));
        assert!(permissions.can_download(&user));
        assert!(permissions.can_annotate(&user));
        assert!(!permissions.can_delete(&user));
    }

    #[test]
    fn test_shared_gallery_key() {
        let participants = vec!["user1".to_string(), "user2".to_string()];
        let mut key = SharedGalleryKey::new(CryptoMode::Hybrid, participants.clone());
        
        assert_eq!(key.key_data.len(), 32);
        assert_eq!(key.participants, participants);
        assert_eq!(key.crypto_mode as u8, CryptoMode::Hybrid as u8);

        let old_key = key.key_data.clone();
        key.rotate().unwrap();
        assert_ne!(key.key_data, old_key);
    }

    #[tokio::test]
    async fn test_gallery_manager() {
        let manager = GalleryManager::new();
        let owner = "owner".to_string();
        let participants = vec!["user1".to_string()];

        let gallery_id = manager.create_gallery(
            "Test Gallery".to_string(),
            owner.clone(),
            &participants,
            false,
        ).await.unwrap();

        let user_galleries = manager.get_user_galleries(&owner).await;
        assert_eq!(user_galleries.len(), 1);
        assert_eq!(user_galleries[0].gallery_id, gallery_id);
    }

    #[test]
    fn test_participant_roles() {
        let viewer = ParticipantRole::Viewer;
        let _contributor = ParticipantRole::Contributor;
        let _moderator = ParticipantRole::Moderator;

        // Test serialization
        let viewer_json = serde_json::to_string(&viewer).unwrap();
        assert!(viewer_json.contains("Viewer"));
    }
}
