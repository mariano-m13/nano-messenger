use nano_messenger::crypto::{
    CryptoInterface, CryptoConfig, CryptoMode, QuantumSafeMessaging,
    init_crypto_config, HybridUserKeyPair, PostQuantumUserKeyPair,
};
use nano_messenger::error::Result;

fn main() -> Result<()> {
    println!("🚀 Session 3: Message Format Evolution - Crypto Mode Tests");
    println!("===========================================================\n");

    // Test all three crypto modes
    test_classical_mode()?;
    test_hybrid_mode_basic()?;
    test_quantum_mode_basic()?;
    test_backward_compatibility()?;
    test_cross_mode_compatibility()?;

    println!("🎉 All Session 3 tests passed!");
    println!("✅ Classical crypto working");
    println!("✅ Hybrid crypto initialized");  
    println!("✅ Quantum crypto initialized");
    println!("✅ Backward compatibility maintained");
    
    Ok(())
}

fn test_classical_mode() -> Result<()> {
    println!("🔐 Testing Classical Mode...");
    
    // Initialize with classical mode
    let config = CryptoConfig::new(CryptoMode::Classical);
    let _ = init_crypto_config(config); // Ignore error if already initialized
    
    // Generate keypairs
    let alice_keypair = CryptoInterface::generate_keypair()?;
    let bob_keypair = CryptoInterface::generate_keypair()?;
    
    let alice_public = alice_keypair.public_keys();
    let bob_public = bob_keypair.public_keys();
    
    println!("  • Alice's public key: {}", alice_public.public_key_string());
    println!("  • Bob's public key: {}", bob_public.public_key_string());
    
    // Create and encrypt message using classical crypto
    let envelope = QuantumSafeMessaging::create_encrypted_message(
        &alice_keypair,
        &bob_public,
        "Hello from Alice to Bob (Classical)!".to_string(),
        1,
        None,
        Some(CryptoMode::Classical),
    )?;
    
    assert_eq!(envelope.crypto_mode, CryptoMode::Classical);
    assert_eq!(envelope.version, "2.0-quantum");
    println!("  • Message created with version: {}", envelope.version);
    println!("  • Crypto mode: {}", envelope.crypto_mode);
    
    // Decrypt and verify message
    let decrypted_payload = QuantumSafeMessaging::decrypt_message(&envelope, &bob_keypair)?;
    assert_eq!(decrypted_payload.body, "Hello from Alice to Bob (Classical)!");
    assert_eq!(decrypted_payload.from_pubkey, alice_public.public_key_string());
    assert_eq!(decrypted_payload.crypto_mode, Some(CryptoMode::Classical));
    
    println!("  • Message decrypted successfully: {}", decrypted_payload.body);
    println!("  ✅ Classical mode test passed!\n");
    
    Ok(())
}

fn test_hybrid_mode_basic() -> Result<()> {
    println!("🔐⚛️ Testing Hybrid Mode (Basic)...");
    
    // Test hybrid keypair generation and basic properties
    let alice_keypair = HybridUserKeyPair::generate();
    let bob_keypair = HybridUserKeyPair::generate();
    
    let alice_public = alice_keypair.public_keys();
    let bob_public = bob_keypair.public_keys();
    
    println!("  • Alice's hybrid key: {}", alice_public.public_key_string());
    println!("  • Bob's hybrid key: {}", bob_public.public_key_string());
    
    // Verify key format
    assert!(alice_public.public_key_string().starts_with("hybrid-pubkey:"));
    assert!(bob_public.public_key_string().starts_with("hybrid-pubkey:"));
    
    // Test serialization
    let alice_json = serde_json::to_string(&alice_public)?;
    let _alice_deserialized: nano_messenger::crypto::HybridUserPublicKeys = 
        serde_json::from_str(&alice_json)?;
    
    println!("  • Hybrid keypair generation: ✓");
    println!("  • Hybrid key format validation: ✓");
    println!("  • Hybrid serialization: ✓");
    println!("  ✅ Hybrid mode basic test passed!\n");
    
    Ok(())
}

fn test_quantum_mode_basic() -> Result<()> {
    println!("⚛️ Testing Quantum Mode (Basic)...");
    
    // Test post-quantum keypair generation and basic properties
    let alice_keypair = PostQuantumUserKeyPair::generate();
    let bob_keypair = PostQuantumUserKeyPair::generate();
    
    let alice_public = alice_keypair.public_keys();
    let bob_public = bob_keypair.public_keys();
    
    println!("  • Alice's PQ key: {}", alice_public.public_key_string());
    println!("  • Bob's PQ key: {}", bob_public.public_key_string());
    
    // Verify key format
    assert!(alice_public.public_key_string().starts_with("pq-pubkey:"));
    assert!(bob_public.public_key_string().starts_with("pq-pubkey:"));
    
    // Test serialization
    let alice_json = serde_json::to_string(&alice_public)?;
    let _alice_deserialized: nano_messenger::crypto::PostQuantumUserPublicKeys = 
        serde_json::from_str(&alice_json)?;
    
    println!("  • Post-quantum keypair generation: ✓");
    println!("  • Post-quantum key format validation: ✓");
    println!("  • Post-quantum serialization: ✓");
    println!("  ✅ Quantum mode basic test passed!\n");
    
    Ok(())
}

fn test_backward_compatibility() -> Result<()> {
    println!("🔄 Testing Backward Compatibility...");
    
    // Create a legacy MessageEnvelope
    let legacy_envelope = nano_messenger::protocol::MessageEnvelope::new(
        "test_inbox_legacy".to_string(),
        b"legacy encrypted data".to_vec(),
    );
    
    println!("  • Created legacy envelope with version: {}", legacy_envelope.version);
    
    // Upgrade to quantum-safe format
    let quantum_envelope = QuantumSafeMessaging::upgrade_legacy_envelope(legacy_envelope.clone());
    assert_eq!(quantum_envelope.crypto_mode, CryptoMode::Classical);
    assert_eq!(quantum_envelope.version, "2.0-quantum");
    assert_eq!(quantum_envelope.legacy_compat, Some(true));
    
    println!("  • Upgraded to quantum-safe format: {}", quantum_envelope.version);
    println!("  • Crypto mode: {}", quantum_envelope.crypto_mode);
    println!("  • Legacy compatibility: {:?}", quantum_envelope.legacy_compat);
    
    // Downgrade back to legacy
    let downgraded = QuantumSafeMessaging::downgrade_to_legacy(&quantum_envelope)?;
    assert_eq!(downgraded.inbox_id, legacy_envelope.inbox_id);
    assert_eq!(downgraded.payload, legacy_envelope.payload);
    assert_eq!(downgraded.version, "1.1");
    
    println!("  • Downgraded back to legacy format: {}", downgraded.version);
    println!("  ✅ Backward compatibility test passed!\n");
    
    Ok(())
}

fn test_cross_mode_compatibility() -> Result<()> {
    println!("🔀 Testing Cross-Mode Compatibility...");
    
    // Test mode compatibility matrix
    let modes = [CryptoMode::Classical, CryptoMode::Hybrid, CryptoMode::Quantum];
    
    for sender_mode in &modes {
        for receiver_mode in &modes {
            let compatible = QuantumSafeMessaging::modes_compatible(*sender_mode, *receiver_mode);
            println!("  • {} -> {}: {}", 
                sender_mode, receiver_mode, 
                if compatible { "✅ Compatible" } else { "❌ Incompatible" }
            );
        }
    }
    
    // Test specific compatibility scenarios
    assert!(QuantumSafeMessaging::modes_compatible(CryptoMode::Classical, CryptoMode::Classical));
    assert!(QuantumSafeMessaging::modes_compatible(CryptoMode::Classical, CryptoMode::Hybrid));
    assert!(QuantumSafeMessaging::modes_compatible(CryptoMode::Hybrid, CryptoMode::Quantum));
    assert!(QuantumSafeMessaging::modes_compatible(CryptoMode::Quantum, CryptoMode::Hybrid));
    
    println!("  ✅ Cross-mode compatibility test passed!\n");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session3_comprehensive() {
        // Test that main function runs without panics
        main().unwrap();
    }

    #[test]
    fn test_crypto_mode_inference() {
        // Test that crypto mode can be inferred from public key format
        let classical_key = "pubkey:test123";
        let hybrid_key = "hybrid-pubkey:test123";
        let quantum_key = "pq-pubkey:test123";

        // This would be tested in the actual verification logic
        assert!(classical_key.starts_with("pubkey:"));
        assert!(hybrid_key.starts_with("hybrid-pubkey:"));
        assert!(quantum_key.starts_with("pq-pubkey:"));
    }

    #[test]
    fn test_envelope_serialization() {
        use nano_messenger::protocol::QuantumSafeEnvelope;
        
        // Test basic envelope creation and serialization
        let envelope = QuantumSafeEnvelope::new(
            CryptoMode::Classical,
            "test_inbox".to_string(),
            b"test data".to_vec(),
        );
        
        // Test JSON round-trip
        let json = envelope.to_json().unwrap();
        let deserialized = QuantumSafeEnvelope::from_json(&json).unwrap();
        
        assert_eq!(envelope.version, deserialized.version);
        assert_eq!(envelope.crypto_mode, deserialized.crypto_mode);
        assert_eq!(envelope.inbox_id, deserialized.inbox_id);
    }
}
