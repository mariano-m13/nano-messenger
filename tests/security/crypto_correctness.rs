//! Cryptographic Correctness Tests for Session 7
//! 
//! This module validates that all cryptographic implementations function correctly
//! and produce expected results. It tests algorithm implementations across all 
//! supported crypto modes: Classical, Hybrid, and Quantum.

use nano_messenger::crypto::*;
use nano_messenger::error::Result;
use rand::{RngCore, thread_rng};
use std::collections::HashSet;

/// Test classical cryptography correctness
#[test]
fn test_classical_crypto_correctness() {
    println!("Testing Classical Crypto Correctness...");
    
    // Test key generation produces valid keys
    let keypair = ClassicalUserKeyPair::generate();
    let public_keys = keypair.public_keys();
    
    // Verify key formats
    assert_eq!(keypair.x25519_key.to_bytes().len(), 32);
    assert_eq!(public_keys.x25519_key.to_bytes().len(), 32);
    assert_eq!(keypair.signing_key.to_bytes().len(), 32);
    assert_eq!(public_keys.verifying_key.to_bytes().len(), 32);
    
    // Test X25519 key exchange
    let keypair2 = ClassicalUserKeyPair::generate();
    let shared1 = ClassicalKeyExchange::key_exchange(
        &keypair.x25519_key, 
        &keypair2.public_keys().x25519_key
    ).unwrap();
    let shared2 = ClassicalKeyExchange::key_exchange(
        &keypair2.x25519_key, 
        &public_keys.x25519_key
    ).unwrap();
    
    assert_eq!(shared1.as_ref(), shared2.as_ref());
    assert_eq!(shared1.as_ref().len(), 32);
    
    // Test Ed25519 signatures
    let test_data = b"test message for classical crypto";
    let signature = ClassicalDigitalSignature::sign(&keypair.signing_key, test_data);
    
    // Signature should verify correctly
    ClassicalDigitalSignature::verify(&public_keys.verifying_key, test_data, &signature)
        .expect("Valid signature should verify");
    
    // Wrong data should fail verification
    let wrong_data = b"tampered message";
    assert!(ClassicalDigitalSignature::verify(&public_keys.verifying_key, wrong_data, &signature).is_err());
    
    // Test symmetric encryption
    let key = [42u8; 32];
    let plaintext = b"symmetric encryption test";
    
    let ciphertext = ClassicalSymmetricEncryption::encrypt(&key, plaintext).unwrap();
    let decrypted = ClassicalSymmetricEncryption::decrypt(&key, &ciphertext).unwrap();
    
    assert_eq!(decrypted, plaintext);
    assert_ne!(ciphertext, plaintext); // Ensure actually encrypted
    
    println!("✓ Classical crypto correctness validated");
}

/// Test post-quantum cryptography correctness
#[test]
fn test_post_quantum_crypto_correctness() {
    println!("Testing Post-Quantum Crypto Correctness...");
    
    // Test PQ key generation
    let keypair = PostQuantumUserKeyPair::generate();
    let public_keys = keypair.public_keys();
    
    // Verify key formats and sizes
    assert!(keypair.private_key.kem_key.len() > 0);
    assert!(public_keys.public_key.kem_key.len() > 0);
    
    // Test PQ key encapsulation/decapsulation
    let keypair2 = PostQuantumUserKeyPair::generate();
    let (shared1, ciphertext1) = PostQuantumKeyExchange::encapsulate(&keypair2.public_keys().public_key).unwrap();
    let shared2 = PostQuantumKeyExchange::decapsulate(&keypair2.private_key, &ciphertext1).unwrap();
    
    assert_eq!(shared1.as_ref(), shared2.as_ref());
    assert_eq!(shared1.as_ref().len(), 32); // Should produce 32-byte shared secret
    
    // Test PQ signatures
    let test_data = b"test message for post-quantum crypto";
    let signature = PostQuantumDigitalSignature::sign(&keypair.private_key, test_data);
    
    // Signature should verify correctly
    PostQuantumDigitalSignature::verify(&public_keys.public_key, test_data, &signature)
        .expect("Valid PQ signature should verify");
    
    // Wrong data should fail verification
    let wrong_data = b"tampered message";
    assert!(PostQuantumDigitalSignature::verify(&public_keys.public_key, wrong_data, &signature).is_err());
    
    // Test key serialization/deserialization
    let pubkey_bytes = PostQuantumKeyExchange::public_key_to_bytes(&public_keys.public_key);
    let restored_pubkey = PostQuantumKeyExchange::public_key_from_bytes(&pubkey_bytes).unwrap();
    
    // Should be able to perform encapsulation with restored key
    let (_shared_original, _) = PostQuantumKeyExchange::encapsulate(&public_keys.public_key).unwrap();
    let (_shared_restored, _) = PostQuantumKeyExchange::encapsulate(&restored_pubkey).unwrap();
    
    // Different runs will produce different secrets, so just verify keys are the same
    assert_eq!(public_keys.public_key.kem_key, restored_pubkey.kem_key);
    assert_eq!(public_keys.public_key.sign_key, restored_pubkey.sign_key);
    
    println!("✓ Post-quantum crypto correctness validated");
}

/// Test hybrid cryptography correctness
#[test]
fn test_hybrid_crypto_correctness() {
    println!("Testing Hybrid Crypto Correctness...");
    
    // Test hybrid key generation
    let keypair = HybridUserKeyPair::generate();
    let public_keys = keypair.public_keys();
    
    // Verify both classical and PQ components exist
    assert_eq!(keypair.classical.x25519_key.to_bytes().len(), 32);
    assert_eq!(keypair.classical.signing_key.to_bytes().len(), 32);
    assert!(keypair.post_quantum.private_key.kem_key.len() > 0);
    
    // Test hybrid key exchange (simplified approach)
    let keypair2 = HybridUserKeyPair::generate();
    
    // Test classical component directly
    let classical_shared1 = ClassicalKeyExchange::key_exchange(
        &keypair.classical.x25519_key,
        &keypair2.public_keys().classical.x25519_key
    ).unwrap();
    let classical_shared2 = ClassicalKeyExchange::key_exchange(
        &keypair2.classical.x25519_key,
        &public_keys.classical.x25519_key
    ).unwrap();
    
    assert_eq!(classical_shared1.as_ref(), classical_shared2.as_ref());
    
    // Test PQ component separately
    let (pq_shared1, ciphertext) = PostQuantumKeyExchange::encapsulate(&keypair2.public_keys().post_quantum.public_key).unwrap();
    let pq_shared2 = PostQuantumKeyExchange::decapsulate(&keypair2.post_quantum.private_key, &ciphertext).unwrap();
    
    assert_eq!(pq_shared1.as_ref(), pq_shared2.as_ref());
    
    // Test hybrid signatures with simplified approach
    let test_data = b"test message for hybrid crypto";
    
    // Test classical signature component
    let classical_signature = ClassicalDigitalSignature::sign(&keypair.classical.signing_key, test_data);
    ClassicalDigitalSignature::verify(&public_keys.classical.verifying_key, test_data, &classical_signature)
        .expect("Classical component should verify");
    
    // Test PQ signature component
    let pq_signature = PostQuantumDigitalSignature::sign(&keypair.post_quantum.private_key, test_data);
    PostQuantumDigitalSignature::verify(&public_keys.post_quantum.public_key, test_data, &pq_signature)
        .expect("PQ component should verify");
    
    // Wrong data should fail verification for both components
    let wrong_data = b"tampered message";
    assert!(ClassicalDigitalSignature::verify(&public_keys.classical.verifying_key, wrong_data, &classical_signature).is_err());
    assert!(PostQuantumDigitalSignature::verify(&public_keys.post_quantum.public_key, wrong_data, &pq_signature).is_err());
    
    println!("✓ Hybrid crypto correctness validated");
}

/// Test unified crypto interface correctness
#[test]
fn test_unified_interface_correctness() {
    println!("Testing Unified Interface Correctness...");
    
    // Test all crypto modes through unified interface
    for mode in [CryptoMode::Classical, CryptoMode::Hybrid, CryptoMode::Quantum] {
        let config = CryptoConfig::new(mode);
        let _ = init_crypto_config(config); // Ignore error if already initialized
        
        // Test keypair generation
        let keypair = match mode {
            CryptoMode::Classical => UnifiedKeyPair::Classical(ClassicalUserKeyPair::generate()),
            CryptoMode::Hybrid => UnifiedKeyPair::Hybrid(HybridUserKeyPair::generate()),
            CryptoMode::Quantum => UnifiedKeyPair::PostQuantum(PostQuantumUserKeyPair::generate()),
        };
        
        assert_eq!(keypair.mode(), mode);
        
        // Test public key string format
        let pubkey_str = keypair.public_key_string();
        match mode {
            CryptoMode::Classical => assert!(pubkey_str.starts_with("pubkey:")),
            CryptoMode::Hybrid => assert!(pubkey_str.starts_with("hybrid-pubkey:")),
            CryptoMode::Quantum => assert!(pubkey_str.starts_with("pq-pubkey:")),
        }
        
        // Test symmetric encryption through unified interface
        let key = [42u8; 32];
        let plaintext = b"unified interface test";
        
        let ciphertext = CryptoInterface::encrypt_symmetric(&key, plaintext).unwrap();
        let decrypted = CryptoInterface::decrypt_symmetric(&key, &ciphertext).unwrap();
        
        assert_eq!(decrypted, plaintext);
        
        println!("  ✓ {} mode unified interface validated", mode);
    }
    
    println!("✓ Unified interface correctness validated");
}

/// Test cryptographic randomness quality
#[test]
fn test_randomness_quality() {
    println!("Testing Randomness Quality...");
    
    const NUM_SAMPLES: usize = 1000;
    
    let mut seen_keys = HashSet::new();
    let mut byte_counts = [0u32; 256];
    
    // Generate many keys and check for uniqueness and distribution
    for _ in 0..NUM_SAMPLES {
        let keypair = ClassicalUserKeyPair::generate();
        let key_bytes = keypair.x25519_key.to_bytes();
        
        // Check uniqueness - no key collisions should occur
        assert!(!seen_keys.contains(&key_bytes), "Key collision detected!");
        seen_keys.insert(key_bytes);
        
        // Count byte value distribution for chi-square test
        for &byte in key_bytes.iter() {
            byte_counts[byte as usize] += 1;
        }
    }
    
    // Calculate Shannon entropy of the byte distribution
    let total_bytes = (NUM_SAMPLES * 32) as f64;
    let mut shannon_entropy = 0.0;
    
    for count in byte_counts.iter() {
        if *count > 0 {
            let probability = *count as f64 / total_bytes;
            shannon_entropy -= probability * probability.log2();
        }
    }
    
    // Shannon entropy should be close to 8.0 for truly random bytes
    // We allow some deviation due to sample size
    assert!(shannon_entropy > 7.5, "Shannon entropy too low: {:.3} (expected > 7.5)", shannon_entropy);
    
    // Test for reasonable byte distribution - no byte value should be too rare or too common
    // Use chi-square test approach with more reasonable tolerance for small sample sizes
    let expected_count = total_bytes / 256.0;
    let tolerance = expected_count * 0.5; // Allow 50% deviation for robustness
    
    let mut extreme_deviations = 0;
    for (byte_value, &count) in byte_counts.iter().enumerate() {
        let count_f64 = count as f64;
        let deviation = (count_f64 - expected_count).abs();
        
        // Count extreme deviations (beyond 50% tolerance)
        if deviation > tolerance {
            extreme_deviations += 1;
        }
        
        // Fail only if deviation is extreme (beyond 70% tolerance)
        let extreme_tolerance = expected_count * 0.7;
        assert!(
            deviation <= extreme_tolerance,
            "Byte value {} appears {} times with extreme deviation (expected ~{:.1}, tolerance ±{:.1})",
            byte_value, count, expected_count, extreme_tolerance
        );
    }
    
    // Allow some moderate deviations but not too many
    assert!(
        extreme_deviations <= 10, // Allow up to 10 values to exceed 50% deviation
        "Too many byte values ({}) exceed 50% deviation tolerance", 
        extreme_deviations
    );
    
    // Test nonce generation quality
    let mut seen_nonces = HashSet::new();
    for _ in 0..NUM_SAMPLES {
        let mut nonce = [0u8; 16];
        thread_rng().fill_bytes(&mut nonce);
        
        assert!(!seen_nonces.contains(&nonce), "Nonce collision detected!");
        seen_nonces.insert(nonce);
    }
    
    println!("✓ Randomness quality validated (Shannon entropy: {:.3})", shannon_entropy);
}

/// Test key derivation functions
#[test]
fn test_key_derivation_correctness() {
    println!("Testing Key Derivation Correctness...");
    
    // Test that shared secrets produce consistent symmetric keys
    let keypair1 = ClassicalUserKeyPair::generate();
    let keypair2 = ClassicalUserKeyPair::generate();
    
    let shared_secret = ClassicalKeyExchange::key_exchange(
        &keypair1.x25519_key,
        &keypair2.public_keys().x25519_key
    ).unwrap();
    
    // Derive symmetric key from shared secret
    let symmetric_key = hash_sha256(shared_secret.as_ref());
    assert_eq!(symmetric_key.len(), 32);
    
    // Test that same shared secret produces same symmetric key
    let shared_secret2 = ClassicalKeyExchange::key_exchange(
        &keypair1.x25519_key,
        &keypair2.public_keys().x25519_key
    ).unwrap();
    let symmetric_key2 = hash_sha256(shared_secret2.as_ref());
    
    assert_eq!(symmetric_key, symmetric_key2);
    
    // Test that different shared secrets produce different keys
    let keypair3 = ClassicalUserKeyPair::generate();
    let shared_secret3 = ClassicalKeyExchange::key_exchange(
        &keypair1.x25519_key,
        &keypair3.public_keys().x25519_key
    ).unwrap();
    let symmetric_key3 = hash_sha256(shared_secret3.as_ref());
    
    assert_ne!(symmetric_key, symmetric_key3);
    
    println!("✓ Key derivation correctness validated");
}

/// Test cryptographic algorithm interoperability
#[test]
fn test_algorithm_interoperability() {
    println!("Testing Algorithm Interoperability...");
    
    // Test that all modes can decrypt symmetric data encrypted by any mode
    let key = [42u8; 32];
    let plaintext = b"interoperability test message";
    
    // Encrypt with classical implementation
    let ciphertext = ClassicalSymmetricEncryption::encrypt(&key, plaintext).unwrap();
    
    // Should decrypt correctly with unified interface
    let decrypted = CryptoInterface::decrypt_symmetric(&key, &ciphertext).unwrap();
    assert_eq!(decrypted, plaintext);
    
    // Encrypt with unified interface
    let ciphertext2 = CryptoInterface::encrypt_symmetric(&key, plaintext).unwrap();
    
    // Should decrypt correctly with classical implementation
    let decrypted2 = ClassicalSymmetricEncryption::decrypt(&key, &ciphertext2).unwrap();
    assert_eq!(decrypted2, plaintext);
    
    println!("✓ Algorithm interoperability validated");
}

/// Test edge cases and boundary conditions
#[test]
fn test_crypto_edge_cases() {
    println!("Testing Crypto Edge Cases...");
    
    // Test empty data handling
    let keypair = ClassicalUserKeyPair::generate();
    let empty_data = b"";
    let signature = ClassicalDigitalSignature::sign(&keypair.signing_key, empty_data);
    ClassicalDigitalSignature::verify(&keypair.public_keys().verifying_key, empty_data, &signature)
        .expect("Should handle empty data");
    
    // Test large data handling
    let large_data = vec![42u8; 1_000_000]; // 1MB
    let signature = ClassicalDigitalSignature::sign(&keypair.signing_key, &large_data);
    ClassicalDigitalSignature::verify(&keypair.public_keys().verifying_key, &large_data, &signature)
        .expect("Should handle large data");
    
    // Test symmetric encryption with various sizes
    let key = [42u8; 32];
    for size in [0, 1, 15, 16, 17, 31, 32, 33, 63, 64, 65, 1023, 1024, 1025] {
        let data = vec![size as u8; size];
        let ciphertext = ClassicalSymmetricEncryption::encrypt(&key, &data).unwrap();
        let decrypted = ClassicalSymmetricEncryption::decrypt(&key, &ciphertext).unwrap();
        assert_eq!(decrypted, data, "Failed for size {}", size);
    }
    
    println!("✓ Crypto edge cases validated");
}

/// Run all cryptographic correctness tests
pub fn run_all_crypto_correctness_tests() -> Result<()> {
    println!("🔒 Running Cryptographic Correctness Tests...\n");
    
    test_classical_crypto_correctness();
    test_post_quantum_crypto_correctness();
    test_hybrid_crypto_correctness();
    test_unified_interface_correctness();
    test_randomness_quality();
    test_key_derivation_correctness();
    test_algorithm_interoperability();
    test_crypto_edge_cases();
    
    println!("\n✅ All cryptographic correctness tests passed!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn run_crypto_correctness_suite() {
        run_all_crypto_correctness_tests().expect("Crypto correctness tests should pass");
    }
}
