//! Image Processing Module
//! 
//! Provides comprehensive image processing capabilities including:
//! - Thumbnail generation in multiple sizes
//! - Image optimization and compression
//! - Format conversion between supported formats
//! - EXIF data extraction and manipulation
//! - Progressive loading support

#![cfg(feature = "image-processing")]

use crate::error::{NanoError, Result};
use image::{
    ImageFormat as ImageLibFormat, 
    ImageError, imageops::FilterType, GenericImageView,
    codecs::jpeg::JpegEncoder,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Cursor;
use std::time::Instant;
use tokio::task;
use log;

/// Image processor for handling all image operations
pub struct ImageProcessor {
    config: ImageOptimizationConfig,
}

impl ImageProcessor {
    /// Create a new image processor
    pub fn new(config: ImageOptimizationConfig) -> Self {
        Self { config }
    }
    
    /// Generate thumbnails with multiple sizes
    pub async fn generate_thumbnails(&self, image_data: &[u8]) -> Result<Vec<(String, Vec<u8>)>> {
        let image_data = image_data.to_vec();
        let config = self.config.clone();
        
        task::spawn_blocking(move || {
            let start_time = Instant::now();
            
            // Load image
            let img = image::load_from_memory(&image_data)
                .map_err(|e| ImageProcessingError::ProcessingFailed { 
                    reason: format!("Failed to load image: {}", e) 
                })?;
            
            let mut thumbnails = Vec::new();
            
            // Generate thumbnails for each configured size
            for &size in &config.thumbnail_sizes {
                let thumbnail_name = format!("thumb_{}x{}", size, size);
                
                // Calculate dimensions maintaining aspect ratio
                let (thumb_width, thumb_height) = calculate_thumbnail_size(
                    img.width(), 
                    img.height(), 
                    size
                );
                
                // Resize image
                let thumbnail = img.resize(
                    thumb_width, 
                    thumb_height, 
                    FilterType::Lanczos3
                );
                
                // Encode to JPEG with quality setting
                let mut thumb_buffer = Vec::new();
                let mut cursor = Cursor::new(&mut thumb_buffer);
                
                thumbnail.write_to(&mut cursor, ImageLibFormat::Jpeg)
                    .map_err(|e| ImageProcessingError::ProcessingFailed { 
                        reason: format!("Failed to encode thumbnail: {}", e) 
                    })?;
                
                thumbnails.push((thumbnail_name, thumb_buffer));
            }
            
            log::debug!(
                "Generated {} thumbnails in {}ms", 
                thumbnails.len(), 
                start_time.elapsed().as_millis()
            );
            
            Ok(thumbnails)
        })
        .await
        .map_err(|e| NanoError::Media(format!("Thumbnail generation task failed: {}", e)))?
    }
    
    /// Optimize image for bandwidth/storage
    pub async fn optimize_image(&self, image_data: &[u8], target_quality: u8) -> Result<OptimizedImage> {
        let image_data = image_data.to_vec();
        let config = self.config.clone();
        
        task::spawn_blocking(move || {
            let start_time = Instant::now();
            let original_size = image_data.len();
            
            // Load image
            let img = image::load_from_memory(&image_data)
                .map_err(|e| ImageProcessingError::ProcessingFailed { 
                    reason: format!("Failed to load image: {}", e) 
                })?;
            
            let (width, height) = img.dimensions();
            
            // Apply max dimension limit if configured
            let img = if let Some(max_dim) = config.max_dimension {
                if width > max_dim || height > max_dim {
                    let scale = (max_dim as f32) / width.max(height) as f32;
                    let new_width = (width as f32 * scale) as u32;
                    let new_height = (height as f32 * scale) as u32;
                    
                    img.resize(new_width, new_height, FilterType::Lanczos3)
                } else {
                    img
                }
            } else {
                img
            };
            
            // Optimize based on format
            let mut optimized_data = Vec::new();
            let mut cursor = Cursor::new(&mut optimized_data);
            
            // Use JPEG with custom quality for optimization
            let mut encoder = JpegEncoder::new_with_quality(&mut cursor, target_quality);
            
            // Convert to RGB if necessary (JPEG doesn't support transparency)
            let rgb_img = img.to_rgb8();
            encoder.encode(
                rgb_img.as_raw(),
                img.width(),
                img.height(),
                image::ColorType::Rgb8,
            ).map_err(|e| ImageProcessingError::ProcessingFailed { 
                reason: format!("Failed to encode optimized image: {}", e) 
            })?;
            
            let processing_time = start_time.elapsed();
            let compression_ratio = optimized_data.len() as f64 / original_size as f64;
            
            log::debug!(
                "Optimized image: {}x{}, {}KB -> {}KB ({:.1}% reduction) in {}ms",
                img.width(),
                img.height(),
                original_size / 1024,
                optimized_data.len() / 1024,
                (1.0 - compression_ratio) * 100.0,
                processing_time.as_millis()
            );
            
            Ok(OptimizedImage {
                data: optimized_data,
                width: img.width(),
                height: img.height(),
                format: ImageFormat::Jpeg,
                quality: Some(target_quality),
                original_size,
                compression_ratio,
                processing_time,
            })
        })
        .await
        .map_err(|e| NanoError::Media(format!("Image optimization task failed: {}", e)))?
    }
    
    /// Convert image between formats
    pub async fn convert_format(&self, image_data: &[u8], target_format: ImageFormat) -> Result<Vec<u8>> {
        let image_data = image_data.to_vec();
        
        task::spawn_blocking(move || {
            let img = image::load_from_memory(&image_data)
                .map_err(|e| ImageProcessingError::ProcessingFailed { 
                    reason: format!("Failed to load image: {}", e) 
                })?;
            
            let mut output = Vec::new();
            let mut cursor = Cursor::new(&mut output);
            
            let lib_format = match target_format {
                ImageFormat::Jpeg => ImageLibFormat::Jpeg,
                ImageFormat::Png => ImageLibFormat::Png,
                ImageFormat::WebP => ImageLibFormat::WebP,
                ImageFormat::Gif => ImageLibFormat::Gif,
                ImageFormat::Tiff => ImageLibFormat::Tiff,
            };
            
            img.write_to(&mut cursor, lib_format)
                .map_err(|e| ImageProcessingError::ProcessingFailed { 
                    reason: format!("Failed to convert image format: {}", e) 
                })?;
            
            Ok(output)
        })
        .await
        .map_err(|e| NanoError::Media(format!("Image conversion task failed: {}", e)))?
    }
    
    /// Extract image metadata including EXIF data
    pub async fn extract_metadata(&self, image_data: &[u8]) -> Result<HashMap<String, String>> {
        let image_data = image_data.to_vec();
        let strip_exif = self.config.strip_exif_data;
        
        task::spawn_blocking(move || {
            let mut metadata = HashMap::new();
            
            // Load image to get basic dimensions
            if let Ok(img) = image::load_from_memory(&image_data) {
                metadata.insert("width".to_string(), img.width().to_string());
                metadata.insert("height".to_string(), img.height().to_string());
                metadata.insert("format".to_string(), format!("{:?}", img.color()));
            }
            
            // Extract EXIF data if not configured to strip it
            if !strip_exif {
                if let Ok(exif_data) = extract_exif_data(&image_data) {
                    for (key, value) in exif_data {
                        metadata.insert(format!("exif_{}", key), value);
                    }
                }
            }
            
            metadata.insert("file_size".to_string(), image_data.len().to_string());
            metadata.insert("processed_at".to_string(), chrono::Utc::now().to_rfc3339());
            
            Ok(metadata)
        })
        .await
        .map_err(|e| NanoError::Media(format!("Metadata extraction task failed: {}", e)))?
    }
    
    /// Create progressive JPEG variant
    pub async fn create_progressive_variants(&self, image_data: &[u8]) -> Result<Vec<(ImageQuality, Vec<u8>)>> {
        let image_data = image_data.to_vec();
        
        task::spawn_blocking(move || {
            let img = image::load_from_memory(&image_data)
                .map_err(|e| ImageProcessingError::ProcessingFailed { 
                    reason: format!("Failed to load image: {}", e) 
                })?;
            
            let mut variants = Vec::new();
            let qualities = [
                (ImageQuality::Thumbnail, 40),
                (ImageQuality::Preview, 65),
                (ImageQuality::Standard, 85),
                (ImageQuality::HighQuality, 95),
            ];
            
            for (quality_level, quality_value) in qualities {
                let mut output = Vec::new();
                let mut cursor = Cursor::new(&mut output);
                
                let mut encoder = JpegEncoder::new_with_quality(&mut cursor, quality_value);
                let rgb_img = img.to_rgb8();
                
                encoder.encode(
                    rgb_img.as_raw(),
                    img.width(),
                    img.height(),
                    image::ColorType::Rgb8,
                ).map_err(|e| ImageProcessingError::ProcessingFailed { 
                    reason: format!("Failed to encode progressive variant: {}", e) 
                })?;
                
                variants.push((quality_level, output));
            }
            
            Ok(variants)
        })
        .await
        .map_err(|e| NanoError::Media(format!("Progressive variant creation task failed: {}", e)))?
    }
    
    /// Get image dimensions without loading full image
    pub async fn get_image_dimensions(&self, image_data: &[u8]) -> Result<(u32, u32)> {
        let image_data = image_data.to_vec();
        
        task::spawn_blocking(move || {
            // Try to read dimensions from header without decoding full image
            let cursor = Cursor::new(&image_data);
            if let Ok(reader) = image::io::Reader::new(cursor).with_guessed_format() {
                if let Ok(dimensions) = reader.into_dimensions() {
                    return Ok(dimensions);
                }
            }
            
            // Fallback to loading full image
            let img = image::load_from_memory(&image_data)
                .map_err(|e| ImageProcessingError::ProcessingFailed { 
                    reason: format!("Failed to get image dimensions: {}", e) 
                })?;
            
            Ok(img.dimensions())
        })
        .await
        .map_err(|e| NanoError::Media(format!("Dimension reading task failed: {}", e)))?
    }
}

/// Configuration for image optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageOptimizationConfig {
    pub thumbnail_sizes: Vec<u32>,
    pub default_quality: u8,
    pub max_dimension: Option<u32>,
    pub strip_exif_data: bool,
    pub progressive_jpeg: bool,
    pub supported_formats: Vec<ImageFormat>,
}

impl Default for ImageOptimizationConfig {
    fn default() -> Self {
        Self {
            thumbnail_sizes: vec![150, 300, 600],
            default_quality: 85,
            max_dimension: Some(4096),
            strip_exif_data: true,
            progressive_jpeg: true,
            supported_formats: vec![
                ImageFormat::Jpeg,
                ImageFormat::Png,
                ImageFormat::WebP,
            ],
        }
    }
}

impl From<&super::ImageProcessingConfig> for ImageOptimizationConfig {
    fn from(config: &super::ImageProcessingConfig) -> Self {
        Self {
            thumbnail_sizes: config.thumbnail_sizes.clone(),
            default_quality: config.target_quality,
            max_dimension: config.max_dimension,
            strip_exif_data: config.strip_exif_data,
            progressive_jpeg: config.progressive_jpeg,
            supported_formats: config.supported_formats.iter()
                .filter_map(|s| s.parse().ok())
                .collect(),
        }
    }
}

/// Supported image formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageFormat {
    Jpeg,
    Png,
    WebP,
    Gif,
    Tiff,
}

impl std::str::FromStr for ImageFormat {
    type Err = String;
    
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "jpeg" | "jpg" => Ok(ImageFormat::Jpeg),
            "png" => Ok(ImageFormat::Png),
            "webp" => Ok(ImageFormat::WebP),
            "gif" => Ok(ImageFormat::Gif),
            "tiff" | "tif" => Ok(ImageFormat::Tiff),
            _ => Err(format!("Unsupported image format: {}", s)),
        }
    }
}

/// Image quality levels for progressive loading
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageQuality {
    Thumbnail,
    Preview,
    Standard,
    HighQuality,
}

/// Result of image optimization
#[derive(Debug, Clone)]
pub struct OptimizedImage {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub format: ImageFormat,
    pub quality: Option<u8>,
    pub original_size: usize,
    pub compression_ratio: f64,
    pub processing_time: std::time::Duration,
}

/// Thumbnail set with multiple sizes
#[derive(Debug, Clone)]
pub struct ThumbnailSet {
    pub small: (Vec<u8>, u32, u32),     // 150x150
    pub medium: (Vec<u8>, u32, u32),    // 300x300  
    pub large: (Vec<u8>, u32, u32),     // 600x600
    pub custom: Vec<(String, Vec<u8>, u32, u32)>,  // Custom sizes
}

impl ThumbnailSet {
    /// Create thumbnail set from list of thumbnails
    pub fn from_thumbnails(thumbnails: Vec<(String, Vec<u8>)>) -> Result<Self> {
        let mut small = None;
        let mut medium = None;
        let mut large = None;
        let mut custom = Vec::new();
        
        for (name, data) in thumbnails {
            // Try to extract dimensions from name (format: thumb_WIDTHxHEIGHT)
            if let Some(dimensions) = extract_dimensions_from_name(&name) {
                let (width, height) = dimensions;
                
                match width {
                    150 => small = Some((data, width, height)),
                    300 => medium = Some((data, width, height)),
                    600 => large = Some((data, width, height)),
                    _ => custom.push((name, data, width, height)),
                }
            } else {
                // If we can't parse dimensions, treat as custom
                custom.push((name, data, 0, 0));
            }
        }
        
        Ok(Self {
            small: small.unwrap_or_else(|| (Vec::new(), 0, 0)),
            medium: medium.unwrap_or_else(|| (Vec::new(), 0, 0)),
            large: large.unwrap_or_else(|| (Vec::new(), 0, 0)),
            custom,
        })
    }
    
    /// Get thumbnail by size preference
    pub fn get_by_size(&self, preferred_size: u32) -> Option<(&Vec<u8>, u32, u32)> {
        match preferred_size {
            0..=200 => Some((&self.small.0, self.small.1, self.small.2)),
            201..=400 => Some((&self.medium.0, self.medium.1, self.medium.2)),
            401..=800 => Some((&self.large.0, self.large.1, self.large.2)),
            _ => {
                // For custom sizes, return the first custom thumbnail if available
                self.custom.first().map(|(_, data, w, h)| (data, *w, *h))
            }
        }
    }
}

/// Image processing errors
#[derive(Debug, thiserror::Error)]
pub enum ImageProcessingError {
    #[error("Processing failed: {reason}")]
    ProcessingFailed { reason: String },
    
    #[error("Unsupported format: {format}")]
    UnsupportedFormat { format: String },
    
    #[error("Invalid image data")]
    InvalidImageData,
    
    #[error("Dimension limit exceeded: {width}x{height} > {max_dimension}")]
    DimensionLimitExceeded { width: u32, height: u32, max_dimension: u32 },
    
    #[error("EXIF processing error: {message}")]
    ExifError { message: String },
}

impl From<ImageError> for ImageProcessingError {
    fn from(err: ImageError) -> Self {
        ImageProcessingError::ProcessingFailed { reason: err.to_string() }
    }
}

impl From<ImageProcessingError> for NanoError {
    fn from(err: ImageProcessingError) -> Self {
        NanoError::Media(err.to_string())
    }
}

/// Calculate thumbnail dimensions maintaining aspect ratio
fn calculate_thumbnail_size(original_width: u32, original_height: u32, max_size: u32) -> (u32, u32) {
    let aspect_ratio = original_width as f32 / original_height as f32;
    
    if original_width > original_height {
        let width = max_size;
        let height = (max_size as f32 / aspect_ratio) as u32;
        (width, height)
    } else {
        let height = max_size;
        let width = (max_size as f32 * aspect_ratio) as u32;
        (width, height)
    }
}

/// Extract EXIF data from image bytes
fn extract_exif_data(_image_data: &[u8]) -> Result<HashMap<String, String>> {
    // EXIF functionality temporarily disabled due to dependency version issues
    // TODO: Re-enable when kamadak-exif dependency is resolved
    /*
    use exif::{In, Reader, Tag};
    
    let mut metadata = HashMap::new();
    
    if let Ok(exif_reader) = Reader::new() {
        if let Ok(exif) = exif_reader.read_from_container(&mut std::io::Cursor::new(image_data)) {
            for field in exif.fields() {
                let tag_name = match field.tag {
                    Tag::Make => "camera_make",
                    Tag::Model => "camera_model",
                    Tag::DateTime => "date_time",
                    Tag::ExifVersion => "exif_version",
                    Tag::ColorSpace => "color_space",
                    Tag::PixelXDimension => "pixel_width",
                    Tag::PixelYDimension => "pixel_height",
                    Tag::Orientation => "orientation",
                    Tag::XResolution => "x_resolution",
                    Tag::YResolution => "y_resolution",
                    Tag::ResolutionUnit => "resolution_unit",
                    Tag::Software => "software",
                    Tag::WhitePoint => "white_point",
                    Tag::YCbCrCoefficients => "ycbcr_coefficients",
                    _ => continue, // Skip unknown/unimportant tags
                };
                
                metadata.insert(tag_name.to_string(), field.display_value().to_string());
            }
        }
    }
    */
    
    // Return empty metadata for now
    Ok(HashMap::new())
}

/// Extract dimensions from thumbnail name (format: thumb_WIDTHxHEIGHT)
fn extract_dimensions_from_name(name: &str) -> Option<(u32, u32)> {
    if let Some(dimensions_part) = name.strip_prefix("thumb_") {
        if let Some(x_pos) = dimensions_part.find('x') {
            let width_str = &dimensions_part[..x_pos];
            let height_str = &dimensions_part[x_pos + 1..];
            
            if let (Ok(width), Ok(height)) = (width_str.parse::<u32>(), height_str.parse::<u32>()) {
                return Some((width, height));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculate_thumbnail_size() {
        // Landscape image
        let (width, height) = calculate_thumbnail_size(1920, 1080, 300);
        assert_eq!(width, 300);
        assert_eq!(height, 168); // ~169
        
        // Portrait image
        let (width, height) = calculate_thumbnail_size(1080, 1920, 300);
        assert_eq!(width, 168); // ~169
        assert_eq!(height, 300);
        
        // Square image
        let (width, height) = calculate_thumbnail_size(1000, 1000, 300);
        assert_eq!(width, 300);
        assert_eq!(height, 300);
    }
    
    #[test]
    fn test_extract_dimensions_from_name() {
        assert_eq!(extract_dimensions_from_name("thumb_150x150"), Some((150, 150)));
        assert_eq!(extract_dimensions_from_name("thumb_300x200"), Some((300, 200)));
        assert_eq!(extract_dimensions_from_name("thumb_invalid"), None);
        assert_eq!(extract_dimensions_from_name("not_a_thumb"), None);
    }
    
    #[test]
    fn test_image_format_parsing() {
        assert_eq!("jpeg".parse::<ImageFormat>().unwrap(), ImageFormat::Jpeg);
        assert_eq!("jpg".parse::<ImageFormat>().unwrap(), ImageFormat::Jpeg);
        assert_eq!("png".parse::<ImageFormat>().unwrap(), ImageFormat::Png);
        assert_eq!("webp".parse::<ImageFormat>().unwrap(), ImageFormat::WebP);
        
        assert!("invalid".parse::<ImageFormat>().is_err());
    }
    
    #[tokio::test]
    async fn test_image_processor_creation() {
        let config = ImageOptimizationConfig::default();
        let processor = ImageProcessor::new(config);
        
        // This is a basic test to ensure the processor can be created
        // Real image processing tests would require test image data
        assert_eq!(processor.config.default_quality, 85);
    }
    
    #[test]
    fn test_thumbnail_set_from_thumbnails() {
        let thumbnails = vec![
            ("thumb_150x150".to_string(), vec![1, 2, 3]),
            ("thumb_300x300".to_string(), vec![4, 5, 6]),
            ("thumb_600x600".to_string(), vec![7, 8, 9]),
            ("thumb_custom_800x400".to_string(), vec![10, 11, 12]),
        ];
        
        let thumbnail_set = ThumbnailSet::from_thumbnails(thumbnails).unwrap();
        
        assert_eq!(thumbnail_set.small.0, vec![1, 2, 3]);
        assert_eq!(thumbnail_set.medium.0, vec![4, 5, 6]);
        assert_eq!(thumbnail_set.large.0, vec![7, 8, 9]);
        assert_eq!(thumbnail_set.custom.len(), 1);
    }
}
