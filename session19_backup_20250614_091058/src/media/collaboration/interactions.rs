/// Media Reactions & Interactions
/// 
/// Provides commenting, reactions, and interactive features for media content
/// with real-time updates and quantum-resistant encryption.

use crate::error::{NanoError, Result};
use crate::media::metadata::UserId;
use crate::media::storage::FileId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

/// Comment ID for tracking comments
pub type CommentId = Uuid;

/// Reaction types for media content
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReactionType {
    Like,
    Love,
    Laugh,
    Wow,
    Sad,
    Angry,
    ThumbsUp,
    ThumbsDown,
    Fire,
    Heart,
}

impl ReactionType {
    /// Get emoji representation
    pub fn emoji(&self) -> &'static str {
        match self {
            ReactionType::Like => "👍",
            ReactionType::Love => "❤️",
            ReactionType::Laugh => "😂",
            ReactionType::Wow => "😮",
            ReactionType::Sad => "😢",
            ReactionType::Angry => "😠",
            ReactionType::ThumbsUp => "👍",
            ReactionType::ThumbsDown => "👎",
            ReactionType::Fire => "🔥",
            ReactionType::Heart => "💖",
        }
    }

    /// Get all available reaction types
    pub fn all() -> Vec<ReactionType> {
        vec![
            ReactionType::Like,
            ReactionType::Love,
            ReactionType::Laugh,
            ReactionType::Wow,
            ReactionType::Sad,
            ReactionType::Angry,
            ReactionType::ThumbsUp,
            ReactionType::ThumbsDown,
            ReactionType::Fire,
            ReactionType::Heart,
        ]
    }
}

/// Media comment with threading support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaComment {
    pub comment_id: CommentId,
    pub file_id: FileId,
    pub author: UserId,
    pub content: String,
    pub timestamp: SystemTime,
    pub edited_at: Option<SystemTime>,
    pub thread_parent: Option<CommentId>,
    pub reply_count: u32,
    pub reactions: HashMap<ReactionType, Vec<UserId>>,
    pub is_deleted: bool,
    pub is_edited: bool,
}

impl MediaComment {
    /// Create a new comment
    pub fn new(file_id: FileId, author: UserId, content: String) -> Self {
        Self {
            comment_id: Uuid::new_v4(),
            file_id,
            author,
            content,
            timestamp: SystemTime::now(),
            edited_at: None,
            thread_parent: None,
            reply_count: 0,
            reactions: HashMap::new(),
            is_deleted: false,
            is_edited: false,
        }
    }

    /// Create a reply to another comment
    pub fn new_reply(
        file_id: FileId,
        author: UserId,
        content: String,
        parent_id: CommentId,
    ) -> Self {
        Self {
            comment_id: Uuid::new_v4(),
            file_id,
            author,
            content,
            timestamp: SystemTime::now(),
            edited_at: None,
            thread_parent: Some(parent_id),
            reply_count: 0,
            reactions: HashMap::new(),
            is_deleted: false,
            is_edited: false,
        }
    }

    /// Edit comment content
    pub fn edit(&mut self, new_content: String) -> Result<()> {
        if self.is_deleted {
            return Err(NanoError::Media("Cannot edit deleted comment".to_string()));
        }

        self.content = new_content;
        self.edited_at = Some(SystemTime::now());
        self.is_edited = true;
        Ok(())
    }

    /// Mark comment as deleted
    pub fn delete(&mut self) {
        self.is_deleted = true;
        self.content = "[Comment deleted]".to_string();
    }

    /// Add reaction to comment
    pub fn add_reaction(&mut self, reaction_type: ReactionType, user_id: UserId) -> bool {
        let users = self.reactions.entry(reaction_type).or_insert_with(Vec::new);
        if !users.contains(&user_id) {
            users.push(user_id);
            true
        } else {
            false
        }
    }

    /// Remove reaction from comment
    pub fn remove_reaction(&mut self, reaction_type: ReactionType, user_id: &UserId) -> bool {
        if let Some(users) = self.reactions.get_mut(&reaction_type) {
            if let Some(pos) = users.iter().position(|u| u == user_id) {
                users.remove(pos);
                return true;
            }
        }
        false
    }

    /// Get total reaction count
    pub fn total_reactions(&self) -> u32 {
        self.reactions.values().map(|users| users.len() as u32).sum()
    }

    /// Get reaction count for specific type
    pub fn reaction_count(&self, reaction_type: ReactionType) -> u32 {
        self.reactions.get(&reaction_type).map_or(0, |users| users.len() as u32)
    }

    /// Check if user has reacted with specific type
    pub fn user_has_reaction(&self, reaction_type: ReactionType, user_id: &UserId) -> bool {
        self.reactions.get(&reaction_type)
            .map_or(false, |users| users.contains(user_id))
    }
}

/// Media annotation (drawings, highlights, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaAnnotation {
    pub annotation_id: Uuid,
    pub file_id: FileId,
    pub author: UserId,
    pub annotation_type: AnnotationType,
    pub content: AnnotationContent,
    pub position: AnnotationPosition,
    pub timestamp: SystemTime,
    pub is_visible: bool,
    pub color: String,
    pub stroke_width: f32,
}

/// Types of annotations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnnotationType {
    Rectangle,
    Circle,
    Arrow,
    FreeHand,
    Text,
    Highlight,
    Timestamp, // For video annotations
}

/// Annotation content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnnotationContent {
    Text(String),
    Drawing(Vec<Point>),
    Shape(ShapeData),
    Timestamp { seconds: f64, text: Option<String> },
}

/// Point for drawing annotations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub pressure: Option<f32>, // For pressure-sensitive drawing
}

/// Shape data for geometric annotations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeData {
    pub start_point: Point,
    pub end_point: Point,
    pub properties: HashMap<String, String>,
}

/// Position of annotation on media
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnotationPosition {
    pub x: f32,         // X coordinate (0.0 - 1.0, relative to media dimensions)
    pub y: f32,         // Y coordinate (0.0 - 1.0, relative to media dimensions)
    pub width: f32,     // Width (0.0 - 1.0, relative to media dimensions)
    pub height: f32,    // Height (0.0 - 1.0, relative to media dimensions)
    pub rotation: f32,  // Rotation in degrees
    pub z_index: i32,   // Layer order
}

impl MediaAnnotation {
    /// Create a new text annotation
    pub fn new_text(
        file_id: FileId,
        author: UserId,
        text: String,
        position: AnnotationPosition,
    ) -> Self {
        Self {
            annotation_id: Uuid::new_v4(),
            file_id,
            author,
            annotation_type: AnnotationType::Text,
            content: AnnotationContent::Text(text),
            position,
            timestamp: SystemTime::now(),
            is_visible: true,
            color: "#000000".to_string(),
            stroke_width: 2.0,
        }
    }

    /// Create a new drawing annotation
    pub fn new_drawing(
        file_id: FileId,
        author: UserId,
        points: Vec<Point>,
        position: AnnotationPosition,
    ) -> Self {
        Self {
            annotation_id: Uuid::new_v4(),
            file_id,
            author,
            annotation_type: AnnotationType::FreeHand,
            content: AnnotationContent::Drawing(points),
            position,
            timestamp: SystemTime::now(),
            is_visible: true,
            color: "#ff0000".to_string(),
            stroke_width: 3.0,
        }
    }

    /// Create a new shape annotation
    pub fn new_shape(
        file_id: FileId,
        author: UserId,
        annotation_type: AnnotationType,
        shape_data: ShapeData,
        position: AnnotationPosition,
    ) -> Self {
        Self {
            annotation_id: Uuid::new_v4(),
            file_id,
            author,
            annotation_type,
            content: AnnotationContent::Shape(shape_data),
            position,
            timestamp: SystemTime::now(),
            is_visible: true,
            color: "#0000ff".to_string(),
            stroke_width: 2.0,
        }
    }

    /// Create a timestamp annotation for video
    pub fn new_timestamp(
        file_id: FileId,
        author: UserId,
        seconds: f64,
        text: Option<String>,
        position: AnnotationPosition,
    ) -> Self {
        Self {
            annotation_id: Uuid::new_v4(),
            file_id,
            author,
            annotation_type: AnnotationType::Timestamp,
            content: AnnotationContent::Timestamp { seconds, text },
            position,
            timestamp: SystemTime::now(),
            is_visible: true,
            color: "#ffff00".to_string(),
            stroke_width: 1.0,
        }
    }

    /// Toggle visibility
    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }

    /// Update position
    pub fn update_position(&mut self, position: AnnotationPosition) {
        self.position = position;
    }

    /// Update style
    pub fn update_style(&mut self, color: String, stroke_width: f32) {
        self.color = color;
        self.stroke_width = stroke_width;
    }
}

/// Comment thread for organizing discussions
#[derive(Debug, Clone)]
pub struct CommentThread {
    pub parent_comment: MediaComment,
    pub replies: Vec<MediaComment>,
    pub total_replies: u32,
    pub last_activity: SystemTime,
}

impl CommentThread {
    /// Create a new comment thread
    pub fn new(parent_comment: MediaComment) -> Self {
        Self {
            last_activity: parent_comment.timestamp,
            parent_comment,
            replies: Vec::new(),
            total_replies: 0,
        }
    }

    /// Add a reply to the thread
    pub fn add_reply(&mut self, reply: MediaComment) {
        self.replies.push(reply);
        self.total_replies += 1;
        self.last_activity = SystemTime::now();
        self.parent_comment.reply_count += 1;
    }

    /// Get replies sorted by timestamp
    pub fn get_sorted_replies(&self) -> Vec<&MediaComment> {
        let mut replies: Vec<&MediaComment> = self.replies.iter().collect();
        replies.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        replies
    }
}

/// Media interactions manager
pub struct MediaInteractions {
    file_id: FileId,
    reactions: Arc<RwLock<HashMap<ReactionType, Vec<UserId>>>>,
    comments: Arc<RwLock<HashMap<CommentId, MediaComment>>>,
    comment_threads: Arc<RwLock<HashMap<CommentId, CommentThread>>>,
    annotations: Arc<RwLock<HashMap<Uuid, MediaAnnotation>>>,
    view_count: Arc<RwLock<u64>>,
    download_count: Arc<RwLock<u64>>,
    event_broadcaster: broadcast::Sender<InteractionEvent>,
}

impl MediaInteractions {
    /// Create new media interactions for a file
    pub fn new(file_id: FileId) -> Self {
        let (event_broadcaster, _) = broadcast::channel(1000);
        
        Self {
            file_id,
            reactions: Arc::new(RwLock::new(HashMap::new())),
            comments: Arc::new(RwLock::new(HashMap::new())),
            comment_threads: Arc::new(RwLock::new(HashMap::new())),
            annotations: Arc::new(RwLock::new(HashMap::new())),
            view_count: Arc::new(RwLock::new(0)),
            download_count: Arc::new(RwLock::new(0)),
            event_broadcaster,
        }
    }

    /// Add a reaction
    pub async fn add_reaction(&self, reaction_type: ReactionType, user_id: UserId) -> Result<bool> {
        let mut reactions = self.reactions.write().await;
        let users = reactions.entry(reaction_type).or_insert_with(Vec::new);
        
        if !users.contains(&user_id) {
            users.push(user_id.clone());
            
            // Broadcast event
            let event = InteractionEvent::ReactionAdded {
                file_id: self.file_id,
                reaction_type,
                user_id,
                timestamp: SystemTime::now(),
            };
            let _ = self.event_broadcaster.send(event);
            
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Remove a reaction
    pub async fn remove_reaction(&self, reaction_type: ReactionType, user_id: &UserId) -> Result<bool> {
        let mut reactions = self.reactions.write().await;
        
        if let Some(users) = reactions.get_mut(&reaction_type) {
            if let Some(pos) = users.iter().position(|u| u == user_id) {
                users.remove(pos);
                
                // Broadcast event
                let event = InteractionEvent::ReactionRemoved {
                    file_id: self.file_id,
                    reaction_type,
                    user_id: user_id.clone(),
                    timestamp: SystemTime::now(),
                };
                let _ = self.event_broadcaster.send(event);
                
                return Ok(true);
            }
        }
        
        Ok(false)
    }

    /// Add a comment
    pub async fn add_comment(&self, author: UserId, content: String, parent_id: Option<CommentId>) -> Result<CommentId> {
        let comment = if let Some(parent_id) = parent_id {
            MediaComment::new_reply(self.file_id, author.clone(), content, parent_id)
        } else {
            MediaComment::new(self.file_id, author.clone(), content)
        };

        let comment_id = comment.comment_id;

        // Store comment
        {
            let mut comments = self.comments.write().await;
            comments.insert(comment_id, comment.clone());
        }

        // Handle threading
        if let Some(parent_id) = parent_id {
            let mut threads = self.comment_threads.write().await;
            if let Some(thread) = threads.get_mut(&parent_id) {
                thread.add_reply(comment.clone());
            }
        } else {
            // Create new thread for top-level comment
            let thread = CommentThread::new(comment.clone());
            let mut threads = self.comment_threads.write().await;
            threads.insert(comment_id, thread);
        }

        // Broadcast event
        let event = InteractionEvent::CommentAdded {
            file_id: self.file_id,
            comment_id,
            author,
            parent_id,
            timestamp: SystemTime::now(),
        };
        let _ = self.event_broadcaster.send(event);

        Ok(comment_id)
    }

    /// Edit a comment
    pub async fn edit_comment(&self, comment_id: &CommentId, editor: &UserId, new_content: String) -> Result<()> {
        let mut comments = self.comments.write().await;
        
        if let Some(comment) = comments.get_mut(comment_id) {
            if comment.author != *editor {
                return Err(NanoError::Media("Only comment author can edit comment".to_string()));
            }
            
            comment.edit(new_content)?;
            
            // Broadcast event
            let event = InteractionEvent::CommentEdited {
                file_id: self.file_id,
                comment_id: *comment_id,
                editor: editor.clone(),
                timestamp: SystemTime::now(),
            };
            let _ = self.event_broadcaster.send(event);
            
            Ok(())
        } else {
            Err(NanoError::Media("Comment not found".to_string()))
        }
    }

    /// Delete a comment
    pub async fn delete_comment(&self, comment_id: &CommentId, deleter: &UserId) -> Result<()> {
        let mut comments = self.comments.write().await;
        
        if let Some(comment) = comments.get_mut(comment_id) {
            if comment.author != *deleter {
                return Err(NanoError::Media("Only comment author can delete comment".to_string()));
            }
            
            comment.delete();
            
            // Broadcast event
            let event = InteractionEvent::CommentDeleted {
                file_id: self.file_id,
                comment_id: *comment_id,
                deleter: deleter.clone(),
                timestamp: SystemTime::now(),
            };
            let _ = self.event_broadcaster.send(event);
            
            Ok(())
        } else {
            Err(NanoError::Media("Comment not found".to_string()))
        }
    }

    /// Add an annotation
    pub async fn add_annotation(&self, annotation: MediaAnnotation, author: &UserId) -> Result<Uuid> {
        if annotation.author != *author {
            return Err(NanoError::Media("Annotation author mismatch".to_string()));
        }

        let annotation_id = annotation.annotation_id;
        
        {
            let mut annotations = self.annotations.write().await;
            annotations.insert(annotation_id, annotation);
        }

        // Broadcast event
        let event = InteractionEvent::AnnotationAdded {
            file_id: self.file_id,
            annotation_id,
            author: author.clone(),
            timestamp: SystemTime::now(),
        };
        let _ = self.event_broadcaster.send(event);

        Ok(annotation_id)
    }

    /// Remove an annotation
    pub async fn remove_annotation(&self, annotation_id: &Uuid, remover: &UserId) -> Result<()> {
        let mut annotations = self.annotations.write().await;
        
        if let Some(annotation) = annotations.get(annotation_id) {
            if annotation.author != *remover {
                return Err(NanoError::Media("Only annotation author can remove annotation".to_string()));
            }
            
            annotations.remove(annotation_id);
            
            // Broadcast event
            let event = InteractionEvent::AnnotationRemoved {
                file_id: self.file_id,
                annotation_id: *annotation_id,
                remover: remover.clone(),
                timestamp: SystemTime::now(),
            };
            let _ = self.event_broadcaster.send(event);
            
            Ok(())
        } else {
            Err(NanoError::Media("Annotation not found".to_string()))
        }
    }

    /// Increment view count
    pub async fn increment_views(&self) {
        let mut view_count = self.view_count.write().await;
        *view_count += 1;
        
        // Broadcast event
        let event = InteractionEvent::ViewCountIncremented {
            file_id: self.file_id,
            new_count: *view_count,
            timestamp: SystemTime::now(),
        };
        let _ = self.event_broadcaster.send(event);
    }

    /// Increment download count
    pub async fn increment_downloads(&self) {
        let mut download_count = self.download_count.write().await;
        *download_count += 1;
        
        // Broadcast event
        let event = InteractionEvent::DownloadCountIncremented {
            file_id: self.file_id,
            new_count: *download_count,
            timestamp: SystemTime::now(),
        };
        let _ = self.event_broadcaster.send(event);
    }

    /// Get interaction statistics
    pub async fn get_stats(&self) -> InteractionStats {
        let reactions = self.reactions.read().await;
        let comments = self.comments.read().await;
        let annotations = self.annotations.read().await;
        let view_count = *self.view_count.read().await;
        let download_count = *self.download_count.read().await;

        let total_reactions = reactions.values().map(|users| users.len() as u64).sum();
        let total_comments = comments.len() as u64;
        let total_annotations = annotations.len() as u64;

        let mut reaction_breakdown = HashMap::new();
        for (reaction_type, users) in reactions.iter() {
            reaction_breakdown.insert(*reaction_type, users.len() as u64);
        }

        InteractionStats {
            file_id: self.file_id,
            total_reactions,
            reaction_breakdown,
            total_comments,
            total_annotations,
            view_count,
            download_count,
        }
    }

    /// Get comments with pagination
    pub async fn get_comments(&self, offset: usize, limit: usize) -> Vec<MediaComment> {
        let comments = self.comments.read().await;
        let mut comment_list: Vec<MediaComment> = comments.values().cloned().collect();
        
        // Sort by timestamp (newest first)
        comment_list.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        comment_list.into_iter().skip(offset).take(limit).collect()
    }

    /// Get annotations
    pub async fn get_annotations(&self) -> Vec<MediaAnnotation> {
        let annotations = self.annotations.read().await;
        annotations.values().cloned().collect()
    }

    /// Subscribe to interaction events
    pub fn subscribe_to_events(&self) -> broadcast::Receiver<InteractionEvent> {
        self.event_broadcaster.subscribe()
    }
}

/// Interaction events for real-time updates
#[derive(Debug, Clone)]
pub enum InteractionEvent {
    ReactionAdded {
        file_id: FileId,
        reaction_type: ReactionType,
        user_id: UserId,
        timestamp: SystemTime,
    },
    ReactionRemoved {
        file_id: FileId,
        reaction_type: ReactionType,
        user_id: UserId,
        timestamp: SystemTime,
    },
    CommentAdded {
        file_id: FileId,
        comment_id: CommentId,
        author: UserId,
        parent_id: Option<CommentId>,
        timestamp: SystemTime,
    },
    CommentEdited {
        file_id: FileId,
        comment_id: CommentId,
        editor: UserId,
        timestamp: SystemTime,
    },
    CommentDeleted {
        file_id: FileId,
        comment_id: CommentId,
        deleter: UserId,
        timestamp: SystemTime,
    },
    AnnotationAdded {
        file_id: FileId,
        annotation_id: Uuid,
        author: UserId,
        timestamp: SystemTime,
    },
    AnnotationRemoved {
        file_id: FileId,
        annotation_id: Uuid,
        remover: UserId,
        timestamp: SystemTime,
    },
    ViewCountIncremented {
        file_id: FileId,
        new_count: u64,
        timestamp: SystemTime,
    },
    DownloadCountIncremented {
        file_id: FileId,
        new_count: u64,
        timestamp: SystemTime,
    },
}

/// Interaction statistics
#[derive(Debug, Clone, Serialize)]
pub struct InteractionStats {
    pub file_id: FileId,
    pub total_reactions: u64,
    pub reaction_breakdown: HashMap<ReactionType, u64>,
    pub total_comments: u64,
    pub total_annotations: u64,
    pub view_count: u64,
    pub download_count: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reaction_types() {
        let like = ReactionType::Like;
        assert_eq!(like.emoji(), "👍");
        
        let all_reactions = ReactionType::all();
        assert!(all_reactions.contains(&ReactionType::Like));
        assert!(all_reactions.contains(&ReactionType::Heart));
    }

    #[test]
    fn test_media_comment() {
        let file_id = FileId::new_v4();
        let author = "test_user".to_string();
        let content = "Great photo!".to_string();
        
        let mut comment = MediaComment::new(file_id, author.clone(), content.clone());
        assert_eq!(comment.author, author);
        assert_eq!(comment.content, content);
        assert!(!comment.is_edited);
        assert!(!comment.is_deleted);

        // Test editing
        comment.edit("Updated comment!".to_string()).unwrap();
        assert!(comment.is_edited);
        assert!(comment.edited_at.is_some());

        // Test reactions
        let user2 = "user2".to_string();
        assert!(comment.add_reaction(ReactionType::Like, user2.clone()));
        assert!(!comment.add_reaction(ReactionType::Like, user2.clone())); // Already exists
        assert_eq!(comment.reaction_count(ReactionType::Like), 1);
        assert!(comment.user_has_reaction(ReactionType::Like, &user2));
    }

    #[test]
    fn test_media_annotation() {
        let file_id = FileId::new_v4();
        let author = "test_user".to_string();
        let position = AnnotationPosition {
            x: 0.5,
            y: 0.5,
            width: 0.2,
            height: 0.1,
            rotation: 0.0,
            z_index: 1,
        };

        let annotation = MediaAnnotation::new_text(
            file_id,
            author.clone(),
            "This is important!".to_string(),
            position,
        );

        assert_eq!(annotation.author, author);
        assert!(matches!(annotation.annotation_type, AnnotationType::Text));
        assert!(matches!(annotation.content, AnnotationContent::Text(_)));
        assert!(annotation.is_visible);
    }

    #[tokio::test]
    async fn test_media_interactions() {
        let file_id = FileId::new_v4();
        let interactions = MediaInteractions::new(file_id);
        let user = "test_user".to_string();

        // Test reactions
        let added = interactions.add_reaction(ReactionType::Like, user.clone()).await.unwrap();
        assert!(added);

        let added_again = interactions.add_reaction(ReactionType::Like, user.clone()).await.unwrap();
        assert!(!added_again); // Already exists

        // Test comments
        let comment_id = interactions.add_comment(
            user.clone(),
            "Great content!".to_string(),
            None,
        ).await.unwrap();

        let comments = interactions.get_comments(0, 10).await;
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].comment_id, comment_id);

        // Test view counting
        interactions.increment_views().await;
        interactions.increment_downloads().await;

        let stats = interactions.get_stats().await;
        assert_eq!(stats.total_reactions, 1);
        assert_eq!(stats.total_comments, 1);
        assert_eq!(stats.view_count, 1);
        assert_eq!(stats.download_count, 1);
    }

    #[test]
    fn test_comment_thread() {
        let file_id = FileId::new_v4();
        let author = "test_user".to_string();
        let parent_comment = MediaComment::new(file_id, author.clone(), "Parent comment".to_string());
        let mut thread = CommentThread::new(parent_comment);

        let reply = MediaComment::new_reply(
            file_id,
            author.clone(),
            "Reply to parent".to_string(),
            thread.parent_comment.comment_id,
        );

        thread.add_reply(reply);
        assert_eq!(thread.total_replies, 1);
        assert_eq!(thread.replies.len(), 1);
    }

    #[test]
    fn test_annotation_position() {
        let position = AnnotationPosition {
            x: 0.25,
            y: 0.75,
            width: 0.5,
            height: 0.2,
            rotation: 45.0,
            z_index: 5,
        };

        // Test serialization
        let json = serde_json::to_string(&position).unwrap();
        let deserialized: AnnotationPosition = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.x, position.x);
        assert_eq!(deserialized.rotation, position.rotation);
    }
}
