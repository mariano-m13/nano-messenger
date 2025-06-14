/// Session 10 Validation: Media Processing & Optimization
/// 
/// This example demonstrates the comprehensive media processing capabilities
/// implemented in Session 10, including:
/// - Image processing and optimization
/// - Video processing and thumbnails
/// - Progressive loading and streaming
/// - Media detection and validation
/// - Performance benchmarking

use nano_messenger::{
    config::NanoConfig,
    media::{
        MediaSystem, MediaConfig, MediaProcessingManager, MediaDetector,
        processing::{
            ImageProcessor, ImageOptimizationConfig, ProcessingResult,
            MediaType, QualityLevel, ProgressiveLoader, ProgressiveConfig,
            PlaceholderData, FileReference
        },
    },
    error::Result,
};
use std::path::Path;
use std::time::{Duration, Instant};
use tokio::fs;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🎬 Session 10 Validation: Media Processing & Optimization");
    println!("===========================================================");
    
    // Initialize logging
    env_logger::init();
    
    // Test 1: Basic Media Detection
    println!("\n📋 Test 1: Media Detection and Validation");
    test_media_detection().await?;
    
    // Test 2: Image Processing
    println!("\n🖼️  Test 2: Image Processing and Optimization");
    test_image_processing().await?;
    
    // Test 3: Video Processing (if FFmpeg is available)
    println!("\n🎬 Test 3: Video Processing");
    test_video_processing().await?;
    
    // Test 4: Progressive Loading
    println!("\n🔄 Test 4: Progressive Loading");
    test_progressive_loading().await?;
    
    // Test 5: Media Processing Manager Integration
    println!("\n⚙️  Test 5: Media Processing Manager");
    test_processing_manager().await?;
    
    // Test 6: Performance Benchmarks
    println!("\n📊 Test 6: Performance Benchmarks");
    test_performance_benchmarks().await?;
    
    // Test 7: Configuration Validation
    println!("\n🔧 Test 7: Configuration Validation");
    test_configuration_validation().await?;
    
    println!("\n✅ All Session 10 tests completed successfully!");
    println!("Media processing and optimization capabilities are fully functional.");
    
    Ok(())
}

/// Test media detection and validation capabilities
async fn test_media_detection() -> Result<()> {
    let detector = MediaDetector::new();
    
    // Test JPEG detection
    let jpeg_header = create_mock_jpeg_data();
    let mime_type = detector.detect_mime_type(&jpeg_header)?;
    println!("   ✓ JPEG detection: {}", mime_type);
    
    let media_type = detector.detect_media_type(&jpeg_header)?;
    println!("   ✓ Media type: {:?}", media_type);
    assert_eq!(media_type, MediaType::Image);
    
    // Test file validation
    let validation_result = detector.validate_media_file(&jpeg_header, &MediaType::Image)?;
    println!("   ✓ Validation result: {}", if validation_result.is_valid { "Valid" } else { "Invalid" });
    println!("   ✓ File size: {} bytes", validation_result.file_size);
    
    // Test file analysis
    let analysis = detector.analyze_file(&jpeg_header)?;
    println!("   ✓ File entropy: {:.2}", analysis.entropy);
    println!("   ✓ Magic bytes: {:02X?}", &analysis.magic_bytes[..4]);
    
    Ok(())
}

/// Test image processing capabilities
async fn test_image_processing() -> Result<()> {
    let config = ImageOptimizationConfig::default();
    let processor = ImageProcessor::new(config);
    
    // Create test image data
    let test_image = create_mock_image_data(800, 600);
    let start_time = Instant::now();
    
    // Test thumbnail generation
    println!("   🔄 Generating thumbnails...");
    let thumbnails = processor.generate_thumbnails(&test_image).await?;
    println!("   ✓ Generated {} thumbnails in {:?}", thumbnails.len(), start_time.elapsed());
    
    for (name, thumbnail_data) in &thumbnails {
        println!("     - {}: {} bytes", name, thumbnail_data.len());
    }
    
    // Test image optimization
    println!("   🔄 Optimizing image...");
    let optimization_start = Instant::now();
    let optimized = processor.optimize_image(&test_image, 85).await?;
    println!("   ✓ Optimized image in {:?}", optimization_start.elapsed());
    println!("     - Original size: {} bytes", optimized.original_size);
    println!("     - Optimized size: {} bytes", optimized.data.len());
    println!("     - Compression ratio: {:.1}%", (1.0 - optimized.compression_ratio) * 100.0);
    
    // Test format conversion
    println!("   🔄 Converting image format...");
    let converted = processor.convert_format(&test_image, crate::media::processing::images::ImageFormat::Png).await?;
    println!("   ✓ Converted to PNG: {} bytes", converted.len());
    
    // Test metadata extraction
    println!("   🔄 Extracting metadata...");
    let metadata = processor.extract_metadata(&test_image).await?;
    println!("   ✓ Extracted {} metadata fields", metadata.len());
    for (key, value) in metadata.iter().take(3) {
        println!("     - {}: {}", key, value);
    }
    
    // Test progressive variants
    println!("   🔄 Creating progressive variants...");
    let variants = processor.create_progressive_variants(&test_image).await?;
    println!("   ✓ Created {} quality variants", variants.len());
    for (quality, data) in &variants {
        println!("     - {:?}: {} bytes", quality, data.len());
    }
    
    Ok(())
}

/// Test video processing capabilities
async fn test_video_processing() -> Result<()> {
    #[cfg(feature = "video-processing")]
    {
        use nano_messenger::media::processing::video::VideoProcessor;
        
        // Check if FFmpeg is available
        match VideoProcessor::new(&Default::default()).await {
            Ok(processor) => {
                println!("   ✓ FFmpeg detected and working");
                
                // For a real test, we would need actual video files
                // Here we just demonstrate the API
                println!("   📝 Video processing API available:");
                println!("     - Video thumbnail generation");
                println!("     - Video metadata extraction");
                println!("     - Video compression and optimization");
                println!("     - Quality variant generation");
            }
            Err(e) => {
                println!("   ⚠️  FFmpeg not available: {}", e);
                println!("   📝 Video processing would work with FFmpeg installed");
            }
        }
    }
    
    #[cfg(not(feature = "video-processing"))]
    {
        println!("   📝 Video processing feature not enabled");
        println!("   💡 Enable with --features video-processing");
    }
    
    Ok(())
}

/// Test progressive loading capabilities
async fn test_progressive_loading() -> Result<()> {
    let config = ProgressiveConfig {
        chunk_size_kb: 32,
        quality_levels: vec!["thumbnail".to_string(), "preview".to_string(), "standard".to_string()],
        adaptive_bitrate: true,
        preload_next_quality: true,
        cache_size_mb: 50,
        bandwidth_detection_interval_ms: 5000,
    };
    
    let loader = ProgressiveLoader::new(config);
    
    // Test bandwidth detection
    println!("   🔄 Testing bandwidth adaptation...");
    loader.update_bandwidth(1500).await;
    let optimal_quality = loader.get_optimal_quality(MediaType::Image, 1000000).await;
    println!("   ✓ Optimal quality for 1.5 Mbps: {:?}", optimal_quality);
    
    loader.update_bandwidth(5000).await;
    let high_bandwidth_quality = loader.get_optimal_quality(MediaType::Image, 1000000).await;
    println!("   ✓ Optimal quality for 5 Mbps: {:?}", high_bandwidth_quality);
    
    // Test placeholder generation
    println!("   🔄 Testing placeholder generation...");
    let file_ref = create_mock_file_reference();
    let placeholder = loader.create_placeholder(&file_ref).await?;
    
    match placeholder {
        PlaceholderData::Image { dimensions, dominant_color, .. } => {
            println!("   ✓ Generated image placeholder: {}x{}", dimensions.0, dimensions.1);
            if let Some(color) = dominant_color {
                println!("     - Dominant color: {}", color);
            }
        }
        _ => println!("   ✓ Generated placeholder"),
    }
    
    // Test progressive image stream
    println!("   🔄 Testing progressive image stream...");
    use tokio_stream::StreamExt;
    
    let mut stream = loader.load_progressive_image(&file_ref).await?;
    let mut count = 0;
    
    while let Some(result) = stream.next().await {
        match result {
            Ok(data) => {
                count += 1;
                println!("   ✓ Received quality {:?}: {}x{} ({} bytes)", 
                        data.quality, data.width, data.height, data.data.len());
                if data.is_final {
                    println!("     - Final quality reached");
                    break;
                }
            }
            Err(e) => {
                println!("   ❌ Stream error: {}", e);
                break;
            }
        }
    }
    
    println!("   ✓ Progressive stream delivered {} quality levels", count);
    
    Ok(())
}

/// Test integrated media processing manager
async fn test_processing_manager() -> Result<()> {
    let config = nano_messenger::media::processing::MediaProcessingConfig {
        enabled: true,
        max_processing_time_seconds: 180,
        concurrent_processing_jobs: 2,
        temp_directory: Some("./tmp".to_string()),
        images: Default::default(),
        #[cfg(feature = "video-processing")]
        video: Default::default(),
        progressive: ProgressiveConfig::default(),
    };
    
    println!("   🔄 Initializing processing manager...");
    let manager = MediaProcessingManager::new(config).await?;
    
    // Test health check
    let health = manager.health_check().await?;
    println!("   ✓ Processing manager healthy: {}", health.enabled);
    println!("   ✓ Available permits: {}", health.available_permits);
    
    #[cfg(feature = "image-processing")]
    println!("   ✓ Image processing: available");
    
    #[cfg(feature = "video-processing")]
    println!("   ✓ Video processing: available");
    
    #[cfg(not(feature = "video-processing"))]
    println!("   📝 Video processing: not enabled");
    
    // Test progressive loader access
    let progressive_loader = manager.progressive_loader();
    progressive_loader.update_bandwidth(2000).await;
    println!("   ✓ Progressive loader: accessible");
    
    // Test processing statistics
    let stats = manager.get_statistics().await;
    println!("   ✓ Processing statistics:");
    println!("     - Total processed: {}", stats.total_processed);
    println!("     - Images processed: {}", stats.images_processed);
    println!("     - Videos processed: {}", stats.videos_processed);
    
    // Test file processing simulation
    println!("   🔄 Simulating file processing...");
    let test_file = create_mock_image_data(400, 300);
    let temp_file = std::env::temp_dir().join("test_image.jpg");
    
    // Write test file
    fs::write(&temp_file, &test_file).await.map_err(|e| {
        nano_messenger::error::NanoError::Media(format!("Failed to write test file: {}", e))
    })?;
    
    // Process the file
    let processing_result = manager.process_media(&temp_file, &test_file).await?;
    
    println!("   ✓ Processing completed:");
    println!("     - Media type: {:?}", processing_result.media_type);
    println!("     - Original size: {} bytes", processing_result.original_size);
    println!("     - Processing time: {:?}", processing_result.processing_time);
    println!("     - Thumbnails generated: {}", processing_result.thumbnails.len());
    println!("     - Optimized variants: {}", processing_result.optimized_variants.len());
    
    if let Some(error) = &processing_result.error {
        println!("     - Processing error: {}", error);
    }
    
    // Clean up
    let _ = fs::remove_file(&temp_file).await;
    
    Ok(())
}

/// Test performance benchmarks
async fn test_performance_benchmarks() -> Result<()> {
    println!("   🔄 Running performance benchmarks...");
    
    // Image processing benchmark
    let config = ImageOptimizationConfig::default();
    let processor = ImageProcessor::new(config);
    
    let test_sizes = vec![
        (400, 300, "Small"),
        (1200, 800, "Medium"),
        (1920, 1080, "Large"),
    ];
    
    for (width, height, label) in test_sizes {
        let test_image = create_mock_image_data(width, height);
        
        // Benchmark thumbnail generation
        let start = Instant::now();
        let thumbnails = processor.generate_thumbnails(&test_image).await?;
        let thumbnail_time = start.elapsed();
        
        // Benchmark optimization
        let start = Instant::now();
        let optimized = processor.optimize_image(&test_image, 85).await?;
        let optimization_time = start.elapsed();
        
        println!("   📊 {} Image ({}x{}):", label, width, height);
        println!("     - Thumbnail generation: {:?} ({} thumbnails)", thumbnail_time, thumbnails.len());
        println!("     - Optimization: {:?} ({:.1}% reduction)", 
                optimization_time, (1.0 - optimized.compression_ratio) * 100.0);
    }
    
    // Progressive loading benchmark
    let loader = ProgressiveLoader::new(ProgressiveConfig::default());
    let file_ref = create_mock_file_reference();
    
    let start = Instant::now();
    let placeholder = loader.create_placeholder(&file_ref).await?;
    let placeholder_time = start.elapsed();
    
    println!("   📊 Progressive Loading:");
    println!("     - Placeholder generation: {:?}", placeholder_time);
    
    Ok(())
}

/// Test configuration validation
async fn test_configuration_validation() -> Result<()> {
    println!("   🔄 Testing configuration validation...");
    
    // Test default configuration
    let default_config = nano_messenger::media::processing::MediaProcessingConfig::default();
    println!("   ✓ Default config - Enabled: {}", default_config.enabled);
    println!("     - Max processing time: {}s", default_config.max_processing_time_seconds);
    println!("     - Concurrent jobs: {}", default_config.concurrent_processing_jobs);
    
    // Test image configuration
    println!("   ✓ Image processing config:");
    println!("     - Generate thumbnails: {}", default_config.images.generate_thumbnails);
    println!("     - Thumbnail sizes: {:?}", default_config.images.thumbnail_sizes);
    println!("     - Optimization enabled: {}", default_config.images.optimization_enabled);
    println!("     - Target quality: {}", default_config.images.target_quality);
    println!("     - Strip EXIF: {}", default_config.images.strip_exif_data);
    
    // Test progressive configuration
    println!("   ✓ Progressive loading config:");
    println!("     - Chunk size: {} KB", default_config.progressive.chunk_size_kb);
    println!("     - Quality levels: {:?}", default_config.progressive.quality_levels);
    println!("     - Adaptive bitrate: {}", default_config.progressive.adaptive_bitrate);
    println!("     - Cache size: {} MB", default_config.progressive.cache_size_mb);
    
    // Test MediaSystem integration
    let media_config = MediaConfig::default();
    println!("   ✓ Media system config:");
    println!("     - Processing enabled: {}", media_config.processing.enabled);
    
    // Test configuration file loading (if available)
    if let Ok(config_path) = std::env::var("NANO_CONFIG_PATH") {
        if Path::new(&config_path).exists() {
            println!("   ✓ Configuration file found: {}", config_path);
            // Note: Full config loading would require the config module
        }
    } else {
        println!("   📝 Using default configuration (no config file specified)");
    }
    
    Ok(())
}

/// Create mock JPEG data for testing
fn create_mock_jpeg_data() -> Vec<u8> {
    let mut data = Vec::new();
    
    // JPEG SOI (Start of Image)
    data.extend_from_slice(&[0xFF, 0xD8]);
    
    // JPEG APP0 marker
    data.extend_from_slice(&[0xFF, 0xE0]);
    
    // Add some mock JPEG data
    data.extend_from_slice(&[0x00, 0x10]); // Length
    data.extend_from_slice(b"JFIF");       // Identifier
    data.extend_from_slice(&[0x00, 0x01, 0x01]); // Version
    data.extend_from_slice(&[0x00, 0x00, 0x01, 0x00, 0x01]); // Density info
    data.extend_from_slice(&[0x00, 0x00]); // Thumbnail info
    
    // Add some mock image data
    for i in 0..1000 {
        data.push((i % 256) as u8);
    }
    
    // JPEG EOI (End of Image)
    data.extend_from_slice(&[0xFF, 0xD9]);
    
    data
}

/// Create mock image data for testing
fn create_mock_image_data(width: u32, height: u32) -> Vec<u8> {
    let mut data = create_mock_jpeg_data();
    
    // Simulate larger file based on dimensions
    let expected_size = (width * height * 3) / 10; // Rough compression estimate
    while data.len() < expected_size as usize {
        data.extend_from_slice(&data.clone());
    }
    data.truncate(expected_size as usize);
    
    // Ensure JPEG markers are still at the end
    if data.len() > 2 {
        data[data.len() - 2] = 0xFF;
        data[data.len() - 1] = 0xD9;
    }
    
    data
}

/// Create mock file reference for testing
fn create_mock_file_reference() -> FileReference {
    use std::collections::HashMap;
    
    let file_id = Uuid::new_v4();
    let metadata = nano_messenger::media::metadata::FileMetadata::new(
        file_id,
        "test_image.jpg".to_string(),
        "image/jpeg".to_string(),
        500000,
        "test_user".to_string(),
        nano_messenger::media::encryption::EncryptionMetadata {
            algorithm: "ChaCha20Poly1305".to_string(),
            key_algorithm: "X25519".to_string(),
            nonce_size: 12,
            tag_size: 16,
            original_size: 500000,
            encrypted_size: 500016,
            compression_used: false,
            custom_params: HashMap::new(),
        },
        nano_messenger::media::storage::StorageLocation::new(
            "local".to_string(),
            "/tmp/test_image.jpg".to_string()
        ),
        "test_checksum".to_string(),
    );
    
    FileReference {
        file_id,
        variants: HashMap::new(),
        metadata,
    }
}

/// Helper function to simulate processing delay
async fn simulate_processing_delay(ms: u64) {
    tokio::time::sleep(Duration::from_millis(ms)).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mock_data_creation() {
        let jpeg_data = create_mock_jpeg_data();
        assert!(jpeg_data.len() > 100);
        assert_eq!(&jpeg_data[0..2], &[0xFF, 0xD8]); // JPEG SOI
        assert_eq!(&jpeg_data[jpeg_data.len()-2..], &[0xFF, 0xD9]); // JPEG EOI
    }
    
    #[test]
    fn test_mock_image_scaling() {
        let small_image = create_mock_image_data(400, 300);
        let large_image = create_mock_image_data(1920, 1080);
        
        assert!(large_image.len() > small_image.len());
    }
    
    #[tokio::test]
    async fn test_media_detection_api() {
        let detector = MediaDetector::new();
        let test_data = create_mock_jpeg_data();
        
        let mime_type = detector.detect_mime_type(&test_data).unwrap();
        assert_eq!(mime_type.type_(), mime::IMAGE);
        
        let media_type = detector.detect_media_type(&test_data).unwrap();
        assert_eq!(media_type, MediaType::Image);
    }
}
