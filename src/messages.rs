use crate::error::Result;
use crate::protocol::MessagePayload;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A stored message in the local database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredMessage {
    pub id: String,                    // Unique message ID
    pub from_pubkey: String,           // Sender's public key
    pub to_pubkey: String,             // Recipient's public key (us)
    pub content: String,               // Message content
    pub timestamp: DateTime<Utc>,      // When message was sent
    pub received_at: DateTime<Utc>,    // When we received it
    pub is_outgoing: bool,             // True if we sent this message
    pub conversation_id: String,       // Conversation identifier
    pub counter: u64,                  // Message counter in conversation
}

impl StoredMessage {
    pub fn from_payload(
        payload: MessagePayload,
        our_pubkey: String,
        received_at: DateTime<Utc>,
        is_outgoing: bool,
    ) -> Self {
        let conversation_id = if is_outgoing {
            format!("{}:{}", our_pubkey, payload.from_pubkey)
        } else {
            format!("{}:{}", payload.from_pubkey, our_pubkey)
        };

        let id = format!("{}:{}:{}", conversation_id, payload.counter, payload.timestamp);

        Self {
            id,
            from_pubkey: payload.from_pubkey.clone(),
            to_pubkey: our_pubkey,
            content: payload.body,
            timestamp: DateTime::from_timestamp(payload.timestamp, 0)
                .unwrap_or_else(|| Utc::now()),
            received_at,
            is_outgoing,
            conversation_id,
            counter: payload.counter,
        }
    }
}

/// Conversation summary for listing
#[derive(Debug, Clone)]
pub struct ConversationSummary {
    pub id: String,
    pub other_pubkey: String,
    pub last_message: String,
    pub last_timestamp: DateTime<Utc>,
    pub unread_count: usize,
    pub message_count: usize,
}

/// Message storage and retrieval
#[derive(Default)]
pub struct MessageStore {
    messages: HashMap<String, StoredMessage>, // message_id -> message
    conversations: HashMap<String, Vec<String>>, // conversation_id -> [message_ids]
    last_read: HashMap<String, DateTime<Utc>>, // conversation_id -> last_read_time
}

impl MessageStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Store a new message
    pub fn store_message(&mut self, message: StoredMessage) -> Result<()> {
        // Check for duplicate
        if self.messages.contains_key(&message.id) {
            return Ok(()); // Already stored
        }

        let conversation_id = message.conversation_id.clone();
        let message_id = message.id.clone();

        // Store the message
        self.messages.insert(message_id.clone(), message);

        // Add to conversation
        let conversation_id_clone = conversation_id.clone();
        self.conversations
            .entry(conversation_id_clone)
            .or_default()
            .push(message_id);

        // Sort conversation messages by timestamp
        if let Some(msg_ids) = self.conversations.get_mut(&conversation_id) {
            msg_ids.sort_by(|a, b| {
                let msg_a = &self.messages[a];
                let msg_b = &self.messages[b];
                msg_a.timestamp.cmp(&msg_b.timestamp)
            });
        }

        Ok(())
    }

    /// Get messages for a conversation
    pub fn get_conversation_messages(
        &self,
        conversation_id: &str,
        limit: Option<usize>,
    ) -> Vec<&StoredMessage> {
        let msg_ids = match self.conversations.get(conversation_id) {
            Some(ids) => ids,
            None => return vec![],
        };

        let mut messages: Vec<&StoredMessage> = msg_ids
            .iter()
            .filter_map(|id| self.messages.get(id))
            .collect();

        // Sort by timestamp (most recent last)
        messages.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        // Apply limit
        if let Some(limit) = limit {
            if messages.len() > limit {
                let start_index = messages.len() - limit;
                messages = messages[start_index..].iter().copied().collect();
            }
        }

        messages
    }

    /// Get messages from a specific sender
    pub fn get_messages_from(
        &self,
        from_pubkey: &str,
        limit: Option<usize>,
    ) -> Vec<&StoredMessage> {
        let mut messages: Vec<&StoredMessage> = self
            .messages
            .values()
            .filter(|msg| msg.from_pubkey == from_pubkey)
            .collect();

        messages.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        if let Some(limit) = limit {
            if messages.len() > limit {
                let start_index = messages.len() - limit;
                messages = messages[start_index..].iter().copied().collect();
            }
        }

        messages
    }

    /// Get all messages (latest first)
    pub fn get_all_messages(&self, limit: Option<usize>) -> Vec<&StoredMessage> {
        let mut messages: Vec<&StoredMessage> = self.messages.values().collect();
        messages.sort_by(|a, b| b.timestamp.cmp(&a.timestamp)); // Newest first

        if let Some(limit) = limit {
            messages.truncate(limit);
        }

        messages
    }

    /// Get conversation summaries
    pub fn get_conversation_summaries(&self) -> Vec<ConversationSummary> {
        let mut summaries = Vec::new();

        for (conversation_id, msg_ids) in &self.conversations {
            if msg_ids.is_empty() {
                continue;
            }

            let messages: Vec<&StoredMessage> = msg_ids
                .iter()
                .filter_map(|id| self.messages.get(id))
                .collect();

            if let Some(last_message) = messages.last() {
                let last_read = self.last_read.get(conversation_id).copied()
                    .unwrap_or_else(|| DateTime::from_timestamp(0, 0).unwrap());

                let unread_count = messages
                    .iter()
                    .filter(|msg| msg.timestamp > last_read && !msg.is_outgoing)
                    .count();

                // Extract other party's pubkey from conversation ID
                let parts: Vec<&str> = conversation_id.split(':').collect();
                let other_pubkey = if parts.len() >= 2 {
                    if parts[0] != last_message.to_pubkey {
                        parts[0].to_string()
                    } else {
                        parts[1].to_string()
                    }
                } else {
                    "unknown".to_string()
                };

                summaries.push(ConversationSummary {
                    id: conversation_id.clone(),
                    other_pubkey,
                    last_message: last_message.content.clone(),
                    last_timestamp: last_message.timestamp,
                    unread_count,
                    message_count: messages.len(),
                });
            }
        }

        // Sort by last message timestamp (newest first)
        summaries.sort_by(|a, b| b.last_timestamp.cmp(&a.last_timestamp));
        summaries
    }

    /// Mark conversation as read
    pub fn mark_conversation_read(&mut self, conversation_id: &str) {
        self.last_read.insert(conversation_id.to_string(), Utc::now());
    }

    /// Get message count
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    /// Get conversation count
    pub fn conversation_count(&self) -> usize {
        self.conversations.len()
    }

    /// Export messages for backup/sync
    pub fn export_messages(&self) -> HashMap<String, StoredMessage> {
        self.messages.clone()
    }

    /// Import messages from backup/sync
    pub fn import_messages(&mut self, messages: HashMap<String, StoredMessage>) -> Result<()> {
        for (id, message) in messages {
            self.messages.insert(id.clone(), message.clone());
            
            // Rebuild conversation index
            let conversation_id = message.conversation_id.clone();
            self.conversations
                .entry(conversation_id)
                .or_default()
                .push(id);
        }

        // Sort all conversations
        for (_, msg_ids) in self.conversations.iter_mut() {
            msg_ids.sort_by(|a, b| {
                let msg_a = &self.messages[a];
                let msg_b = &self.messages[b];
                msg_a.timestamp.cmp(&msg_b.timestamp)
            });
        }

        Ok(())
    }

    /// Search messages by content
    pub fn search_messages(&self, query: &str, limit: Option<usize>) -> Vec<&StoredMessage> {
        let query_lower = query.to_lowercase();
        let mut matches: Vec<&StoredMessage> = self
            .messages
            .values()
            .filter(|msg| msg.content.to_lowercase().contains(&query_lower))
            .collect();

        matches.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        if let Some(limit) = limit {
            matches.truncate(limit);
        }

        matches
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::MessagePayload;

    fn create_test_payload(from: &str, body: &str, counter: u64) -> MessagePayload {
        MessagePayload {
            from_pubkey: from.to_string(),
            timestamp: Utc::now().timestamp(),
            body: body.to_string(),
            room: None,
            counter,
            sig: "test_sig".to_string(),
        }
    }

    #[test]
    fn test_message_store() {
        let mut store = MessageStore::new();
        let our_pubkey = "pubkey:alice".to_string();
        let bob_pubkey = "pubkey:bob".to_string();

        // Create some test messages
        let payload1 = create_test_payload(&bob_pubkey, "Hello Alice!", 1);
        let payload2 = create_test_payload(&bob_pubkey, "How are you?", 2);

        let msg1 = StoredMessage::from_payload(
            payload1,
            our_pubkey.clone(),
            Utc::now(),
            false, // incoming
        );

        let msg2 = StoredMessage::from_payload(
            payload2,
            our_pubkey.clone(),
            Utc::now(),
            false, // incoming
        );

        // Store messages
        store.store_message(msg1.clone()).unwrap();
        store.store_message(msg2.clone()).unwrap();

        // Test retrieval
        assert_eq!(store.message_count(), 2);
        assert_eq!(store.conversation_count(), 1);

        let conversation_id = &msg1.conversation_id;
        let messages = store.get_conversation_messages(conversation_id, None);
        assert_eq!(messages.len(), 2);

        // Test conversation summaries
        let summaries = store.get_conversation_summaries();
        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].other_pubkey, bob_pubkey);
        assert_eq!(summaries[0].message_count, 2);
    }

    #[test]
    fn test_message_search() {
        let mut store = MessageStore::new();
        let our_pubkey = "pubkey:alice".to_string();

        let payload = create_test_payload("pubkey:bob", "Hello world!", 1);
        let msg = StoredMessage::from_payload(payload, our_pubkey, Utc::now(), false);

        store.store_message(msg).unwrap();

        let results = store.search_messages("world", None);
        assert_eq!(results.len(), 1);
        assert!(results[0].content.contains("world"));

        let no_results = store.search_messages("xyz", None);
        assert_eq!(no_results.len(), 0);
    }
}
