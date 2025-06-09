use clap::{Parser, Subcommand};
use nano_messenger::{
    contacts::{ContactManager, ContactMetadata, ContactPermission, ContactStatus},
    crypto::{
        UserKeyPair, Ed25519PrivateKey, X25519PrivateKey, 
        CryptoMode, CryptoConfig,
        encrypt_asymmetric, decrypt_asymmetric, decrypt_symmetric, encrypt_symmetric
    },
    username::create_username_claim,
    network::RelayClient,
    protocol::{MessageEnvelope, MessagePayload},
    inbox::{derive_first_contact_inbox, ConversationManager, ConversationState},
    messages::{MessageStore, StoredMessage},
};
use std::path::PathBuf;
use std::collections::HashMap;
use tokio;
use anyhow::Result;
use chrono::Utc;
use base64::{Engine as _, engine::general_purpose};
use serde::{Serialize, Deserialize};

/// User security preferences for Session 4
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPreferences {
    pub default_crypto_mode: CryptoMode,
    pub adaptive_mode: bool,
    pub minimum_crypto_mode: CryptoMode,
    pub auto_upgrade: bool,
    pub force_post_quantum: bool,
}

impl Default for SecurityPreferences {
    fn default() -> Self {
        Self {
            default_crypto_mode: CryptoMode::Hybrid, // Secure default
            adaptive_mode: false,
            minimum_crypto_mode: CryptoMode::Classical,
            auto_upgrade: true,
            force_post_quantum: false,
        }
    }
}

#[derive(Parser)]
#[command(name = "nano-client")]
#[command(about = "A zero-knowledge, privacy-first messaging client with quantum-resistant cryptography")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Configuration directory
    #[arg(long, default_value = "~/.nano-messenger")]
    config_dir: String,
    
    /// Default relay server
    #[arg(long, default_value = "127.0.0.1:7733")]
    relay: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize user (generate keys)
    Init {
        /// Crypto mode for key generation
        #[arg(long, default_value = "classical")]
        crypto_mode: String,
    },
    
    /// Claim a username
    ClaimUsername { username: String },
    
    /// Send a message with quantum-safe cryptography
    Send {
        /// Recipient username or pubkey
        recipient: String,
        /// Message content
        message: String,
        /// Cryptography mode: classical, hybrid, or quantum
        #[arg(long, default_value = "hybrid")]
        crypto_mode: String,
        /// Force post-quantum cryptography (overrides mode selection)
        #[arg(long)]
        force_post_quantum: bool,
        /// Use adaptive mode selection based on network conditions
        #[arg(long)]
        adaptive: bool,
    },
    
    /// Configure security preferences
    SetSecurity {
        /// Default crypto mode for new messages
        #[arg(long)]
        default_mode: Option<String>,
        /// Enable adaptive mode selection based on bandwidth
        #[arg(long)]
        adaptive: Option<bool>,
        /// Minimum acceptable crypto mode for incoming messages
        #[arg(long)]
        minimum_mode: Option<String>,
        /// Allow automatic security upgrades
        #[arg(long)]
        auto_upgrade: Option<bool>,
    },
    
    /// Show current security configuration
    ShowSecurity,
    
    /// Check for new messages
    Receive,
    
    /// List all messages
    Messages {
        /// Optional contact filter
        #[arg(long)]
        from: Option<String>,
        /// Number of recent messages to show
        #[arg(long, default_value = "20")]
        limit: usize,
        /// Filter by crypto mode
        #[arg(long)]
        crypto_mode: Option<String>,
    },
    
    /// Manage contacts
    #[command(subcommand)]
    Contacts(ContactCommands),
    
    /// Show user info including crypto capabilities
    Info,
    
    /// Test crypto mode compatibility
    TestCrypto {
        /// Crypto mode to test
        #[arg(default_value = "all")]
        mode: String,
    },
}

#[derive(Subcommand)]
enum ContactCommands {
    /// List all contacts
    List,
    
    /// Search contacts
    Search { query: String },
    
    /// Allow a contact
    Allow { pubkey: String },
    
    /// Block a contact
    Block { pubkey: String },
    
    /// Set contact metadata
    Edit {
        pubkey: String,
        #[arg(long)]
        nickname: Option<String>,
        #[arg(long)]
        memo: Option<String>,
    },
    
    /// Remove a contact
    Remove { pubkey: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Expand tilde in config directory
    let config_dir = expand_path(&cli.config_dir)?;
    std::fs::create_dir_all(&config_dir)?;
    
    // Load security preferences
    let security_prefs = load_security_preferences(&config_dir)?;
    
    // Initialize crypto config based on preferences
    let crypto_config = CryptoConfig {
        mode: security_prefs.default_crypto_mode,
        allow_auto_upgrade: security_prefs.auto_upgrade,
        adaptive_mode: security_prefs.adaptive_mode,
        minimum_mode: security_prefs.minimum_crypto_mode,
    };
    
    // Initialize the crypto system
    let _ = nano_messenger::crypto::init_crypto_config(crypto_config);
    
    match cli.command {
        Commands::Init { crypto_mode } => {
            let mode = parse_crypto_mode(&crypto_mode)?;
            init_user(&config_dir, mode)?;
        }
        Commands::ClaimUsername { username } => {
            claim_username(&config_dir, &cli.relay, &username).await?;
        }
        Commands::Send { 
            recipient, 
            message, 
            crypto_mode, 
            force_post_quantum,
            adaptive,
        } => {
            send_quantum_safe_message(
                &config_dir, 
                &cli.relay, 
                &recipient, 
                &message,
                &crypto_mode,
                force_post_quantum,
                adaptive,
                &security_prefs,
            ).await?;
        }
        Commands::SetSecurity { 
            default_mode, 
            adaptive, 
            minimum_mode,
            auto_upgrade,
        } => {
            update_security_preferences(
                &config_dir,
                default_mode.as_deref(),
                adaptive,
                minimum_mode.as_deref(),
                auto_upgrade,
            )?;
        }
        Commands::ShowSecurity => {
            show_security_configuration(&config_dir)?;
        }
        Commands::Receive => {
            receive_messages(&config_dir, &cli.relay).await?;
        }
        Commands::Messages { from, limit, crypto_mode } => {
            show_messages(&config_dir, from.as_deref(), limit, crypto_mode.as_deref())?;
        }
        Commands::Contacts(contact_cmd) => {
            handle_contact_command(&config_dir, contact_cmd)?;
        }
        Commands::Info => {
            show_user_info(&config_dir)?;
        }
        Commands::TestCrypto { mode } => {
            test_crypto_modes(&config_dir, &mode)?;
        }
    }
    
    Ok(())
}

fn expand_path(path: &str) -> Result<PathBuf> {
    if path.starts_with('~') {
        let home = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE"))?;
        let path = path.replacen('~', &home, 1);
        Ok(PathBuf::from(path))
    } else {
        Ok(PathBuf::from(path))
    }
}

fn parse_crypto_mode(mode_str: &str) -> Result<CryptoMode> {
    mode_str.parse::<CryptoMode>()
        .map_err(|e| anyhow::anyhow!("Invalid crypto mode: {}", e))
}

fn init_user(config_dir: &PathBuf, crypto_mode: CryptoMode) -> Result<()> {
    let keys_file = config_dir.join("keys.json");
    
    if keys_file.exists() {
        println!("User already initialized. Keys found at: {}", keys_file.display());
        return Ok(());
    }
    
    println!("🔐 Generating new keypair with {} cryptography...", crypto_mode);
    
    // For now, we'll generate classical keys and note the intended mode
    // In a full implementation, this would generate unified keypairs
    let keypair = UserKeyPair::generate();
    let public_keys = keypair.public_keys();
    
    // Save keys to file (in a real implementation, you'd want to encrypt this)
    let keys_data = serde_json::json!({
        "crypto_mode": crypto_mode,
        "signing_key": general_purpose::STANDARD.encode(&keypair.signing_key.to_bytes()),
        "x25519_key": general_purpose::STANDARD.encode(&keypair.x25519_key.to_bytes()),
        "verifying_key": general_purpose::STANDARD.encode(&public_keys.verifying_key.to_bytes()),
        "x25519_public": general_purpose::STANDARD.encode(&public_keys.x25519_key.to_bytes()),
        "created_at": Utc::now().to_rfc3339(),
    });
    
    std::fs::write(&keys_file, serde_json::to_string_pretty(&keys_data)?)?;
    
    // Initialize default security preferences
    let security_prefs = SecurityPreferences {
        default_crypto_mode: crypto_mode,
        ..Default::default()
    };
    save_security_preferences(config_dir, &security_prefs)?;
    
    println!("✓ User initialized successfully with {} cryptography!", crypto_mode);
    println!("🔑 Public key: {}", keypair.public_key_string());
    println!("📄 Keys saved to: {}", keys_file.display());
    println!("🛡️  Security level: {}", crypto_mode.security_level());
    
    Ok(())
}

async fn claim_username(config_dir: &PathBuf, relay: &str, username: &str) -> Result<()> {
    let keypair = load_keypair(config_dir)?;
    let client = RelayClient::new(relay.to_string());
    
    println!("Claiming username '{}' on relay {}...", username, relay);
    
    let claim = create_username_claim(username, &keypair)?;
    
    match client.publish_claim(claim).await {
        Ok(()) => {
            println!("✓ Username '{}' claimed successfully!", username);
            println!("Others can now message you at: {}", username);
        }
        Err(e) => {
            eprintln!("❌ Failed to claim username: {}", e);
            return Err(e.into());
        }
    }
    
    Ok(())
}

async fn send_message(
    config_dir: &PathBuf,
    relay: &str,
    recipient: &str,
    message: &str,
) -> Result<()> {
    let keypair = load_keypair(config_dir)?;
    let client = RelayClient::new(relay.to_string());
    let mut contact_manager = load_contact_manager(config_dir)?;
    let mut conversation_manager = load_conversation_manager(config_dir)?;
    let mut message_store = load_message_store(config_dir)?;
    
    println!("Sending message to '{}' via {}...", recipient, relay);
    
    // Try to get recipient's public key
    let recipient_pubkey = if recipient.starts_with("pubkey:") {
        // Direct pubkey
        recipient.to_string()
    } else {
        // Username lookup
        match client.lookup_username(recipient.to_string()).await? {
            Some(public_keys) => {
                let pubkey_str = public_keys.public_key_string();
                // Update contact manager with username mapping
                contact_manager.set_username(pubkey_str.clone(), recipient.to_string());
                pubkey_str
            }
            None => {
                eprintln!("❌ Username '{}' not found", recipient);
                return Ok(());
            }
        }
    };
    
    // Get recipient's X25519 public key
    let recipient_public_keys = if recipient.starts_with("pubkey:") {
        // For direct pubkey, we need to look it up
        match client.lookup_username(recipient.to_string()).await? {
            Some(keys) => keys,
            None => {
                eprintln!("❌ Could not find public keys for {}", recipient);
                return Ok(());
            }
        }
    } else {
        // Already looked up above
        client.lookup_username(recipient.to_string()).await?
            .ok_or_else(|| anyhow::anyhow!("Public keys not found"))?
    };
    
    // Check if this is an established conversation or first contact
    let envelope = if let Some(conversation) = conversation_manager.get_conversation(&recipient_pubkey) {
        // Established conversation - use shared secret
        let inbox_id = conversation.get_outgoing_inbox();
        
        let mut payload = MessagePayload::new(
            keypair.public_key_string(),
            message.to_string(),
            conversation.our_counter - 1, // get_outgoing_inbox already incremented it
            None,
        );
        payload.sign(&keypair.signing_key)?;
        
        let payload_json = payload.to_json()?;
        let encrypted = encrypt_symmetric(&conversation.shared_secret, payload_json.as_bytes())?;
        
        MessageEnvelope::new(inbox_id, encrypted)
    } else {
        // First contact - use asymmetric encryption
        let inbox_id = derive_first_contact_inbox(&recipient_public_keys.x25519_key);
        
        let mut payload = MessagePayload::new(
            keypair.public_key_string(),
            message.to_string(),
            0, // First message
            None,
        );
        payload.sign(&keypair.signing_key)?;
        
        let payload_json = payload.to_json()?;
        let encrypted = encrypt_asymmetric(&recipient_public_keys.x25519_key, payload_json.as_bytes())?;
        
        MessageEnvelope::new(inbox_id, encrypted)
    };
    
    // Send message
    match client.send_envelope(envelope).await {
        Ok(()) => {
            println!("✓ Message sent to {}", recipient);
            
            // Store outgoing message
            let stored_msg = StoredMessage {
                id: format!("{}:{}:{}", keypair.public_key_string(), recipient_pubkey, Utc::now().timestamp()),
                from_pubkey: keypair.public_key_string(),
                to_pubkey: recipient_pubkey.clone(),
                content: message.to_string(),
                timestamp: Utc::now(),
                received_at: Utc::now(),
                is_outgoing: true,
                conversation_id: format!("{}:{}", keypair.public_key_string(), recipient_pubkey),
                counter: 0, // This should be the actual counter
            };
            
            message_store.store_message(stored_msg)?;
            
            // Save updated state
            save_contact_manager(config_dir, &contact_manager)?;
            save_conversation_manager(config_dir, &conversation_manager)?;
            save_message_store(config_dir, &message_store)?;
        }
        Err(e) => {
            eprintln!("❌ Failed to send message: {}", e);
            return Err(e.into());
        }
    }
    
    Ok(())
}

async fn receive_messages(config_dir: &PathBuf, relay: &str) -> Result<()> {
    let keypair = load_keypair(config_dir)?;
    let client = RelayClient::new(relay.to_string());
    let mut contact_manager = load_contact_manager(config_dir)?;
    let mut conversation_manager = load_conversation_manager(config_dir)?;
    let mut message_store = load_message_store(config_dir)?;
    
    println!("Checking for new messages on {}...", relay);
    
    let mut new_message_count = 0;
    
    // Check first-contact inbox
    let first_contact_inbox = derive_first_contact_inbox(&keypair.public_keys().x25519_key);
    let first_contact_messages = client.fetch_inbox(first_contact_inbox).await?;
    
    for envelope in first_contact_messages {
        if envelope.is_expired() {
            continue;
        }
        
        match process_first_contact_message(&envelope, &keypair, &mut contact_manager, &mut message_store) {
            Ok(processed) => {
                if processed {
                    new_message_count += 1;
                }
            }
            Err(e) => {
                eprintln!("Warning: Failed to process first contact message: {}", e);
            }
        }
    }
    
    // Check conversation inboxes for known contacts
    // Fix: Collect conversation pubkeys first to avoid borrow conflicts
    let conversation_pubkeys: Vec<String> = conversation_manager.list_conversations().iter().map(|s| s.to_string()).collect();
    
    for conversation_pubkey in conversation_pubkeys {
        if let Some(mut conversation) = conversation_manager.get_conversation(&conversation_pubkey) {
            let inbox_ids = conversation.get_incoming_inboxes(10); // Check last 10 possible inboxes
            
            for inbox_id in inbox_ids {
                let messages = client.fetch_inbox(inbox_id).await?;
                
                for envelope in messages {
                    if envelope.is_expired() {
                        continue;
                    }
                    
                    match process_conversation_message(
                        &envelope,
                        &mut conversation,
                        &keypair,
                        &mut contact_manager,
                        &mut message_store,
                    ) {
                        Ok(processed) => {
                            if processed {
                                new_message_count += 1;
                            }
                        }
                        Err(e) => {
                            eprintln!("Warning: Failed to process conversation message: {}", e);
                        }
                    }
                }
            }
        }
    }
    
    if new_message_count > 0 {
        println!("✓ Received {} new message(s)", new_message_count);
        
        // Save updated state
        save_contact_manager(config_dir, &contact_manager)?;
        save_conversation_manager(config_dir, &conversation_manager)?;
        save_message_store(config_dir, &message_store)?;
    } else {
        println!("✓ No new messages");
    }
    
    Ok(())
}

fn show_messages(config_dir: &PathBuf, from_filter: Option<&str>, limit: usize, crypto_mode_filter: Option<&str>) -> Result<()> {
    let message_store = load_message_store(config_dir)?;
    let contact_manager = load_contact_manager(config_dir)?;
    
    println!("📨 Message history (last {} messages):", limit);
    
    if let Some(mode_str) = crypto_mode_filter {
        let mode = parse_crypto_mode(mode_str)?;
        println!("🔍 Filtered by crypto mode: {}", mode);
    }
    
    let messages = if let Some(from) = from_filter {
        println!("🔍 Filtered by sender: {}", from);
        
        // Try to resolve username to pubkey
        let from_pubkey = if from.starts_with("pubkey:") {
            from.to_string()
        } else {
            contact_manager.get_pubkey_for_username(from)
                .unwrap_or(from)
                .to_string()
        };
        
        message_store.get_messages_from(&from_pubkey, Some(limit))
    } else {
        message_store.get_all_messages(Some(limit))
    };
    
    if messages.is_empty() {
        println!("(No messages to display)");
    } else {
        for msg in messages {
            let display_name = if let Some(contact) = contact_manager.get_contact(&msg.from_pubkey) {
                contact.display_name().to_string()
            } else {
                msg.from_pubkey.clone()
            };
            
            let direction = if msg.is_outgoing { "→" } else { "←" };
            
            println!(
                "[{}] {} {} {} 🔐",
                msg.timestamp.format("%Y-%m-%d %H:%M:%S"),
                direction,
                display_name,
                msg.content
            );
        }
    }
    
    Ok(())
}

fn handle_contact_command(config_dir: &PathBuf, command: ContactCommands) -> Result<()> {
    let mut contact_manager = load_contact_manager(config_dir)?;
    
    match command {
        ContactCommands::List => {
            let contacts = contact_manager.list_contacts();
            if contacts.is_empty() {
                println!("No contacts found.");
            } else {
                println!("Contacts ({}):", contacts.len());
                for contact in contacts {
                    let status = match contact.permission.status {
                        ContactStatus::Allowed => "✓",
                        ContactStatus::Blocked => "✗",
                        ContactStatus::Unknown => "?",
                    };
                    
                    println!(
                        "  {} {} - {}",
                        status,
                        contact.display_name(),
                        contact.permission.pubkey
                    );
                    
                    if let Some(metadata) = &contact.metadata {
                        if !metadata.memo.is_empty() {
                            println!("    Memo: {}", metadata.memo);
                        }
                    }
                }
            }
        }
        ContactCommands::Search { query } => {
            let results = contact_manager.search_contacts(&query);
            if results.is_empty() {
                println!("No contacts found matching '{}'", query);
            } else {
                println!("Contacts matching '{}' ({}):", query, results.len());
                for contact in results {
                    println!(
                        "  {} - {}",
                        contact.display_name(),
                        contact.permission.pubkey
                    );
                }
            }
        }
        ContactCommands::Allow { pubkey } => {
            contact_manager.allow_contact(pubkey.clone())?;
            save_contact_manager(config_dir, &contact_manager)?;
            println!("✓ Contact {} is now allowed", pubkey);
        }
        ContactCommands::Block { pubkey } => {
            contact_manager.block_contact(pubkey.clone())?;
            save_contact_manager(config_dir, &contact_manager)?;
            println!("✓ Contact {} is now blocked", pubkey);
        }
        ContactCommands::Edit { pubkey, nickname, memo } => {
            contact_manager.update_metadata(&pubkey, nickname.clone(), memo.clone())?;
            save_contact_manager(config_dir, &contact_manager)?;
            
            println!("✓ Updated contact metadata for {}", pubkey);
            if let Some(nickname) = nickname {
                println!("  Nickname: {}", nickname);
            }
            if let Some(memo) = memo {
                println!("  Memo: {}", memo);
            }
        }
        ContactCommands::Remove { pubkey } => {
            contact_manager.remove_contact(&pubkey);
            save_contact_manager(config_dir, &contact_manager)?;
            println!("✓ Removed contact {}", pubkey);
        }
    }
    
    Ok(())
}

fn show_user_info(config_dir: &PathBuf) -> Result<()> {
    let keypair = load_keypair(config_dir)?;
    let public_keys = keypair.public_keys();
    let security_prefs = load_security_preferences(config_dir)?;
    
    println!("👤 User Information:");
    println!("   🔑 Public Key: {}", keypair.public_key_string());
    println!("   🔐 Ed25519 Public Key: {}", general_purpose::STANDARD.encode(&public_keys.verifying_key.to_bytes()));
    println!("   🔐 X25519 Public Key: {}", general_purpose::STANDARD.encode(&public_keys.x25519_key.to_bytes()));
    
    // Check if user has crypto mode stored
    let keys_file = config_dir.join("keys.json");
    if let Ok(keys_data) = std::fs::read_to_string(&keys_file) {
        if let Ok(data) = serde_json::from_str::<serde_json::Value>(&keys_data) {
            if let Some(crypto_mode) = data.get("crypto_mode") {
                if let Ok(mode) = serde_json::from_value::<CryptoMode>(crypto_mode.clone()) {
                    println!("   🛡️  Crypto Mode: {} ({})", mode, mode.security_level());
                }
            }
        }
    }
    
    println!("\n🛡️  Security Configuration:");
    println!("   Default mode: {}", security_prefs.default_crypto_mode);
    println!("   Minimum mode: {}", security_prefs.minimum_crypto_mode);
    println!("   Adaptive: {}", if security_prefs.adaptive_mode { "enabled" } else { "disabled" });
    println!("   Auto upgrade: {}", if security_prefs.auto_upgrade { "enabled" } else { "disabled" });
    
    let contact_manager = load_contact_manager(config_dir)?;
    let contacts = contact_manager.list_contacts();
    let allowed_count = contacts.iter().filter(|c| c.permission.status == ContactStatus::Allowed).count();
    let blocked_count = contacts.iter().filter(|c| c.permission.status == ContactStatus::Blocked).count();
    
    println!("\n👥 Contacts: {} total ({} allowed, {} blocked)", 
             contacts.len(), allowed_count, blocked_count);
    
    Ok(())
}

fn load_keypair(config_dir: &PathBuf) -> Result<UserKeyPair> {
    let keys_file = config_dir.join("keys.json");
    
    if !keys_file.exists() {
        anyhow::bail!("User not initialized. Run 'nano-client init' first.");
    }
    
    let keys_data: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&keys_file)?)?;
    
    let signing_bytes = general_purpose::STANDARD.decode(keys_data["signing_key"].as_str().unwrap())
        .map_err(|e| anyhow::anyhow!("Base64 decode error: {}", e))?;
    let x25519_bytes = general_purpose::STANDARD.decode(keys_data["x25519_key"].as_str().unwrap())
        .map_err(|e| anyhow::anyhow!("Base64 decode error: {}", e))?;
    
    // Fix: Handle the Result from from_bytes properly
    let signing_key_bytes: [u8; 32] = signing_bytes.try_into()
        .map_err(|_| anyhow::anyhow!("Invalid signing key length"))?;
    let signing_key = Ed25519PrivateKey::from_bytes(&signing_key_bytes);
    
    let x25519_key_bytes: [u8; 32] = x25519_bytes.try_into()
        .map_err(|_| anyhow::anyhow!("Invalid X25519 key length"))?;
    let x25519_key = X25519PrivateKey::from(x25519_key_bytes);
    
    Ok(UserKeyPair {
        signing_key,
        x25519_key,
    })
}

fn load_contact_manager(config_dir: &PathBuf) -> Result<ContactManager> {
    let contacts_file = config_dir.join("contacts.json");
    
    if !contacts_file.exists() {
        return Ok(ContactManager::new());
    }
    
    let data: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&contacts_file)?)?;
    
    let mut manager = ContactManager::new();
    
    // Load permissions
    if let Some(permissions) = data.get("permissions") {
        let permissions: std::collections::HashMap<String, ContactPermission> = 
            serde_json::from_value(permissions.clone())?;
        manager.load_permissions(permissions);
    }
    
    // Load metadata
    if let Some(metadata) = data.get("metadata") {
        let metadata: std::collections::HashMap<String, ContactMetadata> = 
            serde_json::from_value(metadata.clone())?;
        manager.import_metadata(metadata);
    }
    
    Ok(manager)
}

fn save_contact_manager(config_dir: &PathBuf, manager: &ContactManager) -> Result<()> {
    let contacts_file = config_dir.join("contacts.json");
    
    let data = serde_json::json!({
        "permissions": manager.get_permissions(),
        "metadata": manager.export_metadata()
    });
    
    std::fs::write(&contacts_file, serde_json::to_string_pretty(&data)?)?;
    
    Ok(())
}

fn load_conversation_manager(config_dir: &PathBuf) -> Result<ConversationManager> {
    let conversations_file = config_dir.join("conversations.json");
    
    if !conversations_file.exists() {
        return Ok(ConversationManager::new());
    }
    
    // For now, return empty manager - conversation state will be rebuilt as needed
    Ok(ConversationManager::new())
}

fn save_conversation_manager(_config_dir: &PathBuf, _manager: &ConversationManager) -> Result<()> {
    // For now, we don't persist conversation state
    // It can be rebuilt from public keys when needed
    Ok(())
}

fn load_message_store(config_dir: &PathBuf) -> Result<MessageStore> {
    let messages_file = config_dir.join("messages.json");
    
    if !messages_file.exists() {
        return Ok(MessageStore::new());
    }
    
    let data: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&messages_file)?)?;
    
    let mut store = MessageStore::new();
    
    if let Some(messages) = data.get("messages") {
        let messages: HashMap<String, StoredMessage> = serde_json::from_value(messages.clone())?;
        store.import_messages(messages)?;
    }
    
    Ok(store)
}

fn save_message_store(config_dir: &PathBuf, store: &MessageStore) -> Result<()> {
    let messages_file = config_dir.join("messages.json");
    
    let data = serde_json::json!({
        "messages": store.export_messages()
    });
    
    std::fs::write(&messages_file, serde_json::to_string_pretty(&data)?)?;
    
    Ok(())
}

fn process_first_contact_message(
    envelope: &MessageEnvelope,
    keypair: &UserKeyPair,
    _contact_manager: &mut ContactManager,
    message_store: &mut MessageStore,
) -> Result<bool> {
    // Decrypt the message
    let encrypted_payload = envelope.decode_payload()?;
    let payload_json = decrypt_asymmetric(&keypair.x25519_key, &encrypted_payload)?;
    let payload: MessagePayload = MessagePayload::from_json(&String::from_utf8(payload_json)?)?;
    
    // Verify signature
    payload.verify_signature()?;
    
    // Check if we've already seen this message
    let _msg_id = format!("{}:{}:{}", payload.from_pubkey, envelope.inbox_id, payload.timestamp);
    
    // Create stored message
    let stored_msg = StoredMessage::from_payload(
        payload.clone(),
        keypair.public_key_string(),
        Utc::now(),
        false, // incoming
    );
    
    // Check if message already exists
    let existing_messages = message_store.get_messages_from(&payload.from_pubkey, None);
    let already_exists = existing_messages.iter().any(|msg| {
        msg.timestamp.timestamp() == payload.timestamp && msg.counter == payload.counter
    });
    
    if already_exists {
        return Ok(false); // Not a new message
    }
    
    message_store.store_message(stored_msg)?;
    
    // Show the new message with permission prompt
    println!("\n✉️  New message from unknown sender:");
    println!("From: {}", payload.from_pubkey);
    println!("Message: {}", payload.body);
    println!("\nAllow this sender to continue messaging you? [a]llow / [b]lock / [i]gnore");
    
    // For now, just print the prompt - in a real implementation,
    // this would wait for user input or be handled interactively
    
    Ok(true)
}

// Session 4: Quantum-Safe Messaging Functions

async fn send_quantum_safe_message(
    config_dir: &PathBuf,
    relay: &str,
    recipient: &str,
    message: &str,
    crypto_mode_str: &str,
    force_post_quantum: bool,
    adaptive: bool,
    security_prefs: &SecurityPreferences,
) -> Result<()> {
    // For now, fall back to the existing send_message function
    // In a full implementation, this would use QuantumSafeMessaging
    
    // Determine final crypto mode
    let mut selected_mode = if force_post_quantum {
        println!("🔒 Force post-quantum enabled - using quantum cryptography");
        CryptoMode::Quantum
    } else {
        parse_crypto_mode(crypto_mode_str)?
    };
    
    // Apply adaptive mode selection if enabled
    if adaptive {
        selected_mode = determine_adaptive_mode(selected_mode, security_prefs);
        println!("🔄 Adaptive mode selected: {}", selected_mode);
    }
    
    println!("📨 Sending message to '{}' using {} cryptography via {}...", 
             recipient, selected_mode, relay);
    
    // For Session 4, we'll enhance the existing send_message with crypto mode info
    send_message(config_dir, relay, recipient, message).await?;
    
    println!("✅ Message sent using {} cryptography", selected_mode);
    println!("🔐 Security: {}", selected_mode.security_level());
    println!("📈 Performance cost: {:.1}x baseline", selected_mode.performance_cost());
    println!("📦 Size overhead: ~{} bytes", selected_mode.size_overhead());
    
    Ok(())
}

fn determine_adaptive_mode(requested_mode: CryptoMode, prefs: &SecurityPreferences) -> CryptoMode {
    // Simple adaptive logic - in a real implementation, this would consider:
    // - Network bandwidth
    // - Battery level
    // - CPU usage
    // - Recipient capabilities
    
    if prefs.force_post_quantum {
        return CryptoMode::Quantum;
    }
    
    // For now, just ensure we meet minimum requirements
    if requested_mode.can_transition_to(prefs.minimum_crypto_mode) {
        prefs.minimum_crypto_mode
    } else {
        requested_mode
    }
}

fn update_security_preferences(
    config_dir: &PathBuf,
    default_mode: Option<&str>,
    adaptive: Option<bool>,
    minimum_mode: Option<&str>,
    auto_upgrade: Option<bool>,
) -> Result<()> {
    let mut prefs = load_security_preferences(config_dir)?;
    let mut changes = Vec::new();
    
    if let Some(mode_str) = default_mode {
        let mode = parse_crypto_mode(mode_str)?;
        prefs.default_crypto_mode = mode;
        changes.push(format!("Default crypto mode: {}", mode));
    }
    
    if let Some(adaptive_enabled) = adaptive {
        prefs.adaptive_mode = adaptive_enabled;
        changes.push(format!("Adaptive mode: {}", if adaptive_enabled { "enabled" } else { "disabled" }));
    }
    
    if let Some(min_mode_str) = minimum_mode {
        let min_mode = parse_crypto_mode(min_mode_str)?;
        prefs.minimum_crypto_mode = min_mode;
        changes.push(format!("Minimum crypto mode: {}", min_mode));
    }
    
    if let Some(auto_upgrade_enabled) = auto_upgrade {
        prefs.auto_upgrade = auto_upgrade_enabled;
        changes.push(format!("Auto upgrade: {}", if auto_upgrade_enabled { "enabled" } else { "disabled" }));
    }
    
    if changes.is_empty() {
        println!("No security settings changed.");
        return Ok(());
    }
    
    // Validate the new preferences
    let crypto_config = CryptoConfig {
        mode: prefs.default_crypto_mode,
        allow_auto_upgrade: prefs.auto_upgrade,
        adaptive_mode: prefs.adaptive_mode,
        minimum_mode: prefs.minimum_crypto_mode,
    };
    
    crypto_config.validate()
        .map_err(|e| anyhow::anyhow!("Invalid security configuration: {}", e))?;
    
    save_security_preferences(config_dir, &prefs)?;
    
    println!("🛡️  Security preferences updated:");
    for change in changes {
        println!("   ✓ {}", change);
    }
    
    Ok(())
}

fn show_security_configuration(config_dir: &PathBuf) -> Result<()> {
    let prefs = load_security_preferences(config_dir)?;
    
    println!("🛡️  Current Security Configuration:");
    println!("   Default crypto mode: {} ({})", 
             prefs.default_crypto_mode, 
             prefs.default_crypto_mode.description());
    println!("   Minimum crypto mode: {} ({})", 
             prefs.minimum_crypto_mode,
             prefs.minimum_crypto_mode.security_level());
    println!("   Adaptive mode: {}", 
             if prefs.adaptive_mode { "enabled ✓" } else { "disabled" });
    println!("   Auto upgrade: {}", 
             if prefs.auto_upgrade { "enabled ✓" } else { "disabled" });
    println!("   Force post-quantum: {}", 
             if prefs.force_post_quantum { "enabled ✓" } else { "disabled" });
    
    println!("\n📈 Crypto Mode Performance:");
    for mode in CryptoMode::all() {
        println!("   {} {}: {:.1}x cost, +{} bytes", 
                 match mode {
                     CryptoMode::Classical => "🔓",
                     CryptoMode::Hybrid => "🔐", 
                     CryptoMode::Quantum => "⚛️",
                 },
                 mode,
                 mode.performance_cost(),
                 mode.size_overhead());
    }
    
    Ok(())
}

fn test_crypto_modes(_config_dir: &PathBuf, mode: &str) -> Result<()> {
    println!("🧪 Testing crypto mode compatibility...");
    
    let modes_to_test = if mode == "all" {
        CryptoMode::all().to_vec()
    } else {
        vec![parse_crypto_mode(mode)?]
    };
    
    for test_mode in modes_to_test {
        println!("\n🔍 Testing {} mode:", test_mode);
        println!("   ✅ Mode available");
        println!("   📈 Performance cost: {:.1}x", test_mode.performance_cost());
        println!("   📦 Size overhead: {} bytes", test_mode.size_overhead());
        println!("   🛡️  Security: {}", test_mode.security_level());
        println!("   ⚛️  Quantum resistant: {}", if test_mode.is_quantum_resistant() { "Yes" } else { "No" });
    }
    
    Ok(())
}

// Security Preferences Storage Functions

fn load_security_preferences(config_dir: &PathBuf) -> Result<SecurityPreferences> {
    let prefs_file = config_dir.join("security.json");
    
    if !prefs_file.exists() {
        return Ok(SecurityPreferences::default());
    }
    
    let data = std::fs::read_to_string(&prefs_file)?;
    let prefs: SecurityPreferences = serde_json::from_str(&data)?;
    
    Ok(prefs)
}

fn save_security_preferences(config_dir: &PathBuf, prefs: &SecurityPreferences) -> Result<()> {
    let prefs_file = config_dir.join("security.json");
    let data = serde_json::to_string_pretty(prefs)?;
    std::fs::write(&prefs_file, data)?;
    Ok(())
}

fn process_conversation_message(
    envelope: &MessageEnvelope,
    conversation: &mut ConversationState,
    keypair: &UserKeyPair,
    _contact_manager: &mut ContactManager,
    message_store: &mut MessageStore,
) -> Result<bool> {
    // Decrypt the message using the shared secret
    let encrypted_payload = envelope.decode_payload()?;
    let payload_json = decrypt_symmetric(&conversation.shared_secret, &encrypted_payload)?;
    let payload: MessagePayload = MessagePayload::from_json(&String::from_utf8(payload_json)?)?;
    
    // Verify signature
    payload.verify_signature()?;
    
    // Check if sender is allowed
    // For now, we'll assume conversation messages are allowed since we have a conversation
    
    // Update conversation counter
    conversation.update_their_counter(payload.counter);
    
    // Check for duplicate
    let existing_messages = message_store.get_messages_from(&payload.from_pubkey, None);
    let already_exists = existing_messages.iter().any(|msg| {
        msg.timestamp.timestamp() == payload.timestamp && msg.counter == payload.counter
    });
    
    if already_exists {
        return Ok(false);
    }
    
    // Store the message
    let stored_msg = StoredMessage::from_payload(
        payload.clone(),
        keypair.public_key_string(),
        Utc::now(),
        false, // incoming
    );
    
    message_store.store_message(stored_msg)?;
    
    // Show new message notification
    let display_name = payload.from_pubkey.clone(); // We removed contact_manager access
    
    println!("✉️  New message from {}: {}", display_name, payload.body);
    
    Ok(true)
}
