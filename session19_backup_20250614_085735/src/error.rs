use thiserror::Error;

#[derive(Error, Debug)]
pub enum NanoError {
    #[error("Cryptographic error: {0}")]
    Crypto(String),
    
    #[error("Protocol error: {0}")]
    Protocol(String),
    
    #[error("Network error: {0}")]
    Network(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Base64 decode error: {0}")]
    Base64(#[from] base64::DecodeError),
    
    #[error("Username not found: {0}")]
    UsernameNotFound(String),
    
    #[error("Invalid message format")]
    InvalidMessage,
    
    #[error("Permission denied")]
    PermissionDenied,
    
    #[error("Message expired")]
    MessageExpired,
    
    // Session 9: Media and file attachment errors
    #[error("Media error: {0}")]
    Media(String),
    
    #[error("Storage error: {0}")]
    Storage(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
}

pub type Result<T> = std::result::Result<T, NanoError>;
