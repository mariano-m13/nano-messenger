use std::collections::HashMap;
use crate::crypto::{hash_sha256, derive_shared_secret, X25519PrivateKey, X25519PublicKey};

/// Derives inbox ID for first contact messages
/// Uses: SHA256("first_contact:" + recipient_public_key)
pub fn derive_first_contact_inbox(recipient_public_key: &X25519PublicKey) -> String {
    let mut data = Vec::new();
    data.extend_from_slice(b"first_contact:");
    data.extend_from_slice(recipient_public_key.as_bytes());
    
    let hash = hash_sha256(&data);
    hex::encode(hash)
}

/// Derives inbox ID for ongoing conversation messages
/// Uses: SHA256(shared_secret + counter)
pub fn derive_conversation_inbox(shared_secret: &[u8; 32], counter: u64) -> String {
    let mut data = Vec::new();
    data.extend_from_slice(shared_secret);
    data.extend_from_slice(&counter.to_be_bytes());
    
    let hash = hash_sha256(&data);
    hex::encode(hash)
}

/// Derives multiple inbox IDs for a conversation (for checking recent messages)
/// Returns inbox IDs for counter, counter-1, counter-2, etc.
pub fn derive_recent_inboxes(shared_secret: &[u8; 32], current_counter: u64, count: usize) -> Vec<String> {
    let mut inboxes = Vec::new();
    
    for i in 0..count {
        if current_counter >= i as u64 {
            let counter = current_counter - i as u64;
            inboxes.push(derive_conversation_inbox(shared_secret, counter));
        }
    }
    
    inboxes
}

/// Compute shared secret between two parties using ECDH
pub fn compute_shared_secret(
    our_private: &X25519PrivateKey,
    their_public: &X25519PublicKey,
) -> [u8; 32] {
    derive_shared_secret(our_private, their_public)
}

/// Inbox manager to track conversation state
#[derive(Clone, Debug)]
pub struct ConversationState {
    pub their_public_key: X25519PublicKey,
    pub shared_secret: [u8; 32],
    pub our_counter: u64,        // Counter for messages we send
    pub their_last_counter: u64, // Last counter we saw from them
}

impl ConversationState {
    pub fn new(
        our_private: &X25519PrivateKey,
        their_public: X25519PublicKey,
    ) -> Self {
        let shared_secret = compute_shared_secret(our_private, &their_public);
        
        Self {
            their_public_key: their_public,
            shared_secret,
            our_counter: 1, // Start at 1 (0 was the first contact)
            their_last_counter: 0,
        }
    }

    /// Get the inbox ID for our next outgoing message
    pub fn get_outgoing_inbox(&mut self) -> String {
        let inbox = derive_conversation_inbox(&self.shared_secret, self.our_counter);
        self.our_counter += 1;
        inbox
    }

    /// Get inbox IDs we should check for their incoming messages
    pub fn get_incoming_inboxes(&self, check_count: usize) -> Vec<String> {
        // Check from their_last_counter + 1 forward
        let mut inboxes = Vec::new();
        
        for i in 1..=check_count {
            let counter = self.their_last_counter + i as u64;
            inboxes.push(derive_conversation_inbox(&self.shared_secret, counter));
        }
        
        inboxes
    }

    /// Update the last seen counter from them
    pub fn update_their_counter(&mut self, counter: u64) {
        if counter > self.their_last_counter {
            self.their_last_counter = counter;
        }
    }

    /// Get the first contact inbox for this conversation
    pub fn get_first_contact_inbox(&self) -> String {
        derive_first_contact_inbox(&self.their_public_key)
    }
}

/// Utility to manage multiple conversations
#[derive(Default)]
pub struct ConversationManager {
    conversations: std::collections::HashMap<String, ConversationState>, // pubkey -> state
}

impl ConversationManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Start a new conversation or get existing one
    pub fn get_or_create_conversation(
        &mut self,
        our_private: &X25519PrivateKey,
        their_public_key_str: &str,
        their_x25519_key: X25519PublicKey,
    ) -> &mut ConversationState {
        self.conversations
            .entry(their_public_key_str.to_string())
            .or_insert_with(|| ConversationState::new(our_private, their_x25519_key))
    }

    /// Get conversation state by public key
    pub fn get_conversation(&mut self, their_public_key_str: &str) -> Option<&mut ConversationState> {
        self.conversations.get_mut(their_public_key_str)
    }

    /// List all active conversations
    pub fn list_conversations(&self) -> Vec<&str> {
        self.conversations.keys().map(|s| s.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::UserKeyPair;

    #[test]
    fn test_first_contact_inbox() {
        let alice = UserKeyPair::generate();
        let alice_public = alice.public_keys();
        
        let inbox_id = derive_first_contact_inbox(&alice_public.x25519_key);
        
        // Should be deterministic
        let inbox_id2 = derive_first_contact_inbox(&alice_public.x25519_key);
        assert_eq!(inbox_id, inbox_id2);
        
        // Should be 64 chars (32 bytes hex encoded)
        assert_eq!(inbox_id.len(), 64);
    }

    #[test]
    fn test_conversation_inbox() {
        let shared_secret = [42u8; 32];
        
        let inbox1 = derive_conversation_inbox(&shared_secret, 1);
        let inbox2 = derive_conversation_inbox(&shared_secret, 2);
        
        // Should be different for different counters
        assert_ne!(inbox1, inbox2);
        
        // Should be deterministic
        let inbox1_again = derive_conversation_inbox(&shared_secret, 1);
        assert_eq!(inbox1, inbox1_again);
    }

    #[test]
    fn test_conversation_state() {
        let alice = UserKeyPair::generate();
        let bob = UserKeyPair::generate();
        
        let alice_public = alice.public_keys();
        let bob_public = bob.public_keys();
        
        // Alice creates conversation with Bob
        let mut alice_conv = ConversationState::new(&alice.x25519_key, bob_public.x25519_key);
        
        // Bob creates conversation with Alice (should derive same shared secret)
        let mut bob_conv = ConversationState::new(&bob.x25519_key, alice_public.x25519_key);
        
        // Shared secrets should match
        assert_eq!(alice_conv.shared_secret, bob_conv.shared_secret);
        
        // Alice sends message
        let alice_outbox = alice_conv.get_outgoing_inbox();
        
        // Bob should be able to find it in his incoming inboxes
        let bob_inboxes = bob_conv.get_incoming_inboxes(5);
        assert!(bob_inboxes.contains(&alice_outbox));
        
        // Update Bob's counter
        bob_conv.update_their_counter(1);
        assert_eq!(bob_conv.their_last_counter, 1);
    }

    #[test]
    fn test_recent_inboxes() {
        let shared_secret = [123u8; 32];
        
        let inboxes = derive_recent_inboxes(&shared_secret, 5, 3);
        assert_eq!(inboxes.len(), 3);
        
        // Should contain counter 5, 4, 3
        assert_eq!(inboxes[0], derive_conversation_inbox(&shared_secret, 5));
        assert_eq!(inboxes[1], derive_conversation_inbox(&shared_secret, 4));
        assert_eq!(inboxes[2], derive_conversation_inbox(&shared_secret, 3));
    }

    #[test]
    fn test_conversation_manager() {
        let alice = UserKeyPair::generate();
        let bob = UserKeyPair::generate();
        let charlie = UserKeyPair::generate();
        
        let bob_public = bob.public_keys();
        let charlie_public = charlie.public_keys();
        
        let mut manager = ConversationManager::new();
        
        // Start conversations
        let bob_pubkey_str = bob_public.public_key_string();
        let charlie_pubkey_str = charlie_public.public_key_string();
        
        manager.get_or_create_conversation(&alice.x25519_key, &bob_pubkey_str, bob_public.x25519_key);
        manager.get_or_create_conversation(&alice.x25519_key, &charlie_pubkey_str, charlie_public.x25519_key);
        
        // Should have 2 conversations
        let conversations = manager.list_conversations();
        assert_eq!(conversations.len(), 2);
        assert!(conversations.contains(&bob_pubkey_str.as_str()));
        assert!(conversations.contains(&charlie_pubkey_str.as_str()));
    }
}
