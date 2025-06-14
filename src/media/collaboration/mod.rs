/// Session 11: Collaborative Media Features
/// 
/// Provides shared media galleries, annotations, comments, and collaborative
/// media interactions with quantum-resistant encryption.

pub mod galleries;
pub mod interactions;

// Re-export main types
pub use galleries::{
    SharedGallery, GalleryPermissions, SharedGalleryKey, GalleryManager,
    GalleryInfo, GalleryStats, GalleryEvent, GalleryId
};
pub use interactions::{
    MediaInteractions, MediaComment, MediaAnnotation, ReactionType,
    CommentThread, InteractionStats
};
