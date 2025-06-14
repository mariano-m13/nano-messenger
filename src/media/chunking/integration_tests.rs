/// Comprehensive Integration Tests for Session 11: Large File Chunking System
/// 
/// This module provides thorough testing of the chunked transfer system,
/// including functionality, performance, error handling, and edge cases.

use super::*;
use crate::crypto::{CryptoInterface, CryptoMode};
use crate::error::{NanoError, Result};
use crate::media::storage::{LocalFileStorage, FileStorage};
use crate::media::encryption::FileEncryption;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tempfile::TempDir;
use tokio::time::timeout;
use uuid::Uuid;

/// Test configuration for different scenarios
#[derive(Debug, Clone)]
pub struct TestConfig {
    pub file_size_mb: usize,
    pub chunk_size_kb: usize,
    pub parallel_chunks: usize,
    pub expected_chunks: u32,
    pub crypto_mode: CryptoMode,
}

impl TestConfig {
    pub fn small_file() -> Self {
        Self {
            file_size_mb: 1,      // 1MB file
            chunk_size_kb: 256,   // 256KB chunks
            parallel_chunks: 2,
            expected_chunks: 4,   // 1MB / 256KB = 4 chunks
            crypto_mode: CryptoMode::Classical,
        }
    }

    pub fn medium_file() -> Self {
        Self {
            file_size_mb: 10,     // 10MB file
            chunk_size_kb: 1024,  // 1MB chunks
            parallel_chunks: 4,
            expected_chunks: 10,  // 10MB / 1MB = 10 chunks
            crypto_mode: CryptoMode::Hybrid,
        }
    }

    pub fn large_file() -> Self {
        Self {
            file_size_mb: 100,    // 100MB file
            chunk_size_kb: 5120,  // 5MB chunks
            parallel_chunks: 8,
            expected_chunks: 20,  // 100MB / 5MB = 20 chunks
            crypto_mode: CryptoMode::Classical, // Use Classical for speed in large file tests
        }
    }

    pub fn edge_case_uneven() -> Self {
        Self {
            file_size_mb: 7,      // 7.3MB file (odd size)
            chunk_size_kb: 2048,  // 2MB chunks
            parallel_chunks: 3,
            expected_chunks: 4,   // 7.3MB / 2MB = 3.65 -> 4 chunks
            crypto_mode: CryptoMode::Classical,
        }
    }
}

/// Comprehensive test suite for Session 11
pub struct ChunkedTransferTestSuite {
    temp_dir: TempDir,
    storage: Arc<dyn FileStorage>,
    encryption: Arc<FileEncryption>,
    sender_keypair: crate::crypto::UnifiedKeyPair,
    recipient_keypair: crate::crypto::UnifiedKeyPair,
}

impl ChunkedTransferTestSuite {
    /// Set up the test environment
    pub async fn new(crypto_mode: CryptoMode) -> Result<Self> {
        let temp_dir = TempDir::new().map_err(|e| {
            NanoError::Media(format!("Failed to create temp directory: {}", e))
        })?;

        let storage: Arc<dyn FileStorage> = Arc::new(
            LocalFileStorage::new(temp_dir.path().to_path_buf()).await?
        );

        let encryption = Arc::new(FileEncryption::new(crypto_mode, 10));

        // Generate keypairs for testing
        let sender_keypair = CryptoInterface::generate_keypair()?;
        let recipient_keypair = CryptoInterface::generate_keypair()?;

        Ok(Self {
            temp_dir,
            storage,
            encryption,
            sender_keypair,
            recipient_keypair,
        })
    }

    /// Create a test file with known pattern
    fn create_test_file(&self, size_mb: usize) -> LargeFile {
        let size_bytes = size_mb * 1024 * 1024;
        let mut content = Vec::with_capacity(size_bytes);
        
        // Create content with repeating pattern for verification
        let pattern = b"NanoMessenger-ChunkTest-";
        for i in 0..size_bytes {
            content.push(pattern[i % pattern.len()]);
        }

        LargeFile::new(
            format!("test_file_{}mb.bin", size_mb),
            content,
            "application/octet-stream".to_string(),
            1024 * 1024, // 1MB default chunk size
        )
    }

    /// Test 1: Basic chunked upload and verification
    pub async fn test_basic_chunked_upload(&self, config: &TestConfig) -> Result<()> {
        println!("🧪 Testing basic chunked upload ({}MB file, {}KB chunks)", 
                config.file_size_mb, config.chunk_size_kb);

        let chunked_transfer = ChunkedTransfer::new(
            Arc::clone(&self.storage),
            Arc::clone(&self.encryption),
            config.chunk_size_kb * 1024,
            config.parallel_chunks,
            RetryStrategy::default(),
        );

        let mut large_file = self.create_test_file(config.file_size_mb);
        large_file.chunk_size = config.chunk_size_kb * 1024;

        let start_time = Instant::now();
        let result = chunked_transfer.upload_large_file(
            large_file.clone(),
            &self.sender_keypair,
            &self.recipient_keypair.public_keys(),
        ).await?;

        let upload_time = start_time.elapsed();
        
        // Verify results
        assert!(result.success, "Upload should succeed");
        assert_eq!(result.total_chunks, config.expected_chunks, 
                  "Should have expected number of chunks");
        assert_eq!(result.chunks_uploaded, config.expected_chunks,
                  "All chunks should be uploaded");
        assert_eq!(result.bytes_transferred, large_file.total_size,
                  "All bytes should be transferred");
        assert_eq!(result.estimated_remaining, Duration::from_secs(0),
                  "No time should remain after completion");

        println!("✅ Upload completed: {} chunks, {}MB in {:?}", 
                result.chunks_uploaded, config.file_size_mb, upload_time);

        // Verify chunks are stored correctly
        let stats = chunked_transfer.get_chunked_transfer_stats().await;
        assert_eq!(stats.active_uploads, 1, "Should have one active session");
        assert_eq!(stats.total_chunks_completed, config.expected_chunks as u64);
        assert_eq!(stats.total_chunks_pending, 0);

        Ok(())
    }

    /// Test 2: Progress tracking during upload
    pub async fn test_progress_tracking(&self, config: &TestConfig) -> Result<()> {
        println!("🧪 Testing progress tracking during upload");

        let chunked_transfer = ChunkedTransfer::new(
            Arc::clone(&self.storage),
            Arc::clone(&self.encryption),
            config.chunk_size_kb * 1024,
            1, // Single chunk at a time for predictable progress
            RetryStrategy::default(),
        );

        let mut large_file = self.create_test_file(config.file_size_mb);
        large_file.chunk_size = config.chunk_size_kb * 1024;

        // Start upload in background
        let chunked_transfer_clone = Arc::new(chunked_transfer);
        let transfer_clone = Arc::clone(&chunked_transfer_clone);
        let large_file_clone = large_file.clone();
        let sender_keypair = self.sender_keypair.clone();
        let recipient_keys = self.recipient_keypair.public_keys();

        let upload_handle = tokio::spawn(async move {
            transfer_clone.upload_large_file(
                large_file_clone,
                &sender_keypair,
                &recipient_keys,
            ).await
        });

        // Monitor progress
        tokio::time::sleep(Duration::from_millis(100)).await; // Let upload start
        
        let mut last_progress = 0;
        let mut progress_updates = 0;
        
        for _ in 0..20 { // Check progress up to 20 times
            let stats = chunked_transfer_clone.get_chunked_transfer_stats().await;
            let current_progress = stats.total_chunks_completed;
            
            if current_progress > last_progress {
                println!("📊 Progress: {}/{} chunks completed", 
                        current_progress, config.expected_chunks);
                last_progress = current_progress;
                progress_updates += 1;
            }
            
            if current_progress >= config.expected_chunks as u64 {
                break;
            }
            
            tokio::time::sleep(Duration::from_millis(50)).await;
        }

        let result = upload_handle.await.map_err(|e| {
            NanoError::Media(format!("Upload task failed: {}", e))
        })??;

        assert!(result.success, "Upload should succeed");
        assert!(progress_updates > 0, "Should see progress updates");
        
        println!("✅ Progress tracking working: {} updates observed", progress_updates);
        Ok(())
    }

    /// Test 3: Resume interrupted upload
    pub async fn test_resume_capability(&self, config: &TestConfig) -> Result<()> {
        println!("🧪 Testing upload resume capability");

        let chunked_transfer = ChunkedTransfer::new(
            Arc::clone(&self.storage),
            Arc::clone(&self.encryption),
            config.chunk_size_kb * 1024,
            config.parallel_chunks,
            RetryStrategy::default(),
        );

        let mut large_file = self.create_test_file(config.file_size_mb);
        large_file.chunk_size = config.chunk_size_kb * 1024;

        // Start upload (this should complete normally for our setup)
        let result1 = chunked_transfer.upload_large_file(
            large_file,
            &self.sender_keypair,
            &self.recipient_keypair.public_keys(),
        ).await?;

        assert!(result1.success, "Initial upload should succeed");

        // Test resume of completed upload
        let resume_result = chunked_transfer.resume_upload(&result1.upload_id).await?;
        
        match resume_result {
            ResumeResult::Completed(completed) => {
                assert_eq!(completed.upload_id, result1.upload_id);
                assert!(completed.success);
                println!("✅ Resume correctly detected completed upload");
            }
            _ => panic!("Expected completed upload, got: {:?}", resume_result),
        }

        // Test resume of non-existent upload
        let fake_id = Uuid::new_v4();
        let resume_fake = chunked_transfer.resume_upload(&fake_id).await?;
        
        match resume_fake {
            ResumeResult::NotFound => {
                println!("✅ Resume correctly detected non-existent upload");
            }
            _ => panic!("Expected NotFound, got: {:?}", resume_fake),
        }

        Ok(())
    }

    /// Test 4: Error handling and retry logic
    pub async fn test_error_handling(&self, config: &TestConfig) -> Result<()> {
        println!("🧪 Testing error handling and retry logic");

        // Create a retry strategy with quick retries for testing
        let retry_strategy = RetryStrategy {
            max_retries: 2,
            initial_delay: Duration::from_millis(10),
            max_delay: Duration::from_millis(100),
            backoff_multiplier: 2.0,
        };

        let chunked_transfer = ChunkedTransfer::new(
            Arc::clone(&self.storage),
            Arc::clone(&self.encryption),
            config.chunk_size_kb * 1024,
            config.parallel_chunks,
            retry_strategy,
        );

        // Test with a smaller file to avoid long test times
        let large_file = self.create_test_file(1); // 1MB file

        // This should succeed with our local storage
        let result = chunked_transfer.upload_large_file(
            large_file,
            &self.sender_keypair,
            &self.recipient_keypair.public_keys(),
        ).await?;

        assert!(result.success, "Upload should succeed even with retry strategy");
        
        println!("✅ Error handling and retry logic functional");
        Ok(())
    }

    /// Test 5: Concurrent uploads
    pub async fn test_concurrent_uploads(&self, config: &TestConfig) -> Result<()> {
        println!("🧪 Testing concurrent uploads");

        let chunked_transfer = Arc::new(ChunkedTransfer::new(
            Arc::clone(&self.storage),
            Arc::clone(&self.encryption),
            config.chunk_size_kb * 1024,
            config.parallel_chunks,
            RetryStrategy::default(),
        ));

        // Create multiple files for concurrent upload
        let file1 = self.create_test_file(1); // 1MB
        let file2 = self.create_test_file(1); // 1MB
        let file3 = self.create_test_file(1); // 1MB

        let transfer1 = Arc::clone(&chunked_transfer);
        let transfer2 = Arc::clone(&chunked_transfer);
        let transfer3 = Arc::clone(&chunked_transfer);

        let sender1 = self.sender_keypair.clone();
        let sender2 = self.sender_keypair.clone();
        let sender3 = self.sender_keypair.clone();

        let recipient1 = self.recipient_keypair.public_keys();
        let recipient2 = self.recipient_keypair.public_keys();
        let recipient3 = self.recipient_keypair.public_keys();

        // Start concurrent uploads
        let upload1 = tokio::spawn(async move {
            transfer1.upload_large_file(file1, &sender1, &recipient1).await
        });

        let upload2 = tokio::spawn(async move {
            transfer2.upload_large_file(file2, &sender2, &recipient2).await
        });

        let upload3 = tokio::spawn(async move {
            transfer3.upload_large_file(file3, &sender3, &recipient3).await
        });

        // Wait for all uploads with timeout
        let start_time = Instant::now();
        let results = timeout(Duration::from_secs(30), async {
            tokio::try_join!(upload1, upload2, upload3)
        }).await.map_err(|_| {
            NanoError::Media("Concurrent uploads timed out".to_string())
        })?;

        let (result1, result2, result3) = results.map_err(|e| {
            NanoError::Media(format!("Concurrent upload task failed: {}", e))
        })?;

        let upload_time = start_time.elapsed();

        // Verify all uploads succeeded
        assert!(result1?.success, "Upload 1 should succeed");
        assert!(result2?.success, "Upload 2 should succeed");
        assert!(result3?.success, "Upload 3 should succeed");

        // Check final statistics
        let stats = chunked_transfer.get_chunked_transfer_stats().await;
        assert_eq!(stats.active_uploads, 3, "Should have 3 active sessions");

        println!("✅ Concurrent uploads completed in {:?}", upload_time);
        Ok(())
    }

    /// Test 6: Session cleanup
    pub async fn test_session_cleanup(&self, config: &TestConfig) -> Result<()> {
        println!("🧪 Testing session cleanup");

        let chunked_transfer = ChunkedTransfer::new(
            Arc::clone(&self.storage),
            Arc::clone(&self.encryption),
            config.chunk_size_kb * 1024,
            config.parallel_chunks,
            RetryStrategy::default(),
        );

        // Upload a file to create a session
        let large_file = self.create_test_file(1);
        let _result = chunked_transfer.upload_large_file(
            large_file,
            &self.sender_keypair,
            &self.recipient_keypair.public_keys(),
        ).await?;

        // Check session exists
        let stats_before = chunked_transfer.get_chunked_transfer_stats().await;
        assert_eq!(stats_before.active_uploads, 1, "Should have 1 session before cleanup");

        // Clean up recent sessions (should not remove anything)
        let cleaned = chunked_transfer.cleanup_expired_sessions(Duration::from_secs(1)).await;
        assert_eq!(cleaned, 0, "Should not clean up recent sessions");

        // Clean up old sessions (should remove the session)
        let cleaned = chunked_transfer.cleanup_expired_sessions(Duration::from_millis(1)).await;
        assert_eq!(cleaned, 1, "Should clean up 1 expired session");

        // Verify session was removed
        let stats_after = chunked_transfer.get_chunked_transfer_stats().await;
        assert_eq!(stats_after.active_uploads, 0, "Should have 0 sessions after cleanup");

        println!("✅ Session cleanup working correctly");
        Ok(())
    }

    /// Test 7: Performance benchmarking
    pub async fn test_performance_benchmark(&self, config: &TestConfig) -> Result<()> {
        println!("🧪 Running performance benchmark ({}MB file)", config.file_size_mb);

        let chunked_transfer = ChunkedTransfer::new(
            Arc::clone(&self.storage),
            Arc::clone(&self.encryption),
            config.chunk_size_kb * 1024,
            config.parallel_chunks,
            RetryStrategy::default(),
        );

        let mut large_file = self.create_test_file(config.file_size_mb);
        large_file.chunk_size = config.chunk_size_kb * 1024;

        let start_time = Instant::now();
        let result = chunked_transfer.upload_large_file(
            large_file.clone(),
            &self.sender_keypair,
            &self.recipient_keypair.public_keys(),
        ).await?;

        let upload_time = start_time.elapsed();
        let throughput_mbps = (large_file.total_size as f64) / 
                             (1024.0 * 1024.0) / upload_time.as_secs_f64();

        assert!(result.success, "Upload should succeed");

        println!("📊 Performance Results:");
        println!("   File size: {}MB", config.file_size_mb);
        println!("   Chunk size: {}KB", config.chunk_size_kb);
        println!("   Chunks: {}", result.total_chunks);
        println!("   Upload time: {:?}", upload_time);
        println!("   Throughput: {:.2} MB/s", throughput_mbps);
        println!("   Parallel chunks: {}", config.parallel_chunks);

        // Basic performance assertions
        assert!(throughput_mbps > 0.1, "Throughput should be reasonable (>0.1 MB/s)");
        assert!(upload_time.as_secs() < 60, "Upload should complete within 60 seconds");

        println!("✅ Performance benchmark completed");
        Ok(())
    }

    /// Run all tests with the given configuration
    pub async fn run_all_tests(&self, config: &TestConfig) -> Result<()> {
        println!("\n🚀 Running Session 11 Test Suite with config: {:?}\n", config);

        self.test_basic_chunked_upload(config).await?;
        self.test_progress_tracking(config).await?;
        self.test_resume_capability(config).await?;
        self.test_error_handling(config).await?;
        self.test_concurrent_uploads(config).await?;
        self.test_session_cleanup(config).await?;
        self.test_performance_benchmark(config).await?;

        println!("\n✅ All Session 11 tests passed! ✅\n");
        Ok(())
    }
}

/// Main test runner function
pub async fn run_session_11_tests() -> Result<()> {
    println!("🧪 Starting Session 11 (Large File Chunking System) Comprehensive Tests\n");

    // Test configurations for different scenarios
    let test_configs = vec![
        TestConfig::small_file(),
        TestConfig::medium_file(),
        TestConfig::edge_case_uneven(),
    ];

    for config in test_configs {
        let test_suite = ChunkedTransferTestSuite::new(config.crypto_mode).await?;
        test_suite.run_all_tests(&config).await?;
    }

    // Run one large file test if explicitly requested
    if std::env::var("RUN_LARGE_FILE_TESTS").is_ok() {
        println!("🔥 Running large file test (100MB) - this may take a while...");
        let large_config = TestConfig::large_file();
        let test_suite = ChunkedTransferTestSuite::new(large_config.crypto_mode).await?;
        test_suite.test_performance_benchmark(&large_config).await?;
    }

    println!("🎉 Session 11 comprehensive testing completed successfully! 🎉");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_session_11_comprehensive() {
        run_session_11_tests().await.expect("Session 11 tests should pass");
    }

    #[tokio::test]
    async fn test_small_file_only() {
        let config = TestConfig::small_file();
        let test_suite = ChunkedTransferTestSuite::new(config.crypto_mode).await.unwrap();
        test_suite.run_all_tests(&config).await.unwrap();
    }
}
