/// Quantum-Resistant File Encryption
/// 
/// Provides file encryption using the same hybrid cryptography
/// as the message system, supporting classical, hybrid, and post-quantum modes

use crate::crypto::{
    CryptoMode, CryptoInterface, UnifiedPublicKeys, 
    ClassicalAsymmetricEncryption, HybridAsymmetricEncryption,
    PostQuantumAsymmetricEncryption, hash_sha256,
    traits::AsymmetricEncryption, // Import the trait
};
use crate::error::{NanoError, Result};
use blake2::{Blake2b512, Digest};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit},
    ChaCha20Poly1305, Nonce, Key
};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Read;

/// File encryption key (32 bytes for ChaCha20Poly1305)
pub type FileKey = [u8; 32];

/// File encryption implementation
pub struct FileEncryption {
    crypto_mode: CryptoMode,
    chunk_size_mb: u64,
}

impl FileEncryption {
    /// Create a new file encryption instance
    pub fn new(crypto_mode: CryptoMode, chunk_size_mb: u64) -> Self {
        Self {
            crypto_mode,
            chunk_size_mb,
        }
    }

    /// Get chunk size in bytes
    pub fn chunk_size_bytes(&self) -> usize {
        (self.chunk_size_mb * 1024 * 1024) as usize
    }

    /// Generate a random file encryption key
    pub fn generate_file_key() -> FileKey {
        let mut key = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut key);
        key
    }

    /// Encrypt a file with quantum-resistant encryption
    pub fn encrypt_file(
        &self,
        content: &[u8],
        recipient_keys: &UnifiedPublicKeys,
    ) -> Result<EncryptedFile> {
        // Generate random file encryption key
        let file_key = Self::generate_file_key();

        // Encrypt file content with ChaCha20Poly1305
        let encrypted_content = self.encrypt_content_symmetric(&file_key, content)?;
        let encrypted_size = encrypted_content.len() as u64;

        // Encrypt file key with recipient's public key using appropriate crypto mode
        let encrypted_key = self.encrypt_file_key(&file_key, recipient_keys)?;

        // Generate integrity hash of original content
        let integrity_hash = self.compute_integrity_hash(content);

        // Generate content hash for deduplication
        let content_hash = self.compute_content_hash(content);

        Ok(EncryptedFile {
            encrypted_content,
            encrypted_key,
            integrity_hash,
            content_hash,
            crypto_mode: self.crypto_mode,
            encryption_metadata: EncryptionMetadata {
                algorithm: "ChaCha20Poly1305".to_string(),
                key_algorithm: self.get_key_algorithm_name(),
                nonce_size: 12, // ChaCha20Poly1305 uses 12-byte nonces
                tag_size: 16,   // ChaCha20Poly1305 uses 16-byte authentication tags
                original_size: content.len() as u64,
                encrypted_size,
                compression_used: false,
                custom_params: HashMap::new(),
            },
        })
    }

    /// Decrypt a file
    pub fn decrypt_file(
        &self,
        encrypted_file: &EncryptedFile,
        private_key: &[u8], // Generic private key bytes
    ) -> Result<Vec<u8>> {
        // Decrypt the file key
        let file_key = self.decrypt_file_key(&encrypted_file.encrypted_key, private_key)?;

        // Decrypt the file content
        let content = self.decrypt_content_symmetric(&file_key, &encrypted_file.encrypted_content)?;

        // Verify integrity
        let computed_hash = self.compute_integrity_hash(&content);
        if computed_hash != encrypted_file.integrity_hash {
            return Err(NanoError::Crypto(
                "File integrity verification failed".to_string()
            ));
        }

        Ok(content)
    }

    /// Encrypt file in chunks for large files
    pub fn encrypt_file_chunked<R: Read>(
        &self,
        mut reader: R,
        recipient_keys: &UnifiedPublicKeys,
    ) -> Result<ChunkedEncryptedFile> {
        let chunk_size = self.chunk_size_bytes();
        let file_key = Self::generate_file_key();
        let mut chunks = Vec::new();
        let mut total_original_size = 0u64;
        let mut total_encrypted_size = 0u64;

        // Hash original content as we read it
        let mut hasher = Blake2b512::new();

        let mut buffer = vec![0u8; chunk_size];
        let mut chunk_index = 0;

        loop {
            let bytes_read = reader.read(&mut buffer).map_err(|e| {
                NanoError::Media(format!("Failed to read chunk: {}", e))
            })?;

            if bytes_read == 0 {
                break; // End of file
            }

            let chunk_data = &buffer[..bytes_read];
            hasher.update(chunk_data);
            total_original_size += bytes_read as u64;

            // Encrypt this chunk
            let encrypted_chunk = self.encrypt_content_symmetric(&file_key, chunk_data)?;
            total_encrypted_size += encrypted_chunk.len() as u64;

            chunks.push(EncryptedChunk {
                index: chunk_index,
                encrypted_data: encrypted_chunk,
                original_size: bytes_read as u64,
            });

            chunk_index += 1;
        }

        // Compute final integrity hash
        let integrity_hash = hasher.finalize().to_vec();

        // Encrypt the file key
        let encrypted_key = self.encrypt_file_key(&file_key, recipient_keys)?;

        Ok(ChunkedEncryptedFile {
            chunks,
            encrypted_key,
            integrity_hash,
            crypto_mode: self.crypto_mode,
            chunk_metadata: ChunkMetadata {
                total_chunks: chunk_index,
                chunk_size_bytes: chunk_size as u64,
                total_original_size,
                total_encrypted_size,
            },
            encryption_metadata: EncryptionMetadata {
                algorithm: "ChaCha20Poly1305".to_string(),
                key_algorithm: self.get_key_algorithm_name(),
                nonce_size: 12,
                tag_size: 16,
                original_size: total_original_size,
                encrypted_size: total_encrypted_size,
                compression_used: false,
                custom_params: HashMap::new(),
            },
        })
    }

    /// Decrypt chunked file
    pub fn decrypt_file_chunked(
        &self,
        chunked_file: &ChunkedEncryptedFile,
        private_key: &[u8],
    ) -> Result<Vec<u8>> {
        // Decrypt the file key
        let file_key = self.decrypt_file_key(&chunked_file.encrypted_key, private_key)?;

        let mut content = Vec::with_capacity(chunked_file.chunk_metadata.total_original_size as usize);
        let mut hasher = Blake2b512::new();

        // Sort chunks by index to ensure correct order
        let mut sorted_chunks = chunked_file.chunks.clone();
        sorted_chunks.sort_by_key(|chunk| chunk.index);

        // Decrypt each chunk
        for chunk in &sorted_chunks {
            let chunk_content = self.decrypt_content_symmetric(&file_key, &chunk.encrypted_data)?;
            hasher.update(&chunk_content);
            content.extend_from_slice(&chunk_content);
        }

        // Verify integrity
        let computed_hash = hasher.finalize().to_vec();
        if computed_hash != chunked_file.integrity_hash {
            return Err(NanoError::Crypto(
                "File integrity verification failed".to_string()
            ));
        }

        Ok(content)
    }

    /// Encrypt content with symmetric encryption
    pub fn encrypt_content_symmetric(&self, key: &FileKey, content: &[u8]) -> Result<Vec<u8>> {
        let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
        let nonce = ChaCha20Poly1305::generate_nonce(&mut rand::thread_rng());
        
        let ciphertext = cipher.encrypt(&nonce, content).map_err(|e| {
            NanoError::Crypto(format!("Symmetric encryption failed: {}", e))
        })?;

        // Prepend nonce to ciphertext
        let mut result = Vec::with_capacity(nonce.len() + ciphertext.len());
        result.extend_from_slice(&nonce);
        result.extend_from_slice(&ciphertext);
        
        Ok(result)
    }

    /// Decrypt content with symmetric encryption
    pub fn decrypt_content_symmetric(&self, key: &FileKey, encrypted_content: &[u8]) -> Result<Vec<u8>> {
        if encrypted_content.len() < 12 {
            return Err(NanoError::Crypto(
                "Encrypted content too short to contain nonce".to_string()
            ));
        }

        let (nonce_bytes, ciphertext) = encrypted_content.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        
        let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
        
        cipher.decrypt(nonce, ciphertext).map_err(|e| {
            NanoError::Crypto(format!("Symmetric decryption failed: {}", e))
        })
    }

    /// Encrypt file key using recipient's public key
    fn encrypt_file_key(&self, file_key: &FileKey, recipient_keys: &UnifiedPublicKeys) -> Result<Vec<u8>> {
        match (&self.crypto_mode, recipient_keys) {
            (CryptoMode::Classical, UnifiedPublicKeys::Classical(keys)) => {
                ClassicalAsymmetricEncryption::encrypt(&keys.x25519_key, file_key)
            }
            (CryptoMode::Hybrid, UnifiedPublicKeys::Hybrid(keys)) => {
                let hybrid_public_key = crate::crypto::hybrid::HybridPublicKey {
                    classical: keys.classical.x25519_key,
                    post_quantum: keys.post_quantum.public_key.clone(),
                };
                HybridAsymmetricEncryption::encrypt(&hybrid_public_key, file_key)
            }
            (CryptoMode::Quantum, UnifiedPublicKeys::PostQuantum(keys)) => {
                PostQuantumAsymmetricEncryption::encrypt(&keys.public_key, file_key)
            }
            // Cross-mode compatibility
            (CryptoMode::Hybrid, UnifiedPublicKeys::Classical(keys)) => {
                // Hybrid mode can encrypt for classical recipients
                ClassicalAsymmetricEncryption::encrypt(&keys.x25519_key, file_key)
            }
            (CryptoMode::Quantum, UnifiedPublicKeys::Classical(keys)) => {
                // Post-quantum mode can encrypt for classical recipients (fallback)
                ClassicalAsymmetricEncryption::encrypt(&keys.x25519_key, file_key)
            }
            (CryptoMode::Quantum, UnifiedPublicKeys::Hybrid(keys)) => {
                // Post-quantum mode can encrypt for hybrid recipients
                let hybrid_public_key = crate::crypto::hybrid::HybridPublicKey {
                    classical: keys.classical.x25519_key,
                    post_quantum: keys.post_quantum.public_key.clone(),
                };
                HybridAsymmetricEncryption::encrypt(&hybrid_public_key, file_key)
            }
            _ => {
                Err(NanoError::Crypto(format!(
                    "Cannot encrypt with {:?} mode for {:?} recipient keys",
                    self.crypto_mode, recipient_keys.mode()
                )))
            }
        }
    }

    /// Decrypt file key using private key
    fn decrypt_file_key(&self, encrypted_key: &[u8], private_key: &[u8]) -> Result<FileKey> {
        // This is a simplified implementation - in practice, you'd need to know
        // which type of private key you have and use the appropriate decryption
        
        // For now, assume classical decryption (this would need to be enhanced)
        let result = CryptoInterface::decrypt_symmetric(
            &private_key.try_into().map_err(|_| {
                NanoError::Crypto("Invalid private key size".to_string())
            })?,
            encrypted_key
        )?;

        if result.len() != 32 {
            return Err(NanoError::Crypto(
                "Decrypted file key has invalid size".to_string()
            ));
        }

        let mut file_key = [0u8; 32];
        file_key.copy_from_slice(&result);
        Ok(file_key)
    }

    /// Compute integrity hash of content
    pub fn compute_integrity_hash(&self, content: &[u8]) -> Vec<u8> {
        let mut hasher = Blake2b512::new();
        hasher.update(content);
        hasher.finalize().to_vec()
    }

    /// Compute content hash for deduplication
    pub fn compute_content_hash(&self, content: &[u8]) -> Vec<u8> {
        // Use SHA-256 for content hashing (deterministic)
        hash_sha256(content).to_vec()
    }

    /// Get the name of the key algorithm being used
    fn get_key_algorithm_name(&self) -> String {
        match self.crypto_mode {
            CryptoMode::Classical => "X25519+Ed25519".to_string(),
            CryptoMode::Hybrid => "X25519+ML-KEM-768".to_string(),
            CryptoMode::Quantum | CryptoMode::QuantumSafe => "ML-KEM-768+ML-DSA-65".to_string(),
        }
    }
}

/// Encrypted file data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedFile {
    pub encrypted_content: Vec<u8>,
    pub encrypted_key: Vec<u8>,
    pub integrity_hash: Vec<u8>,
    pub content_hash: Vec<u8>,
    pub crypto_mode: CryptoMode,
    pub encryption_metadata: EncryptionMetadata,
}

/// Chunked encrypted file for large files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkedEncryptedFile {
    pub chunks: Vec<EncryptedChunk>,
    pub encrypted_key: Vec<u8>,
    pub integrity_hash: Vec<u8>,
    pub crypto_mode: CryptoMode,
    pub chunk_metadata: ChunkMetadata,
    pub encryption_metadata: EncryptionMetadata,
}

/// Individual encrypted chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedChunk {
    pub index: u32,
    pub encrypted_data: Vec<u8>,
    pub original_size: u64,
}

/// Metadata about chunks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkMetadata {
    pub total_chunks: u32,
    pub chunk_size_bytes: u64,
    pub total_original_size: u64,
    pub total_encrypted_size: u64,
}

/// Encryption metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionMetadata {
    pub algorithm: String,
    pub key_algorithm: String,
    pub nonce_size: u8,
    pub tag_size: u8,
    pub original_size: u64,
    pub encrypted_size: u64,
    pub compression_used: bool,
    pub custom_params: HashMap<String, String>,
}

impl Default for EncryptionMetadata {
    fn default() -> Self {
        Self {
            algorithm: "ChaCha20Poly1305".to_string(),
            key_algorithm: "X25519+Ed25519".to_string(),
            nonce_size: 12,
            tag_size: 16,
            original_size: 0,
            encrypted_size: 0,
            compression_used: false,
            custom_params: HashMap::new(),
        }
    }
}

impl EncryptionMetadata {
    /// Calculate encryption overhead percentage
    pub fn overhead_percentage(&self) -> f64 {
        if self.original_size == 0 {
            return 0.0;
        }
        
        let overhead = self.encrypted_size as f64 - self.original_size as f64;
        (overhead / self.original_size as f64) * 100.0
    }

    /// Check if encryption is quantum-resistant
    pub fn is_quantum_resistant(&self) -> bool {
        self.key_algorithm.contains("ML-KEM") || self.key_algorithm.contains("ML-DSA")
    }
}

/// File encryption performance metrics
#[derive(Debug, Clone, Serialize)]
pub struct FileEncryptionMetrics {
    pub encryption_time_ms: u64,
    pub decryption_time_ms: u64,
    pub throughput_mbps: f64,
    pub overhead_percentage: f64,
    pub crypto_mode: CryptoMode,
    pub file_size_bytes: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::generate_keypair;

    #[test]
    fn test_file_key_generation() {
        let key1 = FileEncryption::generate_file_key();
        let key2 = FileEncryption::generate_file_key();
        
        // Keys should be different
        assert_ne!(key1, key2);
        
        // Keys should be 32 bytes
        assert_eq!(key1.len(), 32);
        assert_eq!(key2.len(), 32);
    }

    #[test]
    fn test_symmetric_encryption_decryption() {
        let file_encryption = FileEncryption::new(CryptoMode::Classical, 10);
        let key = FileEncryption::generate_file_key();
        let content = b"test file content for encryption";

        let encrypted = file_encryption.encrypt_content_symmetric(&key, content).unwrap();
        let decrypted = file_encryption.decrypt_content_symmetric(&key, &encrypted).unwrap();

        assert_eq!(decrypted, content);
        assert_ne!(encrypted.len(), content.len()); // Should have nonce + tag overhead
    }

    #[test]
    fn test_integrity_and_content_hashing() {
        let file_encryption = FileEncryption::new(CryptoMode::Classical, 10);
        let content1 = b"test content 1";
        let content2 = b"test content 2";

        let hash1 = file_encryption.compute_integrity_hash(content1);
        let hash2 = file_encryption.compute_integrity_hash(content2);
        let hash1_again = file_encryption.compute_integrity_hash(content1);

        // Same content should produce same hash
        assert_eq!(hash1, hash1_again);
        
        // Different content should produce different hashes
        assert_ne!(hash1, hash2);

        // Content hashes should be deterministic
        let content_hash1 = file_encryption.compute_content_hash(content1);
        let content_hash1_again = file_encryption.compute_content_hash(content1);
        assert_eq!(content_hash1, content_hash1_again);
    }

    #[test]
    fn test_file_encryption_classical_mode() {
        let file_encryption = FileEncryption::new(CryptoMode::Classical, 10);
        let keypair = generate_keypair();
        let recipient_keys = keypair.public_keys();
        let content = b"test file for classical encryption";

        let encrypted_file = file_encryption.encrypt_file(
            content,
            &crate::crypto::UnifiedPublicKeys::Classical(recipient_keys)
        ).unwrap();

        assert_eq!(encrypted_file.crypto_mode, CryptoMode::Classical);
        assert_eq!(encrypted_file.encryption_metadata.original_size, content.len() as u64);
        assert!(encrypted_file.encryption_metadata.encrypted_size > content.len() as u64);
        
        // Content should be different from original
        assert_ne!(encrypted_file.encrypted_content, content);
    }

    #[test]
    fn test_encryption_metadata() {
        let metadata = EncryptionMetadata {
            algorithm: "ChaCha20Poly1305".to_string(),
            key_algorithm: "X25519+Ed25519".to_string(),
            nonce_size: 12,
            tag_size: 16,
            original_size: 1000,
            encrypted_size: 1100,
            compression_used: false,
            custom_params: HashMap::new(),
        };

        assert_eq!(metadata.overhead_percentage(), 10.0);
        assert!(!metadata.is_quantum_resistant());

        let pq_metadata = EncryptionMetadata {
            key_algorithm: "ML-KEM-768+ML-DSA-65".to_string(),
            ..metadata
        };
        assert!(pq_metadata.is_quantum_resistant());
    }

    #[test]
    fn test_chunked_encryption_small_data() {
        let file_encryption = FileEncryption::new(CryptoMode::Classical, 1); // 1MB chunks
        let keypair = generate_keypair();
        let recipient_keys = crate::crypto::UnifiedPublicKeys::Classical(keypair.public_keys());
        
        let content = b"small test content that fits in one chunk";
        let mut cursor = std::io::Cursor::new(content);

        let chunked_file = file_encryption.encrypt_file_chunked(&mut cursor, &recipient_keys).unwrap();

        assert_eq!(chunked_file.chunks.len(), 1);
        assert_eq!(chunked_file.chunk_metadata.total_chunks, 1);
        assert_eq!(chunked_file.chunk_metadata.total_original_size, content.len() as u64);
        assert_eq!(chunked_file.chunks[0].index, 0);
        assert_eq!(chunked_file.chunks[0].original_size, content.len() as u64);
    }

    #[test]
    fn test_chunk_size_calculation() {
        let file_encryption_1mb = FileEncryption::new(CryptoMode::Classical, 1);
        let file_encryption_10mb = FileEncryption::new(CryptoMode::Classical, 10);

        assert_eq!(file_encryption_1mb.chunk_size_bytes(), 1024 * 1024);
        assert_eq!(file_encryption_10mb.chunk_size_bytes(), 10 * 1024 * 1024);
    }
}
