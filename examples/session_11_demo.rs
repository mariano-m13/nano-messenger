/// Session 11 Demo: Large File Chunking System
/// 
/// This example demonstrates how to use the Session 11 chunked file transfer system.
/// Run with: cargo run --example session_11_demo

use nano_messenger::{
    media::chunking::{ChunkedTransfer, LargeFile, RetryStrategy},
    media::storage::LocalFileStorage,
    media::encryption::FileEncryption,
    crypto::{CryptoInterface, CryptoMode},
    error::Result,
};
use std::sync::Arc;
use std::time::Instant;
use tempfile::TempDir;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    
    println!("🚀 Session 11 Demo: Large File Chunking System");
    println!("===============================================\n");

    // Set up the demo environment
    let demo = Session11Demo::new().await?;
    
    // Run different demo scenarios
    demo.demo_basic_upload().await?;
    demo.demo_large_file_upload().await?;
    demo.demo_parallel_uploads().await?;
    demo.demo_progress_monitoring().await?;
    
    println!("\n🎉 Demo completed successfully!");
    Ok(())
}

struct Session11Demo {
    chunked_transfer: ChunkedTransfer,
    sender_keypair: nano_messenger::crypto::UnifiedKeyPair,
    recipient_keypair: nano_messenger::crypto::UnifiedKeyPair,
    _temp_dir: TempDir, // Keep alive
}

impl Session11Demo {
    async fn new() -> Result<Self> {
        println!("🔧 Setting up demo environment...");
        
        // Create temporary directory for storage
        let temp_dir = TempDir::new().map_err(|e| {
            nano_messenger::error::NanoError::Media(format!("Failed to create temp directory: {}", e))
        })?;
        
        println!("   📁 Storage directory: {}", temp_dir.path().display());

        // Set up storage
        let storage = Arc::new(
            LocalFileStorage::new(temp_dir.path().to_path_buf()).await?
        );

        // Set up encryption  
        let encryption = Arc::new(FileEncryption::new(CryptoMode::Classical, 5)); // 5MB chunks

        // Create chunked transfer manager
        let chunked_transfer = ChunkedTransfer::new(
            storage,
            encryption,
            1024 * 1024, // 1MB chunk size
            4,           // 4 parallel chunks
            RetryStrategy::default(),
        );

        // Generate keypairs
        let sender_keypair = CryptoInterface::generate_keypair()?;
        let recipient_keypair = CryptoInterface::generate_keypair()?;

        println!("   🔐 Generated keypairs for demo");
        println!("   ⚙️  Chunk size: 1MB, Parallel chunks: 4\n");

        Ok(Self {
            chunked_transfer,
            sender_keypair,
            recipient_keypair,
            _temp_dir: temp_dir,
        })
    }

    async fn demo_basic_upload(&self) -> Result<()> {
        println!("📤 Demo 1: Basic Chunked Upload");
        println!("--------------------------------");

        // Create a test file (5MB)
        let content = self.create_test_content(5 * 1024 * 1024, b"Hello from Session 11! ");
        let large_file = LargeFile::new(
            "demo_file_5mb.txt".to_string(),
            content,
            "text/plain".to_string(),
            1024 * 1024, // 1MB chunks
        );

        println!("   📄 Created test file: {} ({} bytes)", 
                large_file.original_name, large_file.total_size);
        println!("   🧩 Expected chunks: {}", large_file.total_chunks());

        // Upload the file
        let start_time = Instant::now();
        let result = self.chunked_transfer.upload_large_file(
            large_file,
            &self.sender_keypair,
            &self.recipient_keypair.public_keys(),
        ).await?;

        let upload_time = start_time.elapsed();
        let throughput = (result.bytes_transferred as f64) / (1024.0 * 1024.0) / upload_time.as_secs_f64();

        println!("   ✅ Upload completed:");
        println!("      🆔 Upload ID: {}", result.upload_id);
        println!("      📊 Chunks: {}/{}", result.chunks_uploaded, result.total_chunks);
        println!("      📈 Bytes transferred: {}", result.bytes_transferred);
        println!("      ⏱️  Time: {:?}", upload_time);
        println!("      🚀 Throughput: {:.2} MB/s", throughput);
        println!("      ✔️  Success: {}\n", result.success);

        Ok(())
    }

    async fn demo_large_file_upload(&self) -> Result<()> {
        println!("📤 Demo 2: Large File Upload (20MB)");
        println!("-------------------------------------");

        // Create a larger test file
        let content = self.create_test_content(20 * 1024 * 1024, b"Large file chunk data - Session 11 rocks! ");
        let large_file = LargeFile::new(
            "demo_large_file_20mb.bin".to_string(),
            content,
            "application/octet-stream".to_string(),
            2 * 1024 * 1024, // 2MB chunks
        );

        println!("   📄 Created large file: {} ({} MB)", 
                large_file.original_name, large_file.total_size / (1024 * 1024));
        println!("   🧩 Expected chunks: {}", large_file.total_chunks());

        let start_time = Instant::now();
        let result = self.chunked_transfer.upload_large_file(
            large_file,
            &self.sender_keypair,
            &self.recipient_keypair.public_keys(),
        ).await?;

        let upload_time = start_time.elapsed();
        let throughput = (result.bytes_transferred as f64) / (1024.0 * 1024.0) / upload_time.as_secs_f64();

        println!("   ✅ Large file upload completed:");
        println!("      📊 Chunks: {}/{}", result.chunks_uploaded, result.total_chunks);
        println!("      📈 Size: {:.1} MB", result.bytes_transferred as f64 / (1024.0 * 1024.0));
        println!("      ⏱️  Time: {:?}", upload_time);
        println!("      🚀 Throughput: {:.2} MB/s", throughput);
        println!("      ✔️  Success: {}\n", result.success);

        Ok(())
    }

    async fn demo_parallel_uploads(&self) -> Result<()> {
        println!("🔄 Demo 3: Parallel Uploads");
        println!("----------------------------");

        // Create multiple smaller files
        let file1 = LargeFile::new(
            "parallel_file_1.txt".to_string(),
            self.create_test_content(3 * 1024 * 1024, b"File 1 data "),
            "text/plain".to_string(),
            1024 * 1024,
        );

        let file2 = LargeFile::new(
            "parallel_file_2.txt".to_string(),
            self.create_test_content(3 * 1024 * 1024, b"File 2 data "),
            "text/plain".to_string(),
            1024 * 1024,
        );

        let file3 = LargeFile::new(
            "parallel_file_3.txt".to_string(),
            self.create_test_content(3 * 1024 * 1024, b"File 3 data "),
            "text/plain".to_string(),
            1024 * 1024,
        );

        println!("   📄 Created 3 files for parallel upload (3MB each)");

        // Start parallel uploads
        let start_time = Instant::now();
        
        let upload1 = self.chunked_transfer.upload_large_file(
            file1, &self.sender_keypair, &self.recipient_keypair.public_keys()
        );
        
        let upload2 = self.chunked_transfer.upload_large_file(
            file2, &self.sender_keypair, &self.recipient_keypair.public_keys()
        );
        
        let upload3 = self.chunked_transfer.upload_large_file(
            file3, &self.sender_keypair, &self.recipient_keypair.public_keys()
        );

        // Wait for all to complete
        let (result1, result2, result3) = tokio::try_join!(upload1, upload2, upload3)?;
        
        let total_time = start_time.elapsed();
        let total_bytes = result1.bytes_transferred + result2.bytes_transferred + result3.bytes_transferred;
        let total_throughput = (total_bytes as f64) / (1024.0 * 1024.0) / total_time.as_secs_f64();

        println!("   ✅ Parallel uploads completed:");
        println!("      📊 File 1: {} chunks", result1.chunks_uploaded);
        println!("      📊 File 2: {} chunks", result2.chunks_uploaded);
        println!("      📊 File 3: {} chunks", result3.chunks_uploaded);
        println!("      📈 Total size: {:.1} MB", total_bytes as f64 / (1024.0 * 1024.0));
        println!("      ⏱️  Total time: {:?}", total_time);
        println!("      🚀 Combined throughput: {:.2} MB/s", total_throughput);
        println!("      ✔️  All successful: {}\n", 
                result1.success && result2.success && result3.success);

        Ok(())
    }

    async fn demo_progress_monitoring(&self) -> Result<()> {
        println!("📊 Demo 4: Progress Monitoring");
        println!("-------------------------------");

        // Create a file that will take some time to upload
        let content = self.create_test_content(10 * 1024 * 1024, b"Progress monitoring test ");
        let large_file = LargeFile::new(
            "progress_test_10mb.bin".to_string(),
            content,
            "application/octet-stream".to_string(),
            1024 * 1024, // 1MB chunks
        );

        println!("   📄 Created file for progress monitoring: {} MB", 
                large_file.total_size / (1024 * 1024));

        // Start upload in background
        let chunked_transfer = &self.chunked_transfer;
        let upload_future = chunked_transfer.upload_large_file(
            large_file,
            &self.sender_keypair,
            &self.recipient_keypair.public_keys(),
        );

        // Monitor progress (this is a simplified version since we can't easily
        // track the specific upload ID without modifying the API)
        let start_time = Instant::now();
        let result = upload_future.await?;
        let upload_time = start_time.elapsed();

        // Show final statistics
        let stats = chunked_transfer.get_chunked_transfer_stats().await;
        
        println!("   📈 Final Statistics:");
        println!("      🎯 Active uploads: {}", stats.active_uploads);
        println!("      ✅ Completed chunks: {}", stats.total_chunks_completed);
        println!("      ⏳ Pending chunks: {}", stats.total_chunks_pending);
        println!("      📊 Bytes completed: {:.1} MB", 
                stats.total_bytes_completed as f64 / (1024.0 * 1024.0));
        println!("      ⏱️  Upload time: {:?}", upload_time);
        println!("      ✔️  Upload success: {}\n", result.success);

        Ok(())
    }

    /// Helper function to create test content with a repeating pattern
    fn create_test_content(&self, size: usize, pattern: &[u8]) -> Vec<u8> {
        let mut content = Vec::with_capacity(size);
        for i in 0..size {
            content.push(pattern[i % pattern.len()]);
        }
        content
    }
}

// Conditional compilation for the example
#[cfg(not(test))]
fn main() {
    println!("Run this example with: cargo run --example session_11_demo");
}
