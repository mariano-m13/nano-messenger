//! Video Processing Module
//! 
//! Provides video processing capabilities including:
//! - Video thumbnail/preview generation
//! - Video metadata extraction
//! - Video compression and optimization
//! - Format validation and conversion
//! - Duration and quality analysis

#![cfg(feature = "video-processing")]

use crate::error::{NanoError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};
use tokio::{fs, process::Command as AsyncCommand, task};

/// Video processor for handling all video operations
pub struct VideoProcessor {
    config: VideoProcessingConfig,
    ffmpeg_path: String,
    ffprobe_path: String,
}

impl VideoProcessor {
    /// Create a new video processor
    pub async fn new(config: &super::VideoProcessingConfig) -> Result<Self> {
        let processor = Self {
            config: VideoProcessingConfig::from(config),
            ffmpeg_path: detect_ffmpeg_path().await?,
            ffprobe_path: detect_ffprobe_path().await?,
        };
        
        // Verify FFmpeg is working
        processor.verify_ffmpeg().await?;
        
        Ok(processor)
    }
    
    /// Generate video thumbnail at specified timestamp
    pub async fn generate_video_thumbnail(&self, video_path: &Path, timestamp_seconds: f64) -> Result<Vec<u8>> {
        let video_path = video_path.to_path_buf();
        let ffmpeg_path = self.ffmpeg_path.clone();
        let timestamp = timestamp_seconds;
        
        task::spawn_blocking(move || {
            let start_time = Instant::now();
            
            // Create temporary output file
            let temp_dir = tempfile::tempdir()
                .map_err(|e| VideoProcessingError::ProcessingFailed { 
                    reason: format!("Failed to create temp directory: {}", e) 
                })?;
            
            let thumbnail_path = temp_dir.path().join("thumbnail.jpg");
            
            // Run FFmpeg command to extract frame
            let output = Command::new(&ffmpeg_path)
                .args([
                    "-i", video_path.to_str().unwrap(),
                    "-ss", &timestamp.to_string(),
                    "-vframes", "1",
                    "-f", "image2",
                    "-vf", "scale=600:400:force_original_aspect_ratio=decrease,pad=600:400:(ow-iw)/2:(oh-ih)/2",
                    "-q:v", "2", // High quality
                    thumbnail_path.to_str().unwrap(),
                ])
                .output()
                .map_err(|e| VideoProcessingError::ProcessingFailed { 
                    reason: format!("Failed to execute FFmpeg: {}", e) 
                })?;
            
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(VideoProcessingError::ProcessingFailed { 
                    reason: format!("FFmpeg failed: {}", stderr) 
                });
            }
            
            // Read the generated thumbnail
            let thumbnail_data = std::fs::read(&thumbnail_path)
                .map_err(|e| VideoProcessingError::ProcessingFailed { 
                    reason: format!("Failed to read thumbnail: {}", e) 
                })?;
            
            log::debug!(
                "Generated video thumbnail: {} bytes in {}ms",
                thumbnail_data.len(),
                start_time.elapsed().as_millis()
            );
            
            Ok(thumbnail_data)
        })
        .await
        .map_err(|e| NanoError::Media(format!("Video thumbnail generation task failed: {}", e)))?
    }
    
    /// Extract comprehensive video metadata
    pub async fn extract_metadata(&self, video_path: &Path) -> Result<VideoMetadata> {
        let video_path = video_path.to_path_buf();
        let ffprobe_path = self.ffprobe_path.clone();
        
        task::spawn_blocking(move || {
            let start_time = Instant::now();
            
            // Run FFprobe to get video information
            let output = Command::new(&ffprobe_path)
                .args([
                    "-v", "quiet",
                    "-print_format", "json",
                    "-show_format",
                    "-show_streams",
                    video_path.to_str().unwrap(),
                ])
                .output()
                .map_err(|e| VideoProcessingError::ProcessingFailed { 
                    reason: format!("Failed to execute FFprobe: {}", e) 
                })?;
            
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(VideoProcessingError::ProcessingFailed { 
                    reason: format!("FFprobe failed: {}", stderr) 
                });
            }
            
            let json_output = String::from_utf8_lossy(&output.stdout);
            let probe_data: serde_json::Value = serde_json::from_str(&json_output)
                .map_err(|e| VideoProcessingError::ProcessingFailed { 
                    reason: format!("Failed to parse FFprobe output: {}", e) 
                })?;
            
            // Extract metadata from JSON
            let metadata = parse_ffprobe_output(&probe_data)?;
            
            log::debug!(
                "Extracted video metadata in {}ms: duration={:.2}s, resolution={}x{}",
                start_time.elapsed().as_millis(),
                metadata.duration.as_secs_f32(),
                metadata.resolution.0,
                metadata.resolution.1
            );
            
            Ok(metadata)
        })
        .await
        .map_err(|e| NanoError::Media(format!("Video metadata extraction task failed: {}", e)))?
    }
    
    /// Compress video for efficient transfer
    pub async fn compress_video(&self, input_path: &Path, target_bitrate_kbps: u32) -> Result<Vec<u8>> {
        let input_path = input_path.to_path_buf();
        let ffmpeg_path = self.ffmpeg_path.clone();
        let config = self.config.clone();
        
        task::spawn_blocking(move || {
            let start_time = Instant::now();
            
            // Create temporary output file
            let temp_dir = tempfile::tempdir()
                .map_err(|e| VideoProcessingError::ProcessingFailed { 
                    reason: format!("Failed to create temp directory: {}", e) 
                })?;
            
            let output_path = temp_dir.path().join("compressed.mp4");
            
            // Build FFmpeg command for compression
            let mut args = vec![
                "-i".to_string(),
                input_path.to_str().unwrap().to_string(),
                "-c:v".to_string(),
                "libx264".to_string(),
                "-b:v".to_string(),
                format!("{}k", target_bitrate_kbps),
                "-c:a".to_string(),
                "aac".to_string(),
                "-b:a".to_string(),
                "128k".to_string(),
                "-movflags".to_string(),
                "+faststart".to_string(),
            ];
            
            // Add resolution limit if configured
            if config.max_resolution.0 > 0 && config.max_resolution.1 > 0 {
                args.extend([
                    "-vf".to_string(),
                    format!("scale={}:{}:force_original_aspect_ratio=decrease", 
                           config.max_resolution.0, config.max_resolution.1),
                ]);
            }
            
            args.push(output_path.to_str().unwrap().to_string());
            
            // Run FFmpeg compression
            let output = Command::new(&ffmpeg_path)
                .args(args)
                .output()
                .map_err(|e| VideoProcessingError::ProcessingFailed { 
                    reason: format!("Failed to execute FFmpeg compression: {}", e) 
                })?;
            
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(VideoProcessingError::ProcessingFailed { 
                    reason: format!("FFmpeg compression failed: {}", stderr) 
                });
            }
            
            // Read the compressed video
            let compressed_data = std::fs::read(&output_path)
                .map_err(|e| VideoProcessingError::ProcessingFailed { 
                    reason: format!("Failed to read compressed video: {}", e) 
                })?;
            
            log::debug!(
                "Compressed video: {} bytes in {}ms",
                compressed_data.len(),
                start_time.elapsed().as_millis()
            );
            
            Ok(compressed_data)
        })
        .await
        .map_err(|e| NanoError::Media(format!("Video compression task failed: {}", e)))?
    }
    
    /// Validate video file format and properties
    pub async fn validate_video(&self, video_path: &Path) -> Result<VideoValidationResult> {
        let metadata = self.extract_metadata(video_path).await?;
        let config = &self.config;
        
        let mut issues = Vec::new();
        let mut warnings = Vec::new();
        
        // Check duration limit
        if metadata.duration.as_secs() > config.max_video_length_seconds {
            issues.push(format!(
                "Video duration ({:.1}s) exceeds limit ({}s)",
                metadata.duration.as_secs_f32(),
                config.max_video_length_seconds
            ));
        }
        
        // Check resolution limit
        if config.max_resolution.0 > 0 && config.max_resolution.1 > 0 {
            if metadata.resolution.0 > config.max_resolution.0 || 
               metadata.resolution.1 > config.max_resolution.1 {
                warnings.push(format!(
                    "Resolution ({}x{}) exceeds recommended maximum ({}x{})",
                    metadata.resolution.0,
                    metadata.resolution.1,
                    config.max_resolution.0,
                    config.max_resolution.1
                ));
            }
        }
        
        // Check codec support
        if !config.supported_codecs.contains(&metadata.codec) {
            warnings.push(format!(
                "Codec '{}' may not be supported in all clients",
                metadata.codec
            ));
        }
        
        // Check bitrate
        if metadata.bitrate > config.target_bitrate_kbps * 2 {
            warnings.push(format!(
                "High bitrate ({}kbps) may cause playback issues",
                metadata.bitrate
            ));
        }
        
        Ok(VideoValidationResult {
            is_valid: issues.is_empty(),
            issues,
            warnings,
            metadata,
            recommendations: generate_compression_recommendations(&metadata, config),
        })
    }
    
    /// Generate multiple quality variants
    pub async fn generate_quality_variants(&self, input_path: &Path) -> Result<Vec<(VideoQuality, Vec<u8>)>> {
        let qualities = [
            (VideoQuality::Low, 500),
            (VideoQuality::Medium, 1000),
            (VideoQuality::High, 2000),
            (VideoQuality::VeryHigh, 4000),
        ];
        
        let mut variants = Vec::new();
        
        for (quality, bitrate) in qualities {
            match self.compress_video(input_path, bitrate).await {
                Ok(data) => variants.push((quality, data)),
                Err(e) => log::warn!("Failed to generate {} quality variant: {}", quality, e),
            }
        }
        
        Ok(variants)
    }
    
    /// Get video frame at specific timestamp for preview
    pub async fn extract_frame_at_timestamp(&self, video_path: &Path, timestamp: Duration) -> Result<Vec<u8>> {
        self.generate_video_thumbnail(video_path, timestamp.as_secs_f64()).await
    }
    
    /// Verify FFmpeg installation and capabilities
    async fn verify_ffmpeg(&self) -> Result<()> {
        let ffmpeg_path = self.ffmpeg_path.clone();
        let ffprobe_path = self.ffprobe_path.clone();
        
        task::spawn_blocking(move || {
            // Test FFmpeg
            let ffmpeg_output = Command::new(&ffmpeg_path)
                .args(["-version"])
                .output()
                .map_err(|e| VideoProcessingError::ProcessingFailed { 
                    reason: format!("FFmpeg not working: {}", e) 
                })?;
            
            if !ffmpeg_output.status.success() {
                return Err(VideoProcessingError::ProcessingFailed { 
                    reason: "FFmpeg version check failed".to_string() 
                });
            }
            
            // Test FFprobe
            let ffprobe_output = Command::new(&ffprobe_path)
                .args(["-version"])
                .output()
                .map_err(|e| VideoProcessingError::ProcessingFailed { 
                    reason: format!("FFprobe not working: {}", e) 
                })?;
            
            if !ffprobe_output.status.success() {
                return Err(VideoProcessingError::ProcessingFailed { 
                    reason: "FFprobe version check failed".to_string() 
                });
            }
            
            log::info!("FFmpeg and FFprobe verified successfully");
            Ok(())
        })
        .await
        .map_err(|e| NanoError::Media(format!("FFmpeg verification task failed: {}", e)))?
    }
}

/// Video processing configuration
#[derive(Debug, Clone)]
struct VideoProcessingConfig {
    pub max_video_length_seconds: u64,
    pub supported_codecs: Vec<String>,
    pub target_bitrate_kbps: u32,
    pub max_resolution: (u32, u32),
}

impl From<&super::VideoProcessingConfig> for VideoProcessingConfig {
    fn from(config: &super::VideoProcessingConfig) -> Self {
        Self {
            max_video_length_seconds: config.max_video_length_seconds,
            supported_codecs: config.supported_codecs.clone(),
            target_bitrate_kbps: config.target_bitrate_kbps,
            max_resolution: config.max_resolution,
        }
    }
}

/// Video metadata extracted from file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoMetadata {
    pub duration: Duration,
    pub resolution: (u32, u32),
    pub codec: String,
    pub bitrate: u32,
    pub frame_rate: f32,
    pub file_size: u64,
    pub audio_codec: Option<String>,
    pub audio_bitrate: Option<u32>,
    pub container_format: String,
    pub creation_time: Option<String>,
    pub additional_streams: Vec<String>,
}

/// Video thumbnail information
#[derive(Debug, Clone)]
pub struct VideoThumbnail {
    pub data: Vec<u8>,
    pub timestamp: Duration,
    pub width: u32,
    pub height: u32,
    pub format: String,
}

/// Video quality levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VideoQuality {
    Low,      // 500kbps
    Medium,   // 1000kbps  
    High,     // 2000kbps
    VeryHigh, // 4000kbps
}

impl std::fmt::Display for VideoQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoQuality::Low => write!(f, "low"),
            VideoQuality::Medium => write!(f, "medium"),
            VideoQuality::High => write!(f, "high"),
            VideoQuality::VeryHigh => write!(f, "very_high"),
        }
    }
}

/// Video codecs
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VideoCodec {
    H264,
    H265,
    VP9,
    AV1,
    Unknown(String),
}

impl From<&str> for VideoCodec {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "h264" | "avc" | "libx264" => VideoCodec::H264,
            "h265" | "hevc" | "libx265" => VideoCodec::H265,
            "vp9" | "libvpx-vp9" => VideoCodec::VP9,
            "av1" | "libaom-av1" => VideoCodec::AV1,
            _ => VideoCodec::Unknown(s.to_string()),
        }
    }
}

/// Video validation result
#[derive(Debug, Clone)]
pub struct VideoValidationResult {
    pub is_valid: bool,
    pub issues: Vec<String>,
    pub warnings: Vec<String>,
    pub metadata: VideoMetadata,
    pub recommendations: Vec<String>,
}

/// Video processing errors
#[derive(Debug, thiserror::Error)]
pub enum VideoProcessingError {
    #[error("Processing failed: {reason}")]
    ProcessingFailed { reason: String },
    
    #[error("FFmpeg not found or not working")]
    FFmpegNotAvailable,
    
    #[error("Unsupported video format: {format}")]
    UnsupportedFormat { format: String },
    
    #[error("Video too long: {duration}s exceeds limit of {limit}s")]
    VideoTooLong { duration: u64, limit: u64 },
    
    #[error("Invalid video data")]
    InvalidVideoData,
}

impl From<VideoProcessingError> for NanoError {
    fn from(err: VideoProcessingError) -> Self {
        NanoError::Media(err.to_string())
    }
}

/// Detect FFmpeg executable path
async fn detect_ffmpeg_path() -> Result<String> {
    // Try common locations
    let candidates = [
        "ffmpeg",
        "/usr/bin/ffmpeg",
        "/usr/local/bin/ffmpeg",
        "/opt/homebrew/bin/ffmpeg",
        "C:\\ffmpeg\\bin\\ffmpeg.exe",
    ];
    
    for candidate in candidates {
        if let Ok(output) = AsyncCommand::new(candidate)
            .arg("-version")
            .output()
            .await 
        {
            if output.status.success() {
                return Ok(candidate.to_string());
            }
        }
    }
    
    Err(NanoError::Media("FFmpeg not found in PATH".to_string()))
}

/// Detect FFprobe executable path
async fn detect_ffprobe_path() -> Result<String> {
    // Try common locations
    let candidates = [
        "ffprobe",
        "/usr/bin/ffprobe",
        "/usr/local/bin/ffprobe",
        "/opt/homebrew/bin/ffprobe",
        "C:\\ffmpeg\\bin\\ffprobe.exe",
    ];
    
    for candidate in candidates {
        if let Ok(output) = AsyncCommand::new(candidate)
            .arg("-version")
            .output()
            .await 
        {
            if output.status.success() {
                return Ok(candidate.to_string());
            }
        }
    }
    
    Err(NanoError::Media("FFprobe not found in PATH".to_string()))
}

/// Parse FFprobe JSON output into VideoMetadata
fn parse_ffprobe_output(data: &serde_json::Value) -> Result<VideoMetadata> {
    let format_info = data.get("format")
        .ok_or_else(|| VideoProcessingError::ProcessingFailed { 
            reason: "No format information in FFprobe output".to_string() 
        })?;
    
    let streams = data.get("streams")
        .and_then(|s| s.as_array())
        .ok_or_else(|| VideoProcessingError::ProcessingFailed { 
            reason: "No streams information in FFprobe output".to_string() 
        })?;
    
    // Find video stream
    let video_stream = streams.iter()
        .find(|stream| stream.get("codec_type").and_then(|t| t.as_str()) == Some("video"))
        .ok_or_else(|| VideoProcessingError::ProcessingFailed { 
            reason: "No video stream found".to_string() 
        })?;
    
    // Extract basic information
    let duration_str = format_info.get("duration")
        .and_then(|d| d.as_str())
        .unwrap_or("0");
    let duration = Duration::from_secs_f64(duration_str.parse().unwrap_or(0.0));
    
    let width = video_stream.get("width")
        .and_then(|w| w.as_u64())
        .unwrap_or(0) as u32;
    let height = video_stream.get("height")
        .and_then(|h| h.as_u64())
        .unwrap_or(0) as u32;
    
    let codec = video_stream.get("codec_name")
        .and_then(|c| c.as_str())
        .unwrap_or("unknown")
        .to_string();
    
    let bitrate = format_info.get("bit_rate")
        .and_then(|b| b.as_str())
        .and_then(|s| s.parse().ok())
        .unwrap_or(0) / 1000; // Convert to kbps
    
    let frame_rate_str = video_stream.get("r_frame_rate")
        .and_then(|r| r.as_str())
        .unwrap_or("0/1");
    let frame_rate = parse_frame_rate(frame_rate_str);
    
    let file_size = format_info.get("size")
        .and_then(|s| s.as_str())
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    
    // Find audio stream
    let audio_stream = streams.iter()
        .find(|stream| stream.get("codec_type").and_then(|t| t.as_str()) == Some("audio"));
    
    let audio_codec = audio_stream
        .and_then(|s| s.get("codec_name"))
        .and_then(|c| c.as_str())
        .map(|s| s.to_string());
    
    let audio_bitrate = audio_stream
        .and_then(|s| s.get("bit_rate"))
        .and_then(|b| b.as_str())
        .and_then(|s| s.parse().ok())
        .map(|b: u32| b / 1000); // Convert to kbps
    
    let container_format = format_info.get("format_name")
        .and_then(|f| f.as_str())
        .unwrap_or("unknown")
        .to_string();
    
    let creation_time = format_info.get("tags")
        .and_then(|tags| tags.get("creation_time"))
        .and_then(|t| t.as_str())
        .map(|s| s.to_string());
    
    // Collect additional stream types
    let additional_streams: Vec<String> = streams.iter()
        .filter_map(|stream| stream.get("codec_type").and_then(|t| t.as_str()))
        .filter(|&codec_type| codec_type != "video" && codec_type != "audio")
        .map(|s| s.to_string())
        .collect();
    
    Ok(VideoMetadata {
        duration,
        resolution: (width, height),
        codec,
        bitrate,
        frame_rate,
        file_size,
        audio_codec,
        audio_bitrate,
        container_format,
        creation_time,
        additional_streams,
    })
}

/// Parse frame rate string (e.g., "30/1" -> 30.0)
fn parse_frame_rate(frame_rate_str: &str) -> f32 {
    if let Some(slash_pos) = frame_rate_str.find('/') {
        let numerator_str = &frame_rate_str[..slash_pos];
        let denominator_str = &frame_rate_str[slash_pos + 1..];
        
        if let (Ok(numerator), Ok(denominator)) = (
            numerator_str.parse::<f32>(),
            denominator_str.parse::<f32>()
        ) {
            if denominator != 0.0 {
                return numerator / denominator;
            }
        }
    }
    
    // Fallback: try to parse as direct float
    frame_rate_str.parse().unwrap_or(0.0)
}

/// Generate compression recommendations based on video metadata
fn generate_compression_recommendations(metadata: &VideoMetadata, config: &VideoProcessingConfig) -> Vec<String> {
    let mut recommendations = Vec::new();
    
    // Bitrate recommendations
    if metadata.bitrate > config.target_bitrate_kbps * 2 {
        recommendations.push(format!(
            "Consider reducing bitrate from {}kbps to {}kbps for better streaming",
            metadata.bitrate,
            config.target_bitrate_kbps
        ));
    }
    
    // Resolution recommendations
    if metadata.resolution.0 > config.max_resolution.0 || 
       metadata.resolution.1 > config.max_resolution.1 {
        recommendations.push(format!(
            "Consider reducing resolution from {}x{} to {}x{} for compatibility",
            metadata.resolution.0,
            metadata.resolution.1,
            config.max_resolution.0,
            config.max_resolution.1
        ));
    }
    
    // Codec recommendations
    if !config.supported_codecs.contains(&metadata.codec) {
        recommendations.push(format!(
            "Consider converting from {} to a more compatible codec like H.264",
            metadata.codec
        ));
    }
    
    // Duration recommendations
    if metadata.duration.as_secs() > config.max_video_length_seconds {
        recommendations.push(format!(
            "Video is longer than recommended maximum ({}s). Consider splitting or trimming.",
            config.max_video_length_seconds
        ));
    }
    
    recommendations
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_frame_rate() {
        assert_eq!(parse_frame_rate("30/1"), 30.0);
        assert_eq!(parse_frame_rate("60/1"), 60.0);
        assert_eq!(parse_frame_rate("24000/1001"), 23.976);
        assert_eq!(parse_frame_rate("invalid"), 0.0);
        assert_eq!(parse_frame_rate("25.0"), 25.0);
    }
    
    #[test]
    fn test_video_codec_conversion() {
        assert_eq!(VideoCodec::from("h264"), VideoCodec::H264);
        assert_eq!(VideoCodec::from("libx264"), VideoCodec::H264);
        assert_eq!(VideoCodec::from("h265"), VideoCodec::H265);
        assert_eq!(VideoCodec::from("vp9"), VideoCodec::VP9);
        
        match VideoCodec::from("unknown_codec") {
            VideoCodec::Unknown(s) => assert_eq!(s, "unknown_codec"),
            _ => panic!("Expected Unknown variant"),
        }
    }
    
    #[test]
    fn test_video_quality_display() {
        assert_eq!(VideoQuality::Low.to_string(), "low");
        assert_eq!(VideoQuality::Medium.to_string(), "medium");
        assert_eq!(VideoQuality::High.to_string(), "high");
        assert_eq!(VideoQuality::VeryHigh.to_string(), "very_high");
    }
    
    #[tokio::test]
    async fn test_ffmpeg_detection() {
        // This test will only pass if FFmpeg is installed
        // In a real environment, you might want to mock this
        match detect_ffmpeg_path().await {
            Ok(path) => println!("FFmpeg found at: {}", path),
            Err(_) => println!("FFmpeg not found (expected in test environment)"),
        }
    }
}
