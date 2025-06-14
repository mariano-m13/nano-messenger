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
pub mod media; // Session 9: Media and file attachments

#[cfg(test)]
mod test_compilation; // Test compilation fixes

pub mod test_session_11; // Session 11 test runner (available for CLI)
pub mod test_session_12; // Session 12 test runner (available for CLI)

pub use crypto::*;
pub use protocol::*;
pub use inbox::*;
pub use error::*;
pub use username::*; // Re-export username types including UserId
pub use config::*; // Re-export config for easy access
pub use production::*; // Re-export production utilities
pub use media::*; // Re-export media types

pub use test_session_11::{test_session_11, test_session_11_basic, benchmark_session_11};
pub use test_session_12::run_all_session_12_tests;
