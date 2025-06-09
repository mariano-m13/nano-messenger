//! Test suite for nano-messenger
//! 
//! This module contains all tests for the nano-messenger project,
//! organized by functionality and security validation.

pub mod security;

// Re-export main test runners for convenience
pub use security::run_comprehensive_security_validation;
pub use security::SecurityValidationChecklist;
