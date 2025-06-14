/// Media Detection and Validation Module
/// 
/// Provides comprehensive media detection capabilities including:
/// - Accurate MIME type detection from file content
/// - EXIF data extraction from images
/// - Media file integrity validation
/// - File format verification
/// - Content-based analysis

use crate::error::{NanoError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Cursor;

/// Media detector for analyzing and validating media files
pub struct MediaDetector {
    mime_detector: MimeDetector,
}

impl MediaDetector {
    /// Create a new media detector
    pub fn new() -> Self {
        Self {
            mime_detector: MimeDetector::new(),
        }
    }
    
    /// Detect MIME type from file content
    pub fn detect_mime_type(&self, content: &[u8]) -> Result<mime::Mime> {
        self.mime_detector.detect_from_content(content)
    }
    
    /// Detect media type category
    pub fn detect_media_type(&self, content: &[u8]) -> Result<MediaType> {
        let mime_type = self.detect_mime_type(content)?;
        Ok(MediaType::from_mime(&mime_type))
    }
    
    /// Extract EXIF data from image
    pub fn extract_exif_data(&self, image_content: &[u8]) -> Result<ExifData> {
        extract_exif_from_bytes(image_content)
    }
    
    /// Validate media file integrity and structure
    pub fn validate_media_file(&self, content: &[u8], expected_type: &MediaType) -> Result<MediaValidationResult> {
        let start_time = std::time::Instant::now();
        
        // Detect actual type
        let detected_type = self.detect_media_type(content)?;
        let mime_type = self.detect_mime_type(content)?;
        
        let mut validation_result = MediaValidationResult {
            is_valid: true,
            detected_type: detected_type.clone(),
            mime_type: mime_type.clone(),
            file_size: content.len() as u64,
            issues: Vec::new(),
            warnings: Vec::new(),
            metadata: HashMap::new(),
            processing_time: std::time::Duration::default(),
        };
        
        // Check type consistency
        if detected_type != *expected_type {
            validation_result.issues.push(format!(
                "Type mismatch: expected {:?}, detected {:?}",
                expected_type, detected_type
            ));
            validation_result.is_valid = false;
        }
        
        // Perform type-specific validation
        match detected_type {
            MediaType::Image => {
                self.validate_image_content(content, &mut validation_result)?;
            }
            MediaType::Video => {
                self.validate_video_content(content, &mut validation_result)?;
            }
            MediaType::Audio => {
                self.validate_audio_content(content, &mut validation_result)?;
            }
            MediaType::Document => {
                self.validate_document_content(content, &mut validation_result)?;
            }
            MediaType::Unknown => {
                validation_result.warnings.push("Unknown media type detected".to_string());
            }
        }
        
        // Check for common file corruption indicators
        self.check_file_integrity(content, &mut validation_result)?;
        
        validation_result.processing_time = start_time.elapsed();
        
        Ok(validation_result)
    }
    
    /// Get detailed file information
    pub fn analyze_file(&self, content: &[u8]) -> Result<FileAnalysis> {
        let media_type = self.detect_media_type(content)?;
        let mime_type = self.detect_mime_type(content)?;
        
        let mut metadata = HashMap::new();
        
        // Extract type-specific metadata
        match media_type {
            MediaType::Image => {
                if let Ok(exif_data) = self.extract_exif_data(content) {
                    for (key, value) in exif_data.data {
                        metadata.insert(format!("exif_{}", key), value);
                    }
                }
                
                // Try to get image dimensions
                if let Ok(dimensions) = get_image_dimensions(content) {
                    metadata.insert("width".to_string(), dimensions.0.to_string());
                    metadata.insert("height".to_string(), dimensions.1.to_string());
                }
            }
            MediaType::Video => {
                // Basic video header analysis
                if let Ok(video_info) = analyze_video_header(content) {
                    for (key, value) in video_info {
                        metadata.insert(key, value);
                    }
                }
            }
            MediaType::Audio => {
                // Basic audio header analysis
                if let Ok(audio_info) = analyze_audio_header(content) {
                    for (key, value) in audio_info {
                        metadata.insert(key, value);
                    }
                }
            }
            _ => {}
        }
        
        // Add general file information
        metadata.insert("file_size".to_string(), content.len().to_string());
        metadata.insert("mime_type".to_string(), mime_type.to_string());
        
        Ok(FileAnalysis {
            media_type,
            mime_type,
            file_size: content.len() as u64,
            metadata,
            entropy: calculate_entropy(content),
            magic_bytes: get_magic_bytes(content),
            is_compressed: detect_compression(content),
        })
    }
    
    /// Validate image content structure
    fn validate_image_content(&self, content: &[u8], result: &mut MediaValidationResult) -> Result<()> {
        // Check for valid image headers
        if content.len() < 8 {
            result.issues.push("Image file too small to contain valid header".to_string());
            result.is_valid = false;
            return Ok(());
        }
        
        // Validate specific image formats
        let mime_str = result.mime_type.as_ref();
        match mime_str {
            "image/jpeg" => {
                if !content.starts_with(&[0xFF, 0xD8, 0xFF]) {
                    result.issues.push("Invalid JPEG header".to_string());
                    result.is_valid = false;
                }
                
                // Check for proper JPEG ending
                if !content.ends_with(&[0xFF, 0xD9]) {
                    result.warnings.push("JPEG file may be truncated (missing EOI marker)".to_string());
                }
            }
            "image/png" => {
                let png_signature = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
                if !content.starts_with(&png_signature) {
                    result.issues.push("Invalid PNG signature".to_string());
                    result.is_valid = false;
                }
            }
            "image/gif" => {
                if !(content.starts_with(b"GIF87a") || content.starts_with(b"GIF89a")) {
                    result.issues.push("Invalid GIF header".to_string());
                    result.is_valid = false;
                }
            }
            "image/webp" => {
                if !content.starts_with(b"RIFF") || !content[8..12].starts_with(b"WEBP") {
                    result.issues.push("Invalid WebP header".to_string());
                    result.is_valid = false;
                }
            }
            _ => {
                result.warnings.push(format!("Unknown image format: {}", mime_str));
            }
        }
        
        // Try to extract dimensions
        if let Ok(dimensions) = get_image_dimensions(content) {
            result.metadata.insert("width".to_string(), dimensions.0.to_string());
            result.metadata.insert("height".to_string(), dimensions.1.to_string());
            
            // Check for reasonable dimensions
            if dimensions.0 == 0 || dimensions.1 == 0 {
                result.issues.push("Invalid image dimensions".to_string());
                result.is_valid = false;
            } else if dimensions.0 > 50000 || dimensions.1 > 50000 {
                result.warnings.push("Extremely large image dimensions detected".to_string());
            }
        }
        
        Ok(())
    }
    
    /// Validate video content structure
    fn validate_video_content(&self, content: &[u8], result: &mut MediaValidationResult) -> Result<()> {
        if content.len() < 16 {
            result.issues.push("Video file too small to contain valid header".to_string());
            result.is_valid = false;
            return Ok(());
        }
        
        // Check common video format headers
        let mime_str = result.mime_type.as_ref();
        match mime_str {
            "video/mp4" => {
                // MP4 files start with ftyp box
                if content.len() > 8 && &content[4..8] != b"ftyp" {
                    result.warnings.push("Unusual MP4 structure detected".to_string());
                }
            }
            "video/webm" => {
                // WebM is based on Matroska
                if !content.starts_with(&[0x1A, 0x45, 0xDF, 0xA3]) {
                    result.warnings.push("Invalid WebM/Matroska header".to_string());
                }
            }
            "video/avi" => {
                // AVI files start with RIFF...AVI
                if !content.starts_with(b"RIFF") || !content[8..11].starts_with(b"AVI") {
                    result.issues.push("Invalid AVI header".to_string());
                    result.is_valid = false;
                }
            }
            _ => {
                result.warnings.push(format!("Unknown video format: {}", mime_str));
            }
        }
        
        Ok(())
    }
    
    /// Validate audio content structure
    fn validate_audio_content(&self, content: &[u8], result: &mut MediaValidationResult) -> Result<()> {
        if content.len() < 4 {
            result.issues.push("Audio file too small to contain valid header".to_string());
            result.is_valid = false;
            return Ok(());
        }
        
        let mime_str = result.mime_type.as_ref();
        match mime_str {
            "audio/mpeg" => {
                // MP3 files have frame sync
                if content.len() > 2 && (content[0] & 0xFF) == 0xFF && (content[1] & 0xE0) == 0xE0 {
                    // Valid MP3 frame sync
                } else if content.starts_with(b"ID3") {
                    // ID3 tag at beginning is also valid
                } else {
                    result.warnings.push("Unusual MP3 structure detected".to_string());
                }
            }
            "audio/wav" => {
                if !content.starts_with(b"RIFF") || !content[8..12].starts_with(b"WAVE") {
                    result.issues.push("Invalid WAV header".to_string());
                    result.is_valid = false;
                }
            }
            "audio/ogg" => {
                if !content.starts_with(b"OggS") {
                    result.issues.push("Invalid OGG header".to_string());
                    result.is_valid = false;
                }
            }
            _ => {
                result.warnings.push(format!("Unknown audio format: {}", mime_str));
            }
        }
        
        Ok(())
    }
    
    /// Validate document content
    fn validate_document_content(&self, content: &[u8], result: &mut MediaValidationResult) -> Result<()> {
        if content.is_empty() {
            result.issues.push("Empty document file".to_string());
            result.is_valid = false;
            return Ok(());
        }
        
        let mime_str = result.mime_type.as_ref();
        match mime_str {
            "application/pdf" => {
                if !content.starts_with(b"%PDF-") {
                    result.issues.push("Invalid PDF header".to_string());
                    result.is_valid = false;
                } else {
                    // Extract PDF version
                    if content.len() > 8 {
                        let version_str = String::from_utf8_lossy(&content[5..8]);
                        result.metadata.insert("pdf_version".to_string(), version_str.to_string());
                    }
                }
            }
            "application/zip" => {
                // ZIP files start with PK
                if !content.starts_with(&[0x50, 0x4B]) {
                    result.issues.push("Invalid ZIP header".to_string());
                    result.is_valid = false;
                }
            }
            _ => {
                result.warnings.push(format!("Unknown document format: {}", mime_str));
            }
        }
        
        Ok(())
    }
    
    /// Check for common file corruption indicators
    fn check_file_integrity(&self, content: &[u8], result: &mut MediaValidationResult) -> Result<()> {
        // Check for suspiciously repetitive content
        let entropy = calculate_entropy(content);
        if entropy < 1.0 {
            result.warnings.push("Low entropy detected - file may be corrupted or unusual".to_string());
        }
        
        // Check file size consistency
        if content.len() < 100 {
            result.warnings.push("Unusually small file size".to_string());
        }
        
        result.metadata.insert("entropy".to_string(), format!("{:.2}", entropy));
        
        Ok(())
    }
}

impl Default for MediaDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// MIME type detector using content analysis
struct MimeDetector {
    // Add any caching or configuration here if needed
}

impl MimeDetector {
    fn new() -> Self {
        Self {}
    }
    
    fn detect_from_content(&self, content: &[u8]) -> Result<mime::Mime> {
        // Use the mime_guess crate as fallback, but prioritize content-based detection
        let content_based_mime = detect_mime_from_magic_bytes(content);
        
        if let Some(mime) = content_based_mime {
            return Ok(mime);
        }
        
        // Fallback to unknown type
        Ok(mime::APPLICATION_OCTET_STREAM)
    }
}

/// Media type categories
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MediaType {
    Image,
    Video,
    Audio,
    Document,
    Unknown,
}

impl MediaType {
    /// Convert from MIME type to media type category
    pub fn from_mime(mime: &mime::Mime) -> Self {
        match mime.type_() {
            mime::IMAGE => MediaType::Image,
            mime::VIDEO => MediaType::Video,
            mime::AUDIO => MediaType::Audio,
            mime::APPLICATION => {
                match mime.subtype().as_str() {
                    "pdf" | "msword" | "vnd.openxmlformats-officedocument.wordprocessingml.document" => MediaType::Document,
                    _ => MediaType::Unknown,
                }
            }
            mime::TEXT => MediaType::Document,
            _ => MediaType::Unknown,
        }
    }
}

/// EXIF data extracted from images
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExifData {
    pub data: HashMap<String, String>,
    pub camera_make: Option<String>,
    pub camera_model: Option<String>,
    pub date_time: Option<String>,
    pub gps_coordinates: Option<(f64, f64)>,
    pub orientation: Option<u32>,
}

impl Default for ExifData {
    fn default() -> Self {
        Self {
            data: HashMap::new(),
            camera_make: None,
            camera_model: None,
            date_time: None,
            gps_coordinates: None,
            orientation: None,
        }
    }
}

/// Media validation result
#[derive(Debug, Clone)]
pub struct MediaValidationResult {
    pub is_valid: bool,
    pub detected_type: MediaType,
    pub mime_type: mime::Mime,
    pub file_size: u64,
    pub issues: Vec<String>,
    pub warnings: Vec<String>,
    pub metadata: HashMap<String, String>,
    pub processing_time: std::time::Duration,
}

/// Comprehensive file analysis result
#[derive(Debug, Clone)]
pub struct FileAnalysis {
    pub media_type: MediaType,
    pub mime_type: mime::Mime,
    pub file_size: u64,
    pub metadata: HashMap<String, String>,
    pub entropy: f64,
    pub magic_bytes: Vec<u8>,
    pub is_compressed: bool,
}

/// Detection errors
#[derive(Debug, thiserror::Error)]
pub enum DetectionError {
    #[error("Failed to detect MIME type")]
    MimeDetectionFailed,
    
    #[error("Invalid file content")]
    InvalidContent,
    
    #[error("EXIF extraction failed: {message}")]
    ExifError { message: String },
    
    #[error("Unsupported file format")]
    UnsupportedFormat,
}

impl From<DetectionError> for NanoError {
    fn from(err: DetectionError) -> Self {
        NanoError::Media(err.to_string())
    }
}

/// Detect MIME type from magic bytes
fn detect_mime_from_magic_bytes(content: &[u8]) -> Option<mime::Mime> {
    if content.is_empty() {
        return None;
    }
    
    // Image formats
    if content.starts_with(&[0xFF, 0xD8, 0xFF]) {
        return Some(mime::IMAGE_JPEG);
    }
    if content.starts_with(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]) {
        return Some(mime::IMAGE_PNG);
    }
    if content.starts_with(b"GIF87a") || content.starts_with(b"GIF89a") {
        return Some(mime::IMAGE_GIF);
    }
    if content.starts_with(b"RIFF") && content.len() > 12 && &content[8..12] == b"WEBP" {
        return Some("image/webp".parse().ok()?);
    }
    
    // Video formats
    if content.len() > 8 && &content[4..8] == b"ftyp" {
        return Some("video/mp4".parse().ok()?);
    }
    if content.starts_with(&[0x1A, 0x45, 0xDF, 0xA3]) {
        return Some("video/webm".parse().ok()?);
    }
    if content.starts_with(b"RIFF") && content.len() > 11 && &content[8..11] == b"AVI" {
        return Some("video/avi".parse().ok()?);
    }
    
    // Audio formats
    if content.len() > 2 && content[0] == 0xFF && (content[1] & 0xE0) == 0xE0 {
        return Some("audio/mpeg".parse().ok()?);
    }
    if content.starts_with(b"ID3") {
        return Some("audio/mpeg".parse().ok()?);
    }
    if content.starts_with(b"RIFF") && content.len() > 12 && &content[8..12] == b"WAVE" {
        return Some("audio/wav".parse().ok()?);
    }
    if content.starts_with(b"OggS") {
        return Some("audio/ogg".parse().ok()?);
    }
    
    // Document formats
    if content.starts_with(b"%PDF-") {
        return Some("application/pdf".parse().ok()?);
    }
    if content.starts_with(&[0x50, 0x4B, 0x03, 0x04]) || content.starts_with(&[0x50, 0x4B, 0x05, 0x06]) {
        return Some("application/zip".parse().ok()?);
    }
    
    None
}

/// Extract EXIF data from image bytes
fn extract_exif_from_bytes(content: &[u8]) -> Result<ExifData> {
    let mut exif_data = ExifData::default();
    
    // For now, we'll implement a basic EXIF parser or use a simpler approach
    // This is a placeholder implementation to avoid dependency issues
    
    // Check if this is a JPEG with EXIF data
    if content.len() > 10 && content.starts_with(&[0xFF, 0xD8]) {
        // Look for EXIF marker in JPEG
        for i in 2..content.len().saturating_sub(6) {
            if content[i] == 0xFF && content[i + 1] == 0xE1 {
                // Found potential EXIF segment
                let segment_length = u16::from_be_bytes([content[i + 2], content[i + 3]]) as usize;
                if i + 4 + segment_length <= content.len() {
                    let segment = &content[i + 4..i + 4 + segment_length];
                    if segment.starts_with(b"Exif\0\0") {
                        exif_data.data.insert("has_exif".to_string(), "true".to_string());
                        exif_data.data.insert("exif_segment_size".to_string(), segment_length.to_string());
                        break;
                    }
                }
            }
        }
    }
    
    // Add basic file information
    exif_data.data.insert("file_size".to_string(), content.len().to_string());
    exif_data.data.insert("processed_at".to_string(), chrono::Utc::now().to_rfc3339());
    
    Ok(exif_data)
}

/// Get image dimensions without fully loading the image
fn get_image_dimensions(content: &[u8]) -> Result<(u32, u32)> {
    // Use the image crate to read dimensions from header
    let cursor = Cursor::new(content);
    if let Ok(reader) = image::io::Reader::new(cursor).with_guessed_format() {
        if let Ok(dimensions) = reader.into_dimensions() {
            return Ok(dimensions);
        }
    }
    
    Err(NanoError::Media("Failed to extract image dimensions".to_string()))
}

/// Analyze video header for basic information
fn analyze_video_header(content: &[u8]) -> Result<HashMap<String, String>> {
    let mut info = HashMap::new();
    
    if content.len() < 16 {
        return Ok(info);
    }
    
    // Basic MP4 analysis
    if content.len() > 8 && &content[4..8] == b"ftyp" {
        if content.len() > 12 {
            let brand = String::from_utf8_lossy(&content[8..12]);
            info.insert("container".to_string(), "mp4".to_string());
            info.insert("brand".to_string(), brand.to_string());
        }
    }
    
    // Basic WebM/Matroska analysis
    if content.starts_with(&[0x1A, 0x45, 0xDF, 0xA3]) {
        info.insert("container".to_string(), "matroska/webm".to_string());
    }
    
    // Basic AVI analysis
    if content.starts_with(b"RIFF") && content.len() > 11 && &content[8..11] == b"AVI" {
        info.insert("container".to_string(), "avi".to_string());
    }
    
    Ok(info)
}

/// Analyze audio header for basic information
fn analyze_audio_header(content: &[u8]) -> Result<HashMap<String, String>> {
    let mut info = HashMap::new();
    
    if content.len() < 4 {
        return Ok(info);
    }
    
    // MP3 analysis
    if content.len() > 2 && content[0] == 0xFF && (content[1] & 0xE0) == 0xE0 {
        info.insert("format".to_string(), "mp3".to_string());
        // Could extract more MP3 frame header info here
    } else if content.starts_with(b"ID3") {
        info.insert("format".to_string(), "mp3".to_string());
        info.insert("has_id3".to_string(), "true".to_string());
    }
    
    // WAV analysis
    if content.starts_with(b"RIFF") && content.len() > 12 && &content[8..12] == b"WAVE" {
        info.insert("format".to_string(), "wav".to_string());
    }
    
    // OGG analysis
    if content.starts_with(b"OggS") {
        info.insert("format".to_string(), "ogg".to_string());
    }
    
    Ok(info)
}

/// Calculate file entropy (measure of randomness/compression)
fn calculate_entropy(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    
    let mut frequency = [0u32; 256];
    
    // Count byte frequencies
    for &byte in data {
        frequency[byte as usize] += 1;
    }
    
    let len = data.len() as f64;
    let mut entropy = 0.0;
    
    // Calculate Shannon entropy
    for &count in &frequency {
        if count > 0 {
            let p = count as f64 / len;
            entropy -= p * p.log2();
        }
    }
    
    entropy
}

/// Get first few bytes as magic bytes
fn get_magic_bytes(content: &[u8]) -> Vec<u8> {
    content.iter().take(16).copied().collect()
}

/// Detect if content appears to be compressed
fn detect_compression(content: &[u8]) -> bool {
    if content.is_empty() {
        return false;
    }
    
    // Check for compression signatures
    if content.starts_with(&[0x1F, 0x8B]) {
        return true; // gzip
    }
    if content.starts_with(b"BZh") {
        return true; // bzip2
    }
    if content.starts_with(&[0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00]) {
        return true; // xz
    }
    
    // High entropy often indicates compression
    calculate_entropy(content) > 7.5
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mime_detection_from_magic_bytes() {
        // JPEG
        let jpeg_header = [0xFF, 0xD8, 0xFF, 0xE0];
        assert_eq!(detect_mime_from_magic_bytes(&jpeg_header), Some(mime::IMAGE_JPEG));
        
        // PNG
        let png_header = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        assert_eq!(detect_mime_from_magic_bytes(&png_header), Some(mime::IMAGE_PNG));
        
        // PDF
        let pdf_header = b"%PDF-1.4";
        assert_eq!(detect_mime_from_magic_bytes(pdf_header), Some("application/pdf".parse().unwrap()));
        
        // Unknown
        let unknown = [0x00, 0x01, 0x02, 0x03];
        assert_eq!(detect_mime_from_magic_bytes(&unknown), None);
    }
    
    #[test]
    fn test_media_type_from_mime() {
        assert_eq!(MediaType::from_mime(&mime::IMAGE_JPEG), MediaType::Image);
        assert_eq!(MediaType::from_mime(&"video/mp4".parse().unwrap()), MediaType::Video);
        assert_eq!(MediaType::from_mime(&"audio/mpeg".parse().unwrap()), MediaType::Audio);
        assert_eq!(MediaType::from_mime(&"application/pdf".parse().unwrap()), MediaType::Document);
        assert_eq!(MediaType::from_mime(&mime::APPLICATION_OCTET_STREAM), MediaType::Unknown);
    }
    
    #[test]
    fn test_entropy_calculation() {
        // All same bytes (low entropy)
        let uniform = vec![0x00; 100];
        assert!(calculate_entropy(&uniform) < 1.0);
        
        // Random-ish bytes (higher entropy)
        let varied = (0..=255).collect::<Vec<u8>>();
        assert!(calculate_entropy(&varied) > 7.0);
        
        // Empty data
        assert_eq!(calculate_entropy(&[]), 0.0);
    }
    
    #[test]
    fn test_compression_detection() {
        // Gzip signature
        let gzip_header = [0x1F, 0x8B, 0x08, 0x00];
        assert!(detect_compression(&gzip_header));
        
        // Bzip2 signature
        let bzip2_header = b"BZh9";
        assert!(detect_compression(bzip2_header));
        
        // Non-compressed data
        let plain_text = b"Hello, world!";
        assert!(!detect_compression(plain_text));
    }
    
    #[tokio::test]
    async fn test_media_detector_creation() {
        let detector = MediaDetector::new();
        
        // Test with some basic content
        let jpeg_header = [0xFF, 0xD8, 0xFF, 0xE0];
        let mime_type = detector.detect_mime_type(&jpeg_header).unwrap();
        assert_eq!(mime_type, mime::IMAGE_JPEG);
        
        let media_type = detector.detect_media_type(&jpeg_header).unwrap();
        assert_eq!(media_type, MediaType::Image);
    }
    
    #[test]
    fn test_get_magic_bytes() {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05];
        let magic = get_magic_bytes(&data);
        assert_eq!(magic, vec![0x01, 0x02, 0x03, 0x04, 0x05]);
        
        let long_data = vec![0xFF; 20];
        let magic_long = get_magic_bytes(&long_data);
        assert_eq!(magic_long.len(), 16);
    }
}
