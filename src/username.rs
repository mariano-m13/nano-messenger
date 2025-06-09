use crate::crypto::{UserKeyPair, UserPublicKeys};
use crate::error::{NanoError, Result};
use crate::protocol::UsernameClaim;
use std::collections::HashMap;

/// Username registry that can be used by relays or clients
#[derive(Default, Clone)]
pub struct UsernameRegistry {
    claims: HashMap<String, UsernameClaim>,
}

impl UsernameRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a username claim (used by relays)
    pub fn register_claim(&mut self, claim: UsernameClaim) -> Result<()> {
        // Verify the claim's signature
        claim.verify_signature()?;
        
        // Check if username is already taken
        if let Some(existing) = self.claims.get(&claim.username) {
            // Allow updates only if it's from the same public key
            if existing.public_keys.public_key_string() != claim.public_keys.public_key_string() {
                return Err(NanoError::Protocol(format!(
                    "Username '{}' is already claimed by another user",
                    claim.username
                )));
            }
            
            // Allow update if the new claim is newer
            if claim.timestamp <= existing.timestamp {
                return Err(NanoError::Protocol(format!(
                    "Username claim for '{}' is not newer than existing claim",
                    claim.username
                )));
            }
        }
        
        self.claims.insert(claim.username.clone(), claim);
        Ok(())
    }

    /// Look up a username and return the associated public keys
    pub fn lookup_username(&self, username: &str) -> Option<&UserPublicKeys> {
        self.claims.get(username).map(|claim| &claim.public_keys)
    }

    /// Get all registered usernames
    pub fn list_usernames(&self) -> Vec<&str> {
        self.claims.keys().map(|s| s.as_str()).collect()
    }

    /// Check if a username is available
    pub fn is_username_available(&self, username: &str) -> bool {
        !self.claims.contains_key(username)
    }

    /// Get the claim for a username (including metadata)
    pub fn get_claim(&self, username: &str) -> Option<&UsernameClaim> {
        self.claims.get(username)
    }

    /// Remove a claim (for administrative purposes)
    pub fn remove_claim(&mut self, username: &str) -> Option<UsernameClaim> {
        self.claims.remove(username)
    }

    /// Get claims count
    pub fn claim_count(&self) -> usize {
        self.claims.len()
    }
}

/// Helper functions for username validation
pub fn validate_username(username: &str) -> Result<()> {
    if username.is_empty() {
        return Err(NanoError::Protocol("Username cannot be empty".to_string()));
    }
    
    if username.len() > 32 {
        return Err(NanoError::Protocol("Username cannot be longer than 32 characters".to_string()));
    }
    
    // Check for valid characters (alphanumeric, underscore, dash)
    if !username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        return Err(NanoError::Protocol(
            "Username can only contain letters, numbers, underscores, and dashes".to_string()
        ));
    }
    
    // Must start with a letter or number
    if let Some(first_char) = username.chars().next() {
        if !first_char.is_alphanumeric() {
            return Err(NanoError::Protocol(
                "Username must start with a letter or number".to_string()
            ));
        }
    }
    
    Ok(())
}

/// Create a username claim for a given keypair
pub fn create_username_claim(username: &str, keypair: &UserKeyPair) -> Result<UsernameClaim> {
    validate_username(username)?;
    
    let public_keys = keypair.public_keys();
    let mut claim = UsernameClaim::new(username.to_string(), public_keys);
    claim.sign(&keypair.signing_key)?;
    
    Ok(claim)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_username_validation() {
        // Valid usernames
        assert!(validate_username("alice").is_ok());
        assert!(validate_username("alice2024").is_ok());
        assert!(validate_username("alice_bob").is_ok());
        assert!(validate_username("alice-bob").is_ok());
        assert!(validate_username("a1b2c3").is_ok());
        
        // Invalid usernames
        assert!(validate_username("").is_err()); // Empty
        assert!(validate_username("_alice").is_err()); // Starts with underscore
        assert!(validate_username("-alice").is_err()); // Starts with dash
        assert!(validate_username("alice@bob").is_err()); // Invalid character
        assert!(validate_username("alice bob").is_err()); // Space
        assert!(validate_username(&"a".repeat(33)).is_err()); // Too long
    }

    #[test]
    fn test_username_registry() {
        let mut registry = UsernameRegistry::new();
        
        let alice_keypair = UserKeyPair::generate();
        let bob_keypair = UserKeyPair::generate();
        
        // Create claims
        let alice_claim = create_username_claim("alice2024", &alice_keypair).unwrap();
        let bob_claim = create_username_claim("bob2024", &bob_keypair).unwrap();
        
        // Register claims
        registry.register_claim(alice_claim.clone()).unwrap();
        registry.register_claim(bob_claim.clone()).unwrap();
        
        // Test lookups
        assert!(registry.lookup_username("alice2024").is_some());
        assert!(registry.lookup_username("bob2024").is_some());
        assert!(registry.lookup_username("charlie").is_none());
        
        // Test duplicate username with different key (should fail)
        let charlie_keypair = UserKeyPair::generate();
        let charlie_claim = create_username_claim("alice2024", &charlie_keypair).unwrap();
        assert!(registry.register_claim(charlie_claim).is_err());
        
        // Test username list
        let usernames = registry.list_usernames();
        assert_eq!(usernames.len(), 2);
        assert!(usernames.contains(&"alice2024"));
        assert!(usernames.contains(&"bob2024"));
    }

    #[test]
    fn test_username_claim_update() {
        let mut registry = UsernameRegistry::new();
        let alice_keypair = UserKeyPair::generate();
        
        // Create and register first claim
        let claim1 = create_username_claim("alice2024", &alice_keypair).unwrap();
        registry.register_claim(claim1.clone()).unwrap();
        
        // Create a newer claim from the same user by manually setting timestamp
        let mut claim2 = create_username_claim("alice2024", &alice_keypair).unwrap();
        claim2.timestamp = claim1.timestamp + 1; // Ensure newer timestamp
        claim2.sign(&alice_keypair.signing_key).unwrap(); // Re-sign with new timestamp
        
        // Should allow update
        registry.register_claim(claim2.clone()).unwrap();
        
        // Should have the newer claim
        let retrieved = registry.get_claim("alice2024").unwrap();
        assert!(retrieved.timestamp > claim1.timestamp);
    }

    #[test]
    fn test_username_availability() {
        let mut registry = UsernameRegistry::new();
        let keypair = UserKeyPair::generate();
        
        assert!(registry.is_username_available("alice2024"));
        
        let claim = create_username_claim("alice2024", &keypair).unwrap();
        registry.register_claim(claim).unwrap();
        
        assert!(!registry.is_username_available("alice2024"));
        assert!(registry.is_username_available("bob2024"));
    }
}
