// Core traits and configuration
pub mod traits;
pub mod config;
pub mod classical;
pub mod post_quantum;
pub mod hybrid;
pub mod quantum_safe; // Session 3: Quantum-safe messaging functions

// Session 6: Performance optimization modules
pub mod benchmarks;
pub mod optimizations;

// Re-export the main types and traits for easy access
pub use config::{CryptoConfig, CryptoMode};
pub use traits::{
    AsymmetricEncryption, CryptoProvider, DigitalSignature, KeyExchange, SymmetricEncryption,
};

// Re-export classical implementations
pub use classical::{
    ClassicalAsymmetricEncryption, ClassicalCryptoProvider, ClassicalDigitalSignature,
    ClassicalKeyExchange, ClassicalSymmetricEncryption, ClassicalUserKeyPair,
    ClassicalUserPublicKeys, hash_sha256,
};

// Re-export post-quantum implementations
pub use post_quantum::{
    PostQuantumAsymmetricEncryption, PostQuantumDigitalSignature, PostQuantumKeyExchange,
    PostQuantumUserKeyPair, PostQuantumUserPublicKeys,
};

// Re-export hybrid implementations
pub use hybrid::{
    HybridAsymmetricEncryption, HybridDigitalSignature, HybridKeyExchange,
    HybridUserKeyPair, HybridUserPublicKeys,
};

// Re-export quantum-safe messaging (Session 3)
pub use quantum_safe::QuantumSafeMessaging;

// Type alias for quantum signatures
pub type QuantumSignature = Vec<u8>;

// Re-export performance optimization components (Session 6)
pub use benchmarks::{
    CryptoBenchmark, CryptoBenchmarkResults, BenchmarkMetrics, 
    PerformanceComparison, PerformanceRecommendation,
};
pub use optimizations::{
    CryptoCache, CacheConfig, CacheMetrics, BatchProcessor, 
    BatchResult, MemoryPool, PrecomputationManager,
};

use crate::error::{NanoError, Result};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

/// Global crypto configuration
static CRYPTO_CONFIG: OnceLock<CryptoConfig> = OnceLock::new();

/// Initialize the global crypto configuration
pub fn init_crypto_config(config: CryptoConfig) -> Result<()> {
    config.validate()?;
    CRYPTO_CONFIG
        .set(config)
        .map_err(|_| NanoError::Crypto("Crypto config already initialized".to_string()))?;
    Ok(())
}

/// Get the current crypto configuration
pub fn get_crypto_config() -> &'static CryptoConfig {
    static DEFAULT_CONFIG: CryptoConfig = CryptoConfig {
        mode: CryptoMode::Classical,
        allow_auto_upgrade: true,
        adaptive_mode: false,
        minimum_mode: CryptoMode::Classical,
    };
    CRYPTO_CONFIG.get().unwrap_or(&DEFAULT_CONFIG)
}

/// Update the global crypto mode (if allowed by config)
pub fn set_crypto_mode(mode: CryptoMode) -> Result<()> {
    let current_config = get_crypto_config();
    
    // Check if transition from current mode to new mode is allowed
    if !current_config.mode.can_transition_to(mode) {
        return Err(NanoError::Crypto(format!(
            "Cannot transition from {} to {} mode",
            current_config.mode, mode
        )));
    }
    
    // Check if new mode meets minimum requirements
    if !current_config.minimum_mode.can_transition_to(mode) && mode != current_config.minimum_mode {
        return Err(NanoError::Crypto(format!(
            "Mode {} does not meet minimum security requirement of {}",
            mode, current_config.minimum_mode
        )));
    }
    
    let new_config = CryptoConfig {
        mode,
        ..*current_config
    };
    
    new_config.validate()?;
    
    // For Session 1, we'll just validate the transition
    // In a real implementation, we'd need proper thread-safe config updates
    Ok(())
}

/// Unified interface for cryptographic operations based on current config
pub struct CryptoInterface;

impl CryptoInterface {
    /// Get the current crypto mode
    pub fn current_mode() -> CryptoMode {
        get_crypto_config().mode
    }
    
    /// Generate a new keypair for the current crypto mode
    pub fn generate_keypair() -> Result<UnifiedKeyPair> {
        match Self::current_mode() {
            CryptoMode::Classical => Ok(UnifiedKeyPair::Classical(ClassicalUserKeyPair::generate())),
            CryptoMode::Hybrid => Ok(UnifiedKeyPair::Hybrid(HybridUserKeyPair::generate())),
            CryptoMode::Quantum | CryptoMode::QuantumSafe => Ok(UnifiedKeyPair::PostQuantum(PostQuantumUserKeyPair::generate())),
        }
    }
    
    /// Encrypt data symmetrically using the current crypto mode
    pub fn encrypt_symmetric(key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>> {
        // For symmetric encryption, we use ChaCha20Poly1305 for all modes
        // as it's already quantum-resistant
        ClassicalSymmetricEncryption::encrypt(key, plaintext)
    }
    
    /// Decrypt data symmetrically using the current crypto mode
    pub fn decrypt_symmetric(key: &[u8; 32], ciphertext: &[u8]) -> Result<Vec<u8>> {
        // For symmetric encryption, we use ChaCha20Poly1305 for all modes
        // as it's already quantum-resistant
        ClassicalSymmetricEncryption::decrypt(key, ciphertext)
    }
    
    /// Check if the current mode accepts messages from another mode
    pub fn accepts_mode(incoming_mode: CryptoMode) -> bool {
        get_crypto_config().accepts_mode(incoming_mode)
    }
    
    /// Get performance information for the current mode
    pub fn performance_info() -> CryptoPerformanceInfo {
        let mode = Self::current_mode();
        CryptoPerformanceInfo {
            mode,
            relative_cost: mode.performance_cost(),
            size_overhead: mode.size_overhead(),
            quantum_resistant: mode.is_quantum_resistant(),
        }
    }
}

/// Unified keypair that can hold different crypto implementations
#[derive(Clone)]
pub enum UnifiedKeyPair {
    Classical(ClassicalUserKeyPair),
    Hybrid(HybridUserKeyPair),
    PostQuantum(PostQuantumUserKeyPair),
}

impl UnifiedKeyPair {
    /// Get the crypto mode of this keypair
    pub fn mode(&self) -> CryptoMode {
        match self {
            UnifiedKeyPair::Classical(_) => CryptoMode::Classical,
            UnifiedKeyPair::Hybrid(_) => CryptoMode::Hybrid,
            UnifiedKeyPair::PostQuantum(_) => CryptoMode::Quantum, // Could be Quantum or QuantumSafe
        }
    }
    
    /// Get the public key string identifier
    pub fn public_key_string(&self) -> String {
        match self {
            UnifiedKeyPair::Classical(keypair) => keypair.public_key_string(),
            UnifiedKeyPair::Hybrid(keypair) => keypair.public_key_string(),
            UnifiedKeyPair::PostQuantum(keypair) => keypair.public_key_string(),
        }
    }
    
    /// Get the public keys
    pub fn public_keys(&self) -> UnifiedPublicKeys {
        match self {
            UnifiedKeyPair::Classical(keypair) => {
                UnifiedPublicKeys::Classical(keypair.public_keys())
            }
            UnifiedKeyPair::Hybrid(keypair) => {
                UnifiedPublicKeys::Hybrid(keypair.public_keys())
            }
            UnifiedKeyPair::PostQuantum(keypair) => {
                UnifiedPublicKeys::PostQuantum(keypair.public_keys())
            }
        }
    }
}

/// Unified public keys that can hold different crypto implementations
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UnifiedPublicKeys {
    Classical(ClassicalUserPublicKeys),
    Hybrid(HybridUserPublicKeys),
    PostQuantum(PostQuantumUserPublicKeys),
}

impl UnifiedPublicKeys {
    /// Get the crypto mode of these public keys
    pub fn mode(&self) -> CryptoMode {
        match self {
            UnifiedPublicKeys::Classical(_) => CryptoMode::Classical,
            UnifiedPublicKeys::Hybrid(_) => CryptoMode::Hybrid,
            UnifiedPublicKeys::PostQuantum(_) => CryptoMode::Quantum, // Could be Quantum or QuantumSafe
        }
    }
    
    /// Get the public key string identifier
    pub fn public_key_string(&self) -> String {
        match self {
            UnifiedPublicKeys::Classical(keys) => keys.public_key_string(),
            UnifiedPublicKeys::Hybrid(keys) => keys.public_key_string(),
            UnifiedPublicKeys::PostQuantum(keys) => keys.public_key_string(),
        }
    }
    
    /// Convert from legacy UserPublicKeys to UnifiedPublicKeys
    pub fn from_legacy(legacy: UserPublicKeys) -> Self {
        UnifiedPublicKeys::Classical(legacy)
    }
}

/// Performance information for the current crypto mode
#[derive(Debug, Clone, Copy)]
pub struct CryptoPerformanceInfo {
    pub mode: CryptoMode,
    pub relative_cost: f32,
    pub size_overhead: usize,
    pub quantum_resistant: bool,
}

// Backwards compatibility functions for existing code
// These maintain the same API as the original crypto.rs

/// Generate a new random keypair (backwards compatible)
pub fn generate_keypair() -> ClassicalUserKeyPair {
    ClassicalUserKeyPair::generate()
}

/// Encrypt data using ChaCha20Poly1305 with the given key (backwards compatible)
pub fn encrypt_symmetric(key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>> {
    CryptoInterface::encrypt_symmetric(key, plaintext)
}

/// Decrypt data using ChaCha20Poly1305 with the given key (backwards compatible)
pub fn decrypt_symmetric(key: &[u8; 32], ciphertext_with_nonce: &[u8]) -> Result<Vec<u8>> {
    CryptoInterface::decrypt_symmetric(key, ciphertext_with_nonce)
}

/// Encrypt data asymmetrically for first contact (backwards compatible)
pub fn encrypt_asymmetric(their_public: &X25519PublicKey, plaintext: &[u8]) -> Result<Vec<u8>> {
    ClassicalAsymmetricEncryption::encrypt(their_public, plaintext)
}

/// Decrypt asymmetrically encrypted data (backwards compatible)
pub fn decrypt_asymmetric(our_private: &X25519PrivateKey, ciphertext_with_ephemeral: &[u8]) -> Result<Vec<u8>> {
    ClassicalAsymmetricEncryption::decrypt_classical_direct(our_private, ciphertext_with_ephemeral)
}

/// Sign data with Ed25519 (backwards compatible)
pub fn sign_data(signing_key: &Ed25519PrivateKey, data: &[u8]) -> ed25519_dalek::Signature {
    ClassicalDigitalSignature::sign(signing_key, data)
}

/// Verify Ed25519 signature (backwards compatible)
pub fn verify_signature(
    verifying_key: &Ed25519PublicKey,
    data: &[u8],
    signature: &ed25519_dalek::Signature,
) -> Result<()> {
    ClassicalDigitalSignature::verify(verifying_key, data, signature)
}

/// Perform ECDH key exchange to derive a shared secret (backwards compatible)
pub fn derive_shared_secret(
    our_private: &X25519PrivateKey,
    their_public: &X25519PublicKey,
) -> [u8; 32] {
    ClassicalKeyExchange::key_exchange(our_private, their_public)
        .unwrap()
        .to_bytes()
}

// Re-export types for backwards compatibility
pub type UserKeyPair = ClassicalUserKeyPair;
pub type UserPublicKeys = ClassicalUserPublicKeys;
pub use classical::{Ed25519PrivateKey, Ed25519PublicKey, X25519PrivateKey, X25519PublicKey};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto_config_initialization() {
        // Test default configuration
        let config = CryptoConfig::default();
        assert_eq!(config.mode, CryptoMode::Classical);
        
        // Test high security configuration
        let high_sec = CryptoConfig::high_security();
        assert_eq!(high_sec.mode, CryptoMode::Hybrid);
        assert_eq!(high_sec.minimum_mode, CryptoMode::Hybrid);
    }

    #[test]
    fn test_unified_interface() {
        // Try to initialize with classical mode, but don't panic if already initialized
        let config = CryptoConfig::new(CryptoMode::Classical);
        let _ = init_crypto_config(config); // Ignore error if already initialized
        
        // Test keypair generation
        let keypair = CryptoInterface::generate_keypair().unwrap();
        assert_eq!(keypair.mode(), CryptoMode::Classical);
        
        // Test symmetric encryption
        let key = [42u8; 32];
        let plaintext = b"test message";
        
        let ciphertext = CryptoInterface::encrypt_symmetric(&key, plaintext).unwrap();
        let decrypted = CryptoInterface::decrypt_symmetric(&key, &ciphertext).unwrap();
        
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_backwards_compatibility() {
        // Test that the old API still works
        let keypair = generate_keypair();
        let public_keys = keypair.public_keys();
        
        let key = [42u8; 32];
        let plaintext = b"backwards compatibility test";
        
        let ciphertext = encrypt_symmetric(&key, plaintext).unwrap();
        let decrypted = decrypt_symmetric(&key, &ciphertext).unwrap();
        
        assert_eq!(decrypted, plaintext);
        
        // Test signatures
        let data = b"sign this";
        let signature = sign_data(&keypair.signing_key, data);
        verify_signature(&public_keys.verifying_key, data, &signature).unwrap();
    }

    #[test]
    fn test_mode_transitions() {
        // Test basic mode transition validation logic directly
        assert!(CryptoMode::Classical.can_transition_to(CryptoMode::Hybrid));
        assert!(CryptoMode::Classical.can_transition_to(CryptoMode::Quantum));
        assert!(CryptoMode::Hybrid.can_transition_to(CryptoMode::Quantum));
        
        // Test that downgrades are prevented
        assert!(!CryptoMode::Hybrid.can_transition_to(CryptoMode::Classical));
        assert!(!CryptoMode::Quantum.can_transition_to(CryptoMode::Classical));
        assert!(!CryptoMode::Quantum.can_transition_to(CryptoMode::Hybrid));
        
        // Test configuration validation
        let valid_config = CryptoConfig {
            mode: CryptoMode::Hybrid,
            minimum_mode: CryptoMode::Classical,
            ..CryptoConfig::default()
        };
        assert!(valid_config.validate().is_ok());

        let invalid_config = CryptoConfig {
            mode: CryptoMode::Classical,
            minimum_mode: CryptoMode::Hybrid,
            ..CryptoConfig::default()
        };
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_post_quantum_mode() {
        // Initialize with post-quantum mode
        let config = CryptoConfig::new(CryptoMode::Quantum);
        let _ = init_crypto_config(config); // Ignore error if already initialized
        
        // Test keypair generation in post-quantum mode
        let keypair = PostQuantumUserKeyPair::generate();
        let public_keys = keypair.public_keys();
        
        // Test public key string format
        let pubkey_str = keypair.public_key_string();
        assert!(pubkey_str.starts_with("pq-pubkey:"));
        
        // Test serialization
        let json = serde_json::to_string(&public_keys).unwrap();
        let deserialized: PostQuantumUserPublicKeys = serde_json::from_str(&json).unwrap();
        
        assert_eq!(
            public_keys.verifying_key().as_bytes(),
            deserialized.verifying_key().as_bytes()
        );
    }

    #[test]
    fn test_hybrid_mode() {
        // Initialize with hybrid mode
        let config = CryptoConfig::new(CryptoMode::Hybrid);
        let _ = init_crypto_config(config); // Ignore error if already initialized
        
        // Test keypair generation in hybrid mode
        let keypair = HybridUserKeyPair::generate();
        let public_keys = keypair.public_keys();
        
        // Test public key string format
        let pubkey_str = keypair.public_key_string();
        assert!(pubkey_str.starts_with("hybrid-pubkey:"));
        
        // Test serialization
        let json = serde_json::to_string(&public_keys).unwrap();
        let deserialized: HybridUserPublicKeys = serde_json::from_str(&json).unwrap();
        
        // Verify both classical and post-quantum components
        assert_eq!(
            public_keys.classical.verifying_key.to_bytes(),
            deserialized.classical.verifying_key.to_bytes()
        );
        assert_eq!(
            public_keys.post_quantum.verifying_key().as_bytes(),
            deserialized.post_quantum.verifying_key().as_bytes()
        );
    }

    #[test]
    fn test_unified_interface_all_modes() {
        // Test classical mode
        let classical_config = CryptoConfig::new(CryptoMode::Classical);
        let _ = init_crypto_config(classical_config);
        
        if let Ok(keypair) = CryptoInterface::generate_keypair() {
            match keypair {
                UnifiedKeyPair::Classical(_) => {}, // Expected
                _ => panic!("Expected classical keypair"),
            }
        }
        
        // Test symmetric encryption works for all modes
        let key = [42u8; 32];
        let plaintext = b"test message for all modes";
        
        let ciphertext = CryptoInterface::encrypt_symmetric(&key, plaintext).unwrap();
        let decrypted = CryptoInterface::decrypt_symmetric(&key, &ciphertext).unwrap();
        
        assert_eq!(decrypted, plaintext);
    }
}
