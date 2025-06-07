use nano_messenger::{
    protocol::{MessageEnvelope, ProtocolMessage, UsernameClaim},
    username::UsernameRegistry,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use clap::Parser;
use anyhow::Result as AnyhowResult;

#[derive(Parser)]
#[command(name = "nano-relay")]
#[command(about = "Zero-knowledge messaging relay server")]
struct Cli {
    /// Port to listen on
    #[arg(short, long, default_value = "7733")]
    port: u16,
    
    /// Address to bind to
    #[arg(short, long, default_value = "127.0.0.1")]
    address: String,
    
    /// Maximum message cache size per inbox
    #[arg(long, default_value = "100")]
    max_cache_size: usize,
    
    /// Message TTL in seconds
    #[arg(long, default_value = "86400")] // 24 hours
    message_ttl: u64,
}

/// In-memory message storage for each inbox
struct InboxStorage {
    messages: Vec<MessageEnvelope>,
    last_cleanup: std::time::Instant,
}

impl Default for InboxStorage {
    fn default() -> Self {
        Self {
            messages: Vec::new(),
            last_cleanup: std::time::Instant::now(),
        }
    }
}

impl InboxStorage {
    fn add_message(&mut self, envelope: MessageEnvelope, max_size: usize) {
        // Remove expired messages
        self.cleanup_expired();
        
        // Add new message
        self.messages.push(envelope);
        
        // Limit cache size (remove oldest if needed)
        if self.messages.len() > max_size {
            self.messages.remove(0);
        }
    }
    
    fn get_messages(&mut self) -> Vec<MessageEnvelope> {
        self.cleanup_expired();
        self.messages.clone()
    }
    
    fn cleanup_expired(&mut self) {
        let now = std::time::Instant::now();
        
        // Only cleanup once per minute to avoid overhead
        if now.duration_since(self.last_cleanup).as_secs() < 60 {
            return;
        }
        
        self.messages.retain(|msg| !msg.is_expired());
        self.last_cleanup = now;
    }
}

/// Relay server state
struct RelayServer {
    inboxes: Arc<RwLock<HashMap<String, InboxStorage>>>,
    usernames: Arc<RwLock<UsernameRegistry>>,
    config: Cli,
}

impl RelayServer {
    fn new(config: Cli) -> Self {
        Self {
            inboxes: Arc::new(RwLock::new(HashMap::new())),
            usernames: Arc::new(RwLock::new(UsernameRegistry::new())),
            config,
        }
    }
    
    async fn run(&self) -> AnyhowResult<()> {
        let addr = format!("{}:{}", self.config.address, self.config.port);
        let listener = TcpListener::bind(&addr).await?;
        
        println!("🚀 Nano-relay server listening on {}", addr);
        println!("📬 Ready to relay encrypted messages");
        
        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    println!("📡 New connection from {}", addr);
                    let server = self.clone();
                    tokio::spawn(async move {
                        if let Err(e) = server.handle_connection(stream).await {
                            eprintln!("❌ Error handling connection from {}: {}", addr, e);
                        }
                    });
                }
                Err(e) => {
                    eprintln!("❌ Failed to accept connection: {}", e);
                }
            }
        }
    }
    
    async fn handle_connection(&self, mut stream: TcpStream) -> AnyhowResult<()> {
        let mut buffer = vec![0; 4096];
        
        loop {
            match stream.read(&mut buffer).await {
                Ok(0) => {
                    // Connection closed
                    break;
                }
                Ok(n) => {
                    let data = String::from_utf8_lossy(&buffer[..n]);
                    
                    // Try to parse as JSON protocol message
                    match serde_json::from_str::<ProtocolMessage>(&data) {
                        Ok(message) => {
                            let response = self.handle_protocol_message(message).await;
                            let response_json = serde_json::to_string(&response)?;
                            
                            stream.write_all(response_json.as_bytes()).await?;
                            stream.write_all(b"\n").await?;
                        }
                        Err(e) => {
                            let error_response = ProtocolMessage::Error {
                                message: format!("Invalid JSON: {}", e),
                            };
                            let response_json = serde_json::to_string(&error_response)?;
                            stream.write_all(response_json.as_bytes()).await?;
                            stream.write_all(b"\n").await?;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error reading from connection: {}", e);
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    async fn handle_protocol_message(&self, message: ProtocolMessage) -> ProtocolMessage {
        match message {
            ProtocolMessage::SendMessage { envelope } => {
                self.handle_send_message(envelope).await
            }
            ProtocolMessage::FetchInbox { inbox_id } => {
                self.handle_fetch_inbox(inbox_id).await
            }
            ProtocolMessage::PublishClaim { claim } => {
                self.handle_publish_claim(claim).await
            }
            ProtocolMessage::LookupUsername { username } => {
                self.handle_lookup_username(username).await
            }
            _ => ProtocolMessage::Error {
                message: "Unsupported message type".to_string(),
            },
        }
    }
    
    async fn handle_send_message(&self, envelope: MessageEnvelope) -> ProtocolMessage {
        // Basic validation
        if envelope.inbox_id.is_empty() {
            return ProtocolMessage::Error {
                message: "Inbox ID cannot be empty".to_string(),
            };
        }
        
        if envelope.is_expired() {
            return ProtocolMessage::Error {
                message: "Message has expired".to_string(),
            };
        }
        
        // Store message in the target inbox
        let mut inboxes = self.inboxes.write().await;
        let inbox = inboxes.entry(envelope.inbox_id.clone()).or_default();
        inbox.add_message(envelope.clone(), self.config.max_cache_size);
        
        println!("📨 Message stored in inbox: {}", &envelope.inbox_id[..8]);
        
        ProtocolMessage::Success {
            message: "Message delivered".to_string(),
        }
    }
    
    async fn handle_fetch_inbox(&self, inbox_id: String) -> ProtocolMessage {
        let mut inboxes = self.inboxes.write().await;
        
        if let Some(inbox) = inboxes.get_mut(&inbox_id) {
            let messages = inbox.get_messages();
            println!("📬 Fetched {} messages from inbox: {}", messages.len(), &inbox_id[..8]);
            
            ProtocolMessage::InboxMessages { messages }
        } else {
            ProtocolMessage::InboxMessages {
                messages: vec![],
            }
        }
    }
    
    async fn handle_publish_claim(&self, claim: UsernameClaim) -> ProtocolMessage {
        // Verify claim signature
        if let Err(e) = claim.verify_signature() {
            return ProtocolMessage::Error {
                message: format!("Invalid claim signature: {}", e),
            };
        }
        
        let mut registry = self.usernames.write().await;
        
        match registry.register_claim(claim.clone()) {
            Ok(()) => {
                println!("🏷️  Username '{}' claimed", claim.username);
                ProtocolMessage::Success {
                    message: format!("Username '{}' claimed successfully", claim.username),
                }
            }
            Err(e) => ProtocolMessage::Error {
                message: format!("Failed to claim username: {}", e),
            },
        }
    }
    
    async fn handle_lookup_username(&self, username: String) -> ProtocolMessage {
        let registry = self.usernames.read().await;
        
        if let Some(public_keys) = registry.lookup_username(&username) {
            println!("🔍 Username lookup: {} -> found", username);
            ProtocolMessage::UsernameResult {
                username,
                public_keys: Some(public_keys.clone()),
            }
        } else {
            println!("🔍 Username lookup: {} -> not found", username);
            ProtocolMessage::UsernameResult {
                username,
                public_keys: None,
            }
        }
    }
}

impl Clone for RelayServer {
    fn clone(&self) -> Self {
        Self {
            inboxes: Arc::clone(&self.inboxes),
            usernames: Arc::clone(&self.usernames),
            config: Cli {
                port: self.config.port,
                address: self.config.address.clone(),
                max_cache_size: self.config.max_cache_size,
                message_ttl: self.config.message_ttl,
            },
        }
    }
}

#[tokio::main]
async fn main() -> AnyhowResult<()> {
    let config = Cli::parse();
    let server = RelayServer::new(config);
    
    // Handle shutdown gracefully
    tokio::select! {
        result = server.run() => {
            if let Err(e) = result {
                eprintln!("❌ Server error: {}", e);
                std::process::exit(1);
            }
        }
        _ = tokio::signal::ctrl_c() => {
            println!("\n🛑 Received interrupt signal, shutting down...");
        }
    }
    
    Ok(())
}
