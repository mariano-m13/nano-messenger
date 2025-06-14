pub mod scanning;
pub mod forensics;
pub mod access_control;
pub mod encryption;

// Re-export key types and traits for easier access
pub use scanning::{
    MediaSecurityScanner, MalwareScanResult, ContentSafetyResult, SteganographyResult,
    BehaviorAnalysis, ComprehensiveScanResult, MediaFile, FileUpload, ThreatInfo,
    ContentIssue, SecurityAction, SuspiciousPattern
};

pub use forensics::{
    MediaForensics, MediaFingerprint, ProvenanceChain, IntegrityReport, 
    WatermarkedMedia, DigitalWatermark, ProvenanceOperation, ModificationInfo,
    HashChainManager, ProvenanceTracker, IntegrityVerifier
};

pub use access_control::{
    MediaAccessControl, MediaPermissions, MediaAction, AccessDecision, 
    ProtectedMedia, DRMLevel, AccessToken, AccessContext, DeviceInfo,
    NetworkInfo, SessionInfo, AuthenticationLevel, DRMAlgorithm
};

pub use encryption::{
    E2EMediaEncryption, MediaSession, GroupEncryptedMedia, QuantumKeyDistribution,
    HybridSharedKeys, QuantumSharedKeys, MediaKey, MediaSessionId, KeyRotationEvent,
    RotationReason, MediaKeyRotation, FileEncryptionEngine
};

use crate::crypto::CryptoMode;
use crate::username::UserId;
use std::collections::HashMap;
use std::time::SystemTime;
use thiserror::Error;

/// Unified media security error type
#[derive(Debug, Error)]
pub enum MediaSecurityError {
    #[error("Security scanning failed: {0}")]
    ScanningFailed(#[from] scanning::ScanError),
    
    #[error("Forensics operation failed: {0}")]
    ForensicsFailed(#[from] forensics::ForensicsError),
    
    #[error("Access control violation: {0}")]
    AccessControlFailed(#[from] access_control::AccessControlError),
    
    #[error("Encryption operation failed: {0}")]
    EncryptionFailed(#[from] encryption::MediaEncryptionError),
    
    #[error("Security policy violation: {0}")]
    PolicyViolation(String),
    
    #[error("Security configuration error: {0}")]
    ConfigurationError(String),
}

/// Comprehensive media security configuration
#[derive(Debug, Clone)]
pub struct MediaSecurityConfig {
    pub crypto_mode: CryptoMode,
    pub scanning_enabled: bool,
    pub forensics_enabled: bool,
    pub access_control_enabled: bool,
    pub drm_enabled: bool,
    pub e2e_encryption_enabled: bool,
    pub quantum_enhanced: bool,
    pub security_policy: SecurityPolicy,
}

/// Security policy definitions
#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    pub require_encryption_for_sensitive: bool,
    pub mandatory_virus_scanning: bool,
    pub content_analysis_enabled: bool,
    pub steganography_detection: bool,
    pub behavioral_analysis: bool,
    pub forensic_fingerprinting: bool,
    pub access_logging: bool,
    pub drm_for_confidential: bool,
    pub quantum_safe_required: bool,
}

/// Unified media security manager
pub struct MediaSecurityManager {
    pub scanner: MediaSecurityScanner,
    pub forensics: MediaForensics,
    pub access_control: MediaAccessControl,
    pub encryption: E2EMediaEncryption,
    pub qkd: Option<QuantumKeyDistribution>,
    pub config: MediaSecurityConfig,
}

/// Security assessment result for media
#[derive(Debug, Clone)]
pub struct SecurityAssessment {
    pub file_id: String,
    pub overall_risk_score: f32,
    pub scan_results: MalwareScanResult,
    pub content_safety: ContentSafetyResult,
    pub steganography: SteganographyResult,
    pub behavior_analysis: BehaviorAnalysis,
    pub fingerprint: Option<MediaFingerprint>,
    pub access_recommendation: AccessDecision,
    pub encryption_required: bool,
    pub drm_recommended: bool,
}

impl MediaSecurityManager {
    /// Create new media security manager
    pub fn new(config: MediaSecurityConfig) -> Self {
        let crypto_mode = config.crypto_mode.clone();
        
        Self {
            scanner: MediaSecurityScanner::new(),
            forensics: MediaForensics::new(crypto_mode.clone()),
            access_control: MediaAccessControl::new(crypto_mode.clone()),
            encryption: E2EMediaEncryption::new(crypto_mode),
            qkd: if config.quantum_enhanced {
                Some(QuantumKeyDistribution::new())
            } else {
                None
            },
            config,
        }
    }

    /// Perform comprehensive security assessment of media
    pub async fn assess_media_security(
        &self,
        media: &MediaFile,
        upload_context: &UploadContext,
    ) -> Result<SecurityAssessment, MediaSecurityError> {
        let mut assessment = SecurityAssessment {
            file_id: media.file_id.clone(),
            overall_risk_score: 0.0,
            scan_results: MalwareScanResult {
                is_clean: true,
                threats_detected: vec![],
                scan_duration: std::time::Duration::from_secs(0),
                engines_used: vec![],
                confidence_score: 1.0,
            },
            content_safety: ContentSafetyResult {
                is_safe: true,
                confidence_score: 1.0,
                detected_issues: vec![],
                recommended_action: SecurityAction::Allow,
                analysis_metadata: HashMap::new(),
            },
            steganography: SteganographyResult {
                hidden_content_detected: false,
                detection_confidence: 0.0,
                suspected_techniques: vec![],
                estimated_payload_size: None,
                extraction_possible: false,
            },
            behavior_analysis: BehaviorAnalysis {
                is_suspicious: false,
                risk_score: 0.0,
                detected_patterns: vec![],
                recommendations: vec![],
                historical_context: scanning::BehaviorHistoryContext {
                    baseline_established: false,
                    upload_frequency_score: 0.0,
                    file_size_score: 0.0,
                    time_pattern_score: 0.0,
                    content_type_score: 0.0,
                },
            },
            fingerprint: None,
            access_recommendation: AccessDecision::Allow,
            encryption_required: false,
            drm_recommended: false,
        };

        // Perform security scanning if enabled
        if self.config.scanning_enabled {
            let scan_result = self.scanner.comprehensive_scan(
                media,
                &upload_context.user_id,
                &upload_context.upload_history,
            ).await?;
            
            assessment.scan_results = scan_result.malware_scan;
            assessment.content_safety = scan_result.content_safety;
            assessment.steganography = scan_result.steganography;
            assessment.behavior_analysis = scan_result.behavior_analysis;
            assessment.overall_risk_score = scan_result.overall_risk_score;
        }

        // Create forensic fingerprint if enabled
        if self.config.forensics_enabled {
            let fingerprint = self.forensics.create_media_fingerprint(
                &media.content,
                &media.metadata,
                &media.file_id,
                &upload_context.user_id,
            ).await?;
            assessment.fingerprint = Some(fingerprint);
        }

        // Determine access recommendations
        if self.config.access_control_enabled {
            assessment.access_recommendation = self.determine_access_recommendation(&assessment)?;
        }

        // Determine encryption requirements
        assessment.encryption_required = self.should_encrypt_media(&assessment)?;
        assessment.drm_recommended = self.should_apply_drm(&assessment)?;

        Ok(assessment)
    }

    /// Apply security measures based on assessment
    pub async fn apply_security_measures(
        &mut self,
        media: &MediaFile,
        assessment: &SecurityAssessment,
    ) -> Result<SecuredMedia, MediaSecurityError> {
        let mut secured_media = SecuredMedia {
            original_file_id: media.file_id.clone(),
            content: media.content.clone(),
            security_applied: vec![],
            access_restrictions: vec![],
            encryption_metadata: None,
            drm_metadata: None,
            forensic_fingerprint: assessment.fingerprint.clone(),
        };

        // Apply quarantine if threats detected
        if !assessment.scan_results.is_clean {
            secured_media.security_applied.push(SecurityMeasure::Quarantined);
            return Ok(secured_media); // Don't process further if quarantined
        }

        // Apply encryption if required
        if assessment.encryption_required {
            let session = self.encryption.establish_media_session(
                &[media.file_id.clone()], // Simplified participant list
                Some(self.config.crypto_mode.clone()),
            ).await?;
            
            let encrypted_media = self.encryption.encrypt_for_group(
                &media.content,
                &session,
            ).await?;
            
            secured_media.content = encrypted_media.encrypted_content;
            secured_media.security_applied.push(SecurityMeasure::Encrypted);
            secured_media.encryption_metadata = Some(EncryptionMetadata {
                algorithm: encrypted_media.encryption_metadata.algorithm,
                crypto_mode: encrypted_media.encryption_metadata.crypto_mode,
                session_id: session.session_id,
            });
        }

        // Apply DRM if recommended
        if assessment.drm_recommended && self.config.drm_enabled {
            let permissions = MediaPermissions::default(); // Would determine based on content
            let protected_media = self.access_control.apply_drm_protection(
                &secured_media.content,
                &media.file_id,
                &permissions,
                DRMLevel::Standard,
            ).await?;
            
            secured_media.content = protected_media.protected_content;
            secured_media.security_applied.push(SecurityMeasure::DRMProtected);
            secured_media.drm_metadata = Some(DRMMetadata {
                protection_level: protected_media.drm_level,
                license_id: protected_media.license_id,
            });
        }

        Ok(secured_media)
    }

    // Helper methods
    fn determine_access_recommendation(&self, assessment: &SecurityAssessment) -> Result<AccessDecision, MediaSecurityError> {
        if assessment.overall_risk_score > 0.8 {
            Ok(AccessDecision::Deny(access_control::AccessDenialReason::SecurityThreat))
        } else if assessment.overall_risk_score > 0.6 {
            Ok(AccessDecision::Conditional(vec![
                access_control::AccessCondition::RequireSecondaryAuth,
                access_control::AccessCondition::RequireAuditLog,
            ]))
        } else {
            Ok(AccessDecision::Allow)
        }
    }

    fn should_encrypt_media(&self, assessment: &SecurityAssessment) -> Result<bool, MediaSecurityError> {
        // Encrypt if policy requires it for sensitive content
        if self.config.security_policy.require_encryption_for_sensitive {
            // Check if content contains sensitive data
            let has_sensitive_content = !assessment.content_safety.detected_issues.is_empty() ||
                                      assessment.behavior_analysis.is_suspicious;
            return Ok(has_sensitive_content);
        }
        
        // Always encrypt if quantum-safe is required
        Ok(self.config.security_policy.quantum_safe_required)
    }

    fn should_apply_drm(&self, assessment: &SecurityAssessment) -> Result<bool, MediaSecurityError> {
        // Apply DRM for confidential content or if policy requires it
        Ok(self.config.security_policy.drm_for_confidential && 
           assessment.overall_risk_score > 0.3)
    }
}

/// Upload context for security assessment
#[derive(Debug, Clone)]
pub struct UploadContext {
    pub user_id: UserId,
    pub upload_timestamp: SystemTime,
    pub source_ip: String,
    pub user_agent: String,
    pub upload_history: Vec<FileUpload>,
    pub session_info: SessionInfo,
}

/// Secured media after applying security measures
#[derive(Debug, Clone)]
pub struct SecuredMedia {
    pub original_file_id: String,
    pub content: Vec<u8>,
    pub security_applied: Vec<SecurityMeasure>,
    pub access_restrictions: Vec<access_control::AccessRestriction>,
    pub encryption_metadata: Option<EncryptionMetadata>,
    pub drm_metadata: Option<DRMMetadata>,
    pub forensic_fingerprint: Option<MediaFingerprint>,
}

/// Types of security measures that can be applied
#[derive(Debug, Clone)]
pub enum SecurityMeasure {
    Scanned,
    Quarantined,
    Encrypted,
    DRMProtected,
    Fingerprinted,
    AccessControlled,
    Watermarked,
}

/// Encryption metadata for secured media
#[derive(Debug, Clone)]
pub struct EncryptionMetadata {
    pub algorithm: String,
    pub crypto_mode: CryptoMode,
    pub session_id: MediaSessionId,
}

/// DRM metadata for secured media
#[derive(Debug, Clone)]
pub struct DRMMetadata {
    pub protection_level: DRMLevel,
    pub license_id: String,
}

impl Default for MediaSecurityConfig {
    fn default() -> Self {
        Self {
            crypto_mode: CryptoMode::Hybrid,
            scanning_enabled: true,
            forensics_enabled: true,
            access_control_enabled: true,
            drm_enabled: true,
            e2e_encryption_enabled: true,
            quantum_enhanced: false,
            security_policy: SecurityPolicy::default(),
        }
    }
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        Self {
            require_encryption_for_sensitive: true,
            mandatory_virus_scanning: true,
            content_analysis_enabled: true,
            steganography_detection: true,
            behavioral_analysis: true,
            forensic_fingerprinting: true,
            access_logging: true,
            drm_for_confidential: true,
            quantum_safe_required: false,
        }
    }
}
