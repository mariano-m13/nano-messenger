/// Web Browser Support
/// 
/// Provides web browser compatibility features including format conversion,
/// Progressive Web App support, and WebAssembly-based cryptography.

use crate::error::Result;
use crate::media::{
    storage::FileId,
    compatibility::mobile::{MediaCodec, MediaFile},
    collaboration::galleries::SharedGallery,
};
use serde::{Deserialize, Serialize};


/// Browser capabilities detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserCapabilities {
    pub user_agent: String,
    pub browser_name: String,
    pub browser_version: String,
    pub platform: String,
    pub supported_video_codecs: Vec<MediaCodec>,
    pub supported_audio_codecs: Vec<MediaCodec>,
    pub supported_image_formats: Vec<MediaCodec>,
    pub supports_webassembly: bool,
    pub supports_web_workers: bool,
    pub supports_service_worker: bool,
    pub supports_webrtc: bool,
    pub supports_media_source_extensions: bool,
    pub supports_encrypted_media_extensions: bool,
    pub max_video_resolution: (u32, u32),
    pub hardware_acceleration: bool,
    pub local_storage_available: bool,
    pub indexed_db_available: bool,
}

impl BrowserCapabilities {
    /// Create capabilities from user agent string
    pub fn from_user_agent(user_agent: &str) -> Self {
        let (browser_name, browser_version) = Self::parse_user_agent(user_agent);
        let platform = Self::detect_platform(user_agent);
        
        Self {
            user_agent: user_agent.to_string(),
            browser_name: browser_name.clone(),
            browser_version: browser_version.clone(),
            platform: platform.clone(),
            supported_video_codecs: Self::get_supported_video_codecs(&browser_name, &browser_version),
            supported_audio_codecs: Self::get_supported_audio_codecs(&browser_name, &browser_version),
            supported_image_formats: Self::get_supported_image_formats(&browser_name, &browser_version),
            supports_webassembly: Self::supports_webassembly(&browser_name, &browser_version),
            supports_web_workers: true, // Widely supported
            supports_service_worker: Self::supports_service_worker(&browser_name, &browser_version),
            supports_webrtc: Self::supports_webrtc(&browser_name, &browser_version),
            supports_media_source_extensions: Self::supports_mse(&browser_name, &browser_version),
            supports_encrypted_media_extensions: Self::supports_eme(&browser_name, &browser_version),
            max_video_resolution: Self::get_max_video_resolution(&browser_name, &platform),
            hardware_acceleration: Self::has_hardware_acceleration(&browser_name, &platform),
            local_storage_available: true, // Widely supported
            indexed_db_available: true,    // Widely supported
        }
    }

    /// Parse browser name and version from user agent
    fn parse_user_agent(user_agent: &str) -> (String, String) {
        // Simplified user agent parsing - in practice would use a proper library
        if user_agent.contains("Chrome") {
            let version = Self::extract_version(user_agent, "Chrome/");
            ("Chrome".to_string(), version)
        } else if user_agent.contains("Firefox") {
            let version = Self::extract_version(user_agent, "Firefox/");
            ("Firefox".to_string(), version)
        } else if user_agent.contains("Safari") && !user_agent.contains("Chrome") {
            let version = Self::extract_version(user_agent, "Version/");
            ("Safari".to_string(), version)
        } else if user_agent.contains("Edge") {
            let version = Self::extract_version(user_agent, "Edge/");
            ("Edge".to_string(), version)
        } else {
            ("Unknown".to_string(), "0.0".to_string())
        }
    }

    fn extract_version(user_agent: &str, prefix: &str) -> String {
        if let Some(start) = user_agent.find(prefix) {
            let version_start = start + prefix.len();
            let version_part = &user_agent[version_start..];
            if let Some(end) = version_part.find(|c: char| c == ' ' || c == ')' || c == ';') {
                version_part[..end].to_string()
            } else {
                version_part.to_string()
            }
        } else {
            "0.0".to_string()
        }
    }

    fn detect_platform(user_agent: &str) -> String {
        if user_agent.contains("Windows") {
            "Windows".to_string()
        } else if user_agent.contains("Mac") {
            "macOS".to_string()
        } else if user_agent.contains("Linux") {
            "Linux".to_string()
        } else if user_agent.contains("Android") {
            "Android".to_string()
        } else if user_agent.contains("iPhone") || user_agent.contains("iPad") {
            "iOS".to_string()
        } else {
            "Unknown".to_string()
        }
    }

    fn get_supported_video_codecs(browser: &str, version: &str) -> Vec<MediaCodec> {
        let version_number: f32 = version.split('.').next()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0.0);

        match browser {
            "Chrome" => {
                let mut codecs = vec![MediaCodec::H264, MediaCodec::VP8, MediaCodec::VP9];
                if version_number >= 90.0 {
                    codecs.push(MediaCodec::AV1);
                }
                codecs
            }
            "Firefox" => {
                let mut codecs = vec![MediaCodec::H264, MediaCodec::VP8, MediaCodec::VP9];
                if version_number >= 67.0 {
                    codecs.push(MediaCodec::AV1);
                }
                codecs
            }
            "Safari" => vec![MediaCodec::H264, MediaCodec::H265],
            "Edge" => vec![MediaCodec::H264, MediaCodec::VP9, MediaCodec::AV1],
            _ => vec![MediaCodec::H264],
        }
    }

    fn get_supported_audio_codecs(browser: &str, _version: &str) -> Vec<MediaCodec> {
        match browser {
            "Chrome" | "Edge" => vec![MediaCodec::AAC, MediaCodec::MP3, MediaCodec::Opus, MediaCodec::Vorbis],
            "Firefox" => vec![MediaCodec::AAC, MediaCodec::MP3, MediaCodec::Opus, MediaCodec::Vorbis],
            "Safari" => vec![MediaCodec::AAC, MediaCodec::MP3],
            _ => vec![MediaCodec::MP3, MediaCodec::AAC],
        }
    }

    fn get_supported_image_formats(browser: &str, version: &str) -> Vec<MediaCodec> {
        let version_number: f32 = version.split('.').next()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0.0);

        let mut formats = vec![MediaCodec::JPEG, MediaCodec::PNG];

        match browser {
            "Chrome" => {
                formats.push(MediaCodec::WebP);
                if version_number >= 85.0 {
                    formats.push(MediaCodec::AVIF);
                }
            }
            "Firefox" => {
                if version_number >= 65.0 {
                    formats.push(MediaCodec::WebP);
                }
                if version_number >= 93.0 {
                    formats.push(MediaCodec::AVIF);
                }
            }
            "Safari" => {
                if version_number >= 14.0 {
                    formats.push(MediaCodec::WebP);
                }
                formats.push(MediaCodec::HEIF);
            }
            "Edge" => {
                formats.push(MediaCodec::WebP);
                if version_number >= 93.0 {
                    formats.push(MediaCodec::AVIF);
                }
            }
            _ => {}
        }

        formats
    }

    fn supports_webassembly(browser: &str, version: &str) -> bool {
        let version_number: f32 = version.split('.').next()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0.0);

        match browser {
            "Chrome" => version_number >= 57.0,
            "Firefox" => version_number >= 52.0,
            "Safari" => version_number >= 11.0,
            "Edge" => version_number >= 16.0,
            _ => false,
        }
    }

    fn supports_service_worker(browser: &str, version: &str) -> bool {
        let version_number: f32 = version.split('.').next()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0.0);

        match browser {
            "Chrome" => version_number >= 40.0,
            "Firefox" => version_number >= 44.0,
            "Safari" => version_number >= 11.1,
            "Edge" => version_number >= 17.0,
            _ => false,
        }
    }

    fn supports_webrtc(browser: &str, version: &str) -> bool {
        let version_number: f32 = version.split('.').next()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0.0);

        match browser {
            "Chrome" => version_number >= 23.0,
            "Firefox" => version_number >= 22.0,
            "Safari" => version_number >= 11.0,
            "Edge" => version_number >= 12.0,
            _ => false,
        }
    }

    fn supports_mse(browser: &str, version: &str) -> bool {
        let version_number: f32 = version.split('.').next()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0.0);

        match browser {
            "Chrome" => version_number >= 31.0,
            "Firefox" => version_number >= 42.0,
            "Safari" => version_number >= 8.0,
            "Edge" => version_number >= 12.0,
            _ => false,
        }
    }

    fn supports_eme(browser: &str, version: &str) -> bool {
        let version_number: f32 = version.split('.').next()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0.0);

        match browser {
            "Chrome" => version_number >= 42.0,
            "Firefox" => version_number >= 47.0,
            "Safari" => version_number >= 12.1,
            "Edge" => version_number >= 13.0,
            _ => false,
        }
    }

    fn get_max_video_resolution(browser: &str, platform: &str) -> (u32, u32) {
        match (browser, platform) {
            ("Chrome", "Windows") | ("Chrome", "macOS") | ("Chrome", "Linux") => (3840, 2160), // 4K
            ("Firefox", "Windows") | ("Firefox", "macOS") | ("Firefox", "Linux") => (3840, 2160),
            ("Safari", "macOS") => (3840, 2160),
            ("Safari", "iOS") => (1920, 1080),
            ("Chrome", "Android") => (1920, 1080),
            _ => (1920, 1080), // HD fallback
        }
    }

    fn has_hardware_acceleration(browser: &str, platform: &str) -> bool {
        match (browser, platform) {
            ("Chrome", "Windows") | ("Chrome", "macOS") => true,
            ("Safari", "macOS") | ("Safari", "iOS") => true,
            ("Edge", "Windows") => true,
            _ => false,
        }
    }
}

/// Web media package with multiple format fallbacks
#[derive(Debug, Clone)]
pub struct WebMediaPackage {
    pub primary_format: MediaFile,
    pub fallback_formats: Vec<MediaFile>,
    pub streaming_manifest: Option<StreamingManifest>,
    pub poster_image: Option<MediaFile>,
    pub subtitles: Vec<SubtitleTrack>,
    pub metadata: WebMediaMetadata,
}

/// Streaming manifest for adaptive streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingManifest {
    pub manifest_type: StreamingManifestType,
    pub base_url: String,
    pub quality_levels: Vec<QualityVariant>,
    pub duration: f64,
    pub segments: Vec<MediaSegment>,
}

/// Streaming manifest types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StreamingManifestType {
    HLS,  // HTTP Live Streaming (m3u8)
    DASH, // Dynamic Adaptive Streaming over HTTP (mpd)
    MSE,  // Media Source Extensions
}

/// Quality variant for adaptive streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityVariant {
    pub id: String,
    pub bandwidth: u32,
    pub resolution: (u32, u32),
    pub codec: String,
    pub url: String,
}

/// Media segment for streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaSegment {
    pub index: u32,
    pub duration: f64,
    pub url: String,
    pub size: u64,
}

/// Subtitle track
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleTrack {
    pub language: String,
    pub label: String,
    pub url: String,
    pub format: SubtitleFormat,
}

/// Subtitle formats
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SubtitleFormat {
    WebVTT,
    SRT,
    TTML,
}

/// Web media metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebMediaMetadata {
    pub optimized_for_browser: String,
    pub supports_streaming: bool,
    pub requires_polyfill: bool,
    pub estimated_load_time: f64, // seconds
    pub cache_strategy: CacheStrategy,
}

/// Cache strategy for web media
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CacheStrategy {
    NoCache,
    ShortTerm,   // Cache for 1 hour
    MediumTerm,  // Cache for 1 day
    LongTerm,    // Cache for 1 week
    Permanent,   // Cache indefinitely
}

/// Progressive Web App manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PWAManifest {
    pub name: String,
    pub short_name: String,
    pub description: String,
    pub start_url: String,
    pub display: String,
    pub theme_color: String,
    pub background_color: String,
    pub icons: Vec<PWAIcon>,
    pub screenshots: Vec<PWAScreenshot>,
    pub categories: Vec<String>,
    pub share_target: Option<ShareTarget>,
}

/// PWA icon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PWAIcon {
    pub src: String,
    pub sizes: String,
    #[serde(rename = "type")]
    pub icon_type: String,
    pub purpose: Option<String>,
}

/// PWA screenshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PWAScreenshot {
    pub src: String,
    pub sizes: String,
    #[serde(rename = "type")]
    pub image_type: String,
    pub platform: Option<String>,
}

/// Share target for PWA
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareTarget {
    pub action: String,
    pub method: String,
    pub params: ShareParams,
}

/// Share parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareParams {
    pub title: String,
    pub text: String,
    pub url: String,
    pub files: Vec<ShareFileParams>,
}

/// Share file parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareFileParams {
    pub name: String,
    pub accept: Vec<String>,
}

/// Web optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebOptimizationConfig {
    pub target_browsers: Vec<String>,
    pub enable_progressive_loading: bool,
    pub enable_streaming: bool,
    pub enable_webassembly_crypto: bool,
    pub max_file_size_mb: u32,
    pub chunk_size_kb: u32,
    pub cache_strategy: CacheStrategy,
    pub enable_service_worker: bool,
}

impl Default for WebOptimizationConfig {
    fn default() -> Self {
        Self {
            target_browsers: vec![
                "Chrome >= 80".to_string(),
                "Firefox >= 75".to_string(),
                "Safari >= 13".to_string(),
                "Edge >= 80".to_string(),
            ],
            enable_progressive_loading: true,
            enable_streaming: true,
            enable_webassembly_crypto: true,
            max_file_size_mb: 100,
            chunk_size_kb: 1024,
            cache_strategy: CacheStrategy::MediumTerm,
            enable_service_worker: true,
        }
    }
}

/// Web media support manager
pub struct WebMediaSupport {
    pub browser_capabilities: BrowserCapabilities,
    pub webassembly_crypto: bool,
    pub config: WebOptimizationConfig,
}

impl WebMediaSupport {
    /// Create new web media support manager
    pub fn new(
        browser_capabilities: BrowserCapabilities,
        webassembly_crypto: bool,
        config: WebOptimizationConfig,
    ) -> Self {
        Self {
            browser_capabilities,
            webassembly_crypto,
            config,
        }
    }

    /// Prepare media for web browser
    pub async fn prepare_for_web(&self, media: &MediaFile) -> Result<WebMediaPackage> {
        let primary_format = self.optimize_for_browser(media).await?;
        let fallback_formats = self.create_fallback_formats(media).await?;
        
        let streaming_manifest = if self.config.enable_streaming && media.size > (10 * 1024 * 1024) {
            Some(self.create_streaming_manifest(media).await?)
        } else {
            None
        };

        let poster_image = if media.mime_type.starts_with("video/") {
            Some(self.create_poster_image(media).await?)
        } else {
            None
        };

        let metadata = WebMediaMetadata {
            optimized_for_browser: self.browser_capabilities.browser_name.clone(),
            supports_streaming: streaming_manifest.is_some(),
            requires_polyfill: self.requires_polyfill(&primary_format),
            estimated_load_time: self.estimate_load_time(&primary_format),
            cache_strategy: self.config.cache_strategy,
        };

        Ok(WebMediaPackage {
            primary_format,
            fallback_formats,
            streaming_manifest,
            poster_image,
            subtitles: Vec::new(), // Would be populated if available
            metadata,
        })
    }

    /// Create Progressive Web App manifest for a gallery
    pub async fn create_pwa_manifest(&self, gallery: &SharedGallery) -> Result<PWAManifest> {
        Ok(PWAManifest {
            name: format!("Nano Messenger - {}", gallery.title),
            short_name: "Nano Gallery".to_string(),
            description: gallery.description.clone()
                .unwrap_or_else(|| "Quantum-resistant media sharing".to_string()),
            start_url: format!("/gallery/{}", gallery.gallery_id),
            display: "standalone".to_string(),
            theme_color: "#2196F3".to_string(),
            background_color: "#FFFFFF".to_string(),
            icons: vec![
                PWAIcon {
                    src: "/icons/icon-192.png".to_string(),
                    sizes: "192x192".to_string(),
                    icon_type: "image/png".to_string(),
                    purpose: Some("any maskable".to_string()),
                },
                PWAIcon {
                    src: "/icons/icon-512.png".to_string(),
                    sizes: "512x512".to_string(),
                    icon_type: "image/png".to_string(),
                    purpose: Some("any maskable".to_string()),
                },
            ],
            screenshots: vec![
                PWAScreenshot {
                    src: "/screenshots/desktop.png".to_string(),
                    sizes: "1280x720".to_string(),
                    image_type: "image/png".to_string(),
                    platform: Some("wide".to_string()),
                },
                PWAScreenshot {
                    src: "/screenshots/mobile.png".to_string(),
                    sizes: "375x812".to_string(),
                    image_type: "image/png".to_string(),
                    platform: Some("narrow".to_string()),
                },
            ],
            categories: vec![
                "productivity".to_string(),
                "social".to_string(),
                "communication".to_string(),
            ],
            share_target: Some(ShareTarget {
                action: "/share".to_string(),
                method: "POST".to_string(),
                params: ShareParams {
                    title: "title".to_string(),
                    text: "text".to_string(),
                    url: "url".to_string(),
                    files: vec![
                        ShareFileParams {
                            name: "media".to_string(),
                            accept: vec![
                                "image/*".to_string(),
                                "video/*".to_string(),
                                "audio/*".to_string(),
                            ],
                        },
                    ],
                },
            }),
        })
    }

    /// Check if WebAssembly crypto is supported
    pub fn supports_webassembly_crypto(&self) -> bool {
        self.browser_capabilities.supports_webassembly && self.webassembly_crypto
    }

    /// Get best codec for browser
    pub fn get_best_codec_for_browser(&self, media_type: &str) -> Option<MediaCodec> {
        match media_type {
            mime_type if mime_type.starts_with("video/") => {
                // Prefer modern codecs if supported
                for codec in &[MediaCodec::AV1, MediaCodec::VP9, MediaCodec::H265, MediaCodec::H264] {
                    if self.browser_capabilities.supported_video_codecs.contains(codec) {
                        return Some(*codec);
                    }
                }
                None
            }
            mime_type if mime_type.starts_with("audio/") => {
                for codec in &[MediaCodec::Opus, MediaCodec::AAC, MediaCodec::MP3] {
                    if self.browser_capabilities.supported_audio_codecs.contains(codec) {
                        return Some(*codec);
                    }
                }
                None
            }
            mime_type if mime_type.starts_with("image/") => {
                for codec in &[MediaCodec::AVIF, MediaCodec::WebP, MediaCodec::JPEG] {
                    if self.browser_capabilities.supported_image_formats.contains(codec) {
                        return Some(*codec);
                    }
                }
                None
            }
            _ => None,
        }
    }

    /// Generate service worker JavaScript
    pub fn generate_service_worker(&self) -> String {
        format!(r#"
// Nano Messenger Service Worker
// Generated for {} {}

const CACHE_NAME = 'nano-messenger-v1';
const CACHE_STRATEGY = '{}';

self.addEventListener('install', event => {{
    console.log('Service Worker installing');
    self.skipWaiting();
}});

self.addEventListener('activate', event => {{
    console.log('Service Worker activating');
    event.waitUntil(clients.claim());
}});

self.addEventListener('fetch', event => {{
    if (event.request.url.includes('/media/')) {{
        event.respondWith(handleMediaRequest(event.request));
    }} else {{
        event.respondWith(fetch(event.request));
    }}
}});

async function handleMediaRequest(request) {{
    const cache = await caches.open(CACHE_NAME);
    const cachedResponse = await cache.match(request);
    
    if (cachedResponse) {{
        return cachedResponse;
    }}
    
    try {{
        const response = await fetch(request);
        if (response.status === 200) {{
            const responseClone = response.clone();
            await cache.put(request, responseClone);
        }}
        return response;
    }} catch (error) {{
        console.error('Fetch failed:', error);
        throw error;
    }}
}}

// Handle file sharing from other apps
self.addEventListener('message', event => {{
    if (event.data && event.data.type === 'SHARE_TARGET') {{
        // Handle shared files
        console.log('Received shared files:', event.data.files);
    }}
}});
"#, 
            self.browser_capabilities.browser_name,
            self.browser_capabilities.browser_version,
            format!("{:?}", self.config.cache_strategy)
        )
    }

    // Helper methods

    async fn optimize_for_browser(&self, media: &MediaFile) -> Result<MediaFile> {
        let best_codec = self.get_best_codec_for_browser(&media.mime_type);
        
        if let Some(codec) = best_codec {
            if media.codec.is_none() || media.codec != Some(codec) {
                // Would convert to best codec here
                let mut optimized = media.clone();
                optimized.codec = Some(codec);
                optimized.mime_type = self.get_mime_type_for_codec(codec);
                return Ok(optimized);
            }
        }
        
        Ok(media.clone())
    }

    async fn create_fallback_formats(&self, media: &MediaFile) -> Result<Vec<MediaFile>> {
        let mut fallbacks = Vec::new();
        
        // Create basic fallback formats for broader compatibility
        if media.mime_type.starts_with("video/") {
            if self.browser_capabilities.supported_video_codecs.contains(&MediaCodec::H264) {
                let mut h264_version = media.clone();
                h264_version.codec = Some(MediaCodec::H264);
                h264_version.mime_type = "video/mp4".to_string();
                fallbacks.push(h264_version);
            }
        } else if media.mime_type.starts_with("image/") {
            if self.browser_capabilities.supported_image_formats.contains(&MediaCodec::JPEG) {
                let mut jpeg_version = media.clone();
                jpeg_version.codec = Some(MediaCodec::JPEG);
                jpeg_version.mime_type = "image/jpeg".to_string();
                fallbacks.push(jpeg_version);
            }
        }
        
        Ok(fallbacks)
    }

    async fn create_streaming_manifest(&self, media: &MediaFile) -> Result<StreamingManifest> {
        let manifest_type = if self.browser_capabilities.supports_media_source_extensions {
            StreamingManifestType::MSE
        } else {
            StreamingManifestType::HLS
        };

        Ok(StreamingManifest {
            manifest_type,
            base_url: format!("/stream/{}", media.file_id),
            quality_levels: vec![
                QualityVariant {
                    id: "720p".to_string(),
                    bandwidth: 2500000,
                    resolution: (1280, 720),
                    codec: "h264".to_string(),
                    url: format!("/stream/{}/720p.m3u8", media.file_id),
                },
                QualityVariant {
                    id: "480p".to_string(),
                    bandwidth: 1000000,
                    resolution: (854, 480),
                    codec: "h264".to_string(),
                    url: format!("/stream/{}/480p.m3u8", media.file_id),
                },
            ],
            duration: media.duration.map(|d| d.as_secs_f64()).unwrap_or(0.0),
            segments: Vec::new(), // Would be populated with actual segments
        })
    }

    async fn create_poster_image(&self, _media: &MediaFile) -> Result<MediaFile> {
        // Would extract frame from video
        Ok(MediaFile::new(
            FileId::new_v4(),
            vec![0u8; 1024], // Placeholder
            "image/jpeg".to_string(),
        ))
    }

    fn requires_polyfill(&self, media: &MediaFile) -> bool {
        if let Some(codec) = media.codec {
            match codec {
                MediaCodec::AVIF => !self.browser_capabilities.supported_image_formats.contains(&codec),
                MediaCodec::AV1 => !self.browser_capabilities.supported_video_codecs.contains(&codec),
                MediaCodec::Opus => !self.browser_capabilities.supported_audio_codecs.contains(&codec),
                _ => false,
            }
        } else {
            false
        }
    }

    fn estimate_load_time(&self, media: &MediaFile) -> f64 {
        // Rough estimation based on file size and typical connection speeds
        let size_mb = media.size as f64 / (1024.0 * 1024.0);
        
        // Assume average connection speed based on browser/platform
        let speed_mbps = match self.browser_capabilities.platform.as_str() {
            "Android" | "iOS" => 5.0,  // Mobile connection
            _ => 25.0,                  // Desktop/WiFi connection
        };
        
        (size_mb * 8.0) / speed_mbps // Convert to seconds
    }

    fn get_mime_type_for_codec(&self, codec: MediaCodec) -> String {
        match codec {
            MediaCodec::H264 | MediaCodec::H265 | MediaCodec::AV1 => "video/mp4".to_string(),
            MediaCodec::VP8 | MediaCodec::VP9 => "video/webm".to_string(),
            MediaCodec::AAC => "audio/aac".to_string(),
            MediaCodec::MP3 => "audio/mpeg".to_string(),
            MediaCodec::Opus => "audio/opus".to_string(),
            MediaCodec::Vorbis => "audio/ogg".to_string(),
            MediaCodec::JPEG => "image/jpeg".to_string(),
            MediaCodec::PNG => "image/png".to_string(),
            MediaCodec::WebP => "image/webp".to_string(),
            MediaCodec::AVIF => "image/avif".to_string(),
            MediaCodec::HEIF => "image/heif".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browser_capabilities_parsing() {
        let chrome_ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36";
        let caps = BrowserCapabilities::from_user_agent(chrome_ua);
        
        assert_eq!(caps.browser_name, "Chrome");
        assert!(caps.browser_version.starts_with("91"));
        assert_eq!(caps.platform, "Windows");
        assert!(caps.supports_webassembly);
        assert!(caps.supported_video_codecs.contains(&MediaCodec::H264));
    }

    #[test]
    fn test_firefox_capabilities() {
        let firefox_ua = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:89.0) Gecko/20100101 Firefox/89.0";
        let caps = BrowserCapabilities::from_user_agent(firefox_ua);
        
        assert_eq!(caps.browser_name, "Firefox");
        assert_eq!(caps.platform, "macOS");
        assert!(caps.supported_audio_codecs.contains(&MediaCodec::Opus));
    }

    #[test]
    fn test_safari_capabilities() {
        let safari_ua = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.1.1 Safari/605.1.15";
        let caps = BrowserCapabilities::from_user_agent(safari_ua);
        
        assert_eq!(caps.browser_name, "Safari");
        assert!(caps.supported_video_codecs.contains(&MediaCodec::H264));
        assert!(caps.supported_image_formats.contains(&MediaCodec::HEIF));
    }

    #[test]
    fn test_web_optimization_config_defaults() {
        let config = WebOptimizationConfig::default();
        assert!(config.enable_progressive_loading);
        assert!(config.enable_streaming);
        assert!(config.enable_webassembly_crypto);
        assert_eq!(config.max_file_size_mb, 100);
    }

    #[tokio::test]
    async fn test_web_media_preparation() {
        let caps = BrowserCapabilities::from_user_agent(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 Chrome/91.0.4472.124"
        );
        let web_support = WebMediaSupport::new(caps, true, WebOptimizationConfig::default());
        
        let media = MediaFile::new(
            FileId::new_v4(),
            vec![0u8; 1024 * 1024], // 1MB
            "video/mp4".to_string(),
        );

        let package = web_support.prepare_for_web(&media).await.unwrap();
        assert_eq!(package.primary_format.mime_type, "video/mp4");
        assert!(!package.fallback_formats.is_empty());
    }

    #[test]
    fn test_codec_selection() {
        let caps = BrowserCapabilities::from_user_agent(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 Chrome/91.0.4472.124"
        );
        let web_support = WebMediaSupport::new(caps, true, WebOptimizationConfig::default());
        
        let video_codec = web_support.get_best_codec_for_browser("video/mp4");
        assert!(video_codec.is_some());
        
        let image_codec = web_support.get_best_codec_for_browser("image/jpeg");
        assert!(image_codec.is_some());
    }

    #[test]
    fn test_service_worker_generation() {
        let caps = BrowserCapabilities::from_user_agent("Chrome/91.0");
        let web_support = WebMediaSupport::new(caps, true, WebOptimizationConfig::default());
        
        let sw_code = web_support.generate_service_worker();
        assert!(sw_code.contains("Service Worker"));
        assert!(sw_code.contains("handleMediaRequest"));
        assert!(sw_code.contains("CACHE_NAME"));
    }

    #[test]
    fn test_streaming_manifest_types() {
        let hls = StreamingManifestType::HLS;
        let dash = StreamingManifestType::DASH;
        let mse = StreamingManifestType::MSE;
        
        // Test serialization
        let hls_json = serde_json::to_string(&hls).unwrap();
        assert!(hls_json.contains("HLS"));
        
        let dash_json = serde_json::to_string(&dash).unwrap();
        assert!(dash_json.contains("DASH"));
        
        let mse_json = serde_json::to_string(&mse).unwrap();
        assert!(mse_json.contains("MSE"));
    }
}
