use crate::crypto::{UserPublicKeys};
use crate::error::{NanoError, Result};
use chrono::{DateTime, Utc};
use ed25519_dalek::Signature;
use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose};

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

/// The decrypted inner payload of a message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePayload {
    pub from_pubkey: String, // "pubkey:abc123..."
    pub timestamp: i64,      // Unix timestamp when message was created
    pub body: String,        // The actual message content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room: Option<String>, // Optional room/channel name
    pub counter: u64,         // Message counter for inbox derivation
    pub sig: String,          // Base64 encoded signature of the above fields
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
        }

        let signable = SignablePayload {
            from_pubkey: self.from_pubkey.clone(),
            timestamp: self.timestamp,
            body: self.body.clone(),
            room: self.room.clone(),
            counter: self.counter,
        };

        serde_json::to_vec(&signable).map_err(Into::into)
    }

    /// Sign this payload with the given signing key
    pub fn sign(&mut self, signing_key: &crate::crypto::Ed25519PrivateKey) -> Result<()> {
        let data = self.signable_data()?;
        let signature = crate::crypto::sign_data(signing_key, &data);
        self.sig = general_purpose::STANDARD.encode(&signature.to_bytes());
        Ok(())
    }

    /// Verify the signature on this payload
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
    /// Client sends a message envelope to be delivered
    #[serde(rename = "send_message")]
    SendMessage { envelope: MessageEnvelope },
    
    /// Client requests messages from an inbox
    #[serde(rename = "fetch_inbox")]
    FetchInbox { inbox_id: String },
    
    /// Relay responds with messages from an inbox
    #[serde(rename = "inbox_messages")]
    InboxMessages { messages: Vec<MessageEnvelope> },
    
    /// Client publishes a username claim
    #[serde(rename = "publish_claim")]
    PublishClaim { claim: UsernameClaim },
    
    /// Client looks up a username
    #[serde(rename = "lookup_username")]
    LookupUsername { username: String },
    
    /// Relay responds with username lookup result
    #[serde(rename = "username_result")]
    UsernameResult { 
        username: String,
        public_keys: Option<UserPublicKeys>,
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
