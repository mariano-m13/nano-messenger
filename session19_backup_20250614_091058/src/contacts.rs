use crate::error::{NanoError, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Contact permission status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ContactStatus {
    Unknown,  // No permission granted yet
    Allowed,  // Trusted contact, can send messages
    Blocked,  // Blocked contact, messages ignored
}

/// Contact metadata (stored locally only)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactMetadata {
    pub nickname: String,    // Max 50 chars
    pub memo: String,        // Max 200 chars  
    pub added_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
}

impl ContactMetadata {
    pub fn new(nickname: String, memo: String) -> Result<Self> {
        Self::validate_nickname(&nickname)?;
        Self::validate_memo(&memo)?;
        
        let now = Utc::now();
        Ok(Self {
            nickname,
            memo,
            added_at: now,
            last_modified: now,
        })
    }

    pub fn update(&mut self, nickname: Option<String>, memo: Option<String>) -> Result<()> {
        if let Some(nickname) = nickname {
            Self::validate_nickname(&nickname)?;
            self.nickname = nickname;
            self.last_modified = Utc::now();
        }
        
        if let Some(memo) = memo {
            Self::validate_memo(&memo)?;
            self.memo = memo;
            self.last_modified = Utc::now();
        }
        
        Ok(())
    }

    fn validate_nickname(nickname: &str) -> Result<()> {
        if nickname.len() > 50 {
            return Err(NanoError::Protocol("Nickname cannot exceed 50 characters".to_string()));
        }
        Ok(())
    }

    fn validate_memo(memo: &str) -> Result<()> {
        if memo.len() > 200 {
            return Err(NanoError::Protocol("Memo cannot exceed 200 characters".to_string()));
        }
        Ok(())
    }
}

/// Contact permission entry (synced across devices)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactPermission {
    pub pubkey: String,
    pub status: ContactStatus,
    pub first_contact: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl ContactPermission {
    pub fn new(pubkey: String, status: ContactStatus) -> Self {
        let now = Utc::now();
        Self {
            pubkey,
            status,
            first_contact: now,
            last_updated: now,
        }
    }

    pub fn update_status(&mut self, status: ContactStatus) {
        self.status = status;
        self.last_updated = Utc::now();
    }
}

/// Complete contact information
#[derive(Debug, Clone)]
pub struct Contact {
    pub permission: ContactPermission,
    pub metadata: Option<ContactMetadata>,
    pub username: Option<String>, // Last known username
}

impl Contact {
    pub fn display_name(&self) -> &str {
        if let Some(metadata) = &self.metadata {
            if !metadata.nickname.is_empty() {
                return &metadata.nickname;
            }
        }
        
        if let Some(username) = &self.username {
            username
        } else {
            &self.permission.pubkey
        }
    }
}

/// Manages contact permissions and metadata
pub struct ContactManager {
    permissions: HashMap<String, ContactPermission>, // pubkey -> permission (synced)
    metadata: HashMap<String, ContactMetadata>,      // pubkey -> metadata (local only)
    username_to_pubkey: HashMap<String, String>,     // username -> pubkey mapping
}

impl ContactManager {
    pub fn new() -> Self {
        Self {
            permissions: HashMap::new(),
            metadata: HashMap::new(),
            username_to_pubkey: HashMap::new(),
        }
    }

    /// Allow a contact to send messages
    pub fn allow_contact(&mut self, pubkey: String) -> Result<()> {
        match self.permissions.get_mut(&pubkey) {
            Some(permission) => {
                permission.update_status(ContactStatus::Allowed);
            }
            None => {
                let permission = ContactPermission::new(pubkey.clone(), ContactStatus::Allowed);
                self.permissions.insert(pubkey, permission);
            }
        }
        Ok(())
    }

    /// Block a contact from sending messages
    pub fn block_contact(&mut self, pubkey: String) -> Result<()> {
        match self.permissions.get_mut(&pubkey) {
            Some(permission) => {
                permission.update_status(ContactStatus::Blocked);
            }
            None => {
                let permission = ContactPermission::new(pubkey.clone(), ContactStatus::Blocked);
                self.permissions.insert(pubkey, permission);
            }
        }
        Ok(())
    }

    /// Check if a contact is allowed to send messages
    pub fn is_allowed(&self, pubkey: &str) -> bool {
        self.permissions
            .get(pubkey)
            .map(|p| p.status == ContactStatus::Allowed)
            .unwrap_or(false)
    }

    /// Check if a contact is blocked
    pub fn is_blocked(&self, pubkey: &str) -> bool {
        self.permissions
            .get(pubkey)
            .map(|p| p.status == ContactStatus::Blocked)
            .unwrap_or(false)
    }

    /// Get contact status
    pub fn get_status(&self, pubkey: &str) -> ContactStatus {
        self.permissions
            .get(pubkey)
            .map(|p| p.status.clone())
            .unwrap_or(ContactStatus::Unknown)
    }

    /// Set contact metadata (nickname and memo)
    pub fn set_metadata(&mut self, pubkey: String, nickname: String, memo: String) -> Result<()> {
        let metadata = ContactMetadata::new(nickname, memo)?;
        self.metadata.insert(pubkey, metadata);
        Ok(())
    }

    /// Update contact metadata
    pub fn update_metadata(
        &mut self,
        pubkey: &str,
        nickname: Option<String>,
        memo: Option<String>,
    ) -> Result<()> {
        match self.metadata.get_mut(pubkey) {
            Some(metadata) => metadata.update(nickname, memo)?,
            None => {
                let new_metadata = ContactMetadata::new(
                    nickname.unwrap_or_default(),
                    memo.unwrap_or_default(),
                )?;
                self.metadata.insert(pubkey.to_string(), new_metadata);
            }
        }
        Ok(())
    }

    /// Set username for a contact
    pub fn set_username(&mut self, pubkey: String, username: String) {
        self.username_to_pubkey.insert(username, pubkey);
    }

    /// Get pubkey for a username
    pub fn get_pubkey_for_username(&self, username: &str) -> Option<&str> {
        self.username_to_pubkey.get(username).map(|s| s.as_str())
    }

    /// Get all contacts
    pub fn list_contacts(&self) -> Vec<Contact> {
        let mut contacts = Vec::new();
        
        for (pubkey, permission) in &self.permissions {
            let metadata = self.metadata.get(pubkey).cloned();
            let username = self.username_to_pubkey
                .iter()
                .find(|(_, pk)| *pk == pubkey)
                .map(|(u, _)| u.clone());
                
            contacts.push(Contact {
                permission: permission.clone(),
                metadata,
                username,
            });
        }
        
        contacts
    }

    /// Search contacts by nickname or memo
    pub fn search_contacts(&self, query: &str) -> Vec<Contact> {
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();
        
        for contact in self.list_contacts() {
            let mut matches = false;
            
            // Search in nickname
            if let Some(metadata) = &contact.metadata {
                if metadata.nickname.to_lowercase().contains(&query_lower) {
                    matches = true;
                }
                
                // Search in memo
                if metadata.memo.to_lowercase().contains(&query_lower) {
                    matches = true;
                }
            }
            
            // Search in username
            if let Some(username) = &contact.username {
                if username.to_lowercase().contains(&query_lower) {
                    matches = true;
                }
            }
            
            if matches {
                results.push(contact);
            }
        }
        
        results
    }

    /// Get contact by pubkey
    pub fn get_contact(&self, pubkey: &str) -> Option<Contact> {
        let permission = self.permissions.get(pubkey)?.clone();
        let metadata = self.metadata.get(pubkey).cloned();
        let username = self.username_to_pubkey
            .iter()
            .find(|(_, pk)| *pk == pubkey)
            .map(|(u, _)| u.clone());
            
        Some(Contact {
            permission,
            metadata,
            username,
        })
    }

    /// Remove a contact completely
    pub fn remove_contact(&mut self, pubkey: &str) {
        self.permissions.remove(pubkey);
        self.metadata.remove(pubkey);
        
        // Remove username mapping
        self.username_to_pubkey.retain(|_, pk| pk != pubkey);
    }

    /// Get permissions for syncing (without local metadata)
    pub fn get_permissions(&self) -> &HashMap<String, ContactPermission> {
        &self.permissions
    }

    /// Load permissions from sync data
    pub fn load_permissions(&mut self, permissions: HashMap<String, ContactPermission>) {
        self.permissions = permissions;
    }

    /// Export local metadata (for backup)
    pub fn export_metadata(&self) -> &HashMap<String, ContactMetadata> {
        &self.metadata
    }

    /// Import local metadata (from backup)
    pub fn import_metadata(&mut self, metadata: HashMap<String, ContactMetadata>) {
        self.metadata = metadata;
    }
}

impl Default for ContactManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contact_metadata() {
        let metadata = ContactMetadata::new(
            "Alice K.".to_string(),
            "Coffee shop regular".to_string(),
        ).unwrap();
        
        assert_eq!(metadata.nickname, "Alice K.");
        assert_eq!(metadata.memo, "Coffee shop regular");
        
        // Test validation
        assert!(ContactMetadata::new("a".repeat(51), String::new()).is_err());
        assert!(ContactMetadata::new(String::new(), "a".repeat(201)).is_err());
    }

    #[test]
    fn test_contact_manager() {
        let mut manager = ContactManager::new();
        let pubkey = "pubkey:abc123".to_string();
        
        // Initially unknown
        assert_eq!(manager.get_status(&pubkey), ContactStatus::Unknown);
        assert!(!manager.is_allowed(&pubkey));
        assert!(!manager.is_blocked(&pubkey));
        
        // Allow contact
        manager.allow_contact(pubkey.clone()).unwrap();
        assert_eq!(manager.get_status(&pubkey), ContactStatus::Allowed);
        assert!(manager.is_allowed(&pubkey));
        assert!(!manager.is_blocked(&pubkey));
        
        // Block contact
        manager.block_contact(pubkey.clone()).unwrap();
        assert_eq!(manager.get_status(&pubkey), ContactStatus::Blocked);
        assert!(!manager.is_allowed(&pubkey));
        assert!(manager.is_blocked(&pubkey));
        
        // Add metadata
        manager.set_metadata(
            pubkey.clone(),
            "Alice".to_string(),
            "Friend from work".to_string(),
        ).unwrap();
        
        // Add username
        manager.set_username(pubkey.clone(), "alice2024".to_string());
        assert_eq!(manager.get_pubkey_for_username("alice2024"), Some(pubkey.as_str()));
        
        // Get contact
        let contact = manager.get_contact(&pubkey).unwrap();
        assert_eq!(contact.display_name(), "Alice");
        assert_eq!(contact.username, Some("alice2024".to_string()));
    }

    #[test]
    fn test_contact_search() {
        let mut manager = ContactManager::new();
        
        let pubkey1 = "pubkey:abc123".to_string();
        let pubkey2 = "pubkey:def456".to_string();
        
        manager.allow_contact(pubkey1.clone()).unwrap();
        manager.allow_contact(pubkey2.clone()).unwrap();
        
        manager.set_metadata(
            pubkey1.clone(),
            "Alice K.".to_string(),
            "Coffee shop regular".to_string(),
        ).unwrap();
        
        manager.set_metadata(
            pubkey2.clone(),
            "Bob S.".to_string(),
            "Cycling buddy".to_string(),
        ).unwrap();
        
        // Search by nickname
        let results = manager.search_contacts("alice");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].permission.pubkey, pubkey1);
        
        // Search by memo
        let results = manager.search_contacts("coffee");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].permission.pubkey, pubkey1);
        
        // Search by memo (different contact)
        let results = manager.search_contacts("cycling");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].permission.pubkey, pubkey2);
        
        // No matches
        let results = manager.search_contacts("xyz");
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_contact_display_name() {
        let permission = ContactPermission::new(
            "pubkey:abc123".to_string(),
            ContactStatus::Allowed,
        );
        
        // No metadata, no username
        let contact = Contact {
            permission: permission.clone(),
            metadata: None,
            username: None,
        };
        assert_eq!(contact.display_name(), "pubkey:abc123");
        
        // With username, no metadata
        let contact = Contact {
            permission: permission.clone(),
            metadata: None,
            username: Some("alice2024".to_string()),
        };
        assert_eq!(contact.display_name(), "alice2024");
        
        // With nickname (should take precedence)
        let metadata = ContactMetadata::new(
            "Alice K.".to_string(),
            "Friend".to_string(),
        ).unwrap();
        
        let contact = Contact {
            permission,
            metadata: Some(metadata),
            username: Some("alice2024".to_string()),
        };
        assert_eq!(contact.display_name(), "Alice K.");
    }
}
