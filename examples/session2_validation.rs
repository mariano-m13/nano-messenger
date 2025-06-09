// Session 2 Validation Example
// Tests the new post-quantum and hybrid crypto implementations

use nano_messenger::crypto::{
    CryptoConfig, CryptoMode, CryptoInterface,
    PostQuantumUserKeyPair, HybridUserKeyPair,
    init_crypto_config
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Session 2: Post-Quantum Crypto Validation");
    println!("============================================");

    // Test 1: Post-Quantum Mode
    println!("\n📡 Testing Post-Quantum Mode...");
    test_post_quantum_mode()?;

    // Test 2: Hybrid Mode  
    println!("\n🔄 Testing Hybrid Mode...");
    test_hybrid_mode()?;

    // Test 3: Mode Transitions
    println!("\n🔀 Testing Mode Transitions...");
    test_mode_transitions()?;

    // Test 4: Symmetric Encryption (works across all modes)
    println!("\n🔐 Testing Symmetric Encryption...");
    test_symmetric_encryption()?;

    println!("\n✅ All Session 2 tests passed!");
    println!("🎉 Post-quantum and hybrid cryptography working correctly!");

    Ok(())
}

fn test_post_quantum_mode() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize post-quantum mode
    let config = CryptoConfig::new(CryptoMode::Quantum);
    let _ = init_crypto_config(config); // OK if already initialized

    // Generate a post-quantum keypair directly
    let keypair = PostQuantumUserKeyPair::generate();
    let public_keys = keypair.public_keys();
    
    // Test public key string format
    let pubkey_str = keypair.public_key_string();
    println!("  • PQ Public Key: {}", &pubkey_str[..50]);
    assert!(pubkey_str.starts_with("pq-pubkey:"));
    
    // Test serialization
    let json = serde_json::to_string(&public_keys)?;
    let _deserialized: nano_messenger::crypto::PostQuantumUserPublicKeys = 
        serde_json::from_str(&json)?;
    
    println!("  • Post-quantum keypair generation: ✓");
    println!("  • Post-quantum serialization: ✓");
    
    Ok(())
}

fn test_hybrid_mode() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize hybrid mode
    let config = CryptoConfig::new(CryptoMode::Hybrid);
    let _ = init_crypto_config(config); // OK if already initialized

    // Generate a hybrid keypair directly
    let keypair = HybridUserKeyPair::generate();
    let public_keys = keypair.public_keys();
    
    // Test public key string format
    let pubkey_str = keypair.public_key_string();
    println!("  • Hybrid Public Key: {}", &pubkey_str[..50]);
    assert!(pubkey_str.starts_with("hybrid-pubkey:"));
    
    // Test serialization
    let json = serde_json::to_string(&public_keys)?;
    let _deserialized: nano_messenger::crypto::HybridUserPublicKeys = 
        serde_json::from_str(&json)?;
    
    println!("  • Hybrid keypair generation: ✓");
    println!("  • Hybrid serialization: ✓");
    
    Ok(())
}

fn test_mode_transitions() -> Result<(), Box<dyn std::error::Error>> {
    use nano_messenger::crypto::CryptoMode;
    
    // Test allowed transitions (only upgrades, no downgrades)
    assert!(CryptoMode::Classical.can_transition_to(CryptoMode::Hybrid));
    assert!(CryptoMode::Classical.can_transition_to(CryptoMode::Quantum));
    assert!(CryptoMode::Hybrid.can_transition_to(CryptoMode::Quantum));
    
    // Test forbidden transitions (no downgrades allowed)
    assert!(!CryptoMode::Hybrid.can_transition_to(CryptoMode::Classical));
    assert!(!CryptoMode::Quantum.can_transition_to(CryptoMode::Classical));
    assert!(!CryptoMode::Quantum.can_transition_to(CryptoMode::Hybrid));
    
    println!("  • Mode transition logic: ✓");
    
    Ok(())
}

fn test_symmetric_encryption() -> Result<(), Box<dyn std::error::Error>> {
    // Symmetric encryption should work the same across all modes
    // since ChaCha20Poly1305 is already quantum-resistant
    
    let key = [42u8; 32];
    let plaintext = b"Session 2 test message for symmetric encryption";
    
    let ciphertext = CryptoInterface::encrypt_symmetric(&key, plaintext)?;
    let decrypted = CryptoInterface::decrypt_symmetric(&key, &ciphertext)?;
    
    assert_eq!(decrypted, plaintext);
    
    println!("  • Symmetric encryption/decryption: ✓");
    
    Ok(())
}
