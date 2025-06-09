//! Simplified Security Validation Tests for Session 7
//! 
//! This is a streamlined version that focuses on core security validation
//! while avoiding complex field name and type issues.

use nano_messenger::crypto::*;
use nano_messenger::protocol::*;
use nano_messenger::error::Result;
use std::time::Instant;

/// Test basic cryptographic operations work correctly
#[test]
fn test_basic_crypto_operations() {
    println!("Testing Basic Crypto Operations...");
    
    // Classical crypto
    let alice = ClassicalUserKeyPair::generate();
    let bob = ClassicalUserKeyPair::generate();
    
    // Test signatures
    let message = b"test message";
    let signature = ClassicalDigitalSignature::sign(&alice.signing_key, message);
    ClassicalDigitalSignature::verify(&alice.public_keys().verifying_key, message, &signature)
        .expect("Signature should verify");
    
    // Test key exchange
    let shared1 = ClassicalKeyExchange::key_exchange(&alice.x25519_key, &bob.public_keys().x25519_key).unwrap();
    let shared2 = ClassicalKeyExchange::key_exchange(&bob.x25519_key, &alice.public_keys().x25519_key).unwrap();
    assert_eq!(shared1.as_ref(), shared2.as_ref());
    
    // Test symmetric encryption
    let key = [42u8; 32];
    let plaintext = b"encryption test";
    let ciphertext = encrypt_symmetric(&key, plaintext).unwrap();
    let decrypted = decrypt_symmetric(&key, &ciphertext).unwrap();
    assert_eq!(decrypted, plaintext);
    
    println!("✓ Basic crypto operations work correctly");
}

/// Test post-quantum crypto operations
#[test]
fn test_post_quantum_crypto() {
    println!("Testing Post-Quantum Crypto...");
    
    let alice = PostQuantumUserKeyPair::generate();
    let bob = PostQuantumUserKeyPair::generate();
    
    // Test PQ signatures
    let message = b"pq test message";
    let signature = PostQuantumDigitalSignature::sign(&alice.private_key, message);
    PostQuantumDigitalSignature::verify(&alice.public_keys().public_key, message, &signature)
        .expect("PQ signature should verify");
    
    // Test PQ key encapsulation
    let (shared_secret, ciphertext) = PostQuantumKeyExchange::encapsulate(&bob.public_keys().public_key).unwrap();
    let decap_secret = PostQuantumKeyExchange::decapsulate(&bob.private_key, &ciphertext).unwrap();
    assert_eq!(shared_secret.as_ref(), decap_secret.as_ref());
    
    println!("✓ Post-quantum crypto operations work correctly");
}

/// Test hybrid crypto functionality
#[test] 
fn test_hybrid_crypto() {
    println!("Testing Hybrid Crypto...");
    
    let alice = HybridUserKeyPair::generate();
    let _bob = HybridUserKeyPair::generate();
    
    // Test that hybrid keypairs contain both classical and PQ components
    assert_eq!(alice.classical.x25519_key.to_bytes().len(), 32);
    assert_eq!(alice.classical.signing_key.to_bytes().len(), 32);
    assert!(alice.post_quantum.private_key.kem_key.len() > 0);
    
    // Test public key format
    let pubkey_str = alice.public_key_string();
    assert!(pubkey_str.starts_with("hybrid-pubkey:"));
    
    println!("✓ Hybrid crypto structure validated");
}

/// Test protocol message creation and signing
#[test]
fn test_protocol_message_security() {
    println!("Testing Protocol Message Security...");
    
    let alice = ClassicalUserKeyPair::generate();
    let alice_pubkey = alice.public_key_string();
    
    // Create and sign a message
    let mut payload = MessagePayload::new(
        alice_pubkey.clone(),
        "Test protocol message".to_string(),
        1,
        None,
    );
    
    payload.sign(&alice.signing_key).unwrap();
    assert!(!payload.sig.is_empty());
    
    // Verify signature
    payload.verify_signature().expect("Signature should verify");
    
    // Test tampering detection
    let original_body = payload.body.clone();
    payload.body = "Tampered message".to_string();
    assert!(payload.verify_signature().is_err(), "Tampered message should fail verification");
    
    payload.body = original_body;
    payload.verify_signature().expect("Restored message should verify");
    
    println!("✓ Protocol message security validated");
}

/// Test asymmetric encryption
#[test]
fn test_asymmetric_encryption() {
    println!("Testing Asymmetric Encryption...");
    
    let alice = ClassicalUserKeyPair::generate();
    let bob = ClassicalUserKeyPair::generate();
    
    let message = b"secret message for bob";
    
    // Alice encrypts for Bob
    let ciphertext = encrypt_asymmetric(&bob.public_keys().x25519_key, message).unwrap();
    
    // Bob decrypts
    let decrypted = decrypt_asymmetric(&bob.x25519_key, &ciphertext).unwrap();
    assert_eq!(decrypted, message);
    
    // Alice cannot decrypt (no access to Bob's private key)
    assert!(decrypt_asymmetric(&alice.x25519_key, &ciphertext).is_err());
    
    println!("✓ Asymmetric encryption security validated");
}

/// Test username claims
#[test]
fn test_username_claims() {
    println!("Testing Username Claims...");
    
    let alice = ClassicalUserKeyPair::generate();
    let public_keys = alice.public_keys();
    
    let mut claim = UsernameClaim::new("alice2024".to_string(), public_keys);
    claim.sign(&alice.signing_key).unwrap();
    
    // Verify claim
    claim.verify_signature().expect("Username claim should verify");
    
    // Test tampering
    let original_username = claim.username.clone();
    claim.username = "eve2024".to_string();
    assert!(claim.verify_signature().is_err(), "Tampered claim should fail");
    
    claim.username = original_username;
    claim.verify_signature().expect("Restored claim should verify");
    
    println!("✓ Username claims security validated");
}

/// Test message envelope security
#[test]
fn test_message_envelopes() {
    println!("Testing Message Envelopes...");
    
    let test_data = b"envelope test data";
    let envelope = MessageEnvelope::new("test_inbox".to_string(), test_data.to_vec());
    
    // Test basic properties
    assert_eq!(envelope.version, "1.1");
    assert_eq!(envelope.inbox_id, "test_inbox");
    assert_eq!(envelope.decode_payload().unwrap(), test_data);
    assert!(!envelope.nonce.is_empty());
    
    // Test quantum-safe envelope
    let qs_envelope = QuantumSafeEnvelope::new(CryptoMode::Classical, "test_inbox".to_string(), test_data.to_vec());
    assert_eq!(qs_envelope.version, "2.0-quantum");
    assert_eq!(qs_envelope.crypto_mode, CryptoMode::Classical);
    
    // Test conversion
    let legacy_from_qs = qs_envelope.to_legacy();
    assert_eq!(legacy_from_qs.inbox_id, "test_inbox");
    
    println!("✓ Message envelope security validated");
}

/// Test crypto mode transitions
#[test]
fn test_crypto_mode_security() {
    println!("Testing Crypto Mode Security...");
    
    // Test security level ordering
    assert!(CryptoMode::Classical.security_level() < CryptoMode::Hybrid.security_level());
    assert!(CryptoMode::Hybrid.security_level() < CryptoMode::Quantum.security_level());
    
    // Test quantum resistance
    assert!(!CryptoMode::Classical.is_quantum_resistant());
    assert!(CryptoMode::Hybrid.is_quantum_resistant());
    assert!(CryptoMode::Quantum.is_quantum_resistant());
    
    // Test transitions (upgrades allowed, downgrades not)
    assert!(CryptoMode::Classical.can_transition_to(CryptoMode::Hybrid));
    assert!(CryptoMode::Classical.can_transition_to(CryptoMode::Quantum));
    assert!(!CryptoMode::Hybrid.can_transition_to(CryptoMode::Classical));
    assert!(!CryptoMode::Quantum.can_transition_to(CryptoMode::Classical));
    
    println!("✓ Crypto mode security properties validated");
}

/// Test nonce uniqueness
#[test]
fn test_nonce_uniqueness() {
    println!("Testing Nonce Uniqueness...");
    
    let mut seen_nonces = std::collections::HashSet::new();
    
    // Generate many envelopes and check nonce uniqueness
    for i in 1..=1000 {
        let data = format!("message {}", i);
        let data_bytes = data.into_bytes();
        let envelope = MessageEnvelope::new("test".to_string(), data_bytes.clone());
        
        assert!(!seen_nonces.contains(&envelope.nonce), "Nonce collision at iteration {}", i);
        seen_nonces.insert(envelope.nonce.clone());
        
        // Test QS envelopes too
        let qs_envelope = QuantumSafeEnvelope::new(CryptoMode::Classical, "test".to_string(), data_bytes);
        assert!(!seen_nonces.contains(&qs_envelope.nonce), "QS nonce collision at iteration {}", i);
        seen_nonces.insert(qs_envelope.nonce.clone());
    }
    
    println!("✓ Nonce uniqueness validated ({} unique nonces)", seen_nonces.len());
}

/// Test key generation randomness
#[test]
fn test_key_randomness() {
    println!("Testing Key Generation Randomness...");
    
    let mut seen_keys = std::collections::HashSet::new();
    
    // Generate many keys and check for uniqueness
    for i in 1..=100 {
        let keypair = ClassicalUserKeyPair::generate();
        let key_bytes = keypair.x25519_key.to_bytes();
        
        assert!(!seen_keys.contains(&key_bytes), "Key collision at iteration {}", i);
        seen_keys.insert(key_bytes);
    }
    
    println!("✓ Key generation randomness validated ({} unique keys)", seen_keys.len());
}

/// Run all simplified security tests
pub fn run_simplified_security_validation() -> Result<bool> {
    println!("🔒 Running Simplified Security Validation...");
    println!("==========================================");
    
    let start = Instant::now();
    
    // Run core security tests
    test_basic_crypto_operations();
    test_post_quantum_crypto();
    test_hybrid_crypto();
    test_protocol_message_security();
    test_asymmetric_encryption();
    test_username_claims();
    test_message_envelopes();
    test_crypto_mode_security();
    test_nonce_uniqueness();
    test_key_randomness();
    
    let duration = start.elapsed();
    
    println!("🎉 All simplified security tests passed!");
    println!("⏱️  Completed in {:.2}s", duration.as_secs_f64());
    println!("✅ Core security properties validated");
    
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn run_simplified_validation() {
        run_simplified_security_validation().expect("Simplified security validation should pass");
    }
}
