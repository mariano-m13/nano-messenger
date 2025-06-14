/// Sessions 9-12: Complete Media & Security Architecture
/// 
/// This module provides comprehensive quantum-resistant media capabilities
/// for the nano-messenger, including:
/// - File storage abstraction with multiple backends (Session 9)
/// - Quantum-safe file encryption (Session 9)
/// - File metadata management (Session 9)
/// - Secure file transfer protocols (Session 9)
/// - Media processing and optimization (Session 10)
/// - Large file chunking and streaming (Session 11)
/// - Deduplication and collaboration features (Session 11)
/// - Cross-platform compatibility (Session 11)
/// - Comprehensive security & threat detection (Session 12)
/// - GDPR & HIPAA compliance (Session 12)
/// - Enterprise audit & reporting (Session 12)

pub mod storage;
pub mod encryption;
pub mod metadata;
pub mod transfer;
pub mod processing;

// Session 11: Advanced Media Features
pub mod chunking;
pub mod deduplication;
pub mod streaming;
pub mod collaboration;
pub mod compatibility;

// Session 12: Security & Compliance
pub mod security;
pub mod compliance;

// Re-export main types for easy access
pub use storage::{
    FileStorage, LocalFileStorage, StorageLocation, FileId, StorageError
};
pub use encryption::{
    FileEncryption, EncryptedFile, EncryptionMetadata, FileKey
};
pub use metadata::{
    FileMetadata, FilePermissions, MetadataStore, FileReference
};
pub use transfer::{
    FileTransferManager, FileUpload, DecryptedFile, TransferProgress
};
pub use processing::{
    MediaProcessingManager, MediaProcessingConfig, ImageProcessingConfig,
    ProcessingResult, ProcessingStatistics, MediaType
};

// Session 11: Advanced Media Features
pub use chunking::{
    ChunkedTransfer, LargeFile, ChunkedUploadResult, ResumeResult,
    StreamingDownload, RetryStrategy, UploadId, ChunkInfo
};
pub use deduplication::{
    FileDeduplication, DeduplicationResult, FileReference as DedupeFileReference,
    ChunkDeduplicationResult, HashAlgorithm, ContentHash
};
pub use streaming::{
    MediaStreamingServer, EncryptedStream, LiveStream, ScreenShareStream,
    StreamingProtocol, QualityLevel as StreamingQualityLevel, StreamingStats
};
pub use collaboration::{
    SharedGallery, GalleryPermissions, GalleryManager, MediaInteractions,
    MediaComment, MediaAnnotation, ReactionType
};
pub use compatibility::{
    MobileOptimization, WebMediaSupport, DeviceProfile, BrowserCapabilities,
    OptimizedMediaSet, WebMediaPackage
};

// Session 12: Security & Compliance
pub use security::{
    MediaSecurityManager, SecurityPolicy, SecurityAssessment,
    SecuredMedia, SecurityMeasure, UploadContext, MediaSecurityError
};
pub use security::MediaSecurityConfig as AdvancedSecurityConfig;
pub use compliance::{
    MediaComplianceManager, ComplianceConfig, ComplianceAssessment, 
    ComplianceError, DataCategory, ComplianceViolation, RemediationAction,
    MultiRegulationResult, ComplianceDashboard
};

use crate::error::{NanoError, Result};
use crate::crypto::CryptoMode;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;

/// Configuration for media subsystem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaConfig {
    pub enabled: bool,
    pub max_file_size_mb: u64,
    pub allowed_mime_types: Vec<String>,
    pub storage_backend: StorageBackend,
    pub storage_path: PathBuf,
    pub encryption: MediaEncryptionConfig,
    pub security: BasicMediaSecurityConfig,
    pub processing: MediaProcessingConfig,
    
    // Session 11: Advanced Features
    pub chunking: ChunkingConfig,
    pub deduplication: DeduplicationConfig,
    pub streaming: StreamingConfig,
    pub collaboration: CollaborationConfig,
    pub compatibility: CompatibilityConfig,
    
    // Session 12: Security & Compliance
    pub security_advanced: SecurityConfig,
    pub compliance: ComplianceConfig,
}

/// Storage backend configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageBackend {
    Local,
    S3 {
        bucket: String,
        region: String,
        endpoint: Option<String>,
    },
    Distributed {
        nodes: Vec<String>,
    },
}

/// Media encryption configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaEncryptionConfig {
    pub default_crypto_mode: CryptoMode,
    pub require_post_quantum_for_files: bool,
    pub chunk_size_mb: u64,
}

impl Default for MediaEncryptionConfig {
    fn default() -> Self {
        Self {
            default_crypto_mode: CryptoMode::Hybrid,
            require_post_quantum_for_files: true,
            chunk_size_mb: 10,
        }
    }
}

/// Basic media security configuration (from original Session 9)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicMediaSecurityConfig {
    pub virus_scanning_enabled: bool,
    pub content_type_validation: bool,
    pub file_extension_validation: bool,
    pub quarantine_suspicious_files: bool,
}

/// Session 11: Chunking configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkingConfig {
    pub enabled: bool,
    pub chunk_size_mb: u64,
    pub parallel_chunks: usize,
    pub max_retries: u32,
    pub enable_resume: bool,
    pub cleanup_interval_hours: u64,
}

/// Session 11: Deduplication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeduplicationConfig {
    pub enabled: bool,
    pub chunk_level_dedup: bool,
    pub hash_algorithm: String, // "blake2b512", "blake2b256", "sha3_512"
    pub gc_interval_hours: u64,
    pub max_reference_age_days: u64,
}

/// Session 11: Streaming configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingConfig {
    pub enabled: bool,
    pub max_concurrent_streams: u32,
    pub max_viewers_per_stream: u32,
    pub max_bitrate_mbps: u64,
    pub enable_webrtc: bool,
    pub enable_hls: bool,
    pub enable_dash: bool,
}

/// Session 11: Collaboration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationConfig {
    pub enabled: bool,
    pub max_galleries_per_user: u32,
    pub max_items_per_gallery: u32,
    pub enable_real_time_sync: bool,
    pub enable_annotations: bool,
    pub enable_comments: bool,
    pub enable_reactions: bool,
}

/// Session 11: Compatibility configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityConfig {
    pub mobile_optimization: bool,
    pub web_optimization: bool,
    pub progressive_loading: bool,
    pub adaptive_quality: bool,
    pub battery_awareness: bool,
    pub network_awareness: bool,
}

/// Session 12: Advanced Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub enabled: bool,
    pub scanning_enabled: bool,
    pub forensics_enabled: bool,
    pub access_control_enabled: bool,
    pub drm_enabled: bool,
    pub quantum_enhanced: bool,
    pub auto_remediation: bool,
}

// Default implementations for Session 11 configs
impl Default for ChunkingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            chunk_size_mb: 10,
            parallel_chunks: 4,
            max_retries: 3,
            enable_resume: true,
            cleanup_interval_hours: 24,
        }
    }
}

impl Default for DeduplicationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            chunk_level_dedup: true,
            hash_algorithm: "blake2b512".to_string(),
            gc_interval_hours: 6,
            max_reference_age_days: 30,
        }
    }
}

impl Default for StreamingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_concurrent_streams: 100,
            max_viewers_per_stream: 1000,
            max_bitrate_mbps: 10,
            enable_webrtc: true,
            enable_hls: true,
            enable_dash: true,
        }
    }
}

impl Default for CollaborationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_galleries_per_user: 50,
            max_items_per_gallery: 1000,
            enable_real_time_sync: true,
            enable_annotations: true,
            enable_comments: true,
            enable_reactions: true,
        }
    }
}

impl Default for CompatibilityConfig {
    fn default() -> Self {
        Self {
            mobile_optimization: true,
            web_optimization: true,
            progressive_loading: true,
            adaptive_quality: true,
            battery_awareness: true,
            network_awareness: true,
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            scanning_enabled: true,
            forensics_enabled: true,
            access_control_enabled: true,
            drm_enabled: true,
            quantum_enhanced: false,
            auto_remediation: false,
        }
    }
}

impl Default for MediaConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_file_size_mb: 5120, // 5GB for Session 11
            allowed_mime_types: vec![
                "image/*".to_string(),
                "video/*".to_string(),
                "audio/*".to_string(),
                "application/pdf".to_string(),
                "text/*".to_string(),
            ],
            storage_backend: StorageBackend::Local,
            storage_path: PathBuf::from("/var/lib/nano-messenger/files"),
            encryption: MediaEncryptionConfig {
                default_crypto_mode: CryptoMode::Hybrid,
                require_post_quantum_for_files: true,
                chunk_size_mb: 10,
            },
            security: BasicMediaSecurityConfig {
                virus_scanning_enabled: true,
                content_type_validation: true,
                file_extension_validation: true,
                quarantine_suspicious_files: true,
            },
            processing: MediaProcessingConfig::default(),
            // Session 11 defaults
            chunking: ChunkingConfig::default(),
            deduplication: DeduplicationConfig::default(),
            streaming: StreamingConfig::default(),
            collaboration: CollaborationConfig::default(),
            compatibility: CompatibilityConfig::default(),
            // Session 12 defaults
            security_advanced: SecurityConfig::default(),
            compliance: ComplianceConfig::default(),
        }
    }
}

impl MediaConfig {
    /// Validate the media configuration
    pub fn validate(&self) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        if self.max_file_size_mb == 0 {
            return Err(NanoError::Config(
                "Media max file size must be greater than 0".to_string()
            ));
        }

        if self.max_file_size_mb > 5120 { // 5GB limit for Session 11
            return Err(NanoError::Config(
                "Media max file size exceeds 5GB limit for Session 11".to_string()
            ));
        }

        if self.encryption.chunk_size_mb == 0 {
            return Err(NanoError::Config(
                "Media chunk size must be greater than 0".to_string()
            ));
        }

        if self.encryption.chunk_size_mb > 100 {
            return Err(NanoError::Config(
                "Media chunk size should not exceed 100MB for optimal performance".to_string()
            ));
        }

        // Validate storage path is not empty for local storage
        if matches!(self.storage_backend, StorageBackend::Local) {
            if self.storage_path.as_os_str().is_empty() {
                return Err(NanoError::Config(
                    "Local storage path cannot be empty".to_string()
                ));
            }
        }

        Ok(())
    }

    /// Get the maximum file size in bytes
    pub fn max_file_size_bytes(&self) -> u64 {
        self.max_file_size_mb * 1024 * 1024
    }

    /// Get the chunk size in bytes
    pub fn chunk_size_bytes(&self) -> u64 {
        self.encryption.chunk_size_mb * 1024 * 1024
    }

    /// Check if a MIME type is allowed
    pub fn is_mime_type_allowed(&self, mime_type: &str) -> bool {
        for allowed in &self.allowed_mime_types {
            if allowed.ends_with('*') {
                let prefix = &allowed[..allowed.len() - 1];
                if mime_type.starts_with(prefix) {
                    return true;
                }
            } else if allowed == mime_type {
                return true;
            }
        }
        false
    }
}

/// Media subsystem initialization and management
pub struct MediaSystem {
    config: MediaConfig,
    storage: Arc<dyn FileStorage>,
    transfer_manager: Option<FileTransferManager>,
    processing_manager: Option<MediaProcessingManager>,
    
    // Session 11: Advanced Media Features
    chunked_transfer: Option<ChunkedTransfer>,
    deduplication: Option<Arc<FileDeduplication>>,
    streaming_server: Option<MediaStreamingServer>,
    gallery_manager: Option<GalleryManager>,
    mobile_optimization: Option<MobileOptimization>,
    web_support: Option<WebMediaSupport>,
    
    // Session 12: Security & Compliance
    security_manager: Option<MediaSecurityManager>,
    compliance_manager: Option<MediaComplianceManager>,
}

impl MediaSystem {
    /// Create a new media system with the given configuration
    pub async fn new(config: MediaConfig) -> Result<Self> {
        config.validate()?;

        let storage: Arc<dyn FileStorage> = match &config.storage_backend {
            StorageBackend::Local => {
                Arc::new(LocalFileStorage::new(config.storage_path.clone()).await?)
            }
            #[cfg(feature = "s3-storage")]
            StorageBackend::S3 { bucket, region, endpoint } => {
                Arc::new(
                    crate::media::storage::S3FileStorage::new(
                        bucket.clone(),
                        region.clone(),
                        endpoint.clone(),
                    ).await?
                )
            }
            #[cfg(not(feature = "s3-storage"))]
            StorageBackend::S3 { .. } => {
                return Err(NanoError::Config(
                    "S3 storage backend not available. Enable 's3-storage' feature".to_string()
                ));
            }
            StorageBackend::Distributed { .. } => {
                return Err(NanoError::Config(
                    "Distributed storage not yet implemented".to_string()
                ));
            }
        };

        Ok(Self {
            config,
            storage,
            transfer_manager: None,
            processing_manager: None,
            
            // Session 11: Initialize as None, will be set up later
            chunked_transfer: None,
            deduplication: None,
            streaming_server: None,
            gallery_manager: None,
            mobile_optimization: None,
            web_support: None,
            
            // Session 12: Initialize as None, will be set up later
            security_manager: None,
            compliance_manager: None,
        })
    }

    /// Initialize the transfer manager
    pub async fn init_transfer_manager(&mut self) -> Result<()> {
        let encryption = FileEncryption::new(
            self.config.encryption.default_crypto_mode,
            self.config.encryption.chunk_size_mb,
        );

        let metadata_store = MetadataStore::new().await?;

        self.transfer_manager = Some(FileTransferManager::new(
            Arc::clone(&self.storage),
            encryption,
            metadata_store,
        ));

        Ok(())
    }

    /// Initialize the processing manager
    pub async fn init_processing_manager(&mut self) -> Result<()> {
        if self.config.processing.enabled {
            self.processing_manager = Some(
                MediaProcessingManager::new(self.config.processing.clone()).await?
            );
        }
        Ok(())
    }

    /// Initialize both transfer and processing managers
    pub async fn init_all_managers(&mut self) -> Result<()> {
        self.init_transfer_manager().await?;
        self.init_processing_manager().await?;
        Ok(())
    }

    // Session 11: Advanced Media Feature Initialization

    /// Initialize chunked transfer manager
    pub async fn init_chunked_transfer(&mut self) -> Result<()> {
        if !self.config.chunking.enabled {
            return Ok(());
        }

        let encryption = Arc::new(FileEncryption::new(
            self.config.encryption.default_crypto_mode,
            self.config.chunking.chunk_size_mb,
        ));

        let retry_strategy = RetryStrategy {
            max_retries: self.config.chunking.max_retries,
            initial_delay: std::time::Duration::from_millis(500),
            max_delay: std::time::Duration::from_secs(30),
            backoff_multiplier: 2.0,
        };

        self.chunked_transfer = Some(ChunkedTransfer::new(
            Arc::clone(&self.storage),
            encryption,
            (self.config.chunking.chunk_size_mb * 1024 * 1024) as usize,
            self.config.chunking.parallel_chunks,
            retry_strategy,
        ));

        Ok(())
    }

    /// Initialize deduplication manager
    pub async fn init_deduplication(&mut self) -> Result<()> {
        if !self.config.deduplication.enabled {
            return Ok(());
        }

        let hash_algorithm = match self.config.deduplication.hash_algorithm.as_str() {
            "blake2b256" => HashAlgorithm::Blake2b256,
            "sha3_512" => HashAlgorithm::Sha3_512,
            _ => HashAlgorithm::Blake2b512,
        };

        self.deduplication = Some(Arc::new(FileDeduplication::new(
            Arc::clone(&self.storage),
            hash_algorithm,
            self.config.deduplication.chunk_level_dedup,
        )));

        Ok(())
    }

    /// Initialize streaming server
    pub async fn init_streaming_server(&mut self) -> Result<()> {
        if !self.config.streaming.enabled {
            return Ok(());
        }

        let mut protocols = Vec::new();
        if self.config.streaming.enable_webrtc {
            protocols.push(StreamingProtocol::WebRTC);
        }
        if self.config.streaming.enable_hls {
            protocols.push(StreamingProtocol::HLS);
        }
        if self.config.streaming.enable_dash {
            protocols.push(StreamingProtocol::DASH);
        }
        protocols.push(StreamingProtocol::Custom); // Always support quantum-safe protocol

        let encryption = streaming::StreamEncryption::new(self.config.encryption.default_crypto_mode);
        let stream_limits = streaming::StreamLimits {
            max_concurrent_streams: self.config.streaming.max_concurrent_streams,
            max_viewers_per_stream: self.config.streaming.max_viewers_per_stream,
            max_bitrate: self.config.streaming.max_bitrate_mbps * 1_000_000,
            max_duration: std::time::Duration::from_secs(24 * 60 * 60), // 24 hours
        };

        self.streaming_server = Some(MediaStreamingServer::new(
            protocols,
            encryption,
            stream_limits,
        ));

        Ok(())
    }

    /// Initialize gallery manager
    pub async fn init_gallery_manager(&mut self) -> Result<()> {
        if !self.config.collaboration.enabled {
            return Ok(());
        }

        self.gallery_manager = Some(GalleryManager::new());
        Ok(())
    }

    /// Initialize mobile optimization
    pub async fn init_mobile_optimization(&mut self, device_profile: Option<compatibility::mobile::DeviceProfile>) -> Result<()> {
        if !self.config.compatibility.mobile_optimization {
            return Ok(());
        }

        let device_profile = device_profile.unwrap_or_default();
        let network_profile = compatibility::mobile::NetworkProfile::default();
        let battery_config = compatibility::mobile::BatteryAwareConfig::default();

        self.mobile_optimization = Some(MobileOptimization::new(
            device_profile,
            network_profile,
            battery_config,
        ));

        Ok(())
    }

    /// Initialize web support
    pub async fn init_web_support(&mut self, browser_capabilities: Option<compatibility::web::BrowserCapabilities>) -> Result<()> {
        if !self.config.compatibility.web_optimization {
            return Ok(());
        }

        let capabilities = browser_capabilities.unwrap_or_else(|| {
            // Default Chrome capabilities
            compatibility::web::BrowserCapabilities::from_user_agent(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 Chrome/91.0.4472.124"
            )
        });

        let web_config = compatibility::web::WebOptimizationConfig::default();

        self.web_support = Some(WebMediaSupport::new(
            capabilities,
            self.config.encryption.default_crypto_mode == CryptoMode::Hybrid,
            web_config,
        ));

        Ok(())
    }

    // Session 12: Security & Compliance Initialization

    /// Initialize security manager
    pub async fn init_security_manager(&mut self) -> Result<()> {
        if !self.config.security_advanced.enabled {
            return Ok(());
        }

        let security_config = AdvancedSecurityConfig {
            crypto_mode: self.config.encryption.default_crypto_mode.clone(),
            scanning_enabled: self.config.security_advanced.scanning_enabled,
            forensics_enabled: self.config.security_advanced.forensics_enabled,
            access_control_enabled: self.config.security_advanced.access_control_enabled,
            drm_enabled: self.config.security_advanced.drm_enabled,
            e2e_encryption_enabled: true,
            quantum_enhanced: self.config.security_advanced.quantum_enhanced,
            security_policy: security::SecurityPolicy::default(),
        };

        self.security_manager = Some(MediaSecurityManager::new(security_config));
        Ok(())
    }

    /// Initialize compliance manager
    pub async fn init_compliance_manager(&mut self) -> Result<()> {
        self.compliance_manager = Some(MediaComplianceManager::new(self.config.compliance.clone()));
        Ok(())
    }

    /// Initialize all Session 11 advanced features
    pub async fn init_all_advanced_features(&mut self) -> Result<()> {
        self.init_chunked_transfer().await?;
        self.init_deduplication().await?;
        self.init_streaming_server().await?;
        self.init_gallery_manager().await?;
        self.init_mobile_optimization(None).await?;
        self.init_web_support(None).await?;
        Ok(())
    }

    /// Initialize all Session 12 security and compliance features
    pub async fn init_security_and_compliance(&mut self) -> Result<()> {
        self.init_security_manager().await?;
        self.init_compliance_manager().await?;
        Ok(())
    }

    /// Initialize everything (Sessions 9, 10, 11, and 12)
    pub async fn init_complete_system(&mut self) -> Result<()> {
        self.init_all_managers().await?;
        self.init_all_advanced_features().await?;
        self.init_security_and_compliance().await?;
        Ok(())
    }

    /// Get the media configuration
    pub fn config(&self) -> &MediaConfig {
        &self.config
    }

    /// Get a reference to the file transfer manager
    pub fn transfer_manager(&self) -> Option<&FileTransferManager> {
        self.transfer_manager.as_ref()
    }

    /// Get a mutable reference to the file transfer manager
    pub fn transfer_manager_mut(&mut self) -> Option<&mut FileTransferManager> {
        self.transfer_manager.as_mut()
    }

    /// Get a reference to the media processing manager
    pub fn processing_manager(&self) -> Option<&MediaProcessingManager> {
        self.processing_manager.as_ref()
    }

    /// Get a mutable reference to the media processing manager
    pub fn processing_manager_mut(&mut self) -> Option<&mut MediaProcessingManager> {
        self.processing_manager.as_mut()
    }

    // Session 11: Advanced Feature Getters

    /// Get a reference to the chunked transfer manager
    pub fn chunked_transfer(&self) -> Option<&ChunkedTransfer> {
        self.chunked_transfer.as_ref()
    }

    /// Get a reference to the deduplication manager
    pub fn deduplication(&self) -> Option<&Arc<FileDeduplication>> {
        self.deduplication.as_ref()
    }

    /// Get a reference to the streaming server
    pub fn streaming_server(&self) -> Option<&MediaStreamingServer> {
        self.streaming_server.as_ref()
    }

    /// Get a reference to the gallery manager
    pub fn gallery_manager(&self) -> Option<&GalleryManager> {
        self.gallery_manager.as_ref()
    }

    /// Get a reference to the mobile optimization
    pub fn mobile_optimization(&self) -> Option<&MobileOptimization> {
        self.mobile_optimization.as_ref()
    }

    /// Get a reference to the web support
    pub fn web_support(&self) -> Option<&WebMediaSupport> {
        self.web_support.as_ref()
    }

    // Session 12: Security & Compliance Getters

    /// Get a reference to the security manager
    pub fn security_manager(&self) -> Option<&MediaSecurityManager> {
        self.security_manager.as_ref()
    }

    /// Get a reference to the compliance manager
    pub fn compliance_manager(&self) -> Option<&MediaComplianceManager> {
        self.compliance_manager.as_ref()
    }

    /// Check system health and return diagnostics
    pub async fn health_check(&self) -> Result<MediaHealthStatus> {
        let mut issues = Vec::new();

        // Check storage availability
        match self.storage.health_check().await {
            Ok(_) => {},
            Err(e) => issues.push(format!("Storage backend error: {}", e)),
        }

        // Check available disk space (for local storage)
        if let StorageBackend::Local = self.config.storage_backend {
            // This would check available disk space in a real implementation
            // For now, we'll just check that the directory exists
            if !self.config.storage_path.exists() {
                issues.push("Storage directory does not exist".to_string());
            }
        }

        // Check processing manager health
        let processing_statistics = if let Some(ref processing_manager) = self.processing_manager {
            match processing_manager.health_check().await {
                Ok(status) => {
                    if !status.enabled {
                        issues.push("Processing manager is disabled".to_string());
                    }
                    Some(status.statistics)
                }
                Err(e) => {
                    issues.push(format!("Processing manager error: {}", e));
                    None
                }
            }
        } else {
            None
        };

        // Check Session 11 advanced features
        let chunking_enabled = self.config.chunking.enabled && self.chunked_transfer.is_some();
        let deduplication_enabled = self.config.deduplication.enabled && self.deduplication.is_some();
        let streaming_enabled = self.config.streaming.enabled && self.streaming_server.is_some();
        let collaboration_enabled = self.config.collaboration.enabled && self.gallery_manager.is_some();
        let mobile_optimization_enabled = self.config.compatibility.mobile_optimization && self.mobile_optimization.is_some();
        let web_optimization_enabled = self.config.compatibility.web_optimization && self.web_support.is_some();
        
        // Session 12: Security & Compliance feature status
        let security_enabled = self.config.security_advanced.enabled && self.security_manager.is_some();
        let compliance_enabled = self.config.compliance.gdpr_enabled || self.config.compliance.hipaa_enabled;
        let audit_enabled = self.config.compliance.audit_enabled;
        
        let mut advanced_features_count = 0;
        if chunking_enabled { advanced_features_count += 1; }
        if deduplication_enabled { advanced_features_count += 1; }
        if streaming_enabled { advanced_features_count += 1; }
        if collaboration_enabled { advanced_features_count += 1; }
        if mobile_optimization_enabled { advanced_features_count += 1; }
        if web_optimization_enabled { advanced_features_count += 1; }
        if security_enabled { advanced_features_count += 1; }
        if compliance_enabled { advanced_features_count += 1; }
        if audit_enabled { advanced_features_count += 1; }

        Ok(MediaHealthStatus {
            is_healthy: issues.is_empty(),
            issues,
            storage_backend: format!("{:?}", self.config.storage_backend),
            max_file_size_mb: self.config.max_file_size_mb,
            enabled: self.config.enabled,
            processing_enabled: self.config.processing.enabled,
            processing_statistics,
            
            // Session 11 status
            chunking_enabled,
            deduplication_enabled,
            streaming_enabled,
            collaboration_enabled,
            mobile_optimization_enabled,
            web_optimization_enabled,
            advanced_features_count,
            
            // Session 12 status
            security_enabled,
            compliance_enabled,
            audit_enabled,
        })
    }
}

/// Health status of the media subsystem
#[derive(Debug, Serialize)]
pub struct MediaHealthStatus {
    pub is_healthy: bool,
    pub issues: Vec<String>,
    pub storage_backend: String,
    pub max_file_size_mb: u64,
    pub enabled: bool,
    pub processing_enabled: bool,
    pub processing_statistics: Option<ProcessingStatistics>,
    
    // Session 11: Advanced feature status
    pub chunking_enabled: bool,
    pub deduplication_enabled: bool,
    pub streaming_enabled: bool,
    pub collaboration_enabled: bool,
    pub mobile_optimization_enabled: bool,
    pub web_optimization_enabled: bool,
    pub advanced_features_count: u32,
    
    // Session 12: Security & Compliance status
    pub security_enabled: bool,
    pub compliance_enabled: bool,
    pub audit_enabled: bool,
}

/// Media subsystem errors
#[derive(Debug, thiserror::Error)]
pub enum MediaError {
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),

    #[error("Encryption error: {0}")]
    Encryption(String),

    #[error("File too large: {size} bytes exceeds limit of {limit} bytes")]
    FileTooLarge { size: u64, limit: u64 },

    #[error("Unsupported MIME type: {mime_type}")]
    UnsupportedMimeType { mime_type: String },

    #[error("Invalid file: {reason}")]
    InvalidFile { reason: String },

    #[error("Transfer error: {0}")]
    Transfer(String),

    #[error("Configuration error: {0}")]
    Config(String),

    // Session 12: Security & Compliance Errors
    #[error("Security error: {0}")]
    Security(#[from] MediaSecurityError),

    #[error("Compliance error: {0}")]
    Compliance(#[from] ComplianceError),
}

impl From<MediaError> for NanoError {
    fn from(err: MediaError) -> Self {
        NanoError::Media(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_media_config_validation() {
        let valid_config = MediaConfig::default();
        assert!(valid_config.validate().is_ok());

        let mut invalid_config = MediaConfig::default();
        invalid_config.max_file_size_mb = 0;
        assert!(invalid_config.validate().is_err());

        let mut oversized_config = MediaConfig::default();
        oversized_config.max_file_size_mb = 10000; // 10GB > 5GB limit
        assert!(oversized_config.validate().is_err());
    }

    #[test]
    fn test_mime_type_checking() {
        let config = MediaConfig::default();
        
        assert!(config.is_mime_type_allowed("image/jpeg"));
        assert!(config.is_mime_type_allowed("image/png"));
        assert!(config.is_mime_type_allowed("video/mp4"));
        assert!(config.is_mime_type_allowed("application/pdf"));
        assert!(config.is_mime_type_allowed("text/plain"));
        
        assert!(!config.is_mime_type_allowed("application/x-executable"));
        assert!(!config.is_mime_type_allowed("unknown/type"));
    }

    #[test]
    fn test_size_conversions() {
        let config = MediaConfig {
            max_file_size_mb: 100,
            encryption: MediaEncryptionConfig {
                chunk_size_mb: 10,
                ..MediaEncryptionConfig::default()
            },
            ..MediaConfig::default()
        };

        assert_eq!(config.max_file_size_bytes(), 100 * 1024 * 1024);
        assert_eq!(config.chunk_size_bytes(), 10 * 1024 * 1024);
    }

    #[test]
    fn test_session_12_config_defaults() {
        let config = MediaConfig::default();
        
        // Test Session 12 security defaults
        assert!(config.security_advanced.enabled);
        assert!(config.security_advanced.scanning_enabled);
        assert!(config.security_advanced.forensics_enabled);
        assert!(config.security_advanced.access_control_enabled);
        assert!(config.security_advanced.drm_enabled);
        assert!(!config.security_advanced.quantum_enhanced); // Default to false
        assert!(!config.security_advanced.auto_remediation); // Default to false for safety
        
        // Test Session 12 compliance defaults
        assert!(config.compliance.gdpr_enabled);
        assert!(config.compliance.audit_enabled);
        assert!(config.compliance.real_time_monitoring);
    }

    #[tokio::test]
    async fn test_media_system_creation() {
        let temp_dir = TempDir::new().unwrap();
        let mut config = MediaConfig::default();
        config.storage_path = temp_dir.path().to_path_buf();

        let result = MediaSystem::new(config).await;
        assert!(result.is_ok());

        let media_system = result.unwrap();
        assert!(media_system.config().enabled);
    }

    #[tokio::test]
    async fn test_session_12_initialization() {
        let temp_dir = TempDir::new().unwrap();
        let mut config = MediaConfig::default();
        config.storage_path = temp_dir.path().to_path_buf();

        let mut media_system = MediaSystem::new(config).await.unwrap();
        
        // Test security manager initialization
        assert!(media_system.security_manager().is_none());
        media_system.init_security_manager().await.unwrap();
        assert!(media_system.security_manager().is_some());
        
        // Test compliance manager initialization
        assert!(media_system.compliance_manager().is_none());
        media_system.init_compliance_manager().await.unwrap();
        assert!(media_system.compliance_manager().is_some());
        
        // Test complete system initialization
        media_system.init_complete_system().await.unwrap();
        
        // Verify health check includes Session 12 features
        let health = media_system.health_check().await.unwrap();
        assert!(health.security_enabled);
        assert!(health.compliance_enabled);
        assert!(health.audit_enabled);
    }
}
