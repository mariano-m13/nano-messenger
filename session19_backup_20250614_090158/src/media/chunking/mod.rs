/// Session 11: Large File Chunking System
/// 
/// This module provides efficient handling of large files through chunking,
/// parallel processing, and resume capabilities.

pub mod transfer;

pub mod integration_tests; // Available for CLI testing

// Re-export main types
pub use transfer::{
    ChunkedTransfer, LargeFile, ChunkedUploadResult, ResumeResult, 
    StreamingDownload, RetryStrategy, UploadId, ChunkInfo,
    ChunkedTransferStats
};

pub use integration_tests::run_session_11_tests;
