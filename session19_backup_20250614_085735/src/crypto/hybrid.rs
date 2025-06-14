use crate::crypto::classical::{
    ClassicalAsymmetricEncryption, ClassicalDigitalSignature, ClassicalKeyExchange,
    ClassicalUserKeyPair, ClassicalUserPublicKeys,
};
use crate::crypto::post_quantum::{
    PostQuantumAsymmetricEncryption, PostQuantumDigitalSignature, PostQuantumKeyExchange,
    PostQuantumUserKeyPair, PostQuantumUserPublicKeys,
};
use crate::crypto::traits::{
    AsymmetricEncryption, DigitalSignature, KeyExchange,
};
use crate::error::{NanoError, Result};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};

// Re-export necessary types
use crate::crypto::classical::{Ed25519PublicKey as ClassicalEd25519PublicKey, X25519PublicKey};
use crate::crypto::post_quantum::{
    PostQuantumPublicKey, PostQuantumPrivateKey
};

/// Hybrid key exchange combining X25519 + ML-KEM-768
pub struct HybridKeyExchange;

/// Hybrid private key containing both classical and post-quantum components
#[derive(Clone)]
pub struct HybridPrivateKey {
    pub classical: <ClassicalKeyExchange as KeyExchange>::PrivateKey,
    pub post_quantum: PostQuantumPrivateKey,
}

/// Hybrid public key containing both classical and post-quantum components
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HybridPublicKey {
    #[serde(
        serialize_with = "serialize_x25519_key",
        deserialize_with = "deserialize_x25519_key"
    )]
    pub classical: <ClassicalKeyExchange as KeyExchange>::PublicKey,
    #[serde(
        serialize_with = "serialize_pq_key",
        deserialize_with = "deserialize_pq_key"
    )]
    pub post_quantum: PostQuantumPublicKey,
}

/// Hybrid shared secret combining both classical and post-quantum secrets
pub struct HybridSharedSecret {
    pub classical: <ClassicalKeyExchange as KeyExchange>::SharedSecret,
    pub post_quantum: crate::crypto::post_quantum::PostQuantumSharedSecret,
    pub combined: [u8; 32], // Hash of both secrets
}

// Note: HybridSharedSecret intentionally does not implement Clone
// because the underlying classical SharedSecret cannot be cloned for security reasons

impl HybridSharedSecret {
    /// Create a new hybrid shared secret from classical and post-quantum components
    pub fn new(
        classical: <ClassicalKeyExchange as KeyExchange>::SharedSecret,
        post_quantum: crate::crypto::post_quantum::PostQuantumSharedSecret,
    ) -> Self {
        // Combine the secrets using a secure hash
        use crate::crypto::classical::hash_sha256;
        let mut combined_input = Vec::new();
        combined_input.extend_from_slice(classical.as_bytes());
        combined_input.extend_from_slice(post_quantum.as_ref());
        let combined = hash_sha256(&combined_input);

        Self {
            classical,
            post_quantum,
            combined,
        }
    }
}

impl AsRef<[u8]> for HybridSharedSecret {
    fn as_ref(&self) -> &[u8] {
        &self.combined
    }
}

impl KeyExchange for HybridKeyExchange {
    type PrivateKey = HybridPrivateKey;
    type PublicKey = HybridPublicKey;
    type SharedSecret = HybridSharedSecret;

    fn generate_private_key() -> Self::PrivateKey {
        HybridPrivateKey {
            classical: ClassicalKeyExchange::generate_private_key(),
            post_quantum: PostQuantumKeyExchange::generate_private_key(),
        }
    }

    fn derive_public_key(private_key: &Self::PrivateKey) -> Self::PublicKey {
        HybridPublicKey {
            classical: ClassicalKeyExchange::derive_public_key(&private_key.classical),
            post_quantum: PostQuantumKeyExchange::derive_public_key(&private_key.post_quantum),
        }
    }

    fn key_exchange(
        private_key: &Self::PrivateKey,
        public_key: &Self::PublicKey,
    ) -> Result<Self::SharedSecret> {
        // Perform classical key exchange
        let _classical_secret =
            ClassicalKeyExchange::key_exchange(&private_key.classical, &public_key.classical)?;

        // For ML-KEM, we need to handle encapsulation differently
        // This is a simplified approach - in practice, we'd need the ciphertext
        return Err(NanoError::Crypto(
            "Hybrid key exchange requires special handling for ML-KEM encapsulation".to_string(),
        ));
    }

    fn public_key_to_bytes(public_key: &Self::PublicKey) -> Vec<u8> {
        let mut bytes = Vec::new();
        let classical_bytes = ClassicalKeyExchange::public_key_to_bytes(&public_key.classical);
        let pq_bytes = PostQuantumKeyExchange::public_key_to_bytes(&public_key.post_quantum);

        // Length-prefixed encoding
        bytes.extend_from_slice(&(classical_bytes.len() as u32).to_be_bytes());
        bytes.extend_from_slice(&classical_bytes);
        bytes.extend_from_slice(&(pq_bytes.len() as u32).to_be_bytes());
        bytes.extend_from_slice(&pq_bytes);

        bytes
    }

    fn public_key_from_bytes(bytes: &[u8]) -> Result<Self::PublicKey> {
        if bytes.len() < 8 {
            return Err(NanoError::Crypto("Hybrid public key too short".to_string()));
        }

        let mut offset = 0;

        // Read classical key length and data
        let classical_len = u32::from_be_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]) as usize;
        offset += 4;

        if offset + classical_len > bytes.len() {
            return Err(NanoError::Crypto("Invalid hybrid key format".to_string()));
        }

        let classical_bytes = &bytes[offset..offset + classical_len];
        let classical = ClassicalKeyExchange::public_key_from_bytes(classical_bytes)?;
        offset += classical_len;

        // Read post-quantum key length and data
        if offset + 4 > bytes.len() {
            return Err(NanoError::Crypto("Invalid hybrid key format".to_string()));
        }

        let pq_len = u32::from_be_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]) as usize;
        offset += 4;

        if offset + pq_len != bytes.len() {
            return Err(NanoError::Crypto("Invalid hybrid key format".to_string()));
        }

        let pq_bytes = &bytes[offset..];
        let post_quantum = PostQuantumKeyExchange::public_key_from_bytes(pq_bytes)?;

        Ok(HybridPublicKey {
            classical,
            post_quantum,
        })
    }
}

/// Hybrid digital signatures combining Ed25519 + ML-DSA
pub struct HybridDigitalSignature;

/// Hybrid private signing key
#[derive(Clone)]
pub struct HybridSigningKey {
    pub classical: <ClassicalDigitalSignature as DigitalSignature>::PrivateKey,
    pub post_quantum: PostQuantumPrivateKey,
}

/// Hybrid public verifying key
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HybridVerifyingKey {
    #[serde(
        serialize_with = "serialize_ed25519_key",
        deserialize_with = "deserialize_ed25519_key"
    )]
    pub classical: <ClassicalDigitalSignature as DigitalSignature>::PublicKey,
    #[serde(
        serialize_with = "serialize_pq_key",
        deserialize_with = "deserialize_pq_key"
    )]
    pub post_quantum: PostQuantumPublicKey,
}

/// Hybrid signature containing both classical and post-quantum signatures
#[derive(Clone)]
pub struct HybridSignature {
    pub classical: <ClassicalDigitalSignature as DigitalSignature>::Signature,
    pub post_quantum: <PostQuantumDigitalSignature as DigitalSignature>::Signature,
}

impl DigitalSignature for HybridDigitalSignature {
    type PrivateKey = HybridSigningKey;
    type PublicKey = HybridVerifyingKey;
    type Signature = HybridSignature;

    fn generate_private_key() -> Self::PrivateKey {
        HybridSigningKey {
            classical: ClassicalDigitalSignature::generate_private_key(),
            post_quantum: PostQuantumDigitalSignature::generate_private_key(),
        }
    }

    fn derive_public_key(private_key: &Self::PrivateKey) -> Self::PublicKey {
        HybridVerifyingKey {
            classical: ClassicalDigitalSignature::derive_public_key(&private_key.classical),
            post_quantum: PostQuantumDigitalSignature::derive_public_key(&private_key.post_quantum),
        }
    }

    fn sign(private_key: &Self::PrivateKey, data: &[u8]) -> Self::Signature {
        HybridSignature {
            classical: ClassicalDigitalSignature::sign(&private_key.classical, data),
            post_quantum: PostQuantumDigitalSignature::sign(&private_key.post_quantum, data),
        }
    }

    fn verify(
        public_key: &Self::PublicKey,
        data: &[u8],
        signature: &Self::Signature,
    ) -> Result<()> {
        // Both signatures must verify successfully
        ClassicalDigitalSignature::verify(&public_key.classical, data, &signature.classical)?;
        PostQuantumDigitalSignature::verify(
            &public_key.post_quantum,
            data,
            &signature.post_quantum,
        )?;
        Ok(())
    }

    fn public_key_to_bytes(public_key: &Self::PublicKey) -> Vec<u8> {
        let mut bytes = Vec::new();
        let classical_bytes = ClassicalDigitalSignature::public_key_to_bytes(&public_key.classical);
        let pq_bytes = PostQuantumDigitalSignature::public_key_to_bytes(&public_key.post_quantum);

        // Length-prefixed encoding
        bytes.extend_from_slice(&(classical_bytes.len() as u32).to_be_bytes());
        bytes.extend_from_slice(&classical_bytes);
        bytes.extend_from_slice(&(pq_bytes.len() as u32).to_be_bytes());
        bytes.extend_from_slice(&pq_bytes);

        bytes
    }

    fn public_key_from_bytes(bytes: &[u8]) -> Result<Self::PublicKey> {
        if bytes.len() < 8 {
            return Err(NanoError::Crypto("Hybrid verifying key too short".to_string()));
        }

        let mut offset = 0;

        // Read classical key
        let classical_len = u32::from_be_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]) as usize;
        offset += 4;

        let classical_bytes = &bytes[offset..offset + classical_len];
        let classical = ClassicalDigitalSignature::public_key_from_bytes(classical_bytes)?;
        offset += classical_len;

        // Read post-quantum key
        let pq_len = u32::from_be_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]) as usize;
        offset += 4;

        let pq_bytes = &bytes[offset..offset + pq_len];
        let post_quantum = PostQuantumDigitalSignature::public_key_from_bytes(pq_bytes)?;

        Ok(HybridVerifyingKey {
            classical,
            post_quantum,
        })
    }

    fn signature_to_bytes(signature: &Self::Signature) -> Vec<u8> {
        let mut bytes = Vec::new();
        let classical_bytes = ClassicalDigitalSignature::signature_to_bytes(&signature.classical);
        let pq_bytes = PostQuantumDigitalSignature::signature_to_bytes(&signature.post_quantum);

        // Length-prefixed encoding
        bytes.extend_from_slice(&(classical_bytes.len() as u32).to_be_bytes());
        bytes.extend_from_slice(&classical_bytes);
        bytes.extend_from_slice(&(pq_bytes.len() as u32).to_be_bytes());
        bytes.extend_from_slice(&pq_bytes);

        bytes
    }

    fn signature_from_bytes(bytes: &[u8]) -> Result<Self::Signature> {
        if bytes.len() < 8 {
            return Err(NanoError::Crypto("Hybrid signature too short".to_string()));
        }

        let mut offset = 0;

        // Read classical signature
        let classical_len = u32::from_be_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]) as usize;
        offset += 4;

        let classical_bytes = &bytes[offset..offset + classical_len];
        let classical = ClassicalDigitalSignature::signature_from_bytes(classical_bytes)?;
        offset += classical_len;

        // Read post-quantum signature
        let pq_len = u32::from_be_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]) as usize;
        offset += 4;

        let pq_bytes = &bytes[offset..offset + pq_len];
        let post_quantum = PostQuantumDigitalSignature::signature_from_bytes(pq_bytes)?;

        Ok(HybridSignature {
            classical,
            post_quantum,
        })
    }
}

/// Hybrid asymmetric encryption using both X25519+ChaCha20Poly1305 and ML-KEM+ChaCha20Poly1305
pub struct HybridAsymmetricEncryption;

impl AsymmetricEncryption for HybridAsymmetricEncryption {
    type PublicKey = HybridPublicKey;

    fn encrypt(public_key: &Self::PublicKey, plaintext: &[u8]) -> Result<Vec<u8>> {
        // Encrypt with classical method
        let classical_ciphertext = ClassicalAsymmetricEncryption::encrypt(&public_key.classical, plaintext)?;

        // Encrypt with post-quantum method  
        let pq_ciphertext = PostQuantumAsymmetricEncryption::encrypt(&public_key.post_quantum, plaintext)?;

        // Combine both ciphertexts with length prefixes
        let mut result = Vec::new();
        result.extend_from_slice(&(classical_ciphertext.len() as u32).to_be_bytes());
        result.extend_from_slice(&classical_ciphertext);
        result.extend_from_slice(&(pq_ciphertext.len() as u32).to_be_bytes());
        result.extend_from_slice(&pq_ciphertext);

        Ok(result)
    }

    fn decrypt<KX: KeyExchange>(
        private_key: &KX::PrivateKey,
        ciphertext: &[u8],
    ) -> Result<Vec<u8>>
    where
        Self::PublicKey: From<KX::PublicKey>,
    {
        // Direct decryption for hybrid keys
        Self::decrypt_hybrid_direct(private_key, ciphertext)
    }
}

impl HybridAsymmetricEncryption {
    /// Direct decryption method for hybrid keys
    pub fn decrypt_hybrid_direct<T>(
        private_key: &T,
        ciphertext: &[u8],
    ) -> Result<Vec<u8>> {
        if ciphertext.len() < 8 {
            return Err(NanoError::Crypto("Hybrid ciphertext too short".to_string()));
        }

        let mut offset = 0;

        // Read classical ciphertext
        let classical_len = u32::from_be_bytes([
            ciphertext[offset],
            ciphertext[offset + 1],
            ciphertext[offset + 2],
            ciphertext[offset + 3],
        ]) as usize;
        offset += 4;

        let classical_ciphertext = &ciphertext[offset..offset + classical_len];
        offset += classical_len;

        // Read post-quantum ciphertext length
        let pq_len = u32::from_be_bytes([
            ciphertext[offset],
            ciphertext[offset + 1],
            ciphertext[offset + 2],
            ciphertext[offset + 3],
        ]) as usize;
        offset += 4;

        let pq_ciphertext = &ciphertext[offset..offset + pq_len];

        // This is a workaround for Session 2 - proper type design needed
        let private_ptr = private_key as *const T as *const HybridPrivateKey;
        let hybrid_private = unsafe { &*private_ptr };

        // Try to decrypt with both methods - either should work
        if let Ok(plaintext) = ClassicalAsymmetricEncryption::decrypt_classical_direct(
            &hybrid_private.classical,
            classical_ciphertext,
        ) {
            return Ok(plaintext);
        }

        if let Ok(plaintext) = PostQuantumAsymmetricEncryption::decrypt_pq_direct(
            &hybrid_private.post_quantum,
            pq_ciphertext,
        ) {
            return Ok(plaintext);
        }

        Err(NanoError::Crypto("Both hybrid decryption methods failed".to_string()))
    }
}

/// Combined user keypair for hybrid cryptography
#[derive(Clone)]
pub struct HybridUserKeyPair {
    pub classical: ClassicalUserKeyPair,
    pub post_quantum: PostQuantumUserKeyPair,
}

impl HybridUserKeyPair {
    /// Generate a new random hybrid keypair
    pub fn generate() -> Self {
        Self {
            classical: ClassicalUserKeyPair::generate(),
            post_quantum: PostQuantumUserKeyPair::generate(),
        }
    }

    /// Get the public keys
    pub fn public_keys(&self) -> HybridUserPublicKeys {
        HybridUserPublicKeys {
            classical: self.classical.public_keys(),
            post_quantum: self.post_quantum.public_keys(),
        }
    }

    /// Get the public key as a string identifier (using classical part for compatibility)
    pub fn public_key_string(&self) -> String {
        let classical_str = self.classical.public_key_string();
        format!("hybrid-{}", classical_str)
    }
}

/// Public keys for hybrid cryptography
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HybridUserPublicKeys {
    pub classical: ClassicalUserPublicKeys,
    pub post_quantum: PostQuantumUserPublicKeys,
}

impl HybridUserPublicKeys {
    /// Get the public key string identifier
    pub fn public_key_string(&self) -> String {
        let classical_str = self.classical.public_key_string();
        format!("hybrid-{}", classical_str)
    }
}

// Serialization helper functions (reuse from other modules with appropriate type handling)
fn serialize_x25519_key<S>(key: &X25519PublicKey, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&general_purpose::STANDARD.encode(&key.to_bytes()))
}

fn deserialize_x25519_key<'de, D>(deserializer: D) -> std::result::Result<X25519PublicKey, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let encoded: String = Deserialize::deserialize(deserializer)?;
    let bytes = general_purpose::STANDARD
        .decode(&encoded)
        .map_err(serde::de::Error::custom)?;
    ClassicalKeyExchange::public_key_from_bytes(&bytes).map_err(serde::de::Error::custom)
}

fn serialize_ed25519_key<S>(key: &ClassicalEd25519PublicKey, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&general_purpose::STANDARD.encode(&key.to_bytes()))
}

fn deserialize_ed25519_key<'de, D>(deserializer: D) -> std::result::Result<ClassicalEd25519PublicKey, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let encoded: String = Deserialize::deserialize(deserializer)?;
    let bytes = general_purpose::STANDARD
        .decode(&encoded)
        .map_err(serde::de::Error::custom)?;
    ClassicalDigitalSignature::public_key_from_bytes(&bytes).map_err(serde::de::Error::custom)
}

fn serialize_pq_key<S>(
    key: &PostQuantumPublicKey,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let bytes = PostQuantumKeyExchange::public_key_to_bytes(key);
    serializer.serialize_str(&general_purpose::STANDARD.encode(&bytes))
}

fn deserialize_pq_key<'de, D>(
    deserializer: D,
) -> std::result::Result<PostQuantumPublicKey, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let encoded: String = Deserialize::deserialize(deserializer)?;
    let bytes = general_purpose::STANDARD
        .decode(&encoded)
        .map_err(serde::de::Error::custom)?;
    PostQuantumKeyExchange::public_key_from_bytes(&bytes).map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hybrid_signatures() {
        let private_key = HybridDigitalSignature::generate_private_key();
        let public_key = HybridDigitalSignature::derive_public_key(&private_key);

        let data = b"test message for hybrid signatures";
        let signature = HybridDigitalSignature::sign(&private_key, data);

        // Should verify successfully
        HybridDigitalSignature::verify(&public_key, data, &signature).unwrap();

        // Should fail with wrong data
        assert!(HybridDigitalSignature::verify(&public_key, b"wrong data", &signature).is_err());
    }

    #[test]
    fn test_hybrid_keypair() {
        let keypair = HybridUserKeyPair::generate();
        let _public_keys = keypair.public_keys();

        // Test public key string format
        let pubkey_str = keypair.public_key_string();
        assert!(pubkey_str.starts_with("hybrid-pubkey:"));
    }

    #[test]
    fn test_hybrid_key_serialization() {
        let keypair = HybridUserKeyPair::generate();
        let public_keys = keypair.public_keys();

        // Test JSON serialization
        let json = serde_json::to_string(&public_keys).unwrap();
        let deserialized: HybridUserPublicKeys = serde_json::from_str(&json).unwrap();

        // Verify classical components match
        assert_eq!(
            public_keys.classical.verifying_key.to_bytes(),
            deserialized.classical.verifying_key.to_bytes()
        );
        assert_eq!(
            public_keys.classical.x25519_key.to_bytes(),
            deserialized.classical.x25519_key.to_bytes()
        );

        // Verify post-quantum components match
        assert_eq!(
            public_keys.post_quantum.public_key.kem_key,
            deserialized.post_quantum.public_key.kem_key
        );
        assert_eq!(
            public_keys.post_quantum.public_key.sign_key,
            deserialized.post_quantum.public_key.sign_key
        );
    }

    #[test]
    fn test_hybrid_signature_serialization() {
        let private_key = HybridDigitalSignature::generate_private_key();
        let data = b"test data";
        let signature = HybridDigitalSignature::sign(&private_key, data);

        // Test signature serialization round-trip
        let sig_bytes = HybridDigitalSignature::signature_to_bytes(&signature);
        let deserialized_sig = HybridDigitalSignature::signature_from_bytes(&sig_bytes).unwrap();

        // Verify by re-creating the public key and verifying both signatures
        let public_key = HybridDigitalSignature::derive_public_key(&private_key);
        HybridDigitalSignature::verify(&public_key, data, &signature).unwrap();
        HybridDigitalSignature::verify(&public_key, data, &deserialized_sig).unwrap();
    }
}
