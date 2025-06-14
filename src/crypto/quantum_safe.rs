use crate::crypto::{
    CryptoMode, UnifiedKeyPair, UnifiedPublicKeys, CryptoInterface,
    ClassicalAsymmetricEncryption, HybridAsymmetricEncryption, PostQuantumAsymmetricEncryption,
    traits::AsymmetricEncryption,
};
use crate::error::{NanoError, Result};
use crate::protocol::{MessagePayload, QuantumSafeEnvelope};
use base64::{engine::general_purpose, Engine as _};

/// High-level quantum-safe messaging functions for Session 3+
pub struct QuantumSafeMessaging;

impl QuantumSafeMessaging {
    /// Create and encrypt a quantum-safe message
    pub fn create_encrypted_message(
        from_keypair: &UnifiedKeyPair,
        to_public_keys: &UnifiedPublicKeys,
        message_body: String,
        counter: u64,
        room: Option<String>,
        crypto_mode: Option<CryptoMode>,
    ) -> Result<QuantumSafeEnvelope> {
        // Determine crypto mode (use from parameters, or infer from keypair, or use current config)
        let mode = crypto_mode
            .or_else(|| Some(from_keypair.mode()))
            .unwrap_or_else(|| CryptoInterface::current_mode());

        // Create the payload
        let from_pubkey = from_keypair.public_key_string();
        let mut payload = MessagePayload::new_with_mode(
            from_pubkey,
            message_body,
            counter,
            room,
            mode,
        );

        // Sign the payload
        payload.sign_with_mode(from_keypair)?;

        // Serialize payload to JSON
        let payload_json = payload.to_json()?;
        let payload_bytes = payload_json.as_bytes();

        // Encrypt the payload based on crypto mode
        let encrypted_payload = Self::encrypt_payload_with_mode(
            to_public_keys,
            payload_bytes,
            mode,
        )?;

        // Generate inbox ID from recipient's public key
        let inbox_id = Self::derive_inbox_id(&to_public_keys.public_key_string(), counter);

        // Create the quantum-safe envelope
        let mut envelope = QuantumSafeEnvelope::new(mode, inbox_id, encrypted_payload);

        // Add post-quantum specific data if needed
        if mode.is_quantum_resistant() {
            // For quantum-resistant modes, we might need additional data
            // This is where ML-KEM ciphertexts would go in a real implementation
            envelope = envelope.with_pq_data(None, None); // Placeholder for now
        }

        Ok(envelope)
    }

    /// Decrypt and verify a quantum-safe message
    pub fn decrypt_message(
        envelope: &QuantumSafeEnvelope,
        our_keypair: &UnifiedKeyPair,
    ) -> Result<MessagePayload> {
        // Check if crypto modes are compatible
        if !CryptoInterface::accepts_mode(envelope.crypto_mode) {
            return Err(NanoError::Crypto(format!(
                "Message crypto mode {} not accepted by current configuration",
                envelope.crypto_mode
            )));
        }

        // Decrypt the payload
        let encrypted_payload = envelope.decode_payload()?;
        let decrypted_bytes = Self::decrypt_payload_with_mode(
            our_keypair,
            &encrypted_payload,
            envelope.crypto_mode,
        )?;

        // Parse the payload
        let payload_json = String::from_utf8(decrypted_bytes)
            .map_err(|e| NanoError::Crypto(format!("Invalid UTF-8: {}", e)))?;
        let payload = MessagePayload::from_json(&payload_json)?;

        // Verify the signature
        payload.verify_signature_with_mode()?;

        Ok(payload)
    }

    /// Encrypt payload data using the specified crypto mode
    fn encrypt_payload_with_mode(
        to_public_keys: &UnifiedPublicKeys,
        payload_bytes: &[u8],
        mode: CryptoMode,
    ) -> Result<Vec<u8>> {
        match (mode, to_public_keys) {
            (CryptoMode::Classical, UnifiedPublicKeys::Classical(keys)) => {
                ClassicalAsymmetricEncryption::encrypt(&keys.x25519_key, payload_bytes)
            }
            (CryptoMode::Hybrid, UnifiedPublicKeys::Hybrid(keys)) => {
                let hybrid_public = crate::crypto::hybrid::HybridPublicKey {
                    classical: keys.classical.x25519_key.clone(),
                    post_quantum: keys.post_quantum.public_key.clone(),
                };
                HybridAsymmetricEncryption::encrypt(&hybrid_public, payload_bytes)
            }
            (CryptoMode::Quantum, UnifiedPublicKeys::PostQuantum(keys)) => {
                PostQuantumAsymmetricEncryption::encrypt(&keys.public_key, payload_bytes)
            }
            (CryptoMode::QuantumSafe, UnifiedPublicKeys::PostQuantum(keys)) => {
                PostQuantumAsymmetricEncryption::encrypt(&keys.public_key, payload_bytes)
            }
            // Handle cross-mode compatibility
            (CryptoMode::Classical, UnifiedPublicKeys::Hybrid(keys)) => {
                // Use only the classical part of hybrid keys
                ClassicalAsymmetricEncryption::encrypt(&keys.classical.x25519_key, payload_bytes)
            }
            (CryptoMode::Classical, UnifiedPublicKeys::PostQuantum(_)) => {
                Err(NanoError::Crypto(
                    "Cannot encrypt classical message to post-quantum-only recipient".to_string(),
                ))
            }
            (CryptoMode::Hybrid, UnifiedPublicKeys::Classical(_)) => {
                Err(NanoError::Crypto(
                    "Cannot encrypt hybrid message to classical-only recipient".to_string(),
                ))
            }
            (CryptoMode::Quantum, UnifiedPublicKeys::Classical(_)) => {
                Err(NanoError::Crypto(
                    "Cannot encrypt post-quantum message to classical-only recipient".to_string(),
                ))
            }
            (CryptoMode::QuantumSafe, UnifiedPublicKeys::Classical(_)) => {
                Err(NanoError::Crypto(
                    "Cannot encrypt quantum-safe message to classical-only recipient".to_string(),
                ))
            }
            (CryptoMode::Quantum, UnifiedPublicKeys::Hybrid(keys)) => {
                // Use only the post-quantum part of hybrid keys
                PostQuantumAsymmetricEncryption::encrypt(&keys.post_quantum.public_key, payload_bytes)
            }
            (CryptoMode::QuantumSafe, UnifiedPublicKeys::Hybrid(keys)) => {
                // Use only the post-quantum part of hybrid keys
                PostQuantumAsymmetricEncryption::encrypt(&keys.post_quantum.public_key, payload_bytes)
            }
            (CryptoMode::Hybrid, UnifiedPublicKeys::PostQuantum(_)) => {
                Err(NanoError::Crypto(
                    "Cannot encrypt hybrid message to post-quantum-only recipient".to_string(),
                ))
            }
        }
    }

    /// Decrypt payload data using the specified crypto mode
    fn decrypt_payload_with_mode(
        our_keypair: &UnifiedKeyPair,
        encrypted_payload: &[u8],
        mode: CryptoMode,
    ) -> Result<Vec<u8>> {
        match (mode, our_keypair) {
            (CryptoMode::Classical, UnifiedKeyPair::Classical(kp)) => {
                ClassicalAsymmetricEncryption::decrypt_classical_direct(
                    &kp.x25519_key,
                    encrypted_payload,
                )
            }
            (CryptoMode::Hybrid, UnifiedKeyPair::Hybrid(kp)) => {
                let hybrid_private = crate::crypto::hybrid::HybridPrivateKey {
                    classical: kp.classical.x25519_key.clone(),
                    post_quantum: kp.post_quantum.private_key.clone(),
                };
                HybridAsymmetricEncryption::decrypt_hybrid_direct(
                    &hybrid_private,
                    encrypted_payload,
                )
            }
            (CryptoMode::Quantum, UnifiedKeyPair::PostQuantum(kp)) => {
                PostQuantumAsymmetricEncryption::decrypt_pq_direct(
                    &kp.private_key,
                    encrypted_payload,
                )
            }
            (CryptoMode::QuantumSafe, UnifiedKeyPair::PostQuantum(kp)) => {
                PostQuantumAsymmetricEncryption::decrypt_pq_direct(
                    &kp.private_key,
                    encrypted_payload,
                )
            }
            // Handle cross-mode compatibility for decryption
            (CryptoMode::Classical, UnifiedKeyPair::Hybrid(kp)) => {
                // Try classical decryption with the classical part
                ClassicalAsymmetricEncryption::decrypt_classical_direct(
                    &kp.classical.x25519_key,
                    encrypted_payload,
                )
            }
            (CryptoMode::Quantum, UnifiedKeyPair::Hybrid(kp)) => {
                // Try post-quantum decryption with the post-quantum part
                PostQuantumAsymmetricEncryption::decrypt_pq_direct(
                    &kp.post_quantum.private_key,
                    encrypted_payload,
                )
            }
            (CryptoMode::QuantumSafe, UnifiedKeyPair::Hybrid(kp)) => {
                // Try post-quantum decryption with the post-quantum part
                PostQuantumAsymmetricEncryption::decrypt_pq_direct(
                    &kp.post_quantum.private_key,
                    encrypted_payload,
                )
            }
            // Incompatible combinations
            _ => Err(NanoError::Crypto(
                "Incompatible crypto modes for decryption".to_string(),
            )),
        }
    }

    /// Derive inbox ID from recipient public key and counter
    fn derive_inbox_id(recipient_pubkey: &str, counter: u64) -> String {
        use crate::crypto::hash_sha256;
        
        let mut data = Vec::new();
        data.extend_from_slice(recipient_pubkey.as_bytes());
        data.extend_from_slice(&counter.to_be_bytes());
        
        let hash = hash_sha256(&data);
        general_purpose::STANDARD.encode(&hash[..16]) // Use first 16 bytes for inbox ID
    }

    /// Convert legacy MessageEnvelope to QuantumSafeEnvelope for backward compatibility
    pub fn upgrade_legacy_envelope(legacy: crate::protocol::MessageEnvelope) -> QuantumSafeEnvelope {
        QuantumSafeEnvelope::from_legacy(legacy)
    }

    /// Downgrade QuantumSafeEnvelope to legacy format if possible
    pub fn downgrade_to_legacy(envelope: &QuantumSafeEnvelope) -> Result<crate::protocol::MessageEnvelope> {
        if envelope.crypto_mode != CryptoMode::Classical {
            return Err(NanoError::Crypto(
                "Cannot downgrade non-classical message to legacy format".to_string(),
            ));
        }
        Ok(envelope.to_legacy())
    }

    /// Check if two crypto modes are compatible for communication
    pub fn modes_compatible(sender_mode: CryptoMode, receiver_mode: CryptoMode) -> bool {
        match (sender_mode, receiver_mode) {
            // Same modes are always compatible
            (a, b) if a == b => true,
            // Quantum and QuantumSafe are equivalent
            (CryptoMode::Quantum, CryptoMode::QuantumSafe) => true,
            (CryptoMode::QuantumSafe, CryptoMode::Quantum) => true,
            // Classical can communicate with any mode (receiver can downgrade)
            (CryptoMode::Classical, _) => true,
            // Hybrid can communicate with quantum modes (use quantum part)
            (CryptoMode::Hybrid, CryptoMode::Quantum) => true,
            (CryptoMode::Quantum, CryptoMode::Hybrid) => true,
            (CryptoMode::Hybrid, CryptoMode::QuantumSafe) => true,
            (CryptoMode::QuantumSafe, CryptoMode::Hybrid) => true,
            // Other combinations require explicit support
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::{CryptoInterface, CryptoConfig};

    #[test]
    fn test_quantum_safe_messaging_classical() {
        // Initialize crypto config
        let config = CryptoConfig::new(CryptoMode::Classical);
        let _ = crate::crypto::init_crypto_config(config);

        // Generate keypairs
        let alice_keypair = CryptoInterface::generate_keypair().unwrap();
        let bob_keypair = CryptoInterface::generate_keypair().unwrap();
        
        let alice_public = alice_keypair.public_keys();
        let bob_public = bob_keypair.public_keys();

        // Create and encrypt message
        let envelope = QuantumSafeMessaging::create_encrypted_message(
            &alice_keypair,
            &bob_public,
            "Hello, quantum-safe world!".to_string(),
            1,
            None,
            Some(CryptoMode::Classical),
        ).unwrap();

        assert_eq!(envelope.crypto_mode, CryptoMode::Classical);
        assert_eq!(envelope.version, "2.0-quantum");

        // Decrypt and verify message
        let decrypted_payload = QuantumSafeMessaging::decrypt_message(&envelope, &bob_keypair).unwrap();
        assert_eq!(decrypted_payload.body, "Hello, quantum-safe world!");
        assert_eq!(decrypted_payload.from_pubkey, alice_public.public_key_string());
        assert_eq!(decrypted_payload.crypto_mode, Some(CryptoMode::Classical));
    }

    #[test]
    fn test_mode_compatibility() {
        assert!(QuantumSafeMessaging::modes_compatible(CryptoMode::Classical, CryptoMode::Classical));
        assert!(QuantumSafeMessaging::modes_compatible(CryptoMode::Classical, CryptoMode::Hybrid));
        assert!(QuantumSafeMessaging::modes_compatible(CryptoMode::Hybrid, CryptoMode::Quantum));
        assert!(QuantumSafeMessaging::modes_compatible(CryptoMode::Quantum, CryptoMode::Hybrid));
        assert!(QuantumSafeMessaging::modes_compatible(CryptoMode::Quantum, CryptoMode::QuantumSafe));
        assert!(QuantumSafeMessaging::modes_compatible(CryptoMode::QuantumSafe, CryptoMode::Quantum));
        assert!(QuantumSafeMessaging::modes_compatible(CryptoMode::Hybrid, CryptoMode::QuantumSafe));
        assert!(QuantumSafeMessaging::modes_compatible(CryptoMode::QuantumSafe, CryptoMode::Hybrid));
    }

    #[test]
    fn test_legacy_compatibility() {
        use crate::protocol::MessageEnvelope;

        let legacy_envelope = MessageEnvelope::new(
            "test_inbox".to_string(),
            b"encrypted_data".to_vec(),
        );

        // Upgrade legacy to quantum-safe
        let quantum_envelope = QuantumSafeMessaging::upgrade_legacy_envelope(legacy_envelope.clone());
        assert_eq!(quantum_envelope.crypto_mode, CryptoMode::Classical);
        assert_eq!(quantum_envelope.legacy_compat, Some(true));

        // Downgrade back to legacy
        let downgraded = QuantumSafeMessaging::downgrade_to_legacy(&quantum_envelope).unwrap();
        assert_eq!(downgraded.inbox_id, legacy_envelope.inbox_id);
        assert_eq!(downgraded.payload, legacy_envelope.payload);
    }

    #[test]
    fn test_inbox_id_derivation() {
        let pubkey = "pubkey:test123";
        let counter = 42;
        
        let inbox_id1 = QuantumSafeMessaging::derive_inbox_id(pubkey, counter);
        let inbox_id2 = QuantumSafeMessaging::derive_inbox_id(pubkey, counter);
        
        // Should be deterministic
        assert_eq!(inbox_id1, inbox_id2);
        
        // Different counter should give different inbox ID
        let inbox_id3 = QuantumSafeMessaging::derive_inbox_id(pubkey, counter + 1);
        assert_ne!(inbox_id1, inbox_id3);
    }
}
