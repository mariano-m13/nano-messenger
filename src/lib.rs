pub mod crypto;
pub mod protocol;
pub mod inbox;
pub mod error;
pub mod username;
pub mod contacts;
pub mod network;
pub mod messages;
pub mod config; // Session 6: Adaptive configuration
pub mod production; // Session 8: Production hardening

pub use crypto::*;
pub use protocol::*;
pub use inbox::*;
pub use error::*;
pub use config::*; // Re-export config for easy access
pub use production::*; // Re-export production utilities
