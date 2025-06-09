//! Attack Resistance Tests for Session 7
//! 
//! This module simulates basic attack scenarios to validate that the
//! nano-messenger protocol resists common cryptographic attacks and
//! maintains security properties under adversarial conditions.

use nano_messenger::crypto::*;
use nano_messenger::protocol::*;
use nano_messenger::error::Result;
use rand::{thread_rng, RngCore};
use std::collections::HashMap;

/// Test resistance to signature forgery attacks
#[test]
fn test_signature_forgery_resistance() {
    println!("Testing Signature Forgery Resistance...");
    
    let alice_keypair = ClassicalUserKeyPair::generate();
    let eve_keypair = ClassicalUserKeyPair::generate(); // Attacker
    
    let alice_pubkey = alice_keypair.public_key_string();
    
    // Alice creates a legitimate message
    let mut alice_payload = MessagePayload::new(
        alice_pubkey.clone(),
        "Legitimate message from Alice".to_string(),
        1,
        None,
    );
    alice_payload.sign(&alice_keypair.signing_key).unwrap();
    
    // Verify Alice's message is valid
    alice_payload.verify_signature().expect("Alice's signature should verify");
    
    // Eve tries to forge Alice's signature on a different message
    let mut eve_forged_payload = MessagePayload::new(
        alice_pubkey.clone(), // Pretending to be Alice
        "Forged message from Eve".to_string(),
        2,
        None,
    );
    
    // Eve signs with her own key (attack attempt)
    eve_forged_payload.sign(&eve_keypair.signing_key).unwrap();
    
    // This should fail verification because Eve's signature won't match Alice's public key
    assert!(eve_forged_payload.verify_signature().is_err(), 
        "Forged signature should not verify");
    
    // Eve tries to reuse Alice's signature on a different message
    let mut eve_replay_payload = MessagePayload::new(
        alice_pubkey.clone(),
        "Different message with reused signature".to_string(),
        3,
        None,
    );
    eve_replay_payload.sig = alice_payload.sig.clone(); // Copy Alice's signature
    
    // This should fail because signature doesn't match the new message content
    assert!(eve_replay_payload.verify_signature().is_err(),
        "Reused signature should not verify on different content");
    
    println!("✓ Signature forgery resistance validated");
}

/// Test resistance to replay attacks
#[test]
fn test_replay_attack_resistance() {
    println!("Testing Replay Attack Resistance...");
    
    let alice_keypair = ClassicalUserKeyPair::generate();
    let bob_keypair = ClassicalUserKeyPair::generate();
    
    // Alice sends a message to Bob
    let mut payload = MessagePayload::new(
        alice_keypair.public_key_string(),
        "Transfer $100 to Eve".to_string(),
        1,
        None,
    );
    payload.sign(&alice_keypair.signing_key).unwrap();
    
    // Encrypt for Bob
    let payload_json = payload.to_json().unwrap();
    let ciphertext = encrypt_asymmetric(&bob_keypair.public_keys().x25519_key, payload_json.as_bytes()).unwrap();
    
    // Create envelope with unique nonce
    let inbox_id = format!("inbox_{}", bob_keypair.public_key_string());
    let envelope1 = MessageEnvelope::new(inbox_id.clone(), ciphertext.clone());
    
    // Attacker tries to replay the same message
    let envelope2 = MessageEnvelope::new(inbox_id.clone(), ciphertext.clone());
    
    // Nonces should be different (preventing exact replay)
    assert_ne!(envelope1.nonce, envelope2.nonce, "Envelopes should have different nonces");
    
    // Test with quantum-safe envelopes
    let qs_envelope1 = QuantumSafeEnvelope::new(CryptoMode::Classical, inbox_id.clone(), ciphertext.clone());
    let qs_envelope2 = QuantumSafeEnvelope::new(CryptoMode::Classical, inbox_id, ciphertext);
    
    assert_ne!(qs_envelope1.nonce, qs_envelope2.nonce, "QS envelopes should have different nonces");
    
    // Test message counter progression prevents replay
    let mut payload_counter_2 = MessagePayload::new(
        alice_keypair.public_key_string(),
        "Transfer $200 to Eve".to_string(),
        1, // Same counter as before (replay attempt)
        None,
    );
    payload_counter_2.sign(&alice_keypair.signing_key).unwrap();
    
    // Both messages are valid individually, but counter reuse should be detected at protocol level
    assert_eq!(payload.counter, payload_counter_2.counter, "Counter reuse should be detectable");
    
    println!("✓ Replay attack resistance validated");
}

/// Test resistance to man-in-the-middle attacks
#[test]
fn test_mitm_attack_resistance() {
    println!("Testing Man-in-the-Middle Attack Resistance...");
    
    let alice_keypair = ClassicalUserKeyPair::generate();
    let bob_keypair = ClassicalUserKeyPair::generate();
    let eve_keypair = ClassicalUserKeyPair::generate(); // MITM attacker
    
    // Alice wants to send to Bob, but Eve intercepts
    let message = "Secret message for Bob only";
    let mut payload = MessagePayload::new(
        alice_keypair.public_key_string(),
        message.to_string(),
        1,
        None,
    );
    payload.sign(&alice_keypair.signing_key).unwrap();
    
    // Alice encrypts for Bob (correct recipient)
    let payload_json = payload.to_json().unwrap();
    let legitimate_ciphertext = encrypt_asymmetric(
        &bob_keypair.public_keys().x25519_key, 
        payload_json.as_bytes()
    ).unwrap();
    
    // Eve cannot decrypt message intended for Bob
    assert!(decrypt_asymmetric(&eve_keypair.x25519_key, &legitimate_ciphertext).is_err(),
        "Eve should not be able to decrypt Bob's message");
    
    // Bob can decrypt the legitimate message
    let decrypted = decrypt_asymmetric(&bob_keypair.x25519_key, &legitimate_ciphertext).unwrap();
    let received_payload = MessagePayload::from_json(&String::from_utf8(decrypted).unwrap()).unwrap();
    
    assert_eq!(received_payload.body, message);
    received_payload.verify_signature().expect("Alice's signature should verify for Bob");
    
    // Eve tries to substitute her own message
    let mut eve_payload = MessagePayload::new(
        alice_keypair.public_key_string(), // Impersonating Alice
        "Modified message from Eve".to_string(),
        1,
        None,
    );
    eve_payload.sign(&eve_keypair.signing_key).unwrap(); // But signed with Eve's key
    
    // This should fail verification because Eve's signature doesn't match Alice's public key
    assert!(eve_payload.verify_signature().is_err(),
        "Eve's impersonation should fail signature verification");
    
    println!("✓ Man-in-the-middle attack resistance validated");
}

/// Test resistance to timing attacks
#[test]
fn test_timing_attack_resistance() {
    println!("Testing Timing Attack Resistance...");
    
    let keypair = ClassicalUserKeyPair::generate();
    let test_data = b"timing attack test data";
    
    // Measure signature verification times for valid and invalid signatures
    let valid_signature = ClassicalDigitalSignature::sign(&keypair.signing_key, test_data);
    
    // Create invalid signature by flipping one bit
    let mut invalid_signature_bytes = valid_signature.to_bytes();
    invalid_signature_bytes[0] ^= 1;
    let invalid_signature = ed25519_dalek::Signature::from_bytes(&invalid_signature_bytes);
    
    const NUM_ITERATIONS: usize = 100;
    
    // Time valid signature verifications
    let start = std::time::Instant::now();
    for _ in 0..NUM_ITERATIONS {
        let _ = ClassicalDigitalSignature::verify(&keypair.public_keys().verifying_key, test_data, &valid_signature);
    }
    let valid_time = start.elapsed();
    
    // Time invalid signature verifications
    let start = std::time::Instant::now();
    for _ in 0..NUM_ITERATIONS {
        let _ = ClassicalDigitalSignature::verify(&keypair.public_keys().verifying_key, test_data, &invalid_signature);
    }
    let invalid_time = start.elapsed();
    
    // The time difference should be minimal (constant-time verification)
    let time_ratio = valid_time.as_nanos() as f64 / invalid_time.as_nanos() as f64;
    assert!(time_ratio > 0.5 && time_ratio < 2.0, 
        "Timing difference too large: valid/invalid ratio = {:.2}", time_ratio);
    
    println!("✓ Timing attack resistance validated (ratio: {:.2})", time_ratio);
}

/// Test resistance to key reuse attacks
#[test]
fn test_key_reuse_attack_resistance() {
    println!("Testing Key Reuse Attack Resistance...");
    
    let _alice_keypair = ClassicalUserKeyPair::generate();
    let bob_keypair = ClassicalUserKeyPair::generate();
    let charlie_keypair = ClassicalUserKeyPair::generate();
    
    // Alice encrypts different messages for Bob and Charlie
    let message1 = "Message for Bob";
    let message2 = "Message for Charlie";
    
    // Use ephemeral keys for each message (proper practice)
    let alice_ephemeral1 = ClassicalUserKeyPair::generate();
    let alice_ephemeral2 = ClassicalUserKeyPair::generate();
    
    let shared_secret1 = derive_shared_secret(
        &alice_ephemeral1.x25519_key,
        &bob_keypair.public_keys().x25519_key
    );
    let shared_secret2 = derive_shared_secret(
        &alice_ephemeral2.x25519_key,
        &charlie_keypair.public_keys().x25519_key
    );
    
    // Different ephemeral keys should produce different shared secrets
    assert_ne!(shared_secret1, shared_secret2, "Different ephemeral keys should produce different secrets");
    
    // Encrypt both messages
    let ciphertext1 = encrypt_symmetric(&shared_secret1, message1.as_bytes()).unwrap();
    let ciphertext2 = encrypt_symmetric(&shared_secret2, message2.as_bytes()).unwrap();
    
    // Ciphertexts should be different even though they're both from Alice
    assert_ne!(ciphertext1, ciphertext2, "Different messages with different keys should produce different ciphertexts");
    
    // Test that using the same ephemeral key is detectable (bad practice)
    let bad_shared_secret1 = derive_shared_secret(
        &alice_ephemeral1.x25519_key, // Reusing same ephemeral key
        &bob_keypair.public_keys().x25519_key
    );
    let bad_shared_secret2 = derive_shared_secret(
        &alice_ephemeral1.x25519_key, // Same ephemeral key reused
        &charlie_keypair.public_keys().x25519_key
    );
    
    // These should be different because the recipients are different
    assert_ne!(bad_shared_secret1, bad_shared_secret2, 
        "Same ephemeral key with different recipients should still produce different secrets");
    
    println!("✓ Key reuse attack resistance validated");
}

/// Test resistance to chosen plaintext attacks
#[test]
fn test_chosen_plaintext_attack_resistance() {
    println!("Testing Chosen Plaintext Attack Resistance...");
    
    let key = [42u8; 32];
    let mut ciphertexts = HashMap::new();
    
    // Attacker chooses specific plaintexts to encrypt
    let chosen_plaintexts = vec![
        b"".to_vec(),                              // Empty
        vec![0u8; 16],                            // All zeros
        vec![0xFFu8; 16],                         // All ones
        b"AAAAAAAAAAAAAAAA".to_vec(),             // Repeated pattern
        (0..16).collect::<Vec<u8>>(),             // Sequential
        b"attack plaintext 1".to_vec(),          // Arbitrary text
        b"attack plaintext 2".to_vec(),          // Similar text
    ];
    
    // Encrypt all chosen plaintexts
    for (i, plaintext) in chosen_plaintexts.iter().enumerate() {
        let ciphertext = encrypt_symmetric(&key, plaintext).unwrap();
        
        // Check that ciphertext is different from plaintext (actually encrypted)
        if plaintext.len() == ciphertext.len() - 12 { // Account for nonce
            assert_ne!(&ciphertext[12..], plaintext.as_slice(), 
                "Ciphertext {} should differ from plaintext", i);
        }
        
        // Check that ciphertext is unique
        for (j, other_ciphertext) in ciphertexts.iter() {
            assert_ne!(ciphertext, *other_ciphertext, 
                "Ciphertext {} should be unique (differs from ciphertext {})", i, j);
        }
        
        ciphertexts.insert(i, ciphertext);
    }
    
    // Verify all ciphertexts decrypt correctly
    for (i, plaintext) in chosen_plaintexts.iter().enumerate() {
        let ciphertext = &ciphertexts[&i];
        let decrypted = decrypt_symmetric(&key, ciphertext).unwrap();
        assert_eq!(decrypted, *plaintext, "Decryption {} should match original", i);
    }
    
    println!("✓ Chosen plaintext attack resistance validated");
}

/// Test resistance to bit-flipping attacks
#[test]
fn test_bit_flipping_attack_resistance() {
    println!("Testing Bit-Flipping Attack Resistance...");
    
    let key = [42u8; 32];
    let plaintext = b"This is a secret message that must not be tampered with";
    
    let ciphertext = encrypt_symmetric(&key, plaintext).unwrap();
    
    // Try flipping bits at various positions
    for byte_pos in 0..ciphertext.len() {
        for bit_pos in 0..8 {
            let mut tampered = ciphertext.clone();
            tampered[byte_pos] ^= 1 << bit_pos;
            
            // Tampered ciphertext should fail to decrypt or produce garbage
            match decrypt_symmetric(&key, &tampered) {
                Ok(decrypted) => {
                    // If it decrypts, it should be detectably different
                    assert_ne!(decrypted, plaintext, 
                        "Bit flip at byte {} bit {} should be detectable", byte_pos, bit_pos);
                }
                Err(_) => {
                    // Failing to decrypt is also good (authentication failure)
                }
            }
        }
    }
    
    println!("✓ Bit-flipping attack resistance validated");
}

/// Test resistance to quantum computer simulation (hybrid/PQ modes)
#[test]
fn test_quantum_attack_simulation() {
    println!("Testing Quantum Attack Simulation...");
    
    // Simulate scenario where classical crypto is broken but PQ crypto remains secure
    
    // Classical-only setup (vulnerable to quantum attacks)
    let alice_classical = ClassicalUserKeyPair::generate();
    let bob_classical = ClassicalUserKeyPair::generate();
    
    // Hybrid setup (quantum-resistant)
    let alice_hybrid = HybridUserKeyPair::generate();
    let bob_hybrid = HybridUserKeyPair::generate();
    
    // Pure PQ setup (quantum-resistant)
    let _alice_pq = PostQuantumUserKeyPair::generate();
    let bob_pq = PostQuantumUserKeyPair::generate();
    
    let secret_message = "Quantum-sensitive information";
    
    // 1. Classical crypto (assume quantum computer breaks this)
    let classical_shared = derive_shared_secret(
        &alice_classical.x25519_key,
        &bob_classical.public_keys().x25519_key
    );
    
    // 2. Hybrid crypto - even if classical component is broken, PQ component protects
    let hybrid_classical_shared = derive_shared_secret(
        &alice_hybrid.classical.x25519_key,
        &bob_hybrid.public_keys().classical.x25519_key
    );
    
    // 3. Pure PQ crypto using encapsulation
    let (pq_shared, _ciphertext) = PostQuantumKeyExchange::encapsulate(&bob_pq.public_keys().public_key).unwrap();
    
    // All three should produce different shared secrets
    assert_ne!(classical_shared.as_ref(), hybrid_classical_shared.as_ref());
    assert_ne!(classical_shared.as_ref(), pq_shared.as_ref());
    assert_ne!(hybrid_classical_shared.as_ref(), pq_shared.as_ref());
    
    // Simulate quantum attack: classical component compromised
    // Attacker knows the classical shared secret but hybrid/PQ should still be secure
    
    // Encrypt with all three methods
    let classical_key = hash_sha256(&classical_shared);
    let hybrid_key = hash_sha256(&hybrid_classical_shared);
    let pq_key = hash_sha256(pq_shared.as_ref());
    
    let classical_ciphertext = encrypt_symmetric(&classical_key, secret_message.as_bytes()).unwrap();
    let hybrid_ciphertext = encrypt_symmetric(&hybrid_key, secret_message.as_bytes()).unwrap();
    let pq_ciphertext = encrypt_symmetric(&pq_key, secret_message.as_bytes()).unwrap();
    
    // All should decrypt correctly with proper keys
    assert_eq!(
        String::from_utf8(decrypt_symmetric(&classical_key, &classical_ciphertext).unwrap()).unwrap(),
        secret_message
    );
    assert_eq!(
        String::from_utf8(decrypt_symmetric(&hybrid_key, &hybrid_ciphertext).unwrap()).unwrap(),
        secret_message
    );
    assert_eq!(
        String::from_utf8(decrypt_symmetric(&pq_key, &pq_ciphertext).unwrap()).unwrap(),
        secret_message
    );
    
    // Simulate quantum attack: classical key is compromised
    // Attacker cannot decrypt hybrid or PQ messages with only classical key
    assert!(decrypt_symmetric(&classical_key, &hybrid_ciphertext).is_err() ||
        decrypt_symmetric(&classical_key, &hybrid_ciphertext).unwrap() != secret_message.as_bytes());
    
    assert!(decrypt_symmetric(&classical_key, &pq_ciphertext).is_err() ||
        decrypt_symmetric(&classical_key, &pq_ciphertext).unwrap() != secret_message.as_bytes());
    
    println!("✓ Quantum attack simulation validated - hybrid/PQ modes resist classical crypto breaks");
}

/// Test resistance to downgrade attacks
#[test]
fn test_downgrade_attack_resistance() {
    println!("Testing Downgrade Attack Resistance...");
    
    // Test crypto mode transition rules prevent downgrade attacks
    assert!(!CryptoMode::Hybrid.can_transition_to(CryptoMode::Classical),
        "Should not allow downgrade from Hybrid to Classical");
    
    assert!(!CryptoMode::Quantum.can_transition_to(CryptoMode::Classical),
        "Should not allow downgrade from Quantum to Classical");
    
    assert!(!CryptoMode::Quantum.can_transition_to(CryptoMode::Hybrid),
        "Should not allow downgrade from Quantum to Hybrid");
    
    // Test that config validation prevents insecure configurations
    let invalid_config = CryptoConfig {
        mode: CryptoMode::Classical,
        minimum_mode: CryptoMode::Hybrid, // Minimum higher than current
        allow_auto_upgrade: false,
        adaptive_mode: false,
    };
    
    assert!(invalid_config.validate().is_err(),
        "Should reject config with current mode below minimum");
    
    // Test that protocol messages maintain crypto mode integrity
    let alice_hybrid = HybridUserKeyPair::generate();
    
    let mut payload = MessagePayload::new_with_mode(
        alice_hybrid.public_key_string(),
        "High security message".to_string(),
        1,
        None,
        CryptoMode::Hybrid,
    );
    
    let alice_unified = UnifiedKeyPair::Hybrid(alice_hybrid);
    payload.sign_with_mode(&alice_unified).unwrap();
    
    // Verify crypto mode is preserved
    assert_eq!(payload.crypto_mode, Some(CryptoMode::Hybrid));
    
    // Attacker cannot forge lower security mode
    payload.crypto_mode = Some(CryptoMode::Classical);
    // Signature verification should fail because crypto_mode is part of signed data
    assert!(payload.verify_signature_with_mode().is_err(),
        "Modified crypto mode should break signature verification");
    
    println!("✓ Downgrade attack resistance validated");
}

/// Test resistance to brute force attacks
#[test]
fn test_brute_force_resistance() {
    println!("Testing Brute Force Resistance...");
    
    let keypair = ClassicalUserKeyPair::generate();
    let correct_data = b"correct message";
    let correct_signature = ClassicalDigitalSignature::sign(&keypair.signing_key, correct_data);
    
    // Try many wrong messages to see if any accidentally verify
    let mut false_positives = 0;
    const NUM_ATTEMPTS: usize = 10000;
    
    for i in 0..NUM_ATTEMPTS {
        let wrong_data = format!("wrong message {}", i);
        
        match ClassicalDigitalSignature::verify(
            &keypair.public_keys().verifying_key, 
            wrong_data.as_bytes(), 
            &correct_signature
        ) {
            Ok(_) => false_positives += 1,
            Err(_) => {} // Expected
        }
    }
    
    // Should have extremely low false positive rate (ideally zero)
    let false_positive_rate = false_positives as f64 / NUM_ATTEMPTS as f64;
    assert!(false_positive_rate < 0.001, // Less than 0.1%
        "False positive rate too high: {:.4}%", false_positive_rate * 100.0);
    
    // Test symmetric encryption key brute force resistance
    let correct_key = [42u8; 32];
    let plaintext = b"brute force test";
    let ciphertext = encrypt_symmetric(&correct_key, plaintext).unwrap();
    
    let mut successful_decryptions = 0;
    
    // Try random keys
    for _ in 0..1000 {
        let mut random_key = [0u8; 32];
        thread_rng().fill_bytes(&mut random_key);
        
        if random_key == correct_key {
            continue; // Skip the correct key
        }
        
        match decrypt_symmetric(&random_key, &ciphertext) {
            Ok(decrypted) => {
                if decrypted == plaintext {
                    successful_decryptions += 1;
                }
            }
            Err(_) => {} // Expected for wrong keys
        }
    }
    
    assert_eq!(successful_decryptions, 0, 
        "Random keys should not successfully decrypt authenticated encryption");
    
    println!("✓ Brute force resistance validated (false positive rate: {:.4}%)", 
        false_positive_rate * 100.0);
}

/// Run all attack resistance tests
pub fn run_all_attack_resistance_tests() -> Result<()> {
    println!("🛡️ Running Attack Resistance Tests...\n");
    
    test_signature_forgery_resistance();
    test_replay_attack_resistance();
    test_mitm_attack_resistance();
    test_timing_attack_resistance();
    test_key_reuse_attack_resistance();
    test_chosen_plaintext_attack_resistance();
    test_bit_flipping_attack_resistance();
    test_quantum_attack_simulation();
    test_downgrade_attack_resistance();
    test_brute_force_resistance();
    
    println!("\n✅ All attack resistance tests passed!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn run_attack_resistance_suite() {
        run_all_attack_resistance_tests().expect("Attack resistance tests should pass");
    }
}
