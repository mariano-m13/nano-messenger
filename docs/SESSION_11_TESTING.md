# Session 11 Testing Guide: Large File Chunking System

This guide provides comprehensive instructions for testing the Session 11 Large File Chunking System implementation.

## 🎯 What Session 11 Provides

Session 11 implements a **Large File Chunking System** with the following features:

- **Parallel chunk upload** with configurable concurrency
- **Quantum-resistant encryption** per chunk using the unified crypto system
- **Resume capability** for interrupted uploads (24-hour session expiration)
- **Progress tracking** and real-time statistics
- **Retry logic** with exponential backoff
- **Hash-based chunk verification** using Blake2b
- **Session management** with automatic cleanup
- **Storage abstraction** supporting local and S3 backends

## 🚀 Quick Test Methods

### Method 1: Run Automated Tests

```bash
# Quick basic tests (recommended)
cargo test test_session_11_basic

# Full test suite
cargo test test_session_11_comprehensive

# Performance benchmarks
RUN_LARGE_FILE_TESTS=1 cargo test benchmark_basic
```

### Method 2: Interactive Demo

```bash
# Run the interactive demo
cargo run --example session_11_demo
```

### Method 3: Command Line Tool

```bash
# Test with a real file
cargo run --bin session11_cli upload your_large_file.mp4 5

# Generate test file and upload it
cargo run --bin session11_cli generate 50 test_50mb.bin
cargo run --bin session11_cli upload test_50mb.bin 10

# Run built-in tests
cargo run --bin session11_cli test quick
cargo run --bin session11_cli test full
cargo run --bin session11_cli test benchmark
```

### Method 4: Programmatic Testing

```rust
use nano_messenger::{test_session_11, test_session_11_basic, benchmark_session_11};

#[tokio::main]
async fn main() {
    // Quick test
    test_session_11_basic().await.expect("Basic tests should pass");
    
    // Full test suite
    test_session_11().await.expect("Full tests should pass");
    
    // Performance benchmarks
    benchmark_session_11().await.expect("Benchmarks should complete");
}
```

## 📋 Test Scenarios Covered

### 1. **Basic Functionality Tests**
- ✅ File chunking and reconstruction
- ✅ Chunk encryption and decryption
- ✅ Parallel upload coordination
- ✅ Progress tracking accuracy
- ✅ Session state management

### 2. **Error Handling & Resilience**
- ✅ Retry logic with exponential backoff
- ✅ Failed chunk handling
- ✅ Network interruption simulation
- ✅ Invalid input handling
- ✅ Storage backend failures

### 3. **Resume Capability**
- ✅ Interrupted upload detection
- ✅ Progress restoration
- ✅ Completed upload recognition
- ✅ Expired session cleanup
- ✅ Non-existent session handling

### 4. **Concurrency & Performance**
- ✅ Multiple simultaneous uploads
- ✅ Thread safety verification
- ✅ Memory usage optimization
- ✅ Throughput measurement
- ✅ Scalability testing

### 5. **Security Features**
- ✅ Per-chunk encryption verification
- ✅ Integrity hash validation
- ✅ Key management correctness
- ✅ Cross-mode compatibility
- ✅ Quantum resistance validation

## 🔍 Detailed Test Scenarios

### Small File Test (1MB)
- **File size**: 1MB
- **Chunk size**: 256KB
- **Expected chunks**: 4
- **Parallel uploads**: 2
- **Tests**: Basic functionality, progress tracking

### Medium File Test (10MB)
- **File size**: 10MB
- **Chunk size**: 1MB
- **Expected chunks**: 10
- **Parallel uploads**: 4
- **Tests**: Hybrid crypto mode, error handling

### Large File Test (100MB - Optional)
- **File size**: 100MB
- **Chunk size**: 5MB
- **Expected chunks**: 20
- **Parallel uploads**: 8
- **Tests**: Performance benchmarking, stress testing

### Edge Case Test (7.3MB)
- **File size**: 7.3MB (uneven)
- **Chunk size**: 2MB
- **Expected chunks**: 4 (with partial last chunk)
- **Tests**: Boundary conditions, partial chunks

## 📊 Performance Expectations

### Typical Performance Metrics
- **Throughput**: > 0.1 MB/s (minimum acceptable)
- **Upload time**: < 60 seconds for test files
- **Memory usage**: Bounded by chunk size × parallel chunks
- **CPU usage**: Efficient encryption/hashing

### Benchmark Results Example
```
📊 Performance Results:
   File size: 10MB
   Chunk size: 1024KB
   Chunks: 10
   Upload time: 2.3s
   Throughput: 4.35 MB/s
   Parallel chunks: 4
```

## 🐛 Debugging Failed Tests

### Common Issues and Solutions

1. **"Storage directory creation failed"**
   - Ensure write permissions in temp directory
   - Check available disk space

2. **"Chunk upload timeout"**
   - Increase retry strategy timeouts
   - Check system resources

3. **"Encryption failed"**
   - Verify crypto dependencies are installed
   - Check key generation

4. **"Concurrent upload failures"**
   - Reduce parallel chunk count
   - Check system thread limits

### Debug Output
Enable detailed logging:
```bash
RUST_LOG=debug cargo test test_session_11_basic
```

### Manual Verification
Check that chunks are properly stored:
```bash
# After running tests, check temp directory
ls -la /tmp/session11_test_*/chunks/
```

## 📈 Success Criteria

### ✅ Test Passes If:
1. All chunks upload successfully
2. Progress tracking shows incremental updates
3. Resume functionality works correctly
4. Concurrent uploads complete without conflicts
5. Session cleanup removes expired sessions
6. Performance meets minimum thresholds
7. No memory leaks or resource exhaustion

### ❌ Test Fails If:
1. Any chunks fail to upload after retries
2. Progress tracking is inaccurate
3. Resume doesn't work for interrupted uploads
4. Concurrent uploads interfere with each other
5. Sessions don't expire properly
6. Performance is below acceptable thresholds
7. Memory usage grows unbounded

## 🔧 Custom Test Configuration

### Environment Variables
```bash
# Enable large file tests (100MB+)
export RUN_LARGE_FILE_TESTS=1

# Set custom chunk sizes
export CHUNK_SIZE_MB=5

# Set parallel chunk count
export PARALLEL_CHUNKS=8

# Enable debug logging
export RUST_LOG=debug
```

### Configuration Options
```rust
use nano_messenger::media::chunking::RetryStrategy;
use std::time::Duration;

// Custom retry strategy
let retry_strategy = RetryStrategy {
    max_retries: 5,
    initial_delay: Duration::from_millis(100),
    max_delay: Duration::from_secs(10),
    backoff_multiplier: 1.5,
};

// Custom chunked transfer setup
let chunked_transfer = ChunkedTransfer::new(
    storage,
    encryption,
    2 * 1024 * 1024, // 2MB chunks
    8,               // 8 parallel chunks
    retry_strategy,
);
```

## 🎉 Expected Output

When tests pass, you should see output like:

```
🧪 Running Session 11 Test Suite

✅ Upload completed: 10 chunks, 10MB in 2.3s
✅ Progress tracking working: 8 updates observed
✅ Resume correctly detected completed upload
✅ Error handling and retry logic functional
✅ Concurrent uploads completed in 3.1s
✅ Session cleanup working correctly
📊 Performance benchmark completed

🎉 Session 11 comprehensive testing completed successfully! 🎉
```

This confirms that the Large File Chunking System is working correctly and ready for production use.
