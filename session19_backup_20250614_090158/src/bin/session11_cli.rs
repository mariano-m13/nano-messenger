/// Session 11 Command Line Tool
/// 
/// A command-line utility to test the chunked file transfer system with real files.
/// 
/// Usage:
///   cargo run --bin session11_cli upload <file_path> [chunk_size_mb]
///   cargo run --bin session11_cli test [quick|full|benchmark]
///   cargo run --bin session11_cli generate <size_mb> <output_path>

use nano_messenger::{
    media::chunking::{ChunkedTransfer, LargeFile, RetryStrategy},
    media::storage::LocalFileStorage,
    media::encryption::FileEncryption,
    crypto::{CryptoInterface, CryptoMode},
    error::Result,
};
use std::env;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;
use tempfile::TempDir;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }
    
    match args[1].as_str() {
        "upload" => {
            if args.len() < 3 {
                eprintln!("Error: Please provide a file path");
                print_usage();
                return Ok(());
            }
            let file_path = &args[2];
            let chunk_size_mb = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(5);
            upload_file(file_path, chunk_size_mb).await
        }
        "test" => {
            let test_type = args.get(2).map(|s| s.as_str()).unwrap_or("quick");
            run_tests(test_type).await
        }
        "generate" => {
            if args.len() < 4 {
                eprintln!("Error: Please provide size and output path");
                print_usage();
                return Ok(());
            }
            let size_mb: usize = args[2].parse().map_err(|_| {
                nano_messenger::error::NanoError::Media("Invalid size".to_string())
            })?;
            let output_path = &args[3];
            generate_test_file(size_mb, output_path).await
        }
        _ => {
            eprintln!("Error: Unknown command '{}'", args[1]);
            print_usage();
            Ok(())
        }
    }
}

fn print_usage() {
    println!("Session 11 CLI Tool - Large File Chunking System");
    println!("================================================");
    println!();
    println!("Usage:");
    println!("  {} upload <file_path> [chunk_size_mb]     Upload a file using chunked transfer", 
             env::args().next().unwrap_or_else(|| "session11_cli".to_string()));
    println!("  {} test [quick|full|benchmark]            Run test suite", 
             env::args().next().unwrap_or_else(|| "session11_cli".to_string()));
    println!("  {} generate <size_mb> <output_path>       Generate a test file", 
             env::args().next().unwrap_or_else(|| "session11_cli".to_string()));
    println!();
    println!("Examples:");
    println!("  {} upload large_video.mp4 10              Upload with 10MB chunks", 
             env::args().next().unwrap_or_else(|| "session11_cli".to_string()));
    println!("  {} test quick                             Run quick tests", 
             env::args().next().unwrap_or_else(|| "session11_cli".to_string()));
    println!("  {} generate 100 test_100mb.bin            Generate 100MB test file", 
             env::args().next().unwrap_or_else(|| "session11_cli".to_string()));
}

async fn upload_file(file_path: &str, chunk_size_mb: usize) -> Result<()> {
    println!("🚀 Session 11 CLI: Uploading File");
    println!("=================================");
    println!("📄 File: {}", file_path);
    println!("🧩 Chunk size: {}MB", chunk_size_mb);
    println!();

    // Check if file exists
    if !Path::new(file_path).exists() {
        return Err(nano_messenger::error::NanoError::Media(
            format!("File not found: {}", file_path)
        ));
    }

    // Read the file
    println!("📖 Reading file...");
    let content = fs::read(file_path).map_err(|e| {
        nano_messenger::error::NanoError::Media(format!("Failed to read file: {}", e))
    })?;

    let file_size_mb = content.len() as f64 / (1024.0 * 1024.0);
    println!("   Size: {:.2} MB ({} bytes)", file_size_mb, content.len());

    // Set up chunked transfer system
    println!("🔧 Setting up chunked transfer...");
    let temp_dir = TempDir::new().map_err(|e| {
        nano_messenger::error::NanoError::Media(format!("Failed to create temp directory: {}", e))
    })?;

    let storage = Arc::new(LocalFileStorage::new(temp_dir.path().to_path_buf()).await?);
    let encryption = Arc::new(FileEncryption::new(CryptoMode::Classical, chunk_size_mb as u64));
    
    let chunked_transfer = ChunkedTransfer::new(
        storage,
        encryption,
        chunk_size_mb * 1024 * 1024, // Convert to bytes
        4, // 4 parallel chunks
        RetryStrategy::default(),
    );

    // Generate keypairs
    let sender_keypair = CryptoInterface::generate_keypair()?;
    let recipient_keypair = CryptoInterface::generate_keypair()?;

    // Create LargeFile
    let large_file = LargeFile::new(
        Path::new(file_path).file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
        content,
        "application/octet-stream".to_string(),
        chunk_size_mb * 1024 * 1024,
    );

    println!("   Expected chunks: {}", large_file.total_chunks());
    println!();

    // Start upload with progress monitoring
    println!("📤 Starting chunked upload...");
    let start_time = Instant::now();
    
    let result = chunked_transfer.upload_large_file(
        large_file,
        &sender_keypair,
        &recipient_keypair.public_keys(),
    ).await?;

    let upload_time = start_time.elapsed();
    let throughput = (result.bytes_transferred as f64) / (1024.0 * 1024.0) / upload_time.as_secs_f64();

    // Display results
    println!("✅ Upload completed!");
    println!("   🆔 Upload ID: {}", result.upload_id);
    println!("   📊 Chunks uploaded: {}/{}", result.chunks_uploaded, result.total_chunks);
    println!("   📈 Bytes transferred: {} ({:.2} MB)", 
             result.bytes_transferred, result.bytes_transferred as f64 / (1024.0 * 1024.0));
    println!("   ⏱️  Upload time: {:?}", upload_time);
    println!("   🚀 Throughput: {:.2} MB/s", throughput);
    println!("   ✔️  Success: {}", result.success);

    if !result.success {
        if let Some(error) = &result.error_message {
            println!("   ❌ Error: {}", error);
        }
    }

    // Show storage statistics
    let stats = chunked_transfer.get_chunked_transfer_stats().await;
    println!();
    println!("📊 Storage Statistics:");
    println!("   Active uploads: {}", stats.active_uploads);
    println!("   Total chunks completed: {}", stats.total_chunks_completed);
    println!("   Total bytes stored: {:.2} MB", 
             stats.total_bytes_completed as f64 / (1024.0 * 1024.0));

    Ok(())
}

async fn run_tests(test_type: &str) -> Result<()> {
    println!("🧪 Session 11 CLI: Running Tests");
    println!("================================");
    println!("Test type: {}", test_type);
    println!();

    match test_type {
        "quick" => {
            println!("Running quick tests...");
            nano_messenger::test_session_11_basic().await?;
        }
        "full" => {
            println!("Running full test suite...");
            nano_messenger::test_session_11().await?;
        }
        "benchmark" => {
            println!("Running performance benchmarks...");
            // Set environment variable for large file tests
            std::env::set_var("RUN_LARGE_FILE_TESTS", "1");
            nano_messenger::benchmark_session_11().await?;
        }
        _ => {
            eprintln!("Unknown test type: {}. Use 'quick', 'full', or 'benchmark'", test_type);
            return Ok(());
        }
    }

    println!("✅ Tests completed successfully!");
    Ok(())
}

async fn generate_test_file(size_mb: usize, output_path: &str) -> Result<()> {
    println!("🔧 Session 11 CLI: Generating Test File");
    println!("=======================================");
    println!("Size: {}MB", size_mb);
    println!("Output: {}", output_path);
    println!();

    let size_bytes = size_mb * 1024 * 1024;
    let pattern = b"Session11TestData-ChunkedTransfer-NanoMessenger-";
    
    println!("📝 Generating {} bytes of test data...", size_bytes);
    let start_time = Instant::now();
    
    let mut content = Vec::with_capacity(size_bytes);
    for i in 0..size_bytes {
        content.push(pattern[i % pattern.len()]);
    }

    let generation_time = start_time.elapsed();
    println!("   Generated in {:?}", generation_time);

    println!("💾 Writing to file...");
    let write_start = Instant::now();
    
    fs::write(output_path, content).map_err(|e| {
        nano_messenger::error::NanoError::Media(format!("Failed to write file: {}", e))
    })?;

    let write_time = write_start.elapsed();
    let total_time = start_time.elapsed();
    
    println!("   Written in {:?}", write_time);
    println!("   Total time: {:?}", total_time);

    println!("✅ Test file generated successfully!");
    println!("   📄 File: {}", output_path);
    println!("   📊 Size: {}MB ({} bytes)", size_mb, size_bytes);

    // Verify the file
    if let Ok(metadata) = fs::metadata(output_path) {
        println!("   ✔️  Verified size: {} bytes", metadata.len());
    }

    Ok(())
}
