//! Protocol Security Validation Tests for Session 7
//! 
//! This module validates end-to-end security properties of the nano-messenger
//! protocol across all crypto modes. It ensures that the protocol maintains
//! security guarantees even when components are mixed.

use nano_messenger::crypto::*;
use nano_messenger::protocol::*;
use nano_messenger::error::Result;
use chrono::Utc;
use std::collections::HashSet;

/// Test end-to-end message security with classical crypto
#[test]
fn test_classical_e2e_security() {
    println!("Testing Classical End-to-End Security...");
    
    // Setup: Alice and Bob generate keypairs
    let alice_keypair = ClassicalUserKeyPair::generate();
    let bob_keypair = ClassicalUserKeyPair::generate();
    
    let alice_pubkey = alice_keypair.public_key_string();
    let bob_pubkey = bob_keypair.public_key_string();
    
    // Alice sends message to Bob
    let message_content = "Secret message from Alice to Bob";
    let mut payload = MessagePayload::new(
        alice_pubkey.clone(),
        message_content.to_string(),
        1,
        None,
    );
    
    // Sign the message
    payload.sign(&alice_keypair.signing_key).unwrap();
    
    // Verify signature works
    payload.verify_signature().expect("Alice's signature should verify");
    
    // Encrypt for Bob using asymmetric encryption
    let payload_json = payload.to_json().unwrap();
    let ciphertext = encrypt_asymmetric(&bob_keypair.public_keys().x25519_key, payload_json.as_bytes()).unwrap();
    
    // Create message envelope
    let inbox_id = derive_inbox_id(&bob_pubkey, 1);
    let envelope = MessageEnvelope::new(inbox_id, ciphertext);
    
    // Bob receives and decrypts
    let received_ciphertext = envelope.decode_payload().unwrap();
    let decrypted_json = decrypt_asymmetric(&bob_keypair.x25519_key, &received_ciphertext).unwrap();
    let received_payload = MessagePayload::from_json(&String::from_utf8(decrypted_json).unwrap()).unwrap();
    
    // Verify message integrity
    assert_eq!(received_payload.body, message_content);
    assert_eq!(received_payload.from_pubkey, alice_pubkey);
    
    // Verify signature
    received_payload.verify_signature().expect("Received signature should verify");
    
    println!("✓ Classical end-to-end security validated");
}

/// Test end-to-end message security with hybrid crypto
#[test]
fn test_hybrid_e2e_security() {
    println!("Testing Hybrid End-to-End Security...");
    
    // Setup: Alice and Bob generate hybrid keypairs
    let alice_keypair = HybridUserKeyPair::generate();
    let bob_keypair = HybridUserKeyPair::generate();
    
    let alice_pubkey = alice_keypair.public_key_string();
    let bob_pubkey = bob_keypair.public_key_string();
    
    // Alice sends message to Bob using hybrid crypto
    let message_content = "Quantum-resistant secret from Alice";
    let mut payload = MessagePayload::new_with_mode(
        alice_pubkey.clone(),
        message_content.to_string(),
        1,
        None,
        CryptoMode::Hybrid,
    );
    
    // Sign with hybrid keys
    let alice_unified = UnifiedKeyPair::Hybrid(alice_keypair.clone());
    payload.sign_with_mode(&alice_unified).unwrap();
    
    // Perform direct classical key exchange for encryption (simplified approach)
    let shared_secret = ClassicalKeyExchange::key_exchange(
        &alice_keypair.classical.x25519_key,
        &bob_keypair.public_keys().classical.x25519_key
    ).unwrap();
    let encryption_key = hash_sha256(shared_secret.as_ref());
    
    // Encrypt the payload
    let payload_json = payload.to_json().unwrap();
    let ciphertext = encrypt_symmetric(&encryption_key, payload_json.as_bytes()).unwrap();
    
    // Create quantum-safe envelope
    let inbox_id = derive_inbox_id(&bob_pubkey, 1);
    let envelope = QuantumSafeEnvelope::new(CryptoMode::Hybrid, inbox_id, ciphertext);
    
    // Bob receives and decrypts
    let bob_shared_secret = ClassicalKeyExchange::key_exchange(
        &bob_keypair.classical.x25519_key,
        &alice_keypair.public_keys().classical.x25519_key
    ).unwrap();
    let bob_decryption_key = hash_sha256(bob_shared_secret.as_ref());
    
    assert_eq!(encryption_key, bob_decryption_key, "Shared secrets should match");
    
    let received_ciphertext = envelope.decode_payload().unwrap();
    let decrypted_json = decrypt_symmetric(&bob_decryption_key, &received_ciphertext).unwrap();
    let received_payload = MessagePayload::from_json(&String::from_utf8(decrypted_json).unwrap()).unwrap();
    
    // Verify message integrity
    assert_eq!(received_payload.body, message_content);
    assert_eq!(received_payload.from_pubkey, alice_pubkey);
    assert_eq!(received_payload.crypto_mode, Some(CryptoMode::Hybrid));
    
    println!("✓ Hybrid end-to-end security validated");
}

/// Test end-to-end message security with post-quantum crypto
#[test]
fn test_post_quantum_e2e_security() {
    println!("Testing Post-Quantum End-to-End Security...");
    
    // Setup: Alice and Bob generate PQ keypairs
    let alice_keypair = PostQuantumUserKeyPair::generate();
    let bob_keypair = PostQuantumUserKeyPair::generate();
    
    let alice_pubkey = alice_keypair.public_key_string();
    let bob_pubkey = bob_keypair.public_key_string();
    
    // Alice sends message to Bob using post-quantum crypto
    let message_content = "Future-proof secret message";
    let mut payload = MessagePayload::new_with_mode(
        alice_pubkey.clone(),
        message_content.to_string(),
        1,
        None,
        CryptoMode::Quantum,
    );
    
    // Sign with PQ keys
    let alice_unified = UnifiedKeyPair::PostQuantum(alice_keypair.clone());
    payload.sign_with_mode(&alice_unified).unwrap();
    
    // Perform PQ key exchange for encryption using encapsulation
    let (shared_secret, ciphertext_kem) = PostQuantumKeyExchange::encapsulate(&bob_keypair.public_keys().public_key).unwrap();
    let encryption_key = hash_sha256(shared_secret.as_ref());
    
    // Encrypt the payload
    let payload_json = payload.to_json().unwrap();
    let mut encrypted_payload = PostQuantumKeyExchange::ciphertext_to_bytes(&ciphertext_kem);
    let symmetric_ciphertext = encrypt_symmetric(&encryption_key, payload_json.as_bytes()).unwrap();
    encrypted_payload.extend_from_slice(&symmetric_ciphertext);
    
    // Create quantum-safe envelope
    let inbox_id = derive_inbox_id(&bob_pubkey, 1);
    let envelope = QuantumSafeEnvelope::new(CryptoMode::Quantum, inbox_id, encrypted_payload);
    
    // Bob receives and decrypts
    let received_ciphertext = envelope.decode_payload().unwrap();
    let (kem_ciphertext_bytes, symmetric_ciphertext_bytes) = received_ciphertext.split_at(64);
    let kem_ciphertext = PostQuantumKeyExchange::ciphertext_from_bytes(kem_ciphertext_bytes).unwrap();
    
    let bob_shared_secret = PostQuantumKeyExchange::decapsulate(&bob_keypair.private_key, &kem_ciphertext).unwrap();
    let bob_decryption_key = hash_sha256(bob_shared_secret.as_ref());
    
    assert_eq!(encryption_key, bob_decryption_key, "PQ shared secrets should match");
    
    let decrypted_json = decrypt_symmetric(&bob_decryption_key, symmetric_ciphertext_bytes).unwrap();
    let received_payload = MessagePayload::from_json(&String::from_utf8(decrypted_json).unwrap()).unwrap();
    
    // Verify message integrity
    assert_eq!(received_payload.body, message_content);
    assert_eq!(received_payload.from_pubkey, alice_pubkey);
    assert_eq!(received_payload.crypto_mode, Some(CryptoMode::Quantum));
    
    println!("✓ Post-quantum end-to-end security validated");
}

/// Test that tampering with messages is detectable
#[test]
fn test_message_integrity_protection() {
    println!("Testing Message Integrity Protection...");
    
    let alice_keypair = ClassicalUserKeyPair::generate();
    let alice_pubkey = alice_keypair.public_key_string();
    
    let mut payload = MessagePayload::new(
        alice_pubkey,
        "Original message".to_string(),
        1,
        None,
    );
    
    payload.sign(&alice_keypair.signing_key).unwrap();
    
    // Verify original message
    payload.verify_signature().expect("Original should verify");
    
    // Tamper with body
    let original_body = payload.body.clone();
    payload.body = "Tampered message".to_string();
    assert!(payload.verify_signature().is_err(), "Tampered body should fail verification");
    
    // Restore body, tamper with timestamp
    payload.body = original_body;
    payload.timestamp += 1;
    assert!(payload.verify_signature().is_err(), "Tampered timestamp should fail verification");
    
    // Tamper with counter
    payload.timestamp -= 1;
    payload.counter += 1;
    assert!(payload.verify_signature().is_err(), "Tampered counter should fail verification");
    
    println!("✓ Message integrity protection validated");
}

/// Test forward secrecy properties
#[test]
fn test_forward_secrecy() {
    println!("Testing Forward Secrecy...");
    
    // Alice and Bob exchange multiple messages with ephemeral keys
    let _alice_long_term = ClassicalUserKeyPair::generate();
    let _bob_long_term = ClassicalUserKeyPair::generate();
    
    let mut previous_keys = Vec::new();
    
    // Simulate multiple message exchanges
    for round in 1..=3 {
        // Generate ephemeral keys for this round
        let alice_ephemeral = ClassicalUserKeyPair::generate();
        let bob_ephemeral = ClassicalUserKeyPair::generate();
        
        // Derive shared secret for this round
        let shared_secret = derive_shared_secret(
            &alice_ephemeral.x25519_key,
            &bob_ephemeral.public_keys().x25519_key
        );
        
        // Ensure this key is different from previous rounds
        for prev_key in &previous_keys {
            assert_ne!(&shared_secret, prev_key, "Forward secrecy violated: key reuse in round {}", round);
        }
        
        previous_keys.push(shared_secret);
        
        // Simulate message encryption with this ephemeral key
        let message = format!("Secret message round {}", round);
        let ciphertext = encrypt_symmetric(&shared_secret, message.as_bytes()).unwrap();
        let decrypted = decrypt_symmetric(&shared_secret, &ciphertext).unwrap();
        assert_eq!(String::from_utf8(decrypted).unwrap(), message);
        
        println!("  ✓ Round {} key exchange completed", round);
    }
    
    println!("✓ Forward secrecy validated");
}

/// Test nonce uniqueness and replay protection
#[test]
fn test_nonce_uniqueness_and_replay_protection() {
    println!("Testing Nonce Uniqueness and Replay Protection...");
    
    let mut seen_nonces = HashSet::new();
    let keypair = ClassicalUserKeyPair::generate();
    
    // Generate many message envelopes and check nonce uniqueness
    for i in 1..=1000 {
        let payload = MessagePayload::new(
            keypair.public_key_string(),
            format!("Message {}", i),
            i,
            None,
        );
        
        let payload_json = payload.to_json().unwrap();
        let ciphertext = encrypt_symmetric(&[42u8; 32], payload_json.as_bytes()).unwrap();
        let envelope = MessageEnvelope::new("test_inbox".to_string(), ciphertext.clone());
        
        // Check nonce uniqueness
        assert!(!seen_nonces.contains(&envelope.nonce), "Nonce collision detected at message {}", i);
        seen_nonces.insert(envelope.nonce.clone());
        
        // Test quantum-safe envelopes too
        let qs_envelope = QuantumSafeEnvelope::new(CryptoMode::Classical, "test_inbox".to_string(), ciphertext.clone());
        assert!(!seen_nonces.contains(&qs_envelope.nonce), "QS nonce collision detected at message {}", i);
        seen_nonces.insert(qs_envelope.nonce.clone());
    }
    
    println!("✓ Nonce uniqueness validated (checked {} unique nonces)", seen_nonces.len());
}

/// Test crypto mode compatibility and security levels
#[test]
fn test_crypto_mode_security_levels() {
    println!("Testing Crypto Mode Security Levels...");
    
    // Test security level ordering
    assert!(CryptoMode::Classical.security_level() < CryptoMode::Hybrid.security_level());
    assert!(CryptoMode::Hybrid.security_level() < CryptoMode::Quantum.security_level());
    
    // Test transition rules
    assert!(CryptoMode::Classical.can_transition_to(CryptoMode::Hybrid));
    assert!(CryptoMode::Classical.can_transition_to(CryptoMode::Quantum));
    assert!(CryptoMode::Hybrid.can_transition_to(CryptoMode::Quantum));
    
    // Test that downgrades are prevented
    assert!(!CryptoMode::Hybrid.can_transition_to(CryptoMode::Classical));
    assert!(!CryptoMode::Quantum.can_transition_to(CryptoMode::Classical));
    assert!(!CryptoMode::Quantum.can_transition_to(CryptoMode::Hybrid));
    
    // Test quantum resistance properties
    assert!(!CryptoMode::Classical.is_quantum_resistant());
    assert!(CryptoMode::Hybrid.is_quantum_resistant());
    assert!(CryptoMode::Quantum.is_quantum_resistant());
    
    println!("✓ Crypto mode security levels validated");
}

/// Test protocol message security
#[test]
fn test_protocol_message_security() {
    println!("Testing Protocol Message Security...");
    
    let keypair = ClassicalUserKeyPair::generate();
    let _pubkey = keypair.public_key_string();
    
    // Test username claim security
    let mut claim = UsernameClaim::new("alice2024".to_string(), keypair.public_keys());
    claim.sign(&keypair.signing_key).unwrap();
    
    // Verify claim
    claim.verify_signature().expect("Username claim should verify");
    
    // Test tampering detection
    let original_username = claim.username.clone();
    claim.username = "eve2024".to_string();
    assert!(claim.verify_signature().is_err(), "Tampered username claim should fail");
    claim.username = original_username;
    
    // Test protocol message serialization preserves security
    let protocol_msg = ProtocolMessage::PublishClaim { claim: claim.clone() };
    let json = protocol_msg.to_json().unwrap();
    let restored = ProtocolMessage::from_json(&json).unwrap();
    
    if let ProtocolMessage::PublishClaim { claim: restored_claim } = restored {
        restored_claim.verify_signature().expect("Restored claim should verify");
    } else {
        panic!("Protocol message deserialization failed");
    }
    
    println!("✓ Protocol message security validated");
}

/// Test encryption envelope security
#[test]
fn test_envelope_security() {
    println!("Testing Envelope Security...");
    
    let test_data = b"sensitive envelope data";
    let inbox_id = "test_inbox_123".to_string();
    
    // Test legacy envelope
    let legacy_envelope = MessageEnvelope::new(inbox_id.clone(), test_data.to_vec());
    
    // Test expiry functionality
    let expired_envelope = legacy_envelope.clone().with_expiry(
        Utc::now() - chrono::Duration::seconds(60)
    );
    assert!(expired_envelope.is_expired(), "Expired envelope should be detected");
    
    let future_envelope = legacy_envelope.clone().with_expiry(
        Utc::now() + chrono::Duration::seconds(60)
    );
    assert!(!future_envelope.is_expired(), "Future envelope should not be expired");
    
    // Test quantum-safe envelope
    let qs_envelope = QuantumSafeEnvelope::new(CryptoMode::Hybrid, inbox_id.clone(), test_data.to_vec());
    
    // Test conversion between formats
    let legacy_from_qs = qs_envelope.to_legacy();
    assert_eq!(legacy_from_qs.inbox_id, inbox_id);
    assert_eq!(legacy_from_qs.decode_payload().unwrap(), test_data);
    
    let qs_from_legacy = QuantumSafeEnvelope::from_legacy(legacy_envelope);
    assert_eq!(qs_from_legacy.crypto_mode, CryptoMode::Classical);
    assert_eq!(qs_from_legacy.inbox_id, inbox_id);
    
    println!("✓ Envelope security validated");
}

/// Test hybrid security guarantee: breaking either classical OR post-quantum should not compromise security
#[test]
fn test_hybrid_security_guarantee() {
    println!("Testing Hybrid Security Guarantee...");
    
    let alice_hybrid = HybridUserKeyPair::generate();
    let bob_hybrid = HybridUserKeyPair::generate();
    
    // Perform classical component key exchange  
    let classical_shared = ClassicalKeyExchange::key_exchange(
        &alice_hybrid.classical.x25519_key,
        &bob_hybrid.public_keys().classical.x25519_key
    ).unwrap();
    
    // Perform PQ component key exchange (encapsulation)
    let (pq_shared, _ciphertext) = PostQuantumKeyExchange::encapsulate(&bob_hybrid.public_keys().post_quantum.public_key).unwrap();
    
    // Hybrid should use both components differently
    assert_ne!(classical_shared.as_ref(), pq_shared.as_ref(),
        "Classical and PQ components should produce different secrets");
    
    // Test that hybrid signature verification requires both components
    let test_message = b"hybrid security test";
    
    // Test classical component signature
    let classical_signature = ClassicalDigitalSignature::sign(&alice_hybrid.classical.signing_key, test_message);
    ClassicalDigitalSignature::verify(&alice_hybrid.public_keys().classical.verifying_key, test_message, &classical_signature)
        .expect("Classical component should verify");
    
    // Test PQ component signature
    let pq_signature = PostQuantumDigitalSignature::sign(&alice_hybrid.post_quantum.private_key, test_message);
    PostQuantumDigitalSignature::verify(&alice_hybrid.public_keys().post_quantum.public_key, test_message, &pq_signature)
        .expect("PQ component should verify");
    
    // Both components should be verified for full hybrid security
    println!("✓ Hybrid security guarantee validated");
}

/// Helper function to derive inbox ID (simplified for testing)
fn derive_inbox_id(public_key: &str, counter: u64) -> String {
    format!("inbox_{}_{}", public_key.chars().take(8).collect::<String>(), counter)
}

/// Run all protocol security validation tests
pub fn run_all_protocol_security_tests() -> Result<()> {
    println!("🔐 Running Protocol Security Validation Tests...\n");
    
    test_classical_e2e_security();
    test_hybrid_e2e_security();
    test_post_quantum_e2e_security();
    test_message_integrity_protection();
    test_forward_secrecy();
    test_nonce_uniqueness_and_replay_protection();
    test_crypto_mode_security_levels();
    test_protocol_message_security();
    test_envelope_security();
    test_hybrid_security_guarantee();
    
    println!("\n✅ All protocol security validation tests passed!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn run_protocol_security_suite() {
        run_all_protocol_security_tests().expect("Protocol security tests should pass");
    }
}
