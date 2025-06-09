use crate::crypto::{UserPublicKeys, CryptoMode, UnifiedPublicKeys, HybridUserPublicKeys};
use crate::error::{NanoError, Result};
use chrono::{DateTime, Utc};
use ed25519_dalek::Signature;
use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose};
use rand;

/// The outer message envelope sent over TCP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEnvelope {
    pub version: String,
    pub inbox_id: String,
    pub payload: String, // Base64 encoded encrypted payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<i64>, // Unix timestamp
    pub nonce: String,       // For deduplication/replay protection
}

impl MessageEnvelope {
    pub fn new(inbox_id: String, encrypted_payload: Vec<u8>) -> Self {
        Self {
            version: "1.1".to_string(),
            inbox_id,
            payload: general_purpose::STANDARD.encode(&encrypted_payload),
            expiry: None,
            nonce: general_purpose::STANDARD.encode(&rand::random::<[u8; 16]>()),
        }
    }

    pub fn with_expiry(mut self, expiry: DateTime<Utc>) -> Self {
        self.expiry = Some(expiry.timestamp());
        self
    }

    pub fn is_expired(&self) -> bool {
        if let Some(expiry) = self.expiry {
            Utc::now().timestamp() > expiry
        } else {
            false
        }
    }

    pub fn decode_payload(&self) -> Result<Vec<u8>> {
        general_purpose::STANDARD.decode(&self.payload)
            .map_err(|e| NanoError::Crypto(format!("Base64 decode error: {}", e)))
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(self).map_err(Into::into)
    }

    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(Into::into)
    }
}

/// The new quantum-safe message envelope for Session 3+
/// Supports multiple crypto modes with forward compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumSafeEnvelope {
    pub version: String,               // "2.0-quantum"
    pub crypto_mode: CryptoMode,       // Classical/Hybrid/Quantum
    pub inbox_id: String,
    pub payload: String,               // Base64 encoded encrypted payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pq_ciphertext: Option<String>, // ML-KEM data (base64)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pq_signature: Option<String>,  // ML-DSA data (base64)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<i64>,           // Unix timestamp
    pub nonce: String,                 // For deduplication/replay protection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_compat: Option<bool>,   // True if needs legacy format support
}

impl QuantumSafeEnvelope {
    pub fn new(crypto_mode: CryptoMode, inbox_id: String, encrypted_payload: Vec<u8>) -> Self {
        Self {
            version: "2.0-quantum".to_string(),
            crypto_mode,
            inbox_id,
            payload: general_purpose::STANDARD.encode(&encrypted_payload),
            pq_ciphertext: None,
            pq_signature: None,
            expiry: None,
            nonce: general_purpose::STANDARD.encode(&rand::random::<[u8; 16]>()),
            legacy_compat: None,
        }
    }

    pub fn with_expiry(mut self, expiry: DateTime<Utc>) -> Self {
        self.expiry = Some(expiry.timestamp());
        self
    }

    pub fn with_pq_data(mut self, pq_ciphertext: Option<Vec<u8>>, pq_signature: Option<Vec<u8>>) -> Self {
        if let Some(ct) = pq_ciphertext {
            self.pq_ciphertext = Some(general_purpose::STANDARD.encode(&ct));
        }
        if let Some(sig) = pq_signature {
            self.pq_signature = Some(general_purpose::STANDARD.encode(&sig));
        }
        self
    }

    pub fn with_legacy_compat(mut self) -> Self {
        self.legacy_compat = Some(true);
        self
    }

    pub fn is_expired(&self) -> bool {
        if let Some(expiry) = self.expiry {
            Utc::now().timestamp() > expiry
        } else {
            false
        }
    }

    pub fn decode_payload(&self) -> Result<Vec<u8>> {
        general_purpose::STANDARD.decode(&self.payload)
            .map_err(|e| NanoError::Crypto(format!("Base64 decode error: {}", e)))
    }

    pub fn decode_pq_ciphertext(&self) -> Result<Option<Vec<u8>>> {
        match &self.pq_ciphertext {
            Some(ct) => {
                let decoded = general_purpose::STANDARD.decode(ct)
                    .map_err(|e| NanoError::Crypto(format!("PQ ciphertext decode error: {}", e)))?;
                Ok(Some(decoded))
            }
            None => Ok(None),
        }
    }

    pub fn decode_pq_signature(&self) -> Result<Option<Vec<u8>>> {
        match &self.pq_signature {
            Some(sig) => {
                let decoded = general_purpose::STANDARD.decode(sig)
                    .map_err(|e| NanoError::Crypto(format!("PQ signature decode error: {}", e)))?;
                Ok(Some(decoded))
            }
            None => Ok(None),
        }
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(self).map_err(Into::into)
    }

    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(Into::into)
    }

    /// Convert to legacy MessageEnvelope format for backward compatibility
    pub fn to_legacy(&self) -> MessageEnvelope {
        MessageEnvelope {
            version: "1.1".to_string(),
            inbox_id: self.inbox_id.clone(),
            payload: self.payload.clone(),
            expiry: self.expiry,
            nonce: self.nonce.clone(),
        }
    }

    /// Create from legacy MessageEnvelope
    pub fn from_legacy(legacy: MessageEnvelope) -> Self {
        Self {
            version: "2.0-quantum".to_string(),
            crypto_mode: CryptoMode::Classical, // Legacy is always classical
            inbox_id: legacy.inbox_id,
            payload: legacy.payload,
            pq_ciphertext: None,
            pq_signature: None,
            expiry: legacy.expiry,
            nonce: legacy.nonce,
            legacy_compat: Some(true),
        }
    }
}

/// The decrypted inner payload of a message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePayload {
    pub from_pubkey: String, // "pubkey:abc123..." or "pq-pubkey:..." or "hybrid-pubkey:..."
    pub timestamp: i64,      // Unix timestamp when message was created
    pub body: String,        // The actual message content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room: Option<String>, // Optional room/channel name
    pub counter: u64,         // Message counter for inbox derivation
    pub sig: String,          // Base64 encoded signature of the above fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crypto_mode: Option<CryptoMode>, // Crypto mode used (for Session 3+)
}

impl MessagePayload {
    pub fn new(
        from_pubkey: String,
        body: String,
        counter: u64,
        room: Option<String>,
    ) -> Self {
        Self {
            from_pubkey,
            timestamp: Utc::now().timestamp(),
            body,
            room,
            counter,
            sig: String::new(), // Will be filled in after signing
            crypto_mode: None,  // Will be set based on signing method
        }
    }

    /// Create a new payload with explicit crypto mode (Session 3+)
    pub fn new_with_mode(
        from_pubkey: String,
        body: String,
        counter: u64,
        room: Option<String>,
        crypto_mode: CryptoMode,
    ) -> Self {
        Self {
            from_pubkey,
            timestamp: Utc::now().timestamp(),
            body,
            room,
            counter,
            sig: String::new(),
            crypto_mode: Some(crypto_mode),
        }
    }

    /// Get the data that should be signed (everything except the signature)
    pub fn signable_data(&self) -> Result<Vec<u8>> {
        #[derive(Serialize)]
        struct SignablePayload {
            from_pubkey: String,
            timestamp: i64,
            body: String,
            room: Option<String>,
            counter: u64,
            crypto_mode: Option<CryptoMode>,
        }

        let signable = SignablePayload {
            from_pubkey: self.from_pubkey.clone(),
            timestamp: self.timestamp,
            body: self.body.clone(),
            room: self.room.clone(),
            counter: self.counter,
            crypto_mode: self.crypto_mode,
        };

        serde_json::to_vec(&signable).map_err(Into::into)
    }

    /// Sign this payload with the given signing key (legacy method for backward compatibility)
    pub fn sign(&mut self, signing_key: &crate::crypto::Ed25519PrivateKey) -> Result<()> {
        let data = self.signable_data()?;
        let signature = crate::crypto::sign_data(signing_key, &data);
        self.sig = general_purpose::STANDARD.encode(&signature.to_bytes());
        // For legacy signatures, don't set crypto_mode
        Ok(())
    }

    /// Sign this payload with the unified crypto interface (Session 3+)
    pub fn sign_with_mode(&mut self, keypair: &crate::crypto::UnifiedKeyPair) -> Result<()> {
        use crate::crypto::{
            ClassicalDigitalSignature, HybridDigitalSignature, PostQuantumDigitalSignature,
            traits::DigitalSignature, UnifiedKeyPair
        };

        let data = self.signable_data()?;
        
        match keypair {
            UnifiedKeyPair::Classical(kp) => {
                let signature = ClassicalDigitalSignature::sign(&kp.signing_key, &data);
                self.sig = general_purpose::STANDARD.encode(&signature.to_bytes());
                self.crypto_mode = Some(CryptoMode::Classical);
            }
            UnifiedKeyPair::Hybrid(kp) => {
                let hybrid_private = crate::crypto::hybrid::HybridSigningKey {
                    classical: kp.classical.signing_key.clone(),
                    post_quantum: kp.post_quantum.private_key.clone(),
                };
                let signature = HybridDigitalSignature::sign(&hybrid_private, &data);
                let sig_bytes = HybridDigitalSignature::signature_to_bytes(&signature);
                self.sig = general_purpose::STANDARD.encode(&sig_bytes);
                self.crypto_mode = Some(CryptoMode::Hybrid);
            }
            UnifiedKeyPair::PostQuantum(kp) => {
                let signature = PostQuantumDigitalSignature::sign(&kp.private_key, &data);
                let sig_bytes = PostQuantumDigitalSignature::signature_to_bytes(&signature);
                self.sig = general_purpose::STANDARD.encode(&sig_bytes);
                self.crypto_mode = Some(CryptoMode::Quantum);
            }
        }
        
        Ok(())
    }

    /// Verify the signature on this payload (legacy method for backward compatibility)
    pub fn verify_signature(&self) -> Result<()> {
        let data = self.signable_data()?;
        let sig_bytes = general_purpose::STANDARD.decode(&self.sig)
            .map_err(|e| NanoError::Crypto(format!("Base64 decode error: {}", e)))?;
        
        if sig_bytes.len() != 64 {
            return Err(NanoError::Crypto("Invalid signature length".to_string()));
        }

        let signature = Signature::from_bytes(&sig_bytes.try_into().unwrap());
        let verifying_key = UserPublicKeys::from_public_key_string(&self.from_pubkey)?;
        
        crate::crypto::verify_signature(&verifying_key, &data, &signature)
    }

    /// Verify the signature using the unified crypto interface (Session 3+)
    pub fn verify_signature_with_mode(&self) -> Result<()> {
        use crate::crypto::{
            ClassicalDigitalSignature, HybridDigitalSignature, PostQuantumDigitalSignature,
            traits::DigitalSignature, 
            ClassicalUserPublicKeys, PostQuantumUserPublicKeys
        };

        let data = self.signable_data()?;
        let sig_bytes = general_purpose::STANDARD.decode(&self.sig)
            .map_err(|e| NanoError::Crypto(format!("Base64 decode error: {}", e)))?;

        // Determine crypto mode from signature format and/or explicit crypto_mode field
        let crypto_mode = if let Some(mode) = self.crypto_mode {
            mode
        } else {
            // Infer from public key format for backward compatibility
            if self.from_pubkey.starts_with("pq-pubkey:") {
                CryptoMode::Quantum
            } else if self.from_pubkey.starts_with("hybrid-pubkey:") {
                CryptoMode::Hybrid
            } else {
                CryptoMode::Classical
            }
        };

        match crypto_mode {
            CryptoMode::Classical => {
                if sig_bytes.len() != 64 {
                    return Err(NanoError::Crypto("Invalid classical signature length".to_string()));
                }
                let signature = Signature::from_bytes(&sig_bytes.try_into().unwrap());
                let verifying_key = ClassicalUserPublicKeys::from_public_key_string(&self.from_pubkey)?;
                ClassicalDigitalSignature::verify(&verifying_key, &data, &signature)
            }
            CryptoMode::Hybrid => {
                let signature = HybridDigitalSignature::signature_from_bytes(&sig_bytes)?;
                let public_keys = self.parse_hybrid_public_key()?;
                let verifying_key = crate::crypto::hybrid::HybridVerifyingKey {
                    classical: public_keys.classical.verifying_key.clone(),
                    post_quantum: public_keys.post_quantum.public_key.clone(),
                };
                HybridDigitalSignature::verify(&verifying_key, &data, &signature)
            }
            CryptoMode::Quantum => {
                let signature = PostQuantumDigitalSignature::signature_from_bytes(&sig_bytes)?;
                let public_key = PostQuantumUserPublicKeys::from_public_key_string(&self.from_pubkey)?;
                PostQuantumDigitalSignature::verify(&public_key, &data, &signature)
            }
        }
    }

    /// Parse hybrid public key from the public key string
    fn parse_hybrid_public_key(&self) -> Result<HybridUserPublicKeys> {
        // For hybrid keys, we need to parse the combined format
        // This is a simplified approach - in practice, we'd need proper key parsing
        if !self.from_pubkey.starts_with("hybrid-pubkey:") {
            return Err(NanoError::Crypto("Not a hybrid public key".to_string()));
        }
        
        // For now, create a dummy hybrid key for testing
        // In practice, we'd parse the actual hybrid key data
        let classical_keypair = crate::crypto::ClassicalUserKeyPair::generate();
        let pq_keypair = crate::crypto::PostQuantumUserKeyPair::generate();
        
        Ok(HybridUserPublicKeys {
            classical: classical_keypair.public_keys(),
            post_quantum: pq_keypair.public_keys(),
        })
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(self).map_err(Into::into)
    }

    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(Into::into)
    }
}

/// Username claim message that gets published to relays
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsernameClaim {
    pub claim_type: String, // "username_claim"
    pub username: String,
    pub public_keys: UserPublicKeys,
    pub timestamp: i64,
    pub sig: String, // Base64 encoded signature
}

impl UsernameClaim {
    pub fn new(username: String, public_keys: UserPublicKeys) -> Self {
        Self {
            claim_type: "username_claim".to_string(),
            username,
            public_keys,
            timestamp: Utc::now().timestamp(),
            sig: String::new(),
        }
    }

    /// Get the data that should be signed
    pub fn signable_data(&self) -> Result<Vec<u8>> {
        #[derive(Serialize)]
        struct SignableClaim {
            claim_type: String,
            username: String,
            public_keys: UserPublicKeys,
            timestamp: i64,
        }

        let signable = SignableClaim {
            claim_type: self.claim_type.clone(),
            username: self.username.clone(),
            public_keys: self.public_keys.clone(),
            timestamp: self.timestamp,
        };

        serde_json::to_vec(&signable).map_err(Into::into)
    }

    /// Sign this claim
    pub fn sign(&mut self, signing_key: &crate::crypto::Ed25519PrivateKey) -> Result<()> {
        let data = self.signable_data()?;
        let signature = crate::crypto::sign_data(signing_key, &data);
        self.sig = general_purpose::STANDARD.encode(&signature.to_bytes());
        Ok(())
    }

    /// Verify the signature on this claim
    pub fn verify_signature(&self) -> Result<()> {
        let data = self.signable_data()?;
        let sig_bytes = general_purpose::STANDARD.decode(&self.sig)
            .map_err(|e| NanoError::Crypto(format!("Base64 decode error: {}", e)))?;
        
        if sig_bytes.len() != 64 {
            return Err(NanoError::Crypto("Invalid signature length".to_string()));
        }

        let signature = Signature::from_bytes(&sig_bytes.try_into().unwrap());
        
        crate::crypto::verify_signature(&self.public_keys.verifying_key, &data, &signature)
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(self).map_err(Into::into)
    }

    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(Into::into)
    }
}

/// TCP protocol messages between client and relay
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ProtocolMessage {
    /// Client sends a message envelope to be delivered (legacy format)
    #[serde(rename = "send_message")]
    SendMessage { envelope: MessageEnvelope },
    
    /// Client sends a quantum-safe message envelope to be delivered (Session 3+)
    #[serde(rename = "send_quantum_message")]
    SendQuantumMessage { envelope: QuantumSafeEnvelope },
    
    /// Client requests messages from an inbox
    #[serde(rename = "fetch_inbox")]
    FetchInbox { inbox_id: String },
    
    /// Relay responds with messages from an inbox (legacy format)
    #[serde(rename = "inbox_messages")]
    InboxMessages { messages: Vec<MessageEnvelope> },
    
    /// Relay responds with quantum-safe messages from an inbox (Session 3+)
    #[serde(rename = "quantum_inbox_messages")]
    QuantumInboxMessages { messages: Vec<QuantumSafeEnvelope> },
    
    /// Client publishes a username claim
    #[serde(rename = "publish_claim")]
    PublishClaim { claim: UsernameClaim },
    
    /// Client looks up a username
    #[serde(rename = "lookup_username")]
    LookupUsername { username: String },
    
    /// Relay responds with username lookup result (legacy format)
    #[serde(rename = "username_result")]
    UsernameResult { 
        username: String,
        public_keys: Option<UserPublicKeys>,
    },
    
    /// Relay responds with quantum-safe username lookup result (Session 3+)
    #[serde(rename = "quantum_username_result")]
    QuantumUsernameResult {
        username: String,
        public_keys: Option<UnifiedPublicKeys>,
    },
    
    /// Generic success response
    #[serde(rename = "success")]
    Success { message: String },
    
    /// Generic error response
    #[serde(rename = "error")]
    Error { message: String },
}

impl ProtocolMessage {
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(self).map_err(Into::into)
    }

    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::UserKeyPair;

    #[test]
    fn test_message_envelope() {
        let envelope = MessageEnvelope::new(
            "test_inbox".to_string(),
            b"encrypted_data".to_vec(),
        );
        
        assert_eq!(envelope.version, "1.1");
        assert_eq!(envelope.inbox_id, "test_inbox");
        assert_eq!(envelope.decode_payload().unwrap(), b"encrypted_data");
        
        // Test JSON round trip
        let json = envelope.to_json().unwrap();
        let decoded = MessageEnvelope::from_json(&json).unwrap();
        assert_eq!(decoded.inbox_id, envelope.inbox_id);
    }

    #[test]
    fn test_message_payload_signing() {
        let keypair = UserKeyPair::generate();
        let pubkey_str = keypair.public_key_string();
        
        let mut payload = MessagePayload::new(
            pubkey_str,
            "Hello, world!".to_string(),
            1,
            None,
        );
        
        // Sign the payload
        payload.sign(&keypair.signing_key).unwrap();
        assert!(!payload.sig.is_empty());
        
        // Verify the signature
        payload.verify_signature().unwrap();
        
        // Test that tampering breaks verification
        payload.body = "Tampered message".to_string();
        assert!(payload.verify_signature().is_err());
    }

    #[test]
    fn test_username_claim() {
        let keypair = UserKeyPair::generate();
        let public_keys = keypair.public_keys();
        
        let mut claim = UsernameClaim::new("alice2024".to_string(), public_keys);
        claim.sign(&keypair.signing_key).unwrap();
        
        // Verify the claim
        claim.verify_signature().unwrap();
        
        // Test JSON round trip
        let json = claim.to_json().unwrap();
        let decoded = UsernameClaim::from_json(&json).unwrap();
        assert_eq!(decoded.username, claim.username);
    }
}
