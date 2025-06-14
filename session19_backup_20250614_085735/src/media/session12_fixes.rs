/// Session 12 Type System Fixes
/// 
/// This file provides comprehensive trait implementations for all Session 12 types
/// to resolve compilation errors systematically.

use std::time::{Duration, SystemTime};

// Extension trait for Duration to provide missing methods
pub trait DurationExt {
    fn from_days(days: u64) -> Duration;
    fn from_hours(hours: u64) -> Duration;
    fn from_minutes(minutes: u64) -> Duration;
}

impl DurationExt for Duration {
    fn from_days(days: u64) -> Duration {
        Duration::from_secs(days * 24 * 60 * 60)
    }
    
    fn from_hours(hours: u64) -> Duration {
        Duration::from_secs(hours * 60 * 60)
    }
    
    fn from_minutes(minutes: u64) -> Duration {
        Duration::from_secs(minutes * 60)
    }
}

// Type-safe Blake2b wrapper
use blake2::{Blake2b, Digest};
use blake2::digest::consts::U64;

pub type Blake2b512 = Blake2b<U64>;
pub type Blake2bHash = [u8; 64];

/// Create a Blake2b hash with proper type annotations
pub fn create_blake2b_hash(input: &[u8]) -> Blake2bHash {
    let mut hasher = Blake2b512::new();
    hasher.update(input);
    let result = hasher.finalize();
    let mut hash = [0u8; 64];
    hash.copy_from_slice(&result[..64]);
    hash
}

/// Serde support for Blake2b hashes
pub mod blake2b_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use super::Blake2bHash;

    pub fn serialize<S>(hash: &Blake2bHash, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        hash.to_vec().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Blake2bHash, D::Error>
    where
        D: Deserializer<'de>,
    {
        let vec: Vec<u8> = Vec::deserialize(deserializer)?;
        if vec.len() != 64 {
            return Err(serde::de::Error::custom(format!(
                "Expected 64 bytes, got {}", 
                vec.len()
            )));
        }
        let mut hash = [0u8; 64];
        hash.copy_from_slice(&vec);
        Ok(hash)
    }
}

/// Trait for types that need comprehensive trait implementations
macro_rules! impl_comprehensive_traits {
    ($type_name:ty) => {
        impl std::hash::Hash for $type_name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                std::mem::discriminant(self).hash(state);
            }
        }
        
        impl std::cmp::Eq for $type_name {}
        
        impl std::cmp::PartialOrd for $type_name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        
        impl std::cmp::Ord for $type_name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                std::mem::discriminant(self).cmp(&std::mem::discriminant(other))
            }
        }
    };
}

// Apply comprehensive traits to all problematic enums
// Note: These will be applied via derive macros in the actual enum definitions

/// Default implementation for FileMetadata
impl Default for crate::media::metadata::FileMetadata {
    fn default() -> Self {
        Self {
            file_id: String::new(),
            original_name: String::new(),
            file_size: 0,
            mime_type: String::new(),
            upload_timestamp: SystemTime::now(),
            checksum: String::new(),
            last_accessed: SystemTime::now(),
            uploader_id: String::new(),
            encryption_info: None,
            storage_location: String::new(),
            file_version: 1,
            compression_info: None,
            access_history: Vec::new(),
            metadata_version: 1,
        }
    }
}

/// Cryptographic helper functions
pub mod crypto_helpers {
    use chacha20poly1305::{ChaCha20Poly1305, KeyInit, AeadInPlace, Nonce};
    use super::Blake2bHash;
    
    /// Create a properly initialized ChaCha20Poly1305 cipher
    pub fn create_chacha_cipher(key: &[u8; 32]) -> ChaCha20Poly1305 {
        ChaCha20Poly1305::new(key.into())
    }
    
    /// Generate a secure random nonce
    pub fn generate_nonce() -> [u8; 12] {
        use rand::RngCore;
        let mut nonce = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce);
        nonce
    }
}

/// Test helper functions
#[cfg(test)]
pub mod test_helpers {
    use super::*;
    use crate::media::metadata::FileMetadata;
    
    /// Create a test FileMetadata with proper defaults
    pub fn create_test_file_metadata(file_id: &str) -> FileMetadata {
        FileMetadata {
            file_id: file_id.to_string(),
            original_name: format!("test_{}.dat", file_id),
            file_size: 1024,
            mime_type: "application/octet-stream".to_string(),
            upload_timestamp: SystemTime::now(),
            checksum: "test_checksum".to_string(),
            last_accessed: SystemTime::now(),
            uploader_id: "test_user".to_string(),
            encryption_info: None,
            storage_location: "local".to_string(),
            file_version: 1,
            compression_info: None,
            access_history: Vec::new(),
            metadata_version: 1,
        }
    }
}

/// Error handling improvements
pub trait ResultExt<T, E> {
    fn unwrap_or_log(self, message: &str) -> Option<T>;
}

impl<T, E: std::fmt::Debug> ResultExt<T, E> for Result<T, E> {
    fn unwrap_or_log(self, message: &str) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                eprintln!("{}: {:?}", message, err);
                None
            }
        }
    }
}
