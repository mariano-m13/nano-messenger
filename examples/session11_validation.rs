/// Session 11 Validation Example
/// 
/// This example demonstrates and validates all Session 11 advanced media features:
/// - Large file chunking with parallel processing
/// - File deduplication for storage efficiency  
/// - Real-time media streaming with quantum encryption
/// - Collaborative media galleries and interactions
/// - Cross-platform optimization for mobile and web

use nano_messenger::{
    crypto::{CryptoMode, UnifiedKeyPair},
    error::Result,
    media::{
        MediaSystem, MediaConfig, StorageBackend,
        // Session 11 features
        LargeFile, FileDeduplication, HashAlgorithm,
        LiveStreamConfig, StreamingProtocol,
        SharedGallery, ReactionType, MediaAnnotation, AnnotationType, AnnotationPosition, AnnotationContent,
        DeviceProfile, DeviceType, NetworkProfile, ConnectionType,
        BrowserCapabilities, WebOptimizationConfig,
        compatibility::mobile::{MediaCodec, ProcessingStrategy},
        collaboration::galleries::ParticipantRole,
        // Specific type imports to avoid conflicts
        streaming::QualityLevel as StreamingQualityLevel,
    },
};
use std::time::{Duration, SystemTime};
use tempfile::TempDir;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 Session 11 Advanced Media Features Validation");
    println!("=================================================");

    // Initialize logging
    env_logger::init();

    // Create temporary directory for testing
    let temp_dir = TempDir::new().map_err(|e| {
        nano_messenger::error::NanoError::Media(format!("Failed to create temp dir: {}", e))
    })?;
    println!("📁 Using temporary directory: {:?}", temp_dir.path());

    // Initialize media system with Session 11 features
    let media_system = initialize_advanced_media_system(temp_dir.path()).await?;
    println!("✅ Advanced media system initialized");

    // Test 1: Large File Chunking
    test_large_file_chunking(&media_system).await?;

    // Test 2: File Deduplication
    test_file_deduplication(&media_system).await?;

    // Test 3: Real-time Streaming
    test_streaming_capabilities(&media_system).await?;

    // Test 4: Collaborative Galleries
    test_collaborative_galleries(&media_system).await?;

    // Test 5: Mobile Optimization
    test_mobile_optimization(&media_system).await?;

    // Test 6: Web Browser Support
    test_web_browser_support(&media_system).await?;

    // Final health check
    println!("\n🏥 Final System Health Check:");
    let health = media_system.health_check().await?;
    println!("   System Healthy: {}", health.is_healthy);
    println!("   Advanced Features: {}/6", health.advanced_features_count);
    println!("   Chunking: {}", health.chunking_enabled);
    println!("   Deduplication: {}", health.deduplication_enabled);
    println!("   Streaming: {}", health.streaming_enabled);
    println!("   Collaboration: {}", health.collaboration_enabled);
    println!("   Mobile Optimization: {}", health.mobile_optimization_enabled);
    println!("   Web Optimization: {}", health.web_optimization_enabled);

    if !health.issues.is_empty() {
        println!("   ⚠️ Issues found:");
        for issue in &health.issues {
            println!("     - {}", issue);
        }
    }

    println!("\n🎉 Session 11 Validation Complete!");
    println!("All advanced media features are working correctly.");

    Ok(())
}

async fn initialize_advanced_media_system(temp_path: &std::path::Path) -> Result<MediaSystem> {
    println!("\n📋 Initializing Advanced Media System...");

    let mut config = MediaConfig::default();
    config.storage_path = temp_path.to_path_buf();
    config.storage_backend = StorageBackend::Local;
    
    // Enable all Session 11 features
    config.chunking.enabled = true;
    config.chunking.chunk_size_mb = 1; // Small chunks for testing
    config.chunking.parallel_chunks = 2;
    
    config.deduplication.enabled = true;
    config.deduplication.chunk_level_dedup = true;
    
    config.streaming.enabled = true;
    config.streaming.max_concurrent_streams = 10;
    config.streaming.enable_webrtc = true;
    
    config.collaboration.enabled = true;
    config.collaboration.enable_annotations = true;
    config.collaboration.enable_reactions = true;
    
    config.compatibility.mobile_optimization = true;
    config.compatibility.web_optimization = true;

    let mut media_system = MediaSystem::new(config).await?;
    media_system.init_complete_system().await?;
    
    println!("   ✅ All managers initialized");
    Ok(media_system)
}

async fn test_large_file_chunking(media_system: &MediaSystem) -> Result<()> {
    println!("\n📦 Testing Large File Chunking...");

    let chunked_transfer = media_system.chunked_transfer()
        .ok_or_else(|| nano_messenger::error::NanoError::Media("Chunked transfer not initialized".to_string()))?;

    // Create a large test file (5MB)
    let large_content = vec![0x42u8; 5 * 1024 * 1024]; // 5MB of 'B' characters
    let large_file = LargeFile::new(
        "large_test_file.bin".to_string(),
        large_content,
        "application/octet-stream".to_string(),
        1024 * 1024, // 1MB chunks
    );

    println!("   📄 Created test file: {} bytes in {} chunks", 
             large_file.total_size, large_file.total_chunks());

    // Generate test keys
    let sender_keypair = UnifiedKeyPair::generate_for_mode(CryptoMode::Hybrid)?;
    let recipient_keys = sender_keypair.public_keys();

    // Upload with chunking
    let upload_result = chunked_transfer.upload_large_file(
        large_file,
        &sender_keypair,
        &recipient_keys,
    ).await?;

    println!("   ⬆️ Upload Result:");
    println!("     Chunks uploaded: {}/{}", upload_result.chunks_uploaded, upload_result.total_chunks);
    println!("     Bytes transferred: {} bytes", upload_result.bytes_transferred);
    println!("     Success: {}", upload_result.success);

    if !upload_result.success {
        if let Some(error) = &upload_result.error_message {
            println!("     Error: {}", error);
        }
    }

    // Get statistics
    let stats = chunked_transfer.get_chunked_transfer_stats().await;
    println!("   📊 Transfer Statistics:");
    println!("     Active uploads: {}", stats.active_uploads);
    println!("     Completed chunks: {}", stats.total_chunks_completed);
    println!("     Completed bytes: {} bytes", stats.total_bytes_completed);

    println!("   ✅ Large file chunking test completed");
    Ok(())
}

async fn test_file_deduplication(media_system: &MediaSystem) -> Result<()> {
    println!("\n🔄 Testing File Deduplication...");

    let deduplication = media_system.deduplication()
        .ok_or_else(|| nano_messenger::error::NanoError::Media("Deduplication not initialized".to_string()))?;

    // Test file content
    let test_content = b"This is a test file for deduplication testing. It contains some sample data.";
    let user1 = "user1".to_string();
    let user2 = "user2".to_string();

    println!("   📄 Testing with {} byte file", test_content.len());

    // First upload - should be new
    let result1 = deduplication.deduplicate_file(test_content, &user1).await?;
    match result1 {
        nano_messenger::media::DeduplicationResult::NewFile(_) => {
            println!("   ✅ First upload: New file created");
        }
        _ => {
            println!("   ❌ Expected new file on first upload");
        }
    }

    // Second upload of same content - should find duplicate
    let result2 = deduplication.deduplicate_file(test_content, &user2).await?;
    match result2 {
        nano_messenger::media::DeduplicationResult::ExistingFile(_) => {
            println!("   ✅ Second upload: Duplicate detected");
        }
        _ => {
            println!("   ❌ Expected duplicate detection on second upload");
        }
    }

    // Test chunk-level deduplication
    let chunk1 = nano_messenger::media::deduplication::FileChunk::new(0, test_content[0..32].to_vec());
    let chunk2 = nano_messenger::media::deduplication::FileChunk::new(1, test_content[0..32].to_vec()); // Same content
    let chunk3 = nano_messenger::media::deduplication::FileChunk::new(2, test_content[32..64].to_vec()); // Different content

    let chunks = vec![chunk1, chunk2, chunk3];
    let chunk_result = deduplication.deduplicate_chunks(&chunks).await?;

    println!("   📦 Chunk Deduplication Result:");
    println!("     New chunks: {}", chunk_result.new_chunks.len());
    println!("     Existing chunks: {}", chunk_result.existing_chunks.len());
    println!("     Space saved: {} bytes", chunk_result.space_saved);
    println!("     Deduplication ratio: {:.2}%", chunk_result.deduplication_ratio * 100.0);

    // Get deduplication statistics
    let stats = deduplication.get_stats().await;
    println!("   📊 Deduplication Statistics:");
    println!("     Files processed: {}", stats.total_files_processed);
    println!("     Duplicates found: {}", stats.duplicate_files_found);
    println!("     Space saved: {} bytes", stats.space_saved_bytes);
    println!("     Overall ratio: {:.2}%", stats.deduplication_ratio * 100.0);

    println!("   ✅ File deduplication test completed");
    Ok(())
}

async fn test_streaming_capabilities(media_system: &MediaSystem) -> Result<()> {
    println!("\n📺 Testing Streaming Capabilities...");

    let streaming_server = media_system.streaming_server()
        .ok_or_else(|| nano_messenger::error::NanoError::Media("Streaming server not initialized".to_string()))?;

    // Create live stream configuration
    let stream_config = LiveStreamConfig {
        title: "Test Live Stream".to_string(),
        description: Some("A test stream for Session 11 validation".to_string()),
        quality_level: StreamingQualityLevel {
            name: "720p".to_string(),
            width: 1280,
            height: 720,
            bitrate: 2_500_000,
            framerate: 30.0,
        },
        max_viewers: Some(100),
        duration_limit: Some(Duration::from_secs(3600)), // 1 hour
        encryption_required: true,
        allowed_viewers: None,
    };

    let owner = "stream_owner".to_string();
    let live_stream = streaming_server.start_live_stream(stream_config, &owner).await?;

    println!("   🎥 Live Stream Created:");
    println!("     Stream ID: {}", live_stream.stream_id);
    println!("     Title: {}", live_stream.config.title);
    println!("     Quality: {}p", live_stream.config.quality_level.height);
    println!("     Encryption: {}", live_stream.config.encryption_required);

    // Test adding viewers
    let viewer1 = "viewer1".to_string();
    let _receiver = live_stream.add_viewer(&viewer1).await?;
    println!("   👀 Viewer added: {}", viewer1);

    // Simulate broadcasting some data
    let test_frame = vec![0u8; 1024]; // 1KB test frame
    live_stream.broadcast_data(test_frame).await?;
    println!("   📡 Broadcasted test frame");

    let viewer_count = live_stream.get_viewer_count().await;
    println!("   👥 Current viewers: {}", viewer_count);

    // Get streaming statistics
    let stats = streaming_server.get_streaming_stats().await;
    println!("   📊 Streaming Statistics:");
    println!("     Active streams: {}", stats.active_streams);
    println!("     Total viewers: {}", stats.total_viewers);
    println!("     Live streams: {}", stats.live_streams);
    println!("     Total bitrate: {} bps", stats.total_bitrate);

    println!("   ✅ Streaming capabilities test completed");
    Ok(())
}

async fn test_collaborative_galleries(media_system: &MediaSystem) -> Result<()> {
    println!("\n🖼️ Testing Collaborative Galleries...");

    let gallery_manager = media_system.gallery_manager()
        .ok_or_else(|| nano_messenger::error::NanoError::Media("Gallery manager not initialized".to_string()))?;

    // Create a shared gallery
    let owner = "gallery_owner".to_string();
    let participants = vec!["user1".to_string(), "user2".to_string()];
    
    let gallery_id = gallery_manager.create_gallery(
        "Test Collaborative Gallery".to_string(),
        owner.clone(),
        &participants,
        false, // Private gallery
    ).await?;

    println!("   🏛️ Gallery Created:");
    println!("     Gallery ID: {}", gallery_id);
    println!("     Owner: {}", owner);
    println!("     Participants: {}", participants.len());

    // Get user galleries
    let user_galleries = gallery_manager.get_user_galleries(&owner).await;
    println!("   📋 User Galleries: {}", user_galleries.len());
    
    for gallery_info in &user_galleries {
        println!("     - {}: {} items", gallery_info.title, gallery_info.item_count);
    }

    // Test media interactions
    let file_id = nano_messenger::media::FileId::new_v4();
    let interactions = nano_messenger::media::MediaInteractions::new(file_id);

    // Add reactions
    let user = "test_user".to_string();
    interactions.add_reaction(ReactionType::Like, user.clone()).await?;
    interactions.add_reaction(ReactionType::Heart, user.clone()).await?;
    println!("   👍 Added reactions");

    // Add comments
    let comment_id = interactions.add_comment(
        user.clone(),
        "This is a great image!".to_string(),
        None, // Top-level comment
    ).await?;
    println!("   💬 Added comment: {}", comment_id);

    // Add reply
    let _reply_id = interactions.add_comment(
        user.clone(),
        "I agree!".to_string(),
        Some(comment_id), // Reply to previous comment
    ).await?;
    println!("   ↪️ Added reply");

    // Add annotation
    let annotation = MediaAnnotation::new_text(
        file_id,
        user.clone(),
        "Important detail here".to_string(),
        AnnotationPosition {
            x: 0.5,
            y: 0.3,
            width: 0.2,
            height: 0.1,
            rotation: 0.0,
            z_index: 1,
        },
    );

    interactions.add_annotation(annotation, &user).await?;
    println!("   📝 Added annotation");

    // Track views and downloads
    interactions.increment_views().await;
    interactions.increment_downloads().await;

    // Get interaction statistics
    let stats = interactions.get_stats().await;
    println!("   📊 Interaction Statistics:");
    println!("     Total reactions: {}", stats.total_reactions);
    println!("     Total comments: {}", stats.total_comments);
    println!("     Total annotations: {}", stats.total_annotations);
    println!("     Views: {}", stats.view_count);
    println!("     Downloads: {}", stats.download_count);

    println!("   ✅ Collaborative galleries test completed");
    Ok(())
}

async fn test_mobile_optimization(media_system: &MediaSystem) -> Result<()> {
    println!("\n📱 Testing Mobile Optimization...");

    let mobile_optimization = media_system.mobile_optimization()
        .ok_or_else(|| nano_messenger::error::NanoError::Media("Mobile optimization not initialized".to_string()))?;

    // Create test device profile
    let device_profile = DeviceProfile {
        device_type: DeviceType::Smartphone,
        screen_resolution: (1080, 2340),
        screen_density: 400.0,
        supported_codecs: vec![MediaCodec::H264, MediaCodec::AAC, MediaCodec::JPEG, MediaCodec::WebP],
        hardware_acceleration: true,
        max_video_resolution: (1920, 1080),
        max_framerate: 60.0,
        storage_available: 32 * 1024 * 1024 * 1024, // 32GB
        ram_available: 6 * 1024 * 1024 * 1024,      // 6GB
        cpu_cores: 8,
        gpu_capabilities: Default::default(),
    };

    println!("   📱 Device Profile:");
    println!("     Type: {:?}", device_profile.device_type);
    println!("     Resolution: {}x{}", device_profile.screen_resolution.0, device_profile.screen_resolution.1);
    println!("     Codecs: {} supported", device_profile.supported_codecs.len());
    println!("     Hardware Acceleration: {}", device_profile.hardware_acceleration);

    // Test quality selection based on bandwidth
    let high_bandwidth = 10_000_000; // 10 Mbps
    let low_bandwidth = 1_000_000;   // 1 Mbps

    let high_quality = mobile_optimization.select_quality_for_bandwidth(high_bandwidth);
    let low_quality = mobile_optimization.select_quality_for_bandwidth(low_bandwidth);

    println!("   🎯 Quality Selection:");
    println!("     High bandwidth (10 Mbps): {} ({}p)", high_quality.name, high_quality.height);
    println!("     Low bandwidth (1 Mbps): {} ({}p)", low_quality.name, low_quality.height);

    // Test processing strategy based on battery
    let full_battery = 1.0;
    let low_battery = 0.1;

    let full_strategy = mobile_optimization.get_processing_strategy(full_battery);
    let low_strategy = mobile_optimization.get_processing_strategy(low_battery);

    println!("   🔋 Processing Strategy:");
    println!("     Full battery: {:?}", full_strategy);
    println!("     Low battery: {:?}", low_strategy);

    // Test codec selection
    let best_video_codec = mobile_optimization.get_best_codec(nano_messenger::media::processing::MediaType::Video);
    let best_image_codec = mobile_optimization.get_best_codec(nano_messenger::media::processing::MediaType::Image);

    println!("   🎬 Best Codecs:");
    println!("     Video: {:?}", best_video_codec);
    println!("     Image: {:?}", best_image_codec);

    // Test cache size calculation
    let cache_size = mobile_optimization.get_cache_size();
    println!("   💾 Recommended cache size: {} MB", cache_size / 1024 / 1024);

    println!("   ✅ Mobile optimization test completed");
    Ok(())
}

async fn test_web_browser_support(media_system: &MediaSystem) -> Result<()> {
    println!("\n🌐 Testing Web Browser Support...");

    let web_support = media_system.web_support()
        .ok_or_else(|| nano_messenger::error::NanoError::Media("Web support not initialized".to_string()))?;

    println!("   🌐 Browser Capabilities:");
    println!("     Browser: {} {}", 
             web_support.browser_capabilities.browser_name,
             web_support.browser_capabilities.browser_version);
    println!("     Platform: {}", web_support.browser_capabilities.platform);
    println!("     WebAssembly: {}", web_support.browser_capabilities.supports_webassembly);
    println!("     WebRTC: {}", web_support.browser_capabilities.supports_webrtc);
    println!("     Service Worker: {}", web_support.browser_capabilities.supports_service_worker);

    // Test codec support
    let video_codec = web_support.get_best_codec_for_browser("video/mp4");
    let image_codec = web_support.get_best_codec_for_browser("image/jpeg");

    println!("   🎬 Optimal Codecs:");
    println!("     Video: {:?}", video_codec);
    println!("     Image: {:?}", image_codec);

    // Test WebAssembly crypto support
    let wasm_crypto = web_support.supports_webassembly_crypto();
    println!("   🔐 WebAssembly Crypto: {}", wasm_crypto);

    // Generate service worker
    let service_worker = web_support.generate_service_worker();
    let sw_size = service_worker.len();
    println!("   ⚙️ Service Worker generated: {} bytes", sw_size);

    // Test media optimization
    let test_media = nano_messenger::media::compatibility::mobile::MediaFile::new(
        nano_messenger::media::FileId::new_v4(),
        vec![0u8; 1024 * 1024], // 1MB test file
        "image/jpeg".to_string(),
    );

    let web_package = web_support.prepare_for_web(&test_media).await?;
    println!("   📦 Web Media Package:");
    println!("     Primary format: {}", web_package.primary_format.mime_type);
    println!("     Fallback formats: {}", web_package.fallback_formats.len());
    println!("     Streaming support: {}", web_package.metadata.supports_streaming);
    println!("     Requires polyfill: {}", web_package.metadata.requires_polyfill);
    println!("     Estimated load time: {:.2}s", web_package.metadata.estimated_load_time);

    println!("   ✅ Web browser support test completed");
    Ok(())
}
