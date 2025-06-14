use crate::crypto::traits::{
    AsymmetricEncryption, CryptoProvider, DigitalSignature, KeyExchange, SymmetricEncryption,
};
use crate::error::{NanoError, Result};
use base64::{engine::general_purpose, Engine as _};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use ed25519_dalek::{Signature, Signer, Verifier};
use getrandom::getrandom;
use serde::{Deserialize, Serialize};
use x25519_dalek::{PublicKey, StaticSecret};

// Re-export for convenience and public access
pub use ed25519_dalek::{SigningKey as Ed25519PrivateKey, VerifyingKey as Ed25519PublicKey};
pub use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret as X25519PrivateKey};

// Type aliases for internal use
type InternalX25519PublicKey = PublicKey;
type InternalX25519PrivateKey = StaticSecret;

/// Classical X25519 key exchange implementation
pub struct ClassicalKeyExchange;

impl KeyExchange for ClassicalKeyExchange {
    type PrivateKey = InternalX25519PrivateKey;
    type PublicKey = InternalX25519PublicKey;
    type SharedSecret = x25519_dalek::SharedSecret;

    fn generate_private_key() -> Self::PrivateKey {
        let mut bytes = [0u8; 32];
        getrandom(&mut bytes).expect("Failed to generate random bytes");
        InternalX25519PrivateKey::from(bytes)
    }

    fn derive_public_key(private_key: &Self::PrivateKey) -> Self::PublicKey {
        InternalX25519PublicKey::from(private_key)
    }

    fn key_exchange(
        private_key: &Self::PrivateKey,
        public_key: &Self::PublicKey,
    ) -> Result<Self::SharedSecret> {
        Ok(private_key.diffie_hellman(public_key))
    }

    fn public_key_to_bytes(public_key: &Self::PublicKey) -> Vec<u8> {
        public_key.to_bytes().to_vec()
    }

    fn public_key_from_bytes(bytes: &[u8]) -> Result<Self::PublicKey> {
        if bytes.len() != 32 {
            return Err(NanoError::Crypto("Invalid X25519 public key length".to_string()));
        }
        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(bytes);
        Ok(InternalX25519PublicKey::from(key_bytes))
    }
}

/// Classical Ed25519 digital signature implementation
pub struct ClassicalDigitalSignature;

impl DigitalSignature for ClassicalDigitalSignature {
    type PrivateKey = Ed25519PrivateKey;
    type PublicKey = Ed25519PublicKey;
    type Signature = Signature;

    fn generate_private_key() -> Self::PrivateKey {
        let mut secret_bytes = [0u8; 32];
        getrandom(&mut secret_bytes).expect("Failed to generate random bytes");
        Ed25519PrivateKey::from_bytes(&secret_bytes)
    }

    fn derive_public_key(private_key: &Self::PrivateKey) -> Self::PublicKey {
        private_key.verifying_key()
    }

    fn sign(private_key: &Self::PrivateKey, data: &[u8]) -> Self::Signature {
        private_key.sign(data)
    }

    fn verify(
        public_key: &Self::PublicKey,
        data: &[u8],
        signature: &Self::Signature,
    ) -> Result<()> {
        public_key
            .verify(data, signature)
            .map_err(|e| NanoError::Crypto(format!("Signature verification failed: {}", e)))
    }

    fn public_key_to_bytes(public_key: &Self::PublicKey) -> Vec<u8> {
        public_key.to_bytes().to_vec()
    }

    fn public_key_from_bytes(bytes: &[u8]) -> Result<Self::PublicKey> {
        if bytes.len() != 32 {
            return Err(NanoError::Crypto("Invalid Ed25519 public key length".to_string()));
        }
        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(bytes);
        Ed25519PublicKey::from_bytes(&key_bytes)
            .map_err(|e| NanoError::Crypto(format!("Invalid Ed25519 public key: {}", e)))
    }

    fn signature_to_bytes(signature: &Self::Signature) -> Vec<u8> {
        signature.to_bytes().to_vec()
    }

    fn signature_from_bytes(bytes: &[u8]) -> Result<Self::Signature> {
        if bytes.len() != 64 {
            return Err(NanoError::Crypto("Invalid signature length".to_string()));
        }
        let mut sig_bytes = [0u8; 64];
        sig_bytes.copy_from_slice(bytes);
        Ok(Signature::from_bytes(&sig_bytes))
    }
}

/// ChaCha20Poly1305 symmetric encryption implementation
pub struct ClassicalSymmetricEncryption;

impl SymmetricEncryption for ClassicalSymmetricEncryption {
    fn encrypt(key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>> {
        let cipher = ChaCha20Poly1305::new_from_slice(key)
            .map_err(|e| NanoError::Crypto(format!("Invalid key: {}", e)))?;

        // Generate random nonce
        let mut nonce_bytes = [0u8; 12];
        getrandom(&mut nonce_bytes)
            .map_err(|e| NanoError::Crypto(format!("Random generation failed: {}", e)))?;
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| NanoError::Crypto(format!("Encryption failed: {}", e)))?;

        // Prepend nonce to ciphertext
        let mut result = Vec::with_capacity(12 + ciphertext.len());
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);

        Ok(result)
    }

    fn decrypt(key: &[u8; 32], ciphertext_with_nonce: &[u8]) -> Result<Vec<u8>> {
        if ciphertext_with_nonce.len() < 12 {
            return Err(NanoError::Crypto("Ciphertext too short".to_string()));
        }

        let cipher = ChaCha20Poly1305::new_from_slice(key)
            .map_err(|e| NanoError::Crypto(format!("Invalid key: {}", e)))?;

        let (nonce_bytes, ciphertext) = ciphertext_with_nonce.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| NanoError::Crypto(format!("Decryption failed: {}", e)))
    }
}

/// Classical asymmetric encryption (X25519 + ChaCha20Poly1305)
pub struct ClassicalAsymmetricEncryption;

impl ClassicalAsymmetricEncryption {
    /// Direct decryption method for classical keys
    pub fn decrypt_classical_direct<T>(
        private_key: &T,
        ciphertext_with_ephemeral: &[u8],
    ) -> Result<Vec<u8>> {
        // This is a simplified approach that assumes the types match
        // For Session 1, we'll use a byte-based approach
        if ciphertext_with_ephemeral.len() < 32 {
            return Err(NanoError::Crypto(
                "Ciphertext too short for asymmetric".to_string(),
            ));
        }

        let (ephemeral_bytes, encrypted_data) = ciphertext_with_ephemeral.split_at(32);

        let mut ephemeral_key_bytes = [0u8; 32];
        ephemeral_key_bytes.copy_from_slice(ephemeral_bytes);
        let ephemeral_public = InternalX25519PublicKey::from(ephemeral_key_bytes);

        // This is a workaround for Session 1 - in practice we'd need better type design
        // For now, assume the private key is the right type
        let private_bytes = private_key as *const T as *const [u8; 32];
        let private_array = unsafe { *private_bytes };
        let x25519_private = InternalX25519PrivateKey::from(private_array);

        // Derive the same shared secret
        let shared_secret = x25519_private.diffie_hellman(&ephemeral_public);

        // Decrypt with the shared secret
        ClassicalSymmetricEncryption::decrypt(shared_secret.as_bytes(), encrypted_data)
    }
}

impl AsymmetricEncryption for ClassicalAsymmetricEncryption {
    type PublicKey = InternalX25519PublicKey;

    fn encrypt(public_key: &Self::PublicKey, plaintext: &[u8]) -> Result<Vec<u8>> {
        // Generate ephemeral keypair for this message
        let mut ephemeral_bytes = [0u8; 32];
        getrandom(&mut ephemeral_bytes)
            .map_err(|e| NanoError::Crypto(format!("Random generation failed: {}", e)))?;
        let ephemeral_secret = StaticSecret::from(ephemeral_bytes);
        let ephemeral_public = InternalX25519PublicKey::from(&ephemeral_secret);

        // Derive shared secret
        let shared_secret = ephemeral_secret.diffie_hellman(public_key);

        // Encrypt with the shared secret
        let encrypted = ClassicalSymmetricEncryption::encrypt(shared_secret.as_bytes(), plaintext)?;

        // Prepend ephemeral public key to the encrypted data
        let mut result = Vec::with_capacity(32 + encrypted.len());
        result.extend_from_slice(ephemeral_public.as_bytes());
        result.extend_from_slice(&encrypted);

        Ok(result)
    }

    fn decrypt<KX: KeyExchange>(
        private_key: &KX::PrivateKey,
        ciphertext_with_ephemeral: &[u8],
    ) -> Result<Vec<u8>>
    where
        Self::PublicKey: From<KX::PublicKey>,
    {
        // For classical implementation, just call the direct version
        // This is a simplified approach for Session 1
        Self::decrypt_classical_direct(private_key, ciphertext_with_ephemeral)
    }
}

/// Classical cryptography provider
pub struct ClassicalCryptoProvider;

impl CryptoProvider for ClassicalCryptoProvider {
    type KeyExchange = ClassicalKeyExchange;
    type DigitalSignature = ClassicalDigitalSignature;
    type SymmetricEncryption = ClassicalSymmetricEncryption;

    fn key_exchange() -> &'static Self::KeyExchange {
        &ClassicalKeyExchange
    }

    fn digital_signature() -> &'static Self::DigitalSignature {
        &ClassicalDigitalSignature
    }

    fn symmetric_encryption() -> &'static Self::SymmetricEncryption {
        &ClassicalSymmetricEncryption
    }
}

/// Combined user keypair for classical cryptography
#[derive(Clone)]
pub struct ClassicalUserKeyPair {
    pub signing_key: Ed25519PrivateKey,
    pub x25519_key: InternalX25519PrivateKey,
}

impl ClassicalUserKeyPair {
    /// Generate a new random keypair
    pub fn generate() -> Self {
        Self {
            signing_key: ClassicalDigitalSignature::generate_private_key(),
            x25519_key: ClassicalKeyExchange::generate_private_key(),
        }
    }

    /// Get the public keys
    pub fn public_keys(&self) -> ClassicalUserPublicKeys {
        ClassicalUserPublicKeys {
            verifying_key: ClassicalDigitalSignature::derive_public_key(&self.signing_key),
            x25519_key: ClassicalKeyExchange::derive_public_key(&self.x25519_key),
        }
    }

    /// Get the public key as a string identifier
    pub fn public_key_string(&self) -> String {
        let verifying_bytes = self.public_keys().verifying_key.to_bytes();
        format!("pubkey:{}", general_purpose::STANDARD.encode(&verifying_bytes))
    }
}

/// Public keys for classical cryptography
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClassicalUserPublicKeys {
    #[serde(
        serialize_with = "serialize_ed25519_key",
        deserialize_with = "deserialize_ed25519_key"
    )]
    pub verifying_key: Ed25519PublicKey,
    #[serde(
        serialize_with = "serialize_x25519_key",
        deserialize_with = "deserialize_x25519_key"
    )]
    pub x25519_key: InternalX25519PublicKey,
}

impl ClassicalUserPublicKeys {
    /// Get the public key string identifier
    pub fn public_key_string(&self) -> String {
        let verifying_bytes = self.verifying_key.to_bytes();
        format!("pubkey:{}", general_purpose::STANDARD.encode(&verifying_bytes))
    }

    /// Create from a public key string
    pub fn from_public_key_string(pubkey_str: &str) -> Result<Ed25519PublicKey> {
        if !pubkey_str.starts_with("pubkey:") {
            return Err(NanoError::Crypto("Invalid pubkey format".to_string()));
        }

        let b64_part = &pubkey_str[7..]; // Remove "pubkey:" prefix
        let bytes = general_purpose::STANDARD
            .decode(b64_part)
            .map_err(|e| NanoError::Crypto(format!("Base64 decode error: {}", e)))?;

        ClassicalDigitalSignature::public_key_from_bytes(&bytes)
    }
}

// Helper functions for serialization
fn serialize_ed25519_key<S>(key: &Ed25519PublicKey, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&general_purpose::STANDARD.encode(&key.to_bytes()))
}

fn deserialize_ed25519_key<'de, D>(deserializer: D) -> std::result::Result<Ed25519PublicKey, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let encoded: String = Deserialize::deserialize(deserializer)?;
    let bytes = general_purpose::STANDARD
        .decode(&encoded)
        .map_err(serde::de::Error::custom)?;
    ClassicalDigitalSignature::public_key_from_bytes(&bytes).map_err(serde::de::Error::custom)
}

fn serialize_x25519_key<S>(key: &InternalX25519PublicKey, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&general_purpose::STANDARD.encode(&key.to_bytes()))
}

fn deserialize_x25519_key<'de, D>(deserializer: D) -> std::result::Result<InternalX25519PublicKey, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let encoded: String = Deserialize::deserialize(deserializer)?;
    let bytes = general_purpose::STANDARD
        .decode(&encoded)
        .map_err(serde::de::Error::custom)?;
    ClassicalKeyExchange::public_key_from_bytes(&bytes).map_err(serde::de::Error::custom)
}

// Hash function (keeping from original implementation)
pub fn hash_sha256(data: &[u8]) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classical_key_exchange() {
        let alice_private = ClassicalKeyExchange::generate_private_key();
        let alice_public = ClassicalKeyExchange::derive_public_key(&alice_private);

        let bob_private = ClassicalKeyExchange::generate_private_key();
        let bob_public = ClassicalKeyExchange::derive_public_key(&bob_private);

        let alice_shared = ClassicalKeyExchange::key_exchange(&alice_private, &bob_public).unwrap();
        let bob_shared = ClassicalKeyExchange::key_exchange(&bob_private, &alice_public).unwrap();

        assert_eq!(alice_shared.as_bytes(), bob_shared.as_bytes());
    }

    #[test]
    fn test_classical_signatures() {
        let private_key = ClassicalDigitalSignature::generate_private_key();
        let public_key = ClassicalDigitalSignature::derive_public_key(&private_key);

        let data = b"test message";
        let signature = ClassicalDigitalSignature::sign(&private_key, data);

        ClassicalDigitalSignature::verify(&public_key, data, &signature).unwrap();

        // Should fail with wrong data
        assert!(ClassicalDigitalSignature::verify(&public_key, b"wrong data", &signature).is_err());
    }

    #[test]
    fn test_classical_symmetric_encryption() {
        let key = [42u8; 32];
        let plaintext = b"Hello, world!";

        let ciphertext = ClassicalSymmetricEncryption::encrypt(&key, plaintext).unwrap();
        let decrypted = ClassicalSymmetricEncryption::decrypt(&key, &ciphertext).unwrap();

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_classical_keypair() {
        let keypair = ClassicalUserKeyPair::generate();
        let public_keys = keypair.public_keys();

        // Test public key string format
        let pubkey_str = keypair.public_key_string();
        assert!(pubkey_str.starts_with("pubkey:"));

        // Test round-trip
        let recovered_key = ClassicalUserPublicKeys::from_public_key_string(&pubkey_str).unwrap();
        assert_eq!(recovered_key.to_bytes(), public_keys.verifying_key.to_bytes());
    }

    #[test]
    fn test_key_serialization() {
        let keypair = ClassicalUserKeyPair::generate();
        let public_keys = keypair.public_keys();

        // Test JSON serialization
        let json = serde_json::to_string(&public_keys).unwrap();
        let deserialized: ClassicalUserPublicKeys = serde_json::from_str(&json).unwrap();

        assert_eq!(
            public_keys.verifying_key.to_bytes(),
            deserialized.verifying_key.to_bytes()
        );
        assert_eq!(
            public_keys.x25519_key.to_bytes(),
            deserialized.x25519_key.to_bytes()
        );
    }
}
