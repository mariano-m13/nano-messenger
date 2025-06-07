use crate::protocol::ProtocolMessage;
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
