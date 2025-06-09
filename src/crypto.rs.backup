use crate::error::{NanoError, Result};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use ed25519_dalek::{Signature, Signer, Verifier};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use x25519_dalek::{EphemeralSecret, PublicKey, StaticSecret};
use base64::{Engine as _, engine::general_purpose};
use rand::{RngCore, thread_rng};

// Re-export key types for convenience
pub use ed25519_dalek::{SigningKey as Ed25519PrivateKey, VerifyingKey as Ed25519PublicKey};
pub use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret as X25519PrivateKey};

/// Represents a user's complete keypair for the nano-messenger system
#[derive(Clone)]
pub struct UserKeyPair {
    pub signing_key: Ed25519PrivateKey,
    pub x25519_key: X25519PrivateKey,
}

impl UserKeyPair {
    /// Generate a new random keypair
    pub fn generate() -> Self {
        let mut rng = thread_rng();
        
        // Generate Ed25519 signing key using proper method
        let mut secret_bytes = [0u8; 32];
        rng.fill_bytes(&mut secret_bytes);
        let signing_key = Ed25519PrivateKey::from_bytes(&secret_bytes);
        
        // Generate X25519 key using 1.x API
        let mut x25519_bytes = [0u8; 32];
        rng.fill_bytes(&mut x25519_bytes);
        let x25519_key = X25519PrivateKey::from(x25519_bytes);
        
        Self {
            signing_key,
            x25519_key,
        }
    }
    
    /// Get the public keys
    pub fn public_keys(&self) -> UserPublicKeys {
        UserPublicKeys {
            verifying_key: self.signing_key.verifying_key(),
            x25519_key: X25519PublicKey::from(&self.x25519_key),
        }
    }
    
    /// Get the public key as a string identifier (for pubkey references)
    pub fn public_key_string(&self) -> String {
        let verifying_bytes = self.signing_key.verifying_key().to_bytes();
        format!("pubkey:{}", general_purpose::STANDARD.encode(&verifying_bytes))
    }
}

/// Public keys for a user
#[derive(Clone, Debug)]
pub struct UserPublicKeys {
    pub verifying_key: Ed25519PublicKey,
    pub x25519_key: X25519PublicKey,
}

// Custom serialization for UserPublicKeys
impl Serialize for UserPublicKeys {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("UserPublicKeys", 2)?;
        state.serialize_field("verifying_key", &general_purpose::STANDARD.encode(&self.verifying_key.to_bytes()))?;
        state.serialize_field("x25519_key", &general_purpose::STANDARD.encode(&self.x25519_key.to_bytes()))?;
        state.end()
    }
}

// Custom deserialization for UserPublicKeys
impl<'de> Deserialize<'de> for UserPublicKeys {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;

        struct UserPublicKeysVisitor;

        impl<'de> Visitor<'de> for UserPublicKeysVisitor {
            type Value = UserPublicKeys;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct UserPublicKeys")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UserPublicKeys, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut verifying_key = None;
                let mut x25519_key = None;
                
                while let Some(key) = map.next_key()? {
                    match key {
                        "verifying_key" => {
                            let encoded: String = map.next_value()?;
                            let bytes = general_purpose::STANDARD.decode(&encoded)
                                .map_err(de::Error::custom)?;
                            if bytes.len() != 32 {
                                return Err(de::Error::custom("Invalid verifying key length"));
                            }
                            let mut key_bytes = [0u8; 32];
                            key_bytes.copy_from_slice(&bytes);
                            verifying_key = Some(Ed25519PublicKey::from_bytes(&key_bytes)
                                .map_err(de::Error::custom)?);
                        }
                        "x25519_key" => {
                            let encoded: String = map.next_value()?;
                            let bytes = general_purpose::STANDARD.decode(&encoded)
                                .map_err(de::Error::custom)?;
                            if bytes.len() != 32 {
                                return Err(de::Error::custom("Invalid x25519 key length"));
                            }
                            let mut key_bytes = [0u8; 32];
                            key_bytes.copy_from_slice(&bytes);
                            x25519_key = Some(X25519PublicKey::from(key_bytes));
                        }
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                
                let verifying_key = verifying_key.ok_or_else(|| de::Error::missing_field("verifying_key"))?;
                let x25519_key = x25519_key.ok_or_else(|| de::Error::missing_field("x25519_key"))?;
                
                Ok(UserPublicKeys {
                    verifying_key,
                    x25519_key,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["verifying_key", "x25519_key"];
        deserializer.deserialize_struct("UserPublicKeys", FIELDS, UserPublicKeysVisitor)
    }
}

impl UserPublicKeys {
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
        let bytes = general_purpose::STANDARD.decode(b64_part)
            .map_err(|e| NanoError::Crypto(format!("Base64 decode error: {}", e)))?;
        
        if bytes.len() != 32 {
            return Err(NanoError::Crypto("Invalid pubkey length".to_string()));
        }
        
        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(&bytes);
        
        Ed25519PublicKey::from_bytes(&key_bytes)
            .map_err(|e| NanoError::Crypto(format!("Invalid public key: {}", e)))
    }
}

/// Perform ECDH key exchange to derive a shared secret
pub fn derive_shared_secret(
    our_private: &X25519PrivateKey,
    their_public: &X25519PublicKey,
) -> [u8; 32] {
    our_private.diffie_hellman(their_public).to_bytes()
}

/// Encrypt data using ChaCha20Poly1305 with the given key
pub fn encrypt_symmetric(key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>> {
    let cipher = ChaCha20Poly1305::new_from_slice(key)
        .map_err(|e| NanoError::Crypto(format!("Invalid key: {}", e)))?;
    
    // Generate random nonce
    let mut nonce_bytes = [0u8; 12];
    thread_rng().fill_bytes(&mut nonce_bytes);
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

/// Decrypt data using ChaCha20Poly1305 with the given key
pub fn decrypt_symmetric(key: &[u8; 32], ciphertext_with_nonce: &[u8]) -> Result<Vec<u8>> {
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

/// Encrypt data asymmetrically for first contact (using X25519 + ChaCha20Poly1305)
pub fn encrypt_asymmetric(their_public: &X25519PublicKey, plaintext: &[u8]) -> Result<Vec<u8>> {
    // Generate ephemeral keypair for this message
    let mut rng = thread_rng();
    let ephemeral_secret = EphemeralSecret::random_from_rng(&mut rng);
    let ephemeral_public = PublicKey::from(&ephemeral_secret);
    
    // Derive shared secret
    let shared_secret = ephemeral_secret.diffie_hellman(their_public);
    
    // Encrypt with the shared secret
    let encrypted = encrypt_symmetric(shared_secret.as_bytes(), plaintext)?;
    
    // Prepend ephemeral public key to the encrypted data
    let mut result = Vec::with_capacity(32 + encrypted.len());
    result.extend_from_slice(ephemeral_public.as_bytes());
    result.extend_from_slice(&encrypted);
    
    Ok(result)
}

/// Decrypt asymmetrically encrypted data
pub fn decrypt_asymmetric(our_private: &X25519PrivateKey, ciphertext_with_ephemeral: &[u8]) -> Result<Vec<u8>> {
    if ciphertext_with_ephemeral.len() < 32 {
        return Err(NanoError::Crypto("Ciphertext too short for asymmetric".to_string()));
    }
    
    let (ephemeral_bytes, encrypted_data) = ciphertext_with_ephemeral.split_at(32);
    
    let mut ephemeral_key_bytes = [0u8; 32];
    ephemeral_key_bytes.copy_from_slice(ephemeral_bytes);
    let ephemeral_public = X25519PublicKey::from(ephemeral_key_bytes);
    
    // Derive the same shared secret
    let shared_secret = our_private.diffie_hellman(&ephemeral_public);
    
    // Decrypt with the shared secret
    decrypt_symmetric(shared_secret.as_bytes(), encrypted_data)
}

/// Sign data with Ed25519
pub fn sign_data(signing_key: &Ed25519PrivateKey, data: &[u8]) -> Signature {
    signing_key.sign(data)
}

/// Verify Ed25519 signature
pub fn verify_signature(
    verifying_key: &Ed25519PublicKey,
    data: &[u8],
    signature: &Signature,
) -> Result<()> {
    verifying_key
        .verify(data, signature)
        .map_err(|e| NanoError::Crypto(format!("Signature verification failed: {}", e)))
}

/// Hash data with SHA256
pub fn hash_sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let keypair = UserKeyPair::generate();
        let public_keys = keypair.public_keys();
        
        // Test public key string format
        let pubkey_str = keypair.public_key_string();
        assert!(pubkey_str.starts_with("pubkey:"));
        
        // Test round-trip
        let recovered_key = UserPublicKeys::from_public_key_string(&pubkey_str).unwrap();
        assert_eq!(recovered_key.to_bytes(), public_keys.verifying_key.to_bytes());
    }

    #[test]
    fn test_symmetric_encryption() {
        let key = [42u8; 32];
        let plaintext = b"Hello, world!";
        
        let ciphertext = encrypt_symmetric(&key, plaintext).unwrap();
        let decrypted = decrypt_symmetric(&key, &ciphertext).unwrap();
        
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_asymmetric_encryption() {
        let keypair = UserKeyPair::generate();
        let public_keys = keypair.public_keys();
        let plaintext = b"Secret message";
        
        let ciphertext = encrypt_asymmetric(&public_keys.x25519_key, plaintext).unwrap();
        let decrypted = decrypt_asymmetric(&keypair.x25519_key, &ciphertext).unwrap();
        
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_ecdh() {
        let alice = UserKeyPair::generate();
        let bob = UserKeyPair::generate();
        
        let alice_public = alice.public_keys();
        let bob_public = bob.public_keys();
        
        let alice_shared = derive_shared_secret(&alice.x25519_key, &bob_public.x25519_key);
        let bob_shared = derive_shared_secret(&bob.x25519_key, &alice_public.x25519_key);
        
        assert_eq!(alice_shared, bob_shared);
    }

    #[test]
    fn test_signatures() {
        let keypair = UserKeyPair::generate();
        let public_keys = keypair.public_keys();
        let data = b"Sign this message";
        
        let signature = sign_data(&keypair.signing_key, data);
        verify_signature(&public_keys.verifying_key, data, &signature).unwrap();
        
        // Should fail with wrong data
        assert!(verify_signature(&public_keys.verifying_key, b"Wrong data", &signature).is_err());
    }
}
