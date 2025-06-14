/// Session 10: Media Processing & Optimization
/// 
/// This module provides comprehensive media processing capabilities including:
/// - Image processing: thumbnails, optimization, format conversion
/// - Video processing: compression, metadata extraction, thumbnails
/// - Media format detection and validation
/// - Progressive loading and streaming optimization

#[cfg(feature = "image-processing")]
pub mod images;

#[cfg(feature = "video-processing")]
pub mod video;

pub mod detection;
pub mod progressive;

// Re-export main types for easy access
#[cfg(feature = "image-processing")]
pub use images::{
    ImageProcessor, ThumbnailSet, OptimizedImage, ImageOptimizationConfig,
    ImageFormat, ImageQuality, ImageProcessingError
};

#[cfg(feature = "video-processing")]
pub use video::{
    VideoProcessor, VideoMetadata, VideoThumbnail, VideoProcessingError,
    VideoCodec, VideoQuality
};

pub use detection::{
    MediaDetector, MediaType, ExifData, MediaValidationResult, 
    DetectionError
};
pub use progressive::{
    ProgressiveLoader, ProgressiveConfig, QualityLevel, ProgressiveImageStream,
    VideoStream, PlaceholderData, FileReference
};

use crate::error::{NanoError, Result};
use crate::media::storage::FileId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;
use tokio::time::Instant;

/// Configuration for media processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaProcessingConfig {
    pub enabled: bool,
    pub max_processing_time_seconds: u64,
    pub concurrent_processing_jobs: usize,
    pub temp_directory: Option<String>,
    pub images: ImageProcessingConfig,
    #[cfg(feature = "video-processing")]
    pub video: VideoProcessingConfig,
    pub progressive: ProgressiveConfig,
}

impl Default for MediaProcessingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_processing_time_seconds: 300,
            concurrent_processing_jobs: 4,
            temp_directory: None,
            images: ImageProcessingConfig::default(),
            #[cfg(feature = "video-processing")]
            video: VideoProcessingConfig::default(),
            progressive: ProgressiveConfig::default(),
        }
    }
}

/// Configuration for image processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageProcessingConfig {
    pub generate_thumbnails: bool,
    pub thumbnail_sizes: Vec<u32>,
    pub optimization_enabled: bool,
    pub target_quality: u8,
    pub strip_exif_data: bool,
    pub supported_formats: Vec<String>,
    pub max_dimension: Option<u32>,
    pub progressive_jpeg: bool,
}

impl Default for ImageProcessingConfig {
    fn default() -> Self {
        Self {
            generate_thumbnails: true,
            thumbnail_sizes: vec![150, 300, 600],
            optimization_enabled: true,
            target_quality: 85,
            strip_exif_data: true,
            supported_formats: vec![
                "jpeg".to_string(),
                "png".to_string(),
                "webp".to_string(),
                "gif".to_string(),
            ],
            max_dimension: Some(4096),
            progressive_jpeg: true,
        }
    }
}

/// Configuration for video processing
#[cfg(feature = "video-processing")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoProcessingConfig {
    pub generate_thumbnails: bool,
    pub thumbnail_timestamp_seconds: f64,
    pub compression_enabled: bool,
    pub max_video_length_seconds: u64,
    pub supported_codecs: Vec<String>,
    pub target_bitrate_kbps: u32,
    pub max_resolution: (u32, u32),
}

#[cfg(feature = "video-processing")]
impl Default for VideoProcessingConfig {
    fn default() -> Self {
        Self {
            generate_thumbnails: true,
            thumbnail_timestamp_seconds: 5.0,
            compression_enabled: true,
            max_video_length_seconds: 1800, // 30 minutes
            supported_codecs: vec![
                "h264".to_string(),
                "h265".to_string(),
                "vp9".to_string(),
            ],
            target_bitrate_kbps: 2000,
            max_resolution: (1920, 1080),
        }
    }
}

/// Media processing statistics
#[derive(Debug, Clone, Serialize)]
pub struct ProcessingStatistics {
    pub total_processed: u64,
    pub images_processed: u64,
    pub videos_processed: u64,
    pub thumbnails_generated: u64,
    pub processing_errors: u64,
    pub average_processing_time_ms: u64,
    pub total_processing_time_ms: u64,
    pub size_reduction_ratio: f64,
}

impl Default for ProcessingStatistics {
    fn default() -> Self {
        Self {
            total_processed: 0,
            images_processed: 0,
            videos_processed: 0,
            thumbnails_generated: 0,
            processing_errors: 0,
            average_processing_time_ms: 0,
            total_processing_time_ms: 0,
            size_reduction_ratio: 0.0,
        }
    }
}

impl ProcessingStatistics {
    /// Update statistics with a new processing result
    pub fn update(&mut self, 
                  media_type: MediaType, 
                  processing_time: Duration, 
                  original_size: u64, 
                  processed_size: u64,
                  thumbnails_count: u32) {
        
        self.total_processed += 1;
        self.total_processing_time_ms += processing_time.as_millis() as u64;
        self.average_processing_time_ms = self.total_processing_time_ms / self.total_processed;
        
        // Update type-specific counters
        match media_type {
            MediaType::Image => self.images_processed += 1,
            MediaType::Video => self.videos_processed += 1,
            _ => {},
        }
        
        self.thumbnails_generated += thumbnails_count as u64;
        
        // Update size reduction ratio (running average)
        let current_ratio = if original_size > 0 {
            processed_size as f64 / original_size as f64
        } else {
            1.0
        };
        
        self.size_reduction_ratio = (self.size_reduction_ratio * (self.total_processed - 1) as f64 + current_ratio) / self.total_processed as f64;
    }
    
    /// Record a processing error
    pub fn record_error(&mut self) {
        self.processing_errors += 1;
    }
}

/// Media processing manager that coordinates all processing operations
pub struct MediaProcessingManager {
    config: MediaProcessingConfig,
    #[cfg(feature = "image-processing")]
    image_processor: Option<ImageProcessor>,
    #[cfg(feature = "video-processing")]
    video_processor: Option<VideoProcessor>,
    detector: MediaDetector,
    progressive_loader: ProgressiveLoader,
    statistics: tokio::sync::RwLock<ProcessingStatistics>,
    processing_semaphore: tokio::sync::Semaphore,
}

impl MediaProcessingManager {
    /// Create a new media processing manager
    pub async fn new(config: MediaProcessingConfig) -> Result<Self> {
        let processing_semaphore = tokio::sync::Semaphore::new(config.concurrent_processing_jobs);
        
        #[cfg(feature = "image-processing")]
        let image_processor = if config.enabled {
            Some(ImageProcessor::new(ImageOptimizationConfig::from(&config.images)))
        } else {
            None
        };
        
        #[cfg(feature = "video-processing")]
        let video_processor = if config.enabled {
            Some(VideoProcessor::new(&config.video).await?)
        } else {
            None
        };
        
        let progressive_loader = ProgressiveLoader::new(config.progressive.clone());
        
        Ok(Self {
            config,
            #[cfg(feature = "image-processing")]
            image_processor,
            #[cfg(feature = "video-processing")]
            video_processor,
            detector: MediaDetector::new(),
            progressive_loader,
            statistics: tokio::sync::RwLock::new(ProcessingStatistics::default()),
            processing_semaphore,
        })
    }
    
    /// Process media file and generate all required variants
    pub async fn process_media(&self, _file_path: &Path, content: &[u8]) -> Result<ProcessingResult> {
        let _permit = self.processing_semaphore.acquire().await.map_err(|e| {
            NanoError::Media(format!("Failed to acquire processing permit: {}", e))
        })?;
        
        let start_time = Instant::now();
        let original_size = content.len() as u64;
        
        // Detect media type
        let media_type = self.detector.detect_media_type(content)?;
        
        let mut result = ProcessingResult {
            media_type: media_type.clone(),
            original_size,
            processing_time: Duration::default(),
            thumbnails: Vec::new(),
            optimized_variants: HashMap::new(),
            metadata: None,
            error: None,
        };
        
        // Process based on media type
        match media_type {
            #[cfg(feature = "image-processing")]
            MediaType::Image => {
                if let Some(ref processor) = self.image_processor {
                    match self.process_image(processor, content).await {
                        Ok(image_result) => {
                            result.thumbnails = image_result.thumbnails;
                            result.optimized_variants = image_result.optimized_variants;
                            result.metadata = image_result.metadata;
                        }
                        Err(e) => {
                            result.error = Some(format!("Image processing failed: {}", e));
                            self.statistics.write().await.record_error();
                        }
                    }
                }
            }
            
            #[cfg(feature = "video-processing")]
            MediaType::Video => {
                if let Some(ref processor) = self.video_processor {
                    match self.process_video(processor, file_path, content).await {
                        Ok(video_result) => {
                            if let Some(thumbnail) = video_result.thumbnail {
                                result.thumbnails.push(("video_thumbnail".to_string(), thumbnail));
                            }
                            result.metadata = video_result.metadata;
                        }
                        Err(e) => {
                            result.error = Some(format!("Video processing failed: {}", e));
                            self.statistics.write().await.record_error();
                        }
                    }
                }
            }
            
            _ => {
                // For unsupported types, just detect basic metadata
                if let Ok(validation_result) = self.detector.validate_media_file(content, &media_type) {
                    result.metadata = Some(validation_result.metadata);
                }
            }
        }
        
        result.processing_time = start_time.elapsed();
        
        // Update statistics
        let processed_size = result.optimized_variants.values()
            .map(|v| v.len() as u64)
            .sum::<u64>()
            .max(original_size);
        
        self.statistics.write().await.update(
            media_type,
            result.processing_time,
            original_size,
            processed_size,
            result.thumbnails.len() as u32,
        );
        
        Ok(result)
    }
    
    /// Get processing statistics
    pub async fn get_statistics(&self) -> ProcessingStatistics {
        self.statistics.read().await.clone()
    }
    
    /// Get reference to progressive loader
    pub fn progressive_loader(&self) -> &ProgressiveLoader {
        &self.progressive_loader
    }
    
    /// Create progressive file reference
    pub async fn create_progressive_reference(&self, file_id: FileId, metadata: crate::media::metadata::FileMetadata) -> Result<FileReference> {
        Ok(FileReference {
            file_id,
            variants: std::collections::HashMap::new(), // Would be populated with actual variants
            metadata,
        })
    }
    
    /// Check if media processing is enabled and healthy
    pub async fn health_check(&self) -> Result<ProcessingHealthStatus> {
        Ok(ProcessingHealthStatus {
            enabled: self.config.enabled,
            #[cfg(feature = "image-processing")]
            image_processing_available: self.image_processor.is_some(),
            #[cfg(feature = "video-processing")]
            video_processing_available: self.video_processor.is_some(),
            concurrent_jobs_limit: self.config.concurrent_processing_jobs,
            available_permits: self.processing_semaphore.available_permits(),
            statistics: self.get_statistics().await,
        })
    }
    
    #[cfg(feature = "image-processing")]
    async fn process_image(&self, processor: &ImageProcessor, content: &[u8]) -> Result<ImageProcessingResult> {
        // Generate thumbnails
        let thumbnails = if self.config.images.generate_thumbnails {
            processor.generate_thumbnails(content).await?
        } else {
            Vec::new()
        };
        
        // Optimize image
        let mut optimized_variants = HashMap::new();
        if self.config.images.optimization_enabled {
            if let Ok(optimized) = processor.optimize_image(content, self.config.images.target_quality).await {
                optimized_variants.insert("optimized".to_string(), optimized.data);
            }
        }
        
        // Extract metadata (including EXIF)
        let metadata = processor.extract_metadata(content).await.ok();
        
        Ok(ImageProcessingResult {
            thumbnails,
            optimized_variants,
            metadata,
        })
    }
    
    #[cfg(feature = "video-processing")]
    async fn process_video(&self, processor: &VideoProcessor, file_path: &Path, content: &[u8]) -> Result<VideoProcessingResult> {
        // Generate thumbnail
        let thumbnail = if self.config.video.generate_thumbnails {
            processor.generate_video_thumbnail(file_path, self.config.video.thumbnail_timestamp_seconds).await.ok()
        } else {
            None
        };
        
        // Extract metadata
        let metadata = processor.extract_metadata(file_path).await.ok();
        
        Ok(VideoProcessingResult {
            thumbnail,
            metadata,
        })
    }
}

/// Result of media processing operations
#[derive(Debug, Clone)]
pub struct ProcessingResult {
    pub media_type: MediaType,
    pub original_size: u64,
    pub processing_time: Duration,
    pub thumbnails: Vec<(String, Vec<u8>)>,
    pub optimized_variants: HashMap<String, Vec<u8>>,
    pub metadata: Option<HashMap<String, String>>,
    pub error: Option<String>,
}

#[cfg(feature = "image-processing")]
struct ImageProcessingResult {
    thumbnails: Vec<(String, Vec<u8>)>,
    optimized_variants: HashMap<String, Vec<u8>>,
    metadata: Option<HashMap<String, String>>,
}

#[cfg(feature = "video-processing")]
struct VideoProcessingResult {
    thumbnail: Option<Vec<u8>>,
    metadata: Option<HashMap<String, String>>,
}

/// Health status of media processing subsystem
#[derive(Debug, Serialize)]
pub struct ProcessingHealthStatus {
    pub enabled: bool,
    #[cfg(feature = "image-processing")]
    pub image_processing_available: bool,
    #[cfg(feature = "video-processing")]
    pub video_processing_available: bool,
    pub concurrent_jobs_limit: usize,
    pub available_permits: usize,
    pub statistics: ProcessingStatistics,
}

/// Media processing errors
#[derive(Debug, thiserror::Error)]
pub enum MediaProcessingError {
    #[error("Processing timeout: exceeded {seconds} seconds")]
    Timeout { seconds: u64 },
    
    #[error("Unsupported media type: {media_type}")]
    UnsupportedMediaType { media_type: String },
    
    #[error("Processing failed: {reason}")]
    ProcessingFailed { reason: String },
    
    #[error("Resource limit exceeded: {limit}")]
    ResourceLimitExceeded { limit: String },
    
    #[error("Configuration error: {message}")]
    Configuration { message: String },
}

impl From<MediaProcessingError> for NanoError {
    fn from(err: MediaProcessingError) -> Self {
        NanoError::Media(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_processing_config_defaults() {
        let config = MediaProcessingConfig::default();
        assert!(config.enabled);
        assert_eq!(config.concurrent_processing_jobs, 4);
        assert!(config.images.generate_thumbnails);
        assert_eq!(config.images.thumbnail_sizes, vec![150, 300, 600]);
    }
    
    #[test]
    fn test_processing_statistics() {
        let mut stats = ProcessingStatistics::default();
        
        stats.update(
            MediaType::Image,
            Duration::from_millis(1000),
            1000,
            800,
            3
        );
        
        assert_eq!(stats.total_processed, 1);
        assert_eq!(stats.images_processed, 1);
        assert_eq!(stats.thumbnails_generated, 3);
        assert_eq!(stats.average_processing_time_ms, 1000);
        assert_eq!(stats.size_reduction_ratio, 0.8);
    }
    
    #[tokio::test]
    async fn test_media_processing_manager_creation() {
        let config = MediaProcessingConfig::default();
        let manager = MediaProcessingManager::new(config).await;
        assert!(manager.is_ok());
        
        let manager = manager.unwrap();
        let health = manager.health_check().await.unwrap();
        assert!(health.enabled);
    }
}
