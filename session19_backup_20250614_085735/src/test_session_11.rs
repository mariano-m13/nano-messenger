/// Session 11 Test Runner
/// 
/// Easy-to-use test runner for the Large File Chunking System.
/// Run this to verify Session 11 is working correctly.

use crate::media::chunking::run_session_11_tests;
use crate::error::Result;

/// Quick test function to verify Session 11 is working
/// 
/// This runs a comprehensive test suite including:
/// - Basic chunked upload and verification
/// - Progress tracking during uploads
/// - Resume capability for interrupted uploads
/// - Error handling and retry logic
/// - Concurrent uploads
/// - Session cleanup
/// - Performance benchmarking
/// 
/// # Usage
/// 
/// ```rust,no_run
/// use nano_messenger::test_session_11;
/// 
/// #[tokio::main]
/// async fn main() {
///     match test_session_11().await {
///         Ok(()) => println!("✅ Session 11 is working correctly!"),
///         Err(e) => eprintln!("❌ Session 11 test failed: {}", e),
///     }
/// }
/// ```
pub async fn test_session_11() -> Result<()> {
    run_session_11_tests().await
}

/// Quick test for just basic functionality (faster)
/// 
/// This runs only the essential tests:
/// - Basic chunked upload with small file (1MB)
/// - Progress tracking
/// - Session cleanup
pub async fn test_session_11_basic() -> Result<()> {
    use crate::media::chunking::integration_tests::{ChunkedTransferTestSuite, TestConfig};
    
    println!("🧪 Running Session 11 Basic Test Suite\n");
    
    let config = TestConfig::small_file();
    let test_suite = ChunkedTransferTestSuite::new(config.crypto_mode).await?;
    
    // Run essential tests only
    test_suite.test_basic_chunked_upload(&config).await?;
    test_suite.test_progress_tracking(&config).await?;
    test_suite.test_session_cleanup(&config).await?;
    
    println!("✅ Session 11 basic tests passed!");
    Ok(())
}

/// Performance benchmark test
/// 
/// This runs performance tests with different file sizes to measure throughput
/// and upload times. Set RUN_LARGE_FILE_TESTS=1 to include 100MB file test.
pub async fn benchmark_session_11() -> Result<()> {
    use crate::media::chunking::integration_tests::{ChunkedTransferTestSuite, TestConfig};
    
    println!("📊 Running Session 11 Performance Benchmarks\n");
    
    let configs = vec![
        TestConfig::small_file(),   // 1MB
        TestConfig::medium_file(),  // 10MB
    ];
    
    for config in configs {
        let test_suite = ChunkedTransferTestSuite::new(config.crypto_mode).await?;
        test_suite.test_performance_benchmark(&config).await?;
    }
    
    // Large file test if requested
    if std::env::var("RUN_LARGE_FILE_TESTS").is_ok() {
        println!("🔥 Running large file benchmark (100MB)...");
        let large_config = TestConfig::large_file();
        let test_suite = ChunkedTransferTestSuite::new(large_config.crypto_mode).await?;
        test_suite.test_performance_benchmark(&large_config).await?;
    }
    
    println!("📊 Performance benchmarking completed!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_basic_session_11() {
        test_session_11_basic().await.expect("Basic Session 11 test should pass");
    }

    #[tokio::test] 
    async fn benchmark_basic() {
        // Only test small files in CI
        std::env::remove_var("RUN_LARGE_FILE_TESTS");
        benchmark_session_11().await.expect("Benchmark should complete");
    }
}
