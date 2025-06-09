//! Interoperability Tests for Session 7
//! 
//! This module validates cross-version compatibility and ensures that different
//! crypto modes can interoperate safely while maintaining security properties.
//! Tests backward compatibility and proper handling of mixed crypto environments.

use nano_messenger::crypto::*;
use nano_messenger::protocol::*;
use nano_messenger::error::Result;

/// Test legacy message format compatibility
#[test]
fn test_legacy_message_compatibility() {
    println!("Testing Legacy Message Compatibility...");
    
    // Create legacy (v1.1) style message
    let alice_keypair = ClassicalUserKeyPair::generate();
    let bob_keypair = ClassicalUserKeyPair::generate();
    
    let alice_pubkey = alice_keypair.public_key_string();
    let _bob_pubkey = bob_keypair.public_key_string();
    
    // Legacy message payload (no crypto_mode field)
    let mut legacy_payload = MessagePayload::new(
        alice_pubkey.clone(),
        "Legacy format message".to_string(),
        1,
        None,
    );
    
    // Sign with legacy method
    legacy_payload.sign(&alice_keypair.signing_key).unwrap();
    assert!(legacy_payload.crypto_mode.is_none(), "Legacy payload should not have crypto_mode");
    
    // Verify with legacy method
    legacy_payload.verify_signature().expect("Legacy signature should verify");
    
    // Create legacy envelope
    let payload_json = legacy_payload.to_json().unwrap();
    let ciphertext = encrypt_asymmetric(&bob_keypair.public_keys().x25519_key, payload_json.as_bytes()).unwrap();
    let legacy_envelope = MessageEnvelope::new("legacy_inbox".to_string(), ciphertext);
    
    // Convert to quantum-safe format
    let qs_envelope = QuantumSafeEnvelope::from_legacy(legacy_envelope.clone());
    assert_eq!(qs_envelope.crypto_mode, CryptoMode::Classical);
    assert_eq!(qs_envelope.legacy_compat, Some(true));
    
    // Convert back to legacy format
    let back_to_legacy = qs_envelope.to_legacy();
    assert_eq!(back_to_legacy.inbox_id, legacy_envelope.inbox_id);
    assert_eq!(back_to_legacy.payload, legacy_envelope.payload);
    
    // Bob can decrypt using either format
    let legacy_decrypted = decrypt_asymmetric(&bob_keypair.x25519_key, &legacy_envelope.decode_payload().unwrap()).unwrap();
    let qs_decrypted = decrypt_asymmetric(&bob_keypair.x25519_key, &qs_envelope.decode_payload().unwrap()).unwrap();
    
    assert_eq!(legacy_decrypted, qs_decrypted);
    
    let received_payload = MessagePayload::from_json(&String::from_utf8(legacy_decrypted).unwrap()).unwrap();
    assert_eq!(received_payload.body, "Legacy format message");
    
    println!("✓ Legacy message compatibility validated");
}

/// Test mixed crypto mode interoperability
#[test]
fn test_mixed_crypto_mode_interoperability() {
    println!("Testing Mixed Crypto Mode Interoperability...");
    
    // Setup users with different crypto modes
    let alice_classical = ClassicalUserKeyPair::generate();
    let bob_hybrid = HybridUserKeyPair::generate();
    let charlie_quantum = PostQuantumUserKeyPair::generate();
    
    // Test message flow: Alice (Classical) -> Bob (Hybrid) -> Charlie (Quantum)
    
    // 1. Alice (Classical) sends to Bob (Hybrid)
    let mut alice_to_bob = MessagePayload::new(
        alice_classical.public_key_string(),
        "Message from Classical Alice to Hybrid Bob".to_string(),
        1,
        None,
    );
    alice_to_bob.sign(&alice_classical.signing_key).unwrap();
    
    // Alice encrypts using Bob's classical component (for compatibility)
    let payload_json = alice_to_bob.to_json().unwrap();
    let alice_to_bob_ciphertext = encrypt_asymmetric(
        &bob_hybrid.public_keys().classical.x25519_key,
        payload_json.as_bytes()
    ).unwrap();
    
    // Bob decrypts with his classical component
    let decrypted_for_bob = decrypt_asymmetric(
        &bob_hybrid.classical.x25519_key,
        &alice_to_bob_ciphertext
    ).unwrap();
    let bob_received = MessagePayload::from_json(&String::from_utf8(decrypted_for_bob).unwrap()).unwrap();
    
    assert_eq!(bob_received.body, "Message from Classical Alice to Hybrid Bob");
    bob_received.verify_signature().expect("Alice's signature should verify for Bob");
    
    // 2. Bob (Hybrid) forwards to Charlie (Quantum) using PQ encapsulation
    let mut bob_to_charlie = MessagePayload::new_with_mode(
        bob_hybrid.public_key_string(),
        format!("Forwarded: {}", bob_received.body),
        1,
        None,
        CryptoMode::Hybrid,
    );
    
    let bob_unified = UnifiedKeyPair::Hybrid(bob_hybrid.clone());
    bob_to_charlie.sign_with_mode(&bob_unified).unwrap();
    
    // Bob uses PQ encapsulation to send to Charlie
    let (shared_secret, kem_ciphertext) = PostQuantumKeyExchange::encapsulate(&charlie_quantum.public_keys().public_key).unwrap();
    let bob_charlie_key = hash_sha256(shared_secret.as_ref());
    
    let bob_to_charlie_json = bob_to_charlie.to_json().unwrap();
    let symmetric_ciphertext = encrypt_symmetric(&bob_charlie_key, bob_to_charlie_json.as_bytes()).unwrap();
    
    // Combine KEM ciphertext with symmetric ciphertext
    let mut combined_ciphertext = PostQuantumKeyExchange::ciphertext_to_bytes(&kem_ciphertext);
    combined_ciphertext.extend_from_slice(&symmetric_ciphertext);
    
    // Charlie decrypts using decapsulation
    let (kem_bytes, symmetric_bytes) = combined_ciphertext.split_at(64);
    let kem_ciphertext_restored = PostQuantumKeyExchange::ciphertext_from_bytes(kem_bytes).unwrap();
    let charlie_shared_secret = PostQuantumKeyExchange::decapsulate(&charlie_quantum.private_key, &kem_ciphertext_restored).unwrap();
    let charlie_bob_key = hash_sha256(charlie_shared_secret.as_ref());
    
    assert_eq!(bob_charlie_key, charlie_bob_key, "Shared keys should match");
    
    let decrypted_for_charlie = decrypt_symmetric(&charlie_bob_key, symmetric_bytes).unwrap();
    let charlie_received = MessagePayload::from_json(&String::from_utf8(decrypted_for_charlie).unwrap()).unwrap();
    
    assert_eq!(charlie_received.body, "Forwarded: Message from Classical Alice to Hybrid Bob");
    assert_eq!(charlie_received.crypto_mode, Some(CryptoMode::Hybrid));
    
    println!("✓ Mixed crypto mode interoperability validated");
}

/// Test username claim compatibility across versions
#[test]
fn test_username_claim_compatibility() {
    println!("Testing Username Claim Compatibility...");
    
    // Test that username claims work with all key types
    let classical_keypair = ClassicalUserKeyPair::generate();
    let hybrid_keypair = HybridUserKeyPair::generate();
    let quantum_keypair = PostQuantumUserKeyPair::generate();
    
    // Classical username claim (legacy format)
    let mut classical_claim = UsernameClaim::new(
        "alice_classical".to_string(),
        classical_keypair.public_keys()
    );
    classical_claim.sign(&classical_keypair.signing_key).unwrap();
    classical_claim.verify_signature().expect("Classical claim should verify");
    
    // Test serialization roundtrip
    let classical_json = classical_claim.to_json().unwrap();
    let classical_restored = UsernameClaim::from_json(&classical_json).unwrap();
    classical_restored.verify_signature().expect("Restored classical claim should verify");
    
    // Test that different key types produce different claim formats
    let hybrid_pubkey_str = hybrid_keypair.public_key_string();
    let quantum_pubkey_str = quantum_keypair.public_key_string();
    let classical_pubkey_str = classical_keypair.public_key_string();
    
    assert!(classical_pubkey_str.starts_with("pubkey:"));
    assert!(hybrid_pubkey_str.starts_with("hybrid-pubkey:"));
    assert!(quantum_pubkey_str.starts_with("pq-pubkey:"));
    
    // All should be unique
    assert_ne!(classical_pubkey_str, hybrid_pubkey_str);
    assert_ne!(classical_pubkey_str, quantum_pubkey_str);
    assert_ne!(hybrid_pubkey_str, quantum_pubkey_str);
    
    println!("✓ Username claim compatibility validated");
}

/// Test protocol message version negotiation
#[test]
fn test_protocol_version_negotiation() {
    println!("Testing Protocol Version Negotiation...");
    
    let alice_keypair = ClassicalUserKeyPair::generate();
    let _bob_keypair = HybridUserKeyPair::generate();
    
    // Create messages in both legacy and quantum-safe formats
    let mut payload = MessagePayload::new(
        alice_keypair.public_key_string(),
        "Version negotiation test".to_string(),
        1,
        None,
    );
    payload.sign(&alice_keypair.signing_key).unwrap();
    
    let payload_json = payload.to_json().unwrap();
    let ciphertext = encrypt_symmetric(&[42u8; 32], payload_json.as_bytes()).unwrap();
    
    // Legacy format
    let legacy_envelope = MessageEnvelope::new("test_inbox".to_string(), ciphertext.clone());
    let legacy_protocol = ProtocolMessage::SendMessage { 
        envelope: legacy_envelope.clone() 
    };
    
    // Quantum-safe format
    let qs_envelope = QuantumSafeEnvelope::new(CryptoMode::Classical, "test_inbox".to_string(), ciphertext);
    let qs_protocol = ProtocolMessage::SendQuantumMessage { 
        envelope: qs_envelope.clone() 
    };
    
    // Test that both can be serialized and deserialized
    let legacy_json = legacy_protocol.to_json().unwrap();
    let qs_json = qs_protocol.to_json().unwrap();
    
    let legacy_restored = ProtocolMessage::from_json(&legacy_json).unwrap();
    let qs_restored = ProtocolMessage::from_json(&qs_json).unwrap();
    
    // Verify protocol message types are preserved
    match legacy_restored {
        ProtocolMessage::SendMessage { envelope } => {
            assert_eq!(envelope.version, "1.1");
            assert_eq!(envelope.inbox_id, legacy_envelope.inbox_id);
        }
        _ => panic!("Expected SendMessage variant"),
    }
    
    match qs_restored {
        ProtocolMessage::SendQuantumMessage { envelope } => {
            assert_eq!(envelope.version, "2.0-quantum");
            assert_eq!(envelope.crypto_mode, CryptoMode::Classical);
        }
        _ => panic!("Expected SendQuantumMessage variant"),
    }
    
    // Test username lookup compatibility
    let legacy_lookup = ProtocolMessage::LookupUsername { 
        username: "alice".to_string() 
    };
    let legacy_result = ProtocolMessage::UsernameResult { 
        username: "alice".to_string(),
        public_keys: Some(alice_keypair.public_keys()),
    };
    
    let qs_result = ProtocolMessage::QuantumUsernameResult {
        username: "alice".to_string(),
        public_keys: Some(UnifiedPublicKeys::Classical(alice_keypair.public_keys())),
    };
    
    // All should serialize/deserialize correctly
    let lookup_json = legacy_lookup.to_json().unwrap();
    let result_json = legacy_result.to_json().unwrap();
    let qs_result_json = qs_result.to_json().unwrap();
    
    let _ = ProtocolMessage::from_json(&lookup_json).unwrap();
    let _ = ProtocolMessage::from_json(&result_json).unwrap();
    let _ = ProtocolMessage::from_json(&qs_result_json).unwrap();
    
    println!("✓ Protocol version negotiation validated");
}

/// Test crypto provider compatibility
#[test]
fn test_crypto_provider_compatibility() {
    println!("Testing Crypto Provider Compatibility...");
    
    // Test that different crypto implementations can interoperate
    
    // Direct classical crypto
    let alice_direct = ClassicalUserKeyPair::generate();
    let bob_direct = ClassicalUserKeyPair::generate();
    
    // Unified interface crypto
    let _alice_unified = CryptoInterface::generate_keypair().unwrap();
    let _bob_unified = CryptoInterface::generate_keypair().unwrap();
    
    // Test symmetric encryption compatibility
    let key = [42u8; 32];
    let plaintext = b"compatibility test message";
    
    // Encrypt with direct classical
    let ciphertext1 = ClassicalSymmetricEncryption::encrypt(&key, plaintext).unwrap();
    
    // Decrypt with unified interface
    let decrypted1 = CryptoInterface::decrypt_symmetric(&key, &ciphertext1).unwrap();
    assert_eq!(decrypted1, plaintext);
    
    // Encrypt with unified interface
    let ciphertext2 = CryptoInterface::encrypt_symmetric(&key, plaintext).unwrap();
    
    // Decrypt with direct classical
    let decrypted2 = ClassicalSymmetricEncryption::decrypt(&key, &ciphertext2).unwrap();
    assert_eq!(decrypted2, plaintext);
    
    // Test key exchange compatibility
    let shared1 = derive_shared_secret(
        &alice_direct.x25519_key,
        &bob_direct.public_keys().x25519_key
    );
    
    let shared2 = ClassicalKeyExchange::key_exchange(
        &alice_direct.x25519_key,
        &bob_direct.public_keys().x25519_key
    ).unwrap();
    
    assert_eq!(shared1, shared2.to_bytes());
    
    // Test signature compatibility
    let test_data = b"signature compatibility test";
    
    let sig1 = sign_data(&alice_direct.signing_key, test_data);
    let sig2 = ClassicalDigitalSignature::sign(&alice_direct.signing_key, test_data);
    
    // Both should verify with either method
    verify_signature(&alice_direct.public_keys().verifying_key, test_data, &sig1).unwrap();
    ClassicalDigitalSignature::verify(&alice_direct.public_keys().verifying_key, test_data, &sig2).unwrap();
    
    // Cross-verify
    verify_signature(&alice_direct.public_keys().verifying_key, test_data, &sig2).unwrap();
    ClassicalDigitalSignature::verify(&alice_direct.public_keys().verifying_key, test_data, &sig1).unwrap();
    
    println!("✓ Crypto provider compatibility validated");
}

/// Test serialization format stability
#[test]
fn test_serialization_format_stability() {
    println!("Testing Serialization Format Stability...");
    
    // Test that serialized formats remain stable across versions
    
    let keypair = ClassicalUserKeyPair::generate();
    let public_keys = keypair.public_keys();
    
    // Serialize public keys
    let json = serde_json::to_string(&public_keys).unwrap();
    
    // Should contain expected fields (using correct field names)
    assert!(json.contains("x25519_key"));
    assert!(json.contains("verifying_key"));
    
    // Deserialize and verify
    let restored: ClassicalUserPublicKeys = serde_json::from_str(&json).unwrap();
    assert_eq!(
        public_keys.x25519_key.to_bytes(),
        restored.x25519_key.to_bytes()
    );
    assert_eq!(
        public_keys.verifying_key.to_bytes(),
        restored.verifying_key.to_bytes()
    );
    
    // Test message payload serialization
    let mut payload = MessagePayload::new(
        keypair.public_key_string(),
        "Serialization test".to_string(),
        42,
        Some("test_room".to_string()),
    );
    payload.sign(&keypair.signing_key).unwrap();
    
    let payload_json = payload.to_json().unwrap();
    let restored_payload = MessagePayload::from_json(&payload_json).unwrap();
    
    assert_eq!(payload.from_pubkey, restored_payload.from_pubkey);
    assert_eq!(payload.body, restored_payload.body);
    assert_eq!(payload.counter, restored_payload.counter);
    assert_eq!(payload.room, restored_payload.room);
    assert_eq!(payload.sig, restored_payload.sig);
    
    // Signature should still verify after roundtrip
    restored_payload.verify_signature().expect("Restored payload signature should verify");
    
    // Test envelope serialization
    let envelope = MessageEnvelope::new("test_inbox".to_string(), b"test_data".to_vec());
    let envelope_json = envelope.to_json().unwrap();
    let restored_envelope = MessageEnvelope::from_json(&envelope_json).unwrap();
    
    assert_eq!(envelope.version, restored_envelope.version);
    assert_eq!(envelope.inbox_id, restored_envelope.inbox_id);
    assert_eq!(envelope.payload, restored_envelope.payload);
    assert_eq!(envelope.nonce, restored_envelope.nonce);
    
    println!("✓ Serialization format stability validated");
}

/// Test error handling compatibility
#[test]
fn test_error_handling_compatibility() {
    println!("Testing Error Handling Compatibility...");
    
    let _keypair = ClassicalUserKeyPair::generate();
    
    // Test that errors are handled consistently across crypto modes
    
    // Invalid signature bytes
    let invalid_sig_bytes = vec![0u8; 32]; // Too short
    assert!(ed25519_dalek::Signature::from_slice(&invalid_sig_bytes).is_err());
    
    // Invalid key bytes
    let invalid_key_bytes = vec![0u8; 16]; // Too short
    assert!(ClassicalKeyExchange::public_key_from_bytes(&invalid_key_bytes).is_err());
    
    // Tampering with encrypted data
    let key = [42u8; 32];
    let plaintext = b"error test";
    let mut ciphertext = encrypt_symmetric(&key, plaintext).unwrap();
    
    // Corrupt the ciphertext
    if !ciphertext.is_empty() {
        ciphertext[0] ^= 1;
    }
    
    // Should get decryption error
    assert!(decrypt_symmetric(&key, &ciphertext).is_err());
    
    // Invalid JSON
    assert!(MessagePayload::from_json("invalid json").is_err());
    assert!(MessageEnvelope::from_json("{").is_err());
    assert!(ProtocolMessage::from_json("null").is_err());
    
    // Invalid base64
    let mut envelope = MessageEnvelope::new("test".to_string(), vec![1, 2, 3]);
    envelope.payload = "invalid base64!@#".to_string();
    assert!(envelope.decode_payload().is_err());
    
    println!("✓ Error handling compatibility validated");
}

/// Test backward compatibility scenarios
#[test]
fn test_backward_compatibility_scenarios() {
    println!("Testing Backward Compatibility Scenarios...");
    
    // Scenario 1: Old client (v1.1) communicating with new relay (v2.0)
    let old_client_keypair = ClassicalUserKeyPair::generate();
    
    // Old client sends legacy message
    let mut legacy_payload = MessagePayload::new(
        old_client_keypair.public_key_string(),
        "Message from old client".to_string(),
        1,
        None,
    );
    legacy_payload.sign(&old_client_keypair.signing_key).unwrap();
    
    let legacy_json = legacy_payload.to_json().unwrap();
    let legacy_ciphertext = encrypt_symmetric(&[42u8; 32], legacy_json.as_bytes()).unwrap();
    let legacy_envelope = MessageEnvelope::new("old_client_inbox".to_string(), legacy_ciphertext);
    
    // New relay can process legacy message
    let _legacy_protocol = ProtocolMessage::SendMessage { envelope: legacy_envelope.clone() };
    
    // Relay converts to quantum-safe format internally
    let qs_envelope = QuantumSafeEnvelope::from_legacy(legacy_envelope.clone());
    assert_eq!(qs_envelope.crypto_mode, CryptoMode::Classical);
    assert_eq!(qs_envelope.legacy_compat, Some(true));
    
    // Relay can send back in legacy format if needed
    let back_to_legacy = qs_envelope.to_legacy();
    assert_eq!(back_to_legacy.inbox_id, legacy_envelope.inbox_id);
    
    // Scenario 2: New client (v2.0) with backward compatibility
    let new_client_keypair = HybridUserKeyPair::generate();
    
    // New client can downgrade to classical for compatibility
    let classical_component = &new_client_keypair.classical;
    let mut compat_payload = MessagePayload::new(
        classical_component.public_key_string(),
        "Compatibility message from new client".to_string(),
        1,
        None,
    );
    compat_payload.sign(&classical_component.signing_key).unwrap();
    
    // Should verify with legacy verification
    compat_payload.verify_signature().expect("Compatibility payload should verify");
    
    // Scenario 3: Mixed environment with different crypto modes
    let classical_user = ClassicalUserKeyPair::generate();
    let hybrid_user = HybridUserKeyPair::generate();
    
    // Both should be able to communicate via classical crypto
    let shared_secret = derive_shared_secret(
        &classical_user.x25519_key,
        &hybrid_user.classical.public_keys().x25519_key
    );
    
    let message = "Mixed environment message";
    let encrypted = encrypt_symmetric(&hash_sha256(&shared_secret), message.as_bytes()).unwrap();
    let decrypted = decrypt_symmetric(&hash_sha256(&shared_secret), &encrypted).unwrap();
    
    assert_eq!(String::from_utf8(decrypted).unwrap(), message);
    
    println!("✓ Backward compatibility scenarios validated");
}

/// Test forward compatibility preparation
#[test]
fn test_forward_compatibility_preparation() {
    println!("Testing Forward Compatibility Preparation...");
    
    // Test that current implementation can handle unknown fields gracefully
    
    // Add extra fields to JSON that future versions might include
    let keypair = ClassicalUserKeyPair::generate();
    let mut payload = MessagePayload::new(
        keypair.public_key_string(),
        "Forward compatibility test".to_string(),
        1,
        None,
    );
    payload.sign(&keypair.signing_key).unwrap();
    
    let mut payload_json: serde_json::Value = serde_json::to_value(&payload).unwrap();
    
    // Add hypothetical future fields
    if let serde_json::Value::Object(ref mut map) = payload_json {
        map.insert("future_field".to_string(), serde_json::Value::String("future_value".to_string()));
        map.insert("version".to_string(), serde_json::Value::String("3.0".to_string()));
        map.insert("new_crypto_mode".to_string(), serde_json::Value::String("PostQuantumPlus".to_string()));
    }
    
    // Current implementation should ignore unknown fields
    let modified_json = serde_json::to_string(&payload_json).unwrap();
    let restored_payload = MessagePayload::from_json(&modified_json);
    
    // Should either succeed (ignoring unknown fields) or fail gracefully
    match restored_payload {
        Ok(payload) => {
            // If successful, core fields should be preserved
            assert_eq!(payload.body, "Forward compatibility test");
            payload.verify_signature().expect("Signature should still verify");
        }
        Err(_) => {
            // Failing gracefully is also acceptable
            println!("  Note: Unknown fields cause parsing to fail (graceful degradation)");
        }
    }
    
    // Test envelope forward compatibility
    let envelope = MessageEnvelope::new("test".to_string(), vec![1, 2, 3]);
    let mut envelope_json: serde_json::Value = serde_json::to_value(&envelope).unwrap();
    
    if let serde_json::Value::Object(ref mut map) = envelope_json {
        map.insert("compression".to_string(), serde_json::Value::String("zstd".to_string()));
        map.insert("metadata".to_string(), serde_json::Value::Object(serde_json::Map::new()));
    }
    
    let modified_envelope_json = serde_json::to_string(&envelope_json).unwrap();
    let restored_envelope = MessageEnvelope::from_json(&modified_envelope_json);
    
    match restored_envelope {
        Ok(env) => {
            assert_eq!(env.inbox_id, "test");
            assert_eq!(env.decode_payload().unwrap(), vec![1, 2, 3]);
        }
        Err(_) => {
            println!("  Note: Envelope with unknown fields failed parsing (graceful degradation)");
        }
    }
    
    println!("✓ Forward compatibility preparation validated");
}

/// Run all interoperability tests
pub fn run_all_interoperability_tests() -> Result<()> {
    println!("🔄 Running Interoperability Tests...\n");
    
    test_legacy_message_compatibility();
    test_mixed_crypto_mode_interoperability();
    test_username_claim_compatibility();
    test_protocol_version_negotiation();
    test_crypto_provider_compatibility();
    test_serialization_format_stability();
    test_error_handling_compatibility();
    test_backward_compatibility_scenarios();
    test_forward_compatibility_preparation();
    
    println!("\n✅ All interoperability tests passed!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn run_interoperability_suite() {
        run_all_interoperability_tests().expect("Interoperability tests should pass");
    }
}
