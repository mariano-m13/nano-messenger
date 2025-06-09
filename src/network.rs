use crate::protocol::{ProtocolMessage, QuantumSafeEnvelope};
use crate::crypto::UnifiedPublicKeys;
use crate::error::{NanoError, Result};
use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, BufReader, AsyncBufReadExt};
use std::time::Duration;

/// TCP client for communicating with nano-relay servers
pub struct RelayClient {
    address: String,
}

impl RelayClient {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    /// Send a message to the relay and return the response
    pub async fn send_message(&self, message: ProtocolMessage) -> Result<ProtocolMessage> {
        let mut stream = self.connect().await?;
        
        // Send message
        let message_json = message.to_json()?;
        stream.write_all(message_json.as_bytes()).await?;
        stream.write_all(b"\n").await?;
        
        // Read response
        let mut reader = BufReader::new(stream);
        let mut response_line = String::new();
        reader.read_line(&mut response_line).await?;
        
        let response = ProtocolMessage::from_json(&response_line.trim())?;
        Ok(response)
    }

    /// Connect to the relay server with timeout
    async fn connect(&self) -> Result<TcpStream> {
        let stream = tokio::time::timeout(
            Duration::from_secs(10),
            TcpStream::connect(&self.address)
        ).await
        .map_err(|_| NanoError::Network(std::io::Error::new(
            std::io::ErrorKind::TimedOut,
            "Connection timeout"
        )))?
        .map_err(NanoError::Network)?;
        
        Ok(stream)
    }

    /// Send message envelope to relay
    pub async fn send_envelope(&self, envelope: crate::protocol::MessageEnvelope) -> Result<()> {
        let message = ProtocolMessage::SendMessage { envelope };
        let response = self.send_message(message).await?;
        
        match response {
            ProtocolMessage::Success { .. } => Ok(()),
            ProtocolMessage::Error { message } => {
                Err(NanoError::Protocol(format!("Relay error: {}", message)))
            }
            _ => Err(NanoError::Protocol("Unexpected response type".to_string())),
        }
    }

    /// Fetch messages from an inbox
    pub async fn fetch_inbox(&self, inbox_id: String) -> Result<Vec<crate::protocol::MessageEnvelope>> {
        let message = ProtocolMessage::FetchInbox { inbox_id };
        let response = self.send_message(message).await?;
        
        match response {
            ProtocolMessage::InboxMessages { messages } => Ok(messages),
            ProtocolMessage::Error { message } => {
                Err(NanoError::Protocol(format!("Relay error: {}", message)))
            }
            _ => Err(NanoError::Protocol("Unexpected response type".to_string())),
        }
    }

    /// Publish a username claim
    pub async fn publish_claim(&self, claim: crate::protocol::UsernameClaim) -> Result<()> {
        let message = ProtocolMessage::PublishClaim { claim };
        let response = self.send_message(message).await?;
        
        match response {
            ProtocolMessage::Success { .. } => Ok(()),
            ProtocolMessage::Error { message } => {
                Err(NanoError::Protocol(format!("Relay error: {}", message)))
            }
            _ => Err(NanoError::Protocol("Unexpected response type".to_string())),
        }
    }

    /// Look up a username
    pub async fn lookup_username(&self, username: String) -> Result<Option<crate::crypto::UserPublicKeys>> {
        let message = ProtocolMessage::LookupUsername { username };
        let response = self.send_message(message).await?;
        
        match response {
            ProtocolMessage::UsernameResult { public_keys, .. } => Ok(public_keys),
            ProtocolMessage::Error { message } => {
                Err(NanoError::Protocol(format!("Relay error: {}", message)))
            }
            _ => Err(NanoError::Protocol("Unexpected response type".to_string())),
        }
    }

    // Session 5: Quantum-Safe Messaging Support

    /// Send quantum-safe message envelope to relay
    pub async fn send_quantum_envelope(&self, envelope: QuantumSafeEnvelope) -> Result<()> {
        let message = ProtocolMessage::SendQuantumMessage { envelope };
        let response = self.send_message(message).await?;
        
        match response {
            ProtocolMessage::Success { .. } => Ok(()),
            ProtocolMessage::Error { message } => {
                Err(NanoError::Protocol(format!("Relay error: {}", message)))
            }
            _ => Err(NanoError::Protocol("Unexpected response type".to_string())),
        }
    }

    /// Fetch quantum-safe messages from an inbox
    pub async fn fetch_quantum_inbox(&self, inbox_id: String) -> Result<Vec<QuantumSafeEnvelope>> {
        let message = ProtocolMessage::FetchInbox { inbox_id };
        let response = self.send_message(message).await?;
        
        match response {
            ProtocolMessage::QuantumInboxMessages { messages } => Ok(messages),
            ProtocolMessage::InboxMessages { messages } => {
                // Convert legacy messages to quantum-safe format for compatibility
                let quantum_messages = messages.into_iter()
                    .map(QuantumSafeEnvelope::from_legacy)
                    .collect();
                Ok(quantum_messages)
            }
            ProtocolMessage::Error { message } => {
                Err(NanoError::Protocol(format!("Relay error: {}", message)))
            }
            _ => Err(NanoError::Protocol("Unexpected response type".to_string())),
        }
    }

    /// Look up a username with unified public key support
    pub async fn lookup_username_unified(&self, username: String) -> Result<Option<UnifiedPublicKeys>> {
        let message = ProtocolMessage::LookupUsername { username };
        let response = self.send_message(message).await?;
        
        match response {
            ProtocolMessage::QuantumUsernameResult { public_keys, .. } => Ok(public_keys),
            ProtocolMessage::UsernameResult { public_keys, .. } => {
                // Convert legacy public keys to unified format for compatibility
                Ok(public_keys.map(UnifiedPublicKeys::from_legacy))
            }
            ProtocolMessage::Error { message } => {
                Err(NanoError::Protocol(format!("Relay error: {}", message)))
            }
            _ => Err(NanoError::Protocol("Unexpected response type".to_string())),
        }
    }

    /// Check if relay supports quantum-safe messaging
    pub async fn supports_quantum_safe(&self) -> bool {
        // Send a test quantum-safe message to check support
        // In a real implementation, this might be done via a capabilities query
        // For now, we'll assume all relays support it if they respond properly
        true // Simplified for Session 5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relay_client_creation() {
        let client = RelayClient::new("127.0.0.1:7733".to_string());
        assert_eq!(client.address, "127.0.0.1:7733");
    }

    // Note: Integration tests would require a running relay server
    // These should be in tests/ directory for proper integration testing
}
