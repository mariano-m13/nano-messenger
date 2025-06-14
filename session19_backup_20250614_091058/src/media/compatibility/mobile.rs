/// Mobile Optimization
/// 
/// Provides intelligent media optimization for mobile devices based on
/// device capabilities, network conditions, and battery status.

use crate::error::Result;
use crate::media::{
    storage::FileId,
    processing::MediaType,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Media codec support for different platforms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MediaCodec {
    // Video codecs
    H264,
    H265,
    VP8,
    VP9,
    AV1,
    
    // Audio codecs
    AAC,
    MP3,
    Opus,
    Vorbis,
    
    // Image formats
    JPEG,
    PNG,
    WebP,
    AVIF,
    HEIF,
}

impl MediaCodec {
    /// Get codec efficiency score (higher is better)
    pub fn efficiency_score(&self) -> f32 {
        match self {
            MediaCodec::H264 => 0.7,
            MediaCodec::H265 => 0.9,
            MediaCodec::VP8 => 0.6,
            MediaCodec::VP9 => 0.8,
            MediaCodec::AV1 => 1.0,
            MediaCodec::AAC => 0.8,
            MediaCodec::MP3 => 0.6,
            MediaCodec::Opus => 1.0,
            MediaCodec::Vorbis => 0.7,
            MediaCodec::JPEG => 0.6,
            MediaCodec::PNG => 0.4,
            MediaCodec::WebP => 0.8,
            MediaCodec::AVIF => 1.0,
            MediaCodec::HEIF => 0.9,
        }
    }

    /// Check if codec supports hardware acceleration
    pub fn supports_hardware_acceleration(&self) -> bool {
        matches!(self, MediaCodec::H264 | MediaCodec::H265 | MediaCodec::VP8 | MediaCodec::VP9)
    }

    /// Get typical file extension
    pub fn file_extension(&self) -> &'static str {
        match self {
            MediaCodec::H264 => "mp4",
            MediaCodec::H265 => "mp4",
            MediaCodec::VP8 => "webm",
            MediaCodec::VP9 => "webm",
            MediaCodec::AV1 => "mp4",
            MediaCodec::AAC => "aac",
            MediaCodec::MP3 => "mp3",
            MediaCodec::Opus => "opus",
            MediaCodec::Vorbis => "ogg",
            MediaCodec::JPEG => "jpg",
            MediaCodec::PNG => "png",
            MediaCodec::WebP => "webp",
            MediaCodec::AVIF => "avif",
            MediaCodec::HEIF => "heic",
        }
    }
}

/// Device profile containing capabilities and limitations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceProfile {
    pub device_type: DeviceType,
    pub screen_resolution: (u32, u32),
    pub screen_density: f32,  // DPI
    pub supported_codecs: Vec<MediaCodec>,
    pub hardware_acceleration: bool,
    pub max_video_resolution: (u32, u32),
    pub max_framerate: f32,
    pub storage_available: u64,
    pub ram_available: u64,
    pub cpu_cores: u32,
    pub gpu_capabilities: GpuCapabilities,
}

/// Device type enumeration
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DeviceType {
    Smartphone,
    Tablet,
    Desktop,
    Laptop,
    SmartTV,
    WebBrowser,
    Unknown,
}

/// GPU capabilities for hardware acceleration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuCapabilities {
    pub has_gpu: bool,
    pub supports_h264_decode: bool,
    pub supports_h265_decode: bool,
    pub supports_vp9_decode: bool,
    pub max_decode_resolution: (u32, u32),
    pub simultaneous_decode_streams: u32,
}

impl Default for GpuCapabilities {
    fn default() -> Self {
        Self {
            has_gpu: false,
            supports_h264_decode: false,
            supports_h265_decode: false,
            supports_vp9_decode: false,
            max_decode_resolution: (1920, 1080),
            simultaneous_decode_streams: 1,
        }
    }
}

/// Network profile for bandwidth-aware optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkProfile {
    pub connection_type: ConnectionType,
    pub bandwidth_mbps: f32,
    pub latency_ms: u32,
    pub is_metered: bool,
    pub signal_strength: f32,  // 0.0 - 1.0
    pub data_saver_enabled: bool,
}

/// Network connection types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ConnectionType {
    WiFi,
    Cellular5G,
    Cellular4G,
    Cellular3G,
    Ethernet,
    Unknown,
}

impl ConnectionType {
    /// Get typical bandwidth range for connection type
    pub fn typical_bandwidth_mbps(&self) -> (f32, f32) {
        match self {
            ConnectionType::WiFi => (10.0, 100.0),
            ConnectionType::Cellular5G => (50.0, 1000.0),
            ConnectionType::Cellular4G => (5.0, 50.0),
            ConnectionType::Cellular3G => (1.0, 10.0),
            ConnectionType::Ethernet => (50.0, 1000.0),
            ConnectionType::Unknown => (1.0, 10.0),
        }
    }

    /// Check if connection is typically metered
    pub fn is_typically_metered(&self) -> bool {
        matches!(self, ConnectionType::Cellular5G | ConnectionType::Cellular4G | ConnectionType::Cellular3G)
    }
}

/// Mobile-specific quality level for bandwidth adaptation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MobileQualityLevel {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub bitrate: u32,
    pub framerate: f32,
}

/// Processing strategy based on device capabilities
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ProcessingStrategy {
    /// High quality processing with all features
    HighQuality,
    /// Balanced processing for mid-range devices
    Balanced,
    /// Minimal processing for low-end devices
    PowerSaver,
    /// Optimized for battery preservation
    BatteryOptimized,
    /// Fast processing with reduced quality
    FastProcessing,
}

impl ProcessingStrategy {
    /// Get quality multiplier for this strategy
    pub fn quality_multiplier(&self) -> f32 {
        match self {
            ProcessingStrategy::HighQuality => 1.0,
            ProcessingStrategy::Balanced => 0.8,
            ProcessingStrategy::PowerSaver => 0.6,
            ProcessingStrategy::BatteryOptimized => 0.5,
            ProcessingStrategy::FastProcessing => 0.7,
        }
    }

    /// Get processing timeout for this strategy
    pub fn processing_timeout(&self) -> Duration {
        match self {
            ProcessingStrategy::HighQuality => Duration::from_secs(300),  // 5 minutes
            ProcessingStrategy::Balanced => Duration::from_secs(120),     // 2 minutes
            ProcessingStrategy::PowerSaver => Duration::from_secs(60),    // 1 minute
            ProcessingStrategy::BatteryOptimized => Duration::from_secs(30), // 30 seconds
            ProcessingStrategy::FastProcessing => Duration::from_secs(45), // 45 seconds
        }
    }
}

/// Battery-aware configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatteryAwareConfig {
    pub enabled: bool,
    pub low_battery_threshold: f32,      // 0.0 - 1.0
    pub critical_battery_threshold: f32, // 0.0 - 1.0
    pub low_battery_strategy: ProcessingStrategy,
    pub critical_battery_strategy: ProcessingStrategy,
    pub disable_background_processing: bool,
}

impl Default for BatteryAwareConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            low_battery_threshold: 0.2,      // 20%
            critical_battery_threshold: 0.1, // 10%
            low_battery_strategy: ProcessingStrategy::BatteryOptimized,
            critical_battery_strategy: ProcessingStrategy::PowerSaver,
            disable_background_processing: true,
        }
    }
}

/// Optimized media set for different quality levels
#[derive(Debug, Clone)]
pub struct OptimizedMediaSet {
    pub original: MediaFile,
    pub optimized_versions: HashMap<String, MediaFile>,
    pub thumbnails: HashMap<String, MediaFile>,
    pub metadata: OptimizationMetadata,
}

/// Media file representation
#[derive(Debug, Clone)]
pub struct MediaFile {
    pub file_id: FileId,
    pub content: Vec<u8>,
    pub mime_type: String,
    pub size: u64,
    pub resolution: Option<(u32, u32)>,
    pub duration: Option<Duration>,
    pub codec: Option<MediaCodec>,
    pub bitrate: Option<u32>,
    pub quality_level: Option<MobileQualityLevel>,
}

impl MediaFile {
    /// Create a new media file
    pub fn new(file_id: FileId, content: Vec<u8>, mime_type: String) -> Self {
        Self {
            file_id,
            size: content.len() as u64,
            content,
            mime_type,
            resolution: None,
            duration: None,
            codec: None,
            bitrate: None,
            quality_level: None,
        }
    }

    /// Set video properties
    pub fn with_video_properties(
        mut self,
        resolution: (u32, u32),
        duration: Duration,
        codec: MediaCodec,
        bitrate: u32,
    ) -> Self {
        self.resolution = Some(resolution);
        self.duration = Some(duration);
        self.codec = Some(codec);
        self.bitrate = Some(bitrate);
        self
    }

    /// Set image properties
    pub fn with_image_properties(mut self, resolution: (u32, u32), codec: MediaCodec) -> Self {
        self.resolution = Some(resolution);
        self.codec = Some(codec);
        self
    }

    /// Calculate compression ratio compared to original
    pub fn compression_ratio(&self, original_size: u64) -> f32 {
        if original_size == 0 {
            return 1.0;
        }
        self.size as f32 / original_size as f32
    }
}

/// Optimization metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationMetadata {
    pub optimization_strategy: ProcessingStrategy,
    pub target_device: DeviceType,
    pub network_optimized: bool,
    pub battery_optimized: bool,
    pub processing_time: Duration,
    pub size_reduction: f32,  // Percentage
    pub quality_score: f32,   // 0.0 - 1.0
}

/// Mobile optimization manager
pub struct MobileOptimization {
    pub device_profile: DeviceProfile,
    pub network_profile: NetworkProfile,
    pub battery_config: BatteryAwareConfig,
}

impl MobileOptimization {
    /// Create a new mobile optimization manager
    pub fn new(
        device_profile: DeviceProfile,
        network_profile: NetworkProfile,
        battery_config: BatteryAwareConfig,
    ) -> Self {
        Self {
            device_profile,
            network_profile,
            battery_config,
        }
    }

    /// Optimize media for mobile device
    pub async fn optimize_for_mobile(&self, media: &MediaFile) -> Result<OptimizedMediaSet> {
        let strategy = self.get_optimization_strategy(1.0); // Assume full battery for now
        let target_quality = self.calculate_target_quality(&strategy);

        let mut optimized_versions = HashMap::new();
        let mut thumbnails = HashMap::new();

        // Create different quality versions based on device capabilities
        if self.should_create_low_quality_version() {
            let low_quality = self.create_low_quality_version(media).await?;
            optimized_versions.insert("low".to_string(), low_quality);
        }

        if self.should_create_medium_quality_version() {
            let medium_quality = self.create_medium_quality_version(media).await?;
            optimized_versions.insert("medium".to_string(), medium_quality);
        }

        // Create thumbnails
        if media.mime_type.starts_with("image/") || media.mime_type.starts_with("video/") {
            let thumbnail = self.create_thumbnail(media).await?;
            thumbnails.insert("thumbnail".to_string(), thumbnail);
        }

        let metadata = OptimizationMetadata {
            optimization_strategy: strategy,
            target_device: self.device_profile.device_type,
            network_optimized: self.network_profile.is_metered || self.network_profile.bandwidth_mbps < 10.0,
            battery_optimized: self.battery_config.enabled,
            processing_time: Duration::from_millis(100), // Placeholder
            size_reduction: 50.0, // Placeholder
            quality_score: target_quality,
        };

        Ok(OptimizedMediaSet {
            original: media.clone(),
            optimized_versions,
            thumbnails,
            metadata,
        })
    }

    /// Select quality level based on available bandwidth
    pub fn select_quality_for_bandwidth(&self, available_bandwidth: u32) -> MobileQualityLevel {
        let bandwidth_mbps = available_bandwidth as f32 / 1_000_000.0;

        if bandwidth_mbps >= 5.0 && !self.network_profile.is_metered {
            MobileQualityLevel {
                name: "High".to_string(),
                width: 1920,
                height: 1080,
                bitrate: 5_000_000,
                framerate: 30.0,
            }
        } else if bandwidth_mbps >= 2.0 {
            MobileQualityLevel {
                name: "Medium".to_string(),
                width: 1280,
                height: 720,
                bitrate: 2_500_000,
                framerate: 30.0,
            }
        } else {
            MobileQualityLevel {
                name: "Low".to_string(),
                width: 854,
                height: 480,
                bitrate: 1_000_000,
                framerate: 24.0,
            }
        }
    }

    /// Get processing strategy based on battery level
    pub fn get_processing_strategy(&self, battery_level: f32) -> ProcessingStrategy {
        if !self.battery_config.enabled {
            return ProcessingStrategy::HighQuality;
        }

        if battery_level <= self.battery_config.critical_battery_threshold {
            self.battery_config.critical_battery_strategy
        } else if battery_level <= self.battery_config.low_battery_threshold {
            self.battery_config.low_battery_strategy
        } else if self.network_profile.is_metered {
            ProcessingStrategy::Balanced
        } else {
            match self.device_profile.device_type {
                DeviceType::Smartphone => ProcessingStrategy::Balanced,
                DeviceType::Tablet => ProcessingStrategy::HighQuality,
                DeviceType::Desktop => ProcessingStrategy::HighQuality,
                DeviceType::Laptop => ProcessingStrategy::Balanced,
                _ => ProcessingStrategy::PowerSaver,
            }
        }
    }

    /// Check if device supports codec
    pub fn supports_codec(&self, codec: MediaCodec) -> bool {
        self.device_profile.supported_codecs.contains(&codec)
    }

    /// Get best codec for device
    pub fn get_best_codec(&self, media_type: MediaType) -> Option<MediaCodec> {
        let candidate_codecs = match media_type {
            MediaType::Video => vec![MediaCodec::AV1, MediaCodec::H265, MediaCodec::VP9, MediaCodec::H264, MediaCodec::VP8],
            MediaType::Audio => vec![MediaCodec::Opus, MediaCodec::AAC, MediaCodec::Vorbis, MediaCodec::MP3],
            MediaType::Image => vec![MediaCodec::AVIF, MediaCodec::HEIF, MediaCodec::WebP, MediaCodec::JPEG, MediaCodec::PNG],
            MediaType::Document => return None, // Documents don't need codec conversion
            MediaType::Unknown => return None, // Unknown media types don't have codec conversion
        };

        // Find the best supported codec
        for codec in candidate_codecs {
            if self.supports_codec(codec) {
                return Some(codec);
            }
        }

        None
    }

    /// Calculate network-aware timeout
    pub fn get_network_timeout(&self) -> Duration {
        let base_timeout = Duration::from_secs(30);
        
        match self.network_profile.connection_type {
            ConnectionType::WiFi | ConnectionType::Ethernet => base_timeout,
            ConnectionType::Cellular5G => base_timeout * 2,
            ConnectionType::Cellular4G => base_timeout * 3,
            ConnectionType::Cellular3G => base_timeout * 5,
            ConnectionType::Unknown => base_timeout * 4,
        }
    }

    /// Get device-specific cache size
    pub fn get_cache_size(&self) -> u64 {
        let available_storage = self.device_profile.storage_available;
        let cache_percentage = match self.device_profile.device_type {
            DeviceType::Smartphone => 0.02,  // 2% of available storage
            DeviceType::Tablet => 0.05,      // 5% of available storage
            DeviceType::Desktop => 0.10,     // 10% of available storage
            DeviceType::Laptop => 0.05,      // 5% of available storage
            _ => 0.01,                        // 1% for unknown devices
        };

        let max_cache = match self.device_profile.device_type {
            DeviceType::Smartphone => 500 * 1024 * 1024,   // 500MB max
            DeviceType::Tablet => 2 * 1024 * 1024 * 1024,  // 2GB max
            DeviceType::Desktop => 10 * 1024 * 1024 * 1024, // 10GB max
            DeviceType::Laptop => 5 * 1024 * 1024 * 1024,  // 5GB max
            _ => 100 * 1024 * 1024,                         // 100MB max
        };

        std::cmp::min((available_storage as f64 * cache_percentage) as u64, max_cache)
    }

    // Helper methods for optimization

    fn get_optimization_strategy(&self, battery_level: f32) -> ProcessingStrategy {
        self.get_processing_strategy(battery_level)
    }

    fn calculate_target_quality(&self, strategy: &ProcessingStrategy) -> f32 {
        let base_quality = strategy.quality_multiplier();
        
        // Adjust for network conditions
        let network_multiplier = if self.network_profile.is_metered {
            0.8
        } else if self.network_profile.bandwidth_mbps < 5.0 {
            0.7
        } else {
            1.0
        };

        // Adjust for device capabilities
        let device_multiplier = match self.device_profile.device_type {
            DeviceType::Smartphone => 0.8,
            DeviceType::Tablet => 0.9,
            DeviceType::Desktop => 1.0,
            DeviceType::Laptop => 0.95,
            _ => 0.7,
        };

        (base_quality * network_multiplier * device_multiplier).min(1.0)
    }

    fn should_create_low_quality_version(&self) -> bool {
        self.network_profile.is_metered || 
        self.network_profile.bandwidth_mbps < 2.0 ||
        matches!(self.device_profile.device_type, DeviceType::Smartphone)
    }

    fn should_create_medium_quality_version(&self) -> bool {
        self.network_profile.bandwidth_mbps >= 2.0 && 
        self.network_profile.bandwidth_mbps < 10.0
    }

    async fn create_low_quality_version(&self, media: &MediaFile) -> Result<MediaFile> {
        // Simplified implementation - would use actual media processing
        let mut optimized = media.clone();
        optimized.size = (media.size as f64 * 0.3) as u64; // 30% of original size
        optimized.content = vec![0u8; optimized.size as usize]; // Placeholder
        
        if let Some((width, height)) = media.resolution {
            optimized.resolution = Some((width / 2, height / 2));
        }
        
        if let Some(bitrate) = media.bitrate {
            optimized.bitrate = Some(bitrate / 3);
        }

        Ok(optimized)
    }

    async fn create_medium_quality_version(&self, media: &MediaFile) -> Result<MediaFile> {
        // Simplified implementation - would use actual media processing
        let mut optimized = media.clone();
        optimized.size = (media.size as f64 * 0.6) as u64; // 60% of original size
        optimized.content = vec![0u8; optimized.size as usize]; // Placeholder
        
        if let Some((width, height)) = media.resolution {
            optimized.resolution = Some((width * 3 / 4, height * 3 / 4));
        }
        
        if let Some(bitrate) = media.bitrate {
            optimized.bitrate = Some(bitrate * 2 / 3);
        }

        Ok(optimized)
    }

    async fn create_thumbnail(&self, media: &MediaFile) -> Result<MediaFile> {
        // Simplified implementation - would generate actual thumbnails
        let thumbnail_size = match self.device_profile.device_type {
            DeviceType::Smartphone => (150, 150),
            DeviceType::Tablet => (200, 200),
            _ => (300, 300),
        };

        let mut thumbnail = media.clone();
        thumbnail.resolution = Some(thumbnail_size);
        thumbnail.size = 5 * 1024; // 5KB thumbnail
        thumbnail.content = vec![0u8; thumbnail.size as usize]; // Placeholder
        thumbnail.mime_type = "image/jpeg".to_string();
        thumbnail.codec = Some(MediaCodec::JPEG);

        Ok(thumbnail)
    }
}

impl Default for DeviceProfile {
    fn default() -> Self {
        Self {
            device_type: DeviceType::Unknown,
            screen_resolution: (1920, 1080),
            screen_density: 96.0,
            supported_codecs: vec![
                MediaCodec::H264,
                MediaCodec::AAC,
                MediaCodec::JPEG,
                MediaCodec::PNG,
                MediaCodec::MP3,
            ],
            hardware_acceleration: false,
            max_video_resolution: (1920, 1080),
            max_framerate: 30.0,
            storage_available: 1024 * 1024 * 1024, // 1GB
            ram_available: 512 * 1024 * 1024,      // 512MB
            cpu_cores: 2,
            gpu_capabilities: GpuCapabilities::default(),
        }
    }
}

impl Default for NetworkProfile {
    fn default() -> Self {
        Self {
            connection_type: ConnectionType::Unknown,
            bandwidth_mbps: 5.0,
            latency_ms: 100,
            is_metered: false,
            signal_strength: 0.8,
            data_saver_enabled: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_codec_properties() {
        assert_eq!(MediaCodec::H264.file_extension(), "mp4");
        assert!(MediaCodec::H264.supports_hardware_acceleration());
        assert!(MediaCodec::AV1.efficiency_score() > MediaCodec::H264.efficiency_score());
    }

    #[test]
    fn test_connection_type_properties() {
        let wifi = ConnectionType::WiFi;
        let cellular = ConnectionType::Cellular4G;
        
        assert!(!wifi.is_typically_metered());
        assert!(cellular.is_typically_metered());
        
        let (min, max) = wifi.typical_bandwidth_mbps();
        assert!(max > min);
    }

    #[test]
    fn test_processing_strategy() {
        let high_quality = ProcessingStrategy::HighQuality;
        let power_saver = ProcessingStrategy::PowerSaver;
        
        assert!(high_quality.quality_multiplier() > power_saver.quality_multiplier());
        assert!(high_quality.processing_timeout() > power_saver.processing_timeout());
    }

    #[test]
    fn test_device_profile_defaults() {
        let profile = DeviceProfile::default();
        assert_eq!(profile.device_type as u8, DeviceType::Unknown as u8);
        assert!(!profile.hardware_acceleration);
        assert!(profile.supported_codecs.contains(&MediaCodec::H264));
    }

    #[tokio::test]
    async fn test_mobile_optimization() {
        let device_profile = DeviceProfile::default();
        let network_profile = NetworkProfile::default();
        let battery_config = BatteryAwareConfig::default();
        
        let optimization = MobileOptimization::new(device_profile, network_profile, battery_config);
        
        let media = MediaFile::new(
            FileId::new_v4(),
            vec![0u8; 1024 * 1024], // 1MB file
            "image/jpeg".to_string(),
        );

        let optimized = optimization.optimize_for_mobile(&media).await.unwrap();
        assert!(!optimized.optimized_versions.is_empty() || !optimized.thumbnails.is_empty());
    }

    #[test]
    fn test_quality_selection() {
        let device_profile = DeviceProfile::default();
        let network_profile = NetworkProfile {
            bandwidth_mbps: 10.0,
            is_metered: false,
            ..NetworkProfile::default()
        };
        let battery_config = BatteryAwareConfig::default();
        
        let optimization = MobileOptimization::new(device_profile, network_profile, battery_config);
        
        let quality = optimization.select_quality_for_bandwidth(5_000_000); // 5 Mbps
        assert_eq!(quality.name, "High");
        
        let low_quality = optimization.select_quality_for_bandwidth(500_000); // 0.5 Mbps
        assert_eq!(low_quality.name, "Low");
    }

    #[test]
    fn test_codec_selection() {
        let mut device_profile = DeviceProfile::default();
        device_profile.supported_codecs = vec![MediaCodec::H264, MediaCodec::VP9, MediaCodec::JPEG];
        
        let optimization = MobileOptimization::new(
            device_profile,
            NetworkProfile::default(),
            BatteryAwareConfig::default(),
        );
        
        let best_video_codec = optimization.get_best_codec(MediaType::Video);
        assert_eq!(best_video_codec, Some(MediaCodec::VP9)); // VP9 is better than H264
        
        let best_image_codec = optimization.get_best_codec(MediaType::Image);
        assert_eq!(best_image_codec, Some(MediaCodec::JPEG)); // Only JPEG supported
    }

    #[test]
    fn test_battery_aware_strategy() {
        let device_profile = DeviceProfile::default();
        let network_profile = NetworkProfile::default();
        let battery_config = BatteryAwareConfig::default();
        
        let optimization = MobileOptimization::new(device_profile, network_profile, battery_config);
        
        let high_battery_strategy = optimization.get_processing_strategy(0.8); // 80%
        let low_battery_strategy = optimization.get_processing_strategy(0.1);  // 10%
        
        assert_ne!(high_battery_strategy as u8, low_battery_strategy as u8);
    }
}
