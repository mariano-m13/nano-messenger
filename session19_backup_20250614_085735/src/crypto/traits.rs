use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Trait for key exchange implementations
pub trait KeyExchange {
    type PrivateKey: Clone + Send + Sync;
    type PublicKey: Clone + Send + Sync + Serialize + for<'de> Deserialize<'de>;
    type SharedSecret: AsRef<[u8]>;

    /// Generate a new private key
    fn generate_private_key() -> Self::PrivateKey;

    /// Derive the public key from a private key
    fn derive_public_key(private_key: &Self::PrivateKey) -> Self::PublicKey;

    /// Perform key exchange to derive shared secret
    fn key_exchange(
        private_key: &Self::PrivateKey,
        public_key: &Self::PublicKey,
    ) -> Result<Self::SharedSecret>;

    /// Get the public key as bytes for serialization
    fn public_key_to_bytes(public_key: &Self::PublicKey) -> Vec<u8>;

    /// Recreate public key from bytes
    fn public_key_from_bytes(bytes: &[u8]) -> Result<Self::PublicKey>;
}

/// Trait for digital signature implementations
pub trait DigitalSignature {
    type PrivateKey: Clone + Send + Sync;
    type PublicKey: Clone + Send + Sync + Serialize + for<'de> Deserialize<'de>;
    type Signature: Clone + Send + Sync;

    /// Generate a new private key
    fn generate_private_key() -> Self::PrivateKey;

    /// Derive the public key from a private key
    fn derive_public_key(private_key: &Self::PrivateKey) -> Self::PublicKey;

    /// Sign data with the private key
    fn sign(private_key: &Self::PrivateKey, data: &[u8]) -> Self::Signature;

    /// Verify a signature with the public key
    fn verify(
        public_key: &Self::PublicKey,
        data: &[u8],
        signature: &Self::Signature,
    ) -> Result<()>;

    /// Get the public key as bytes for serialization
    fn public_key_to_bytes(public_key: &Self::PublicKey) -> Vec<u8>;

    /// Recreate public key from bytes
    fn public_key_from_bytes(bytes: &[u8]) -> Result<Self::PublicKey>;

    /// Get signature as bytes for serialization
    fn signature_to_bytes(signature: &Self::Signature) -> Vec<u8>;

    /// Recreate signature from bytes
    fn signature_from_bytes(bytes: &[u8]) -> Result<Self::Signature>;
}

/// Trait for symmetric encryption implementations
pub trait SymmetricEncryption {
    /// Encrypt data with a 32-byte key
    fn encrypt(key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>>;

    /// Decrypt data with a 32-byte key
    fn decrypt(key: &[u8; 32], ciphertext: &[u8]) -> Result<Vec<u8>>;
}

/// Trait for asymmetric encryption (using key exchange + symmetric)
pub trait AsymmetricEncryption {
    type PublicKey: Clone + Send + Sync;

    /// Encrypt data for a recipient's public key
    fn encrypt(public_key: &Self::PublicKey, plaintext: &[u8]) -> Result<Vec<u8>>;

    /// Decrypt data with our private key (generic over key exchange implementation)
    fn decrypt<KX: KeyExchange>(
        private_key: &KX::PrivateKey,
        ciphertext: &[u8],
    ) -> Result<Vec<u8>>
    where
        Self::PublicKey: From<KX::PublicKey>;
}

/// Combined cryptographic provider trait
pub trait CryptoProvider: Send + Sync {
    type KeyExchange: KeyExchange;
    type DigitalSignature: DigitalSignature;
    type SymmetricEncryption: SymmetricEncryption;

    /// Get a reference to the key exchange implementation
    fn key_exchange() -> &'static Self::KeyExchange;

    /// Get a reference to the digital signature implementation
    fn digital_signature() -> &'static Self::DigitalSignature;

    /// Get a reference to the symmetric encryption implementation
    fn symmetric_encryption() -> &'static Self::SymmetricEncryption;
}
