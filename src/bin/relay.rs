use nano_messenger::{
    protocol::{MessageEnvelope, QuantumSafeEnvelope, ProtocolMessage, UsernameClaim},
    username::UsernameRegistry,
    crypto::{CryptoMode},
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use clap::Parser;
use anyhow::Result as AnyhowResult;
use serde::{Serialize, Deserialize};

#[derive(Parser)]
#[command(name = "nano-relay")]
#[command(about = "Zero-knowledge messaging relay server with quantum-safe crypto policy enforcement")]
pub struct Cli {
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
    
    /// Require post-quantum cryptography for all messages
    #[arg(long)]
    require_post_quantum: bool,
    
    /// Minimum acceptable crypto mode for incoming messages
    #[arg(long, default_value = "classical")]
    minimum_crypto_mode: String,
    
    /// Enable adaptive crypto mode recommendations based on relay load
    #[arg(long)]
    adaptive_recommendations: bool,
    
    /// Reject classical-only messages (enforce quantum resistance)
    #[arg(long)]
    reject_classical: bool,
    
    /// Log all crypto policy decisions for compliance
    #[arg(long)]
    log_crypto_policy: bool,
}

/// Crypto policy configuration for the relay
#[derive(Debug, Clone)]
pub struct CryptoPolicyConfig {
    pub require_post_quantum: bool,
    pub minimum_crypto_mode: CryptoMode,
    pub adaptive_recommendations: bool,
    pub reject_classical: bool,
    pub log_policy_decisions: bool,
}

impl CryptoPolicyConfig {
    pub fn from_cli(cli: &Cli) -> AnyhowResult<Self> {
        let minimum_mode = cli.minimum_crypto_mode.parse::<CryptoMode>()
            .map_err(|e| anyhow::anyhow!("Invalid minimum crypto mode: {}", e))?;
        
        Ok(Self {
            require_post_quantum: cli.require_post_quantum,
            minimum_crypto_mode: minimum_mode,
            adaptive_recommendations: cli.adaptive_recommendations,
            reject_classical: cli.reject_classical,
            log_policy_decisions: cli.log_crypto_policy,
        })
    }
    
    /// Check if a crypto mode is acceptable according to policy
    pub fn accepts_crypto_mode(&self, mode: CryptoMode) -> bool {
        // Check post-quantum requirement
        if self.require_post_quantum && !mode.is_quantum_resistant() {
            return false;
        }
        
        // Check classical rejection policy
        if self.reject_classical && mode == CryptoMode::Classical {
            return false;
        }
        
        // Check minimum mode requirement
        mode.can_transition_to(self.minimum_crypto_mode) || mode == self.minimum_crypto_mode
    }
    
    /// Get policy violation reason for logging
    pub fn get_violation_reason(&self, mode: CryptoMode) -> Option<String> {
        if self.require_post_quantum && !mode.is_quantum_resistant() {
            return Some(format!("Post-quantum cryptography required, but message uses {}", mode));
        }
        
        if self.reject_classical && mode == CryptoMode::Classical {
            return Some("Classical cryptography explicitly rejected by relay policy".to_string());
        }
        
        if !mode.can_transition_to(self.minimum_crypto_mode) && mode != self.minimum_crypto_mode {
            return Some(format!("Message crypto mode {} below minimum required {}", mode, self.minimum_crypto_mode));
        }
        
        None
    }
    
    /// Get recommended crypto mode based on current relay conditions
    pub fn get_recommended_mode(&self) -> CryptoMode {
        if self.adaptive_recommendations {
            // In a real implementation, this would consider:
            // - Relay load (high load -> suggest classical for performance)
            // - Network conditions (low bandwidth -> suggest classical)
            // - Time of day (off-peak -> suggest quantum for security)
            // - Relay capacity (suggest optimal mode for current conditions)
            
            // Simple adaptive logic for demo
            if self.require_post_quantum {
                CryptoMode::Quantum // Fastest quantum-resistant mode
            } else {
                CryptoMode::Hybrid // Best overall security/performance balance
            }
        } else {
            self.minimum_crypto_mode
        }
    }
}

/// Policy enforcement statistics for monitoring
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PolicyStats {
    pub total_messages: u64,
    pub accepted_messages: u64,
    pub rejected_messages: u64,
    pub classical_messages: u64,
    pub hybrid_messages: u64,
    pub quantum_messages: u64,
    pub policy_violations: u64,
}

impl PolicyStats {
    pub fn record_message(&mut self, mode: CryptoMode, accepted: bool) {
        self.total_messages += 1;
        
        if accepted {
            self.accepted_messages += 1;
        } else {
            self.rejected_messages += 1;
            self.policy_violations += 1;
        }
        
        match mode {
            CryptoMode::Classical => self.classical_messages += 1,
            CryptoMode::Hybrid => self.hybrid_messages += 1,
            CryptoMode::Quantum => self.quantum_messages += 1,
            CryptoMode::QuantumSafe => self.quantum_messages += 1,
        }
    }
}

/// Enhanced message storage supporting both legacy and quantum-safe envelopes
#[derive(Debug, Clone)]
enum StoredMessage {
    Legacy(MessageEnvelope),
    QuantumSafe(QuantumSafeEnvelope),
}

impl StoredMessage {
    fn is_expired(&self) -> bool {
        match self {
            StoredMessage::Legacy(envelope) => envelope.is_expired(),
            StoredMessage::QuantumSafe(envelope) => envelope.is_expired(),
        }
    }
    
    fn _inbox_id(&self) -> &str {
        match self {
            StoredMessage::Legacy(envelope) => &envelope.inbox_id,
            StoredMessage::QuantumSafe(envelope) => &envelope.inbox_id,
        }
    }
    
    fn _crypto_mode(&self) -> CryptoMode {
        match self {
            StoredMessage::Legacy(_) => CryptoMode::Classical,
            StoredMessage::QuantumSafe(envelope) => envelope.crypto_mode,
        }
    }
}

/// Enhanced inbox storage supporting mixed message types
struct InboxStorage {
    messages: Vec<StoredMessage>,
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
    fn add_message(&mut self, message: StoredMessage, max_size: usize) {
        // Remove expired messages
        self.cleanup_expired();
        
        // Add new message
        self.messages.push(message);
        
        // Limit cache size (remove oldest if needed)
        if self.messages.len() > max_size {
            self.messages.remove(0);
        }
    }
    
    fn _get_legacy_messages(&mut self) -> Vec<MessageEnvelope> {
        self.cleanup_expired();
        self.messages.iter()
            .filter_map(|msg| match msg {
                StoredMessage::Legacy(envelope) => Some(envelope.clone()),
                StoredMessage::QuantumSafe(envelope) => {
                    // Convert quantum-safe to legacy if possible
                    if envelope.crypto_mode == CryptoMode::Classical {
                        Some(envelope.to_legacy())
                    } else {
                        None // Can't represent non-classical crypto in legacy format
                    }
                }
            })
            .collect()
    }
    
    fn get_quantum_safe_messages(&mut self) -> Vec<QuantumSafeEnvelope> {
        self.cleanup_expired();
        self.messages.iter()
            .map(|msg| match msg {
                StoredMessage::Legacy(envelope) => QuantumSafeEnvelope::from_legacy(envelope.clone()),
                StoredMessage::QuantumSafe(envelope) => envelope.clone(),
            })
            .collect()
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

/// Enhanced relay server with crypto policy enforcement
struct RelayServer {
    inboxes: Arc<RwLock<HashMap<String, InboxStorage>>>,
    usernames: Arc<RwLock<UsernameRegistry>>,
    config: Cli,
    crypto_policy: CryptoPolicyConfig,
    policy_stats: Arc<RwLock<PolicyStats>>,
}

impl RelayServer {
    fn new(config: Cli) -> AnyhowResult<Self> {
        let crypto_policy = CryptoPolicyConfig::from_cli(&config)?;
        
        Ok(Self {
            inboxes: Arc::new(RwLock::new(HashMap::new())),
            usernames: Arc::new(RwLock::new(UsernameRegistry::new())),
            config,
            crypto_policy,
            policy_stats: Arc::new(RwLock::new(PolicyStats::default())),
        })
    }
    
    async fn run(&self) -> AnyhowResult<()> {
        let addr = format!("{}:{}", self.config.address, self.config.port);
        let listener = TcpListener::bind(&addr).await?;
        
        println!("🚀 Nano-relay server listening on {}", addr);
        println!("📬 Ready to relay encrypted messages");
        println!("🛡️  Crypto Policy Configuration:");
        println!("   Require post-quantum: {}", self.crypto_policy.require_post_quantum);
        println!("   Minimum crypto mode: {}", self.crypto_policy.minimum_crypto_mode);
        println!("   Reject classical: {}", self.crypto_policy.reject_classical);
        println!("   Adaptive recommendations: {}", self.crypto_policy.adaptive_recommendations);
        println!("   Policy logging: {}", self.crypto_policy.log_policy_decisions);
        
        // Start stats monitoring task
        let stats_clone = Arc::clone(&self.policy_stats);
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(300)); // 5 minutes
            loop {
                interval.tick().await;
                let stats = stats_clone.read().await;
                println!("📊 Policy Stats: {} total, {} accepted, {} rejected, {} violations", 
                         stats.total_messages, stats.accepted_messages, 
                         stats.rejected_messages, stats.policy_violations);
            }
        });
        
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
        let mut buffer = vec![0; 8192]; // Increased buffer for quantum-safe messages
        
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
                self.handle_send_legacy_message(envelope).await
            }
            ProtocolMessage::SendQuantumMessage { envelope } => {
                self.handle_send_quantum_safe_message(envelope).await
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
    
    async fn handle_send_legacy_message(&self, envelope: MessageEnvelope) -> ProtocolMessage {
        // Legacy messages are always classical crypto
        let crypto_mode = CryptoMode::Classical;
        
        // Apply crypto policy enforcement
        if !self.crypto_policy.accepts_crypto_mode(crypto_mode) {
            let reason = self.crypto_policy.get_violation_reason(crypto_mode)
                .unwrap_or_else(|| "Policy violation".to_string());
                
            // Log policy violation
            if self.crypto_policy.log_policy_decisions {
                println!("🚫 Policy violation: Legacy message rejected - {}", reason);
            }
            
            // Update stats
            let mut stats = self.policy_stats.write().await;
            stats.record_message(crypto_mode, false);
            
            return ProtocolMessage::Error {
                message: format!("Message rejected by crypto policy: {}", reason),
            };
        }
        
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
        inbox.add_message(StoredMessage::Legacy(envelope.clone()), self.config.max_cache_size);
        
        // Log policy acceptance
        if self.crypto_policy.log_policy_decisions {
            println!("✅ Policy accepted: Legacy message (classical) stored in inbox: {}", &envelope.inbox_id[..8]);
        } else {
            println!("📨 Legacy message stored in inbox: {}", &envelope.inbox_id[..8]);
        }
        
        // Update stats
        let mut stats = self.policy_stats.write().await;
        stats.record_message(crypto_mode, true);
        
        ProtocolMessage::Success {
            message: "Message delivered".to_string(),
        }
    }
    
    async fn handle_send_quantum_safe_message(&self, envelope: QuantumSafeEnvelope) -> ProtocolMessage {
        let crypto_mode = envelope.crypto_mode;
        
        // Apply crypto policy enforcement
        if !self.crypto_policy.accepts_crypto_mode(crypto_mode) {
            let reason = self.crypto_policy.get_violation_reason(crypto_mode)
                .unwrap_or_else(|| "Policy violation".to_string());
                
            // Log policy violation
            if self.crypto_policy.log_policy_decisions {
                println!("🚫 Policy violation: {} message rejected - {}", crypto_mode, reason);
            }
            
            // Update stats
            let mut stats = self.policy_stats.write().await;
            stats.record_message(crypto_mode, false);
            
            return ProtocolMessage::Error {
                message: format!("Message rejected by crypto policy: {}", reason),
            };
        }
        
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
        inbox.add_message(StoredMessage::QuantumSafe(envelope.clone()), self.config.max_cache_size);
        
        // Log policy acceptance with crypto mode details
        if self.crypto_policy.log_policy_decisions {
            println!("✅ Policy accepted: {} message stored in inbox: {} ({})", 
                     crypto_mode, &envelope.inbox_id[..8], crypto_mode.security_level());
        } else {
            println!("📨 {} message stored in inbox: {}", crypto_mode, &envelope.inbox_id[..8]);
        }
        
        // Update stats
        let mut stats = self.policy_stats.write().await;
        stats.record_message(crypto_mode, true);
        
        // Provide adaptive recommendations if enabled
        if self.crypto_policy.adaptive_recommendations {
            let recommended_mode = self.crypto_policy.get_recommended_mode();
            if recommended_mode != crypto_mode {
                println!("💡 Adaptive recommendation: Consider using {} for optimal performance", recommended_mode);
            }
        }
        
        ProtocolMessage::Success {
            message: format!("Quantum-safe message delivered ({})", crypto_mode),
        }
    }
    
    async fn handle_fetch_inbox(&self, inbox_id: String) -> ProtocolMessage {
        let mut inboxes = self.inboxes.write().await;
        
        if let Some(inbox) = inboxes.get_mut(&inbox_id) {
            // For backward compatibility, check for legacy client request patterns
            // If the client is legacy, return legacy messages only
            // For now, return quantum-safe messages (can represent all types)
            let messages = inbox.get_quantum_safe_messages();
            println!("📬 Fetched {} quantum-safe messages from inbox: {}", messages.len(), &inbox_id[..8]);
            
            ProtocolMessage::QuantumInboxMessages { messages }
        } else {
            ProtocolMessage::QuantumInboxMessages {
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
            // For now, return legacy format for compatibility
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
    
    /// Get current policy statistics (for monitoring/admin interface)
    pub async fn get_policy_stats(&self) -> PolicyStats {
        self.policy_stats.read().await.clone()
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
                require_post_quantum: self.config.require_post_quantum,
                minimum_crypto_mode: self.config.minimum_crypto_mode.clone(),
                adaptive_recommendations: self.config.adaptive_recommendations,
                reject_classical: self.config.reject_classical,
                log_crypto_policy: self.config.log_crypto_policy,
            },
            crypto_policy: self.crypto_policy.clone(),
            policy_stats: Arc::clone(&self.policy_stats),
        }
    }
}

#[tokio::main]
async fn main() -> AnyhowResult<()> {
    let config = Cli::parse();
    
    // Validate configuration
    if config.require_post_quantum && config.minimum_crypto_mode == "classical" {
        eprintln!("⚠️  Warning: require_post_quantum is enabled but minimum_crypto_mode is classical");
        eprintln!("   Consider setting --minimum-crypto-mode hybrid or quantum");
    }
    
    let server = RelayServer::new(config)?;
    
    println!("🛡️  Session 5: Relay with Crypto Policy Enforcement");
    println!("=====================================================");
    
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
            
            // Print final stats
            let final_stats = server.get_policy_stats().await;
            println!("📊 Final Policy Statistics:");
            println!("   Total messages: {}", final_stats.total_messages);
            println!("   Accepted: {} ({:.1}%)", 
                     final_stats.accepted_messages, 
                     (final_stats.accepted_messages as f64 / final_stats.total_messages.max(1) as f64) * 100.0);
            println!("   Rejected: {} ({:.1}%)", 
                     final_stats.rejected_messages,
                     (final_stats.rejected_messages as f64 / final_stats.total_messages.max(1) as f64) * 100.0);
            println!("   By mode - Classical: {}, Hybrid: {}, Quantum: {}", 
                     final_stats.classical_messages, 
                     final_stats.hybrid_messages, 
                     final_stats.quantum_messages);
        }
    }
    
    Ok(())
}
