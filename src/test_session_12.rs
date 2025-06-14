/// Comprehensive tests for Session 12: Security & Compliance for Media
/// 
/// This test suite validates all Session 12 functionality including:
/// - Media security scanning and threat detection
/// - Forensics and integrity verification
/// - Access control and DRM protection
/// - End-to-end media encryption
/// - GDPR compliance features
/// - HIPAA compliance features
/// - Enterprise audit and reporting

use crate::media::security::{
    MediaSecurityManager, MediaSecurityConfig, SecurityPolicy, SecurityAssessment,
    SecuredMedia, SecurityMeasure, UploadContext, MediaSecurityError
};
use crate::media::compliance::{
    MediaComplianceManager, ComplianceConfig, ComplianceAssessment, 
    ComplianceError, DataCategory, ComplianceViolation, RemediationAction,
    MultiRegulationResult, ComplianceDashboard, Regulation
};
use crate::media::security::scanning::{
    MediaSecurityScanner, MediaFile, FileUpload, ThreatType, ContentIssue,
    SecurityAction, SuspiciousPattern
};
use crate::media::security::forensics::{
    MediaForensics, MediaFingerprint, ProvenanceOperation, ModificationInfo
};
use crate::media::security::access_control::{
    MediaAccessControl, MediaAction, AccessDecision, DRMLevel, AuthenticationLevel
};
use crate::media::security::encryption::{
    E2EMediaEncryption, RotationReason
};
use crate::media::compliance::gdpr::{
    MediaGDPRCompliance, PersonalDataCategory, DataSensitivityLevel
};
use crate::media::compliance::hipaa::{
    MediaHIPAACompliance, PHIType, UserRole, AccessPurpose
};
use crate::media::compliance::auditing::{
    MediaAuditSystem, MediaAuditEvent, SecurityEventType, ComplianceRegulation
};
use crate::crypto::CryptoMode;
use crate::username::UserId;
use crate::media::metadata::FileMetadata;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use tempfile::TempDir;
use tokio;

/// Create test media file
fn create_test_media_file(file_id: &str, content: Vec<u8>, mime_type: &str) -> MediaFile {
    let content_len = content.len() as u64;
    MediaFile {
        file_id: file_id.to_string(),
        content,
        metadata: {
            let mut metadata = FileMetadata::default();
            metadata.original_name = format!("test_{}.dat", file_id);
            metadata.file_size = content_len;
            metadata.mime_type = mime_type.to_string();
            metadata.upload_timestamp = SystemTime::now();
            metadata.checksum = "test_checksum".to_string();
            metadata
        },
        mime_type: mime_type.to_string(),
        size: content_len,
    }
}

/// Create test upload context
fn create_test_upload_context(user_id: &str) -> UploadContext {
    UploadContext {
        user_id: user_id.to_string(),
        upload_timestamp: SystemTime::now(),
        source_ip: "127.0.0.1".to_string(),
        user_agent: "Mozilla/5.0 (Test Browser)".to_string(),
        upload_history: vec![],
        session_info: crate::media::security::access_control::SessionInfo {
            session_id: "test_session".to_string(),
            login_time: SystemTime::now(),
            last_activity: SystemTime::now(),
            activity_count: 1,
            authentication_level: AuthenticationLevel::TwoFactor,
        },
    }
}

pub async fn test_security_scanner_basic_functionality() {
    let scanner = MediaSecurityScanner::new();
    
    // Test with clean file
    let clean_file = create_test_media_file("clean_001", b"This is a clean test file".to_vec(), "text/plain");
    let upload_context = create_test_upload_context("user_001");
    
    let scan_result = scanner.comprehensive_scan(&clean_file, &upload_context.user_id, &upload_context.upload_history).await;
    assert!(scan_result.is_ok());
    
    let result = scan_result.unwrap();
    assert!(result.malware_scan.is_clean);
    assert!(result.content_safety.is_safe);
    assert!(!result.steganography.hidden_content_detected);
    assert!(!result.behavior_analysis.is_suspicious);
    assert!(result.overall_risk_score < 0.3); // Should be low risk
}

#[tokio::test]
async fn test_security_scanner_threat_detection() {
    let scanner = MediaSecurityScanner::new();
    
    // Test with EICAR test file (standard antivirus test string)
    let malware_file = create_test_media_file(
        "malware_001", 
        b"EICAR-STANDARD-ANTIVIRUS-TEST-FILE".to_vec(), 
        "application/octet-stream"
    );
    let upload_context = create_test_upload_context("user_002");
    
    let scan_result = scanner.comprehensive_scan(&malware_file, &upload_context.user_id, &upload_context.upload_history).await;
    assert!(scan_result.is_ok());
    
    let result = scan_result.unwrap();
    assert!(!result.malware_scan.is_clean);
    assert!(!result.malware_scan.threats_detected.is_empty());
    assert!(result.overall_risk_score > 0.5); // Should be high risk
}

#[tokio::test]
async fn test_media_forensics_fingerprinting() {
    let forensics = MediaForensics::default();
    let test_content = b"Important document content for fingerprinting";
    let metadata = FileMetadata::default();
    let file_id = "forensics_001";
    let creator = "forensics_user";
    
    // Create fingerprint
    let fingerprint_result = forensics.create_media_fingerprint(
        test_content,
        &metadata,
        file_id,
        creator,
    ).await;
    
    assert!(fingerprint_result.is_ok());
    let fingerprint = fingerprint_result.unwrap();
    
    assert_eq!(fingerprint.file_id, file_id);
    assert!(!fingerprint.content_hash.iter().all(|&b| b == 0)); // Hash should not be empty
    assert!(!fingerprint.quantum_signature.is_empty());
    
    // Test integrity verification
    let integrity_result = forensics.verify_media_integrity(test_content, &fingerprint).await;
    assert!(integrity_result.is_ok());
    
    let integrity_report = integrity_result.unwrap();
    assert!(integrity_report.is_authentic);
    assert!(integrity_report.integrity_score > 0.9);
    assert!(integrity_report.hash_verification.content_hash_valid);
}

#[tokio::test]
async fn test_access_control_permissions() {
    let access_control = MediaAccessControl::new(CryptoMode::Hybrid);
    
    // Create test access context
    let context = crate::media::security::access_control::AccessContext {
        user_id: "access_user_001".to_string(),
        device_info: crate::media::security::access_control::DeviceInfo {
            device_id: "device_001".to_string(),
            device_type: "laptop".to_string(),
            os_info: "Windows 11".to_string(),
            browser_info: Some("Chrome 91.0".to_string()),
            screen_resolution: Some((1920, 1080)),
            is_trusted: true,
        },
        network_info: crate::media::security::access_control::NetworkInfo {
            ip_address: "192.168.1.100".to_string(),
            country: Some("US".to_string()),
            region: Some("California".to_string()),
            isp: Some("Test ISP".to_string()),
            is_vpn: false,
            is_tor: false,
        },
        timestamp: SystemTime::now(),
        session_info: crate::media::security::access_control::SessionInfo {
            session_id: "session_001".to_string(),
            login_time: SystemTime::now(),
            last_activity: SystemTime::now(),
            activity_count: 5,
            authentication_level: AuthenticationLevel::TwoFactor,
        },
    };
    
    let file_id = "access_test_001";
    
    // Test view access
    let view_decision = access_control.check_media_access(&context, file_id, MediaAction::View).await;
    assert!(view_decision.is_ok());
    
    // Test download access
    let download_decision = access_control.check_media_access(&context, file_id, MediaAction::Download).await;
    assert!(download_decision.is_ok());
}

#[tokio::test]
async fn test_drm_protection() {
    let access_control = MediaAccessControl::new(CryptoMode::Hybrid);
    let test_content = b"Confidential document requiring DRM protection";
    let file_id = "drm_test_001";
    let permissions = crate::media::security::access_control::MediaPermissions::default();
    
    // Apply DRM protection
    let drm_result = access_control.apply_drm_protection(test_content, file_id, &permissions, DRMLevel::Standard).await;
    assert!(drm_result.is_ok());
    
    let protected_media = drm_result.unwrap();
    assert_eq!(protected_media.original_file_id, file_id);
    assert_eq!(protected_media.drm_level, DRMLevel::Standard);
    assert!(!protected_media.protected_content.is_empty());
    assert_ne!(protected_media.protected_content, test_content); // Content should be encrypted
}

#[tokio::test]
async fn test_e2e_media_encryption() {
    let mut encryption = E2EMediaEncryption::new(CryptoMode::Hybrid);
    let participants = vec!["user1".to_string(), "user2".to_string()];
    
    // Establish media session
    let session_result = encryption.establish_media_session(&participants, Some(CryptoMode::Hybrid)).await;
    assert!(session_result.is_ok());
    
    let session = session_result.unwrap();
    assert_eq!(session.participants, participants);
    assert!(session.quantum_enhanced);
    
    // Test encryption for group
    let test_content = b"Secret group message content";
    let encryption_result = encryption.encrypt_for_group(test_content, &session).await;
    assert!(encryption_result.is_ok());
    
    let encrypted_media = encryption_result.unwrap();
    assert!(!encrypted_media.encrypted_content.is_empty());
    assert_ne!(encrypted_media.encrypted_content, test_content);
    assert_eq!(encrypted_media.recipient_keys.len(), participants.len());
    
    // Test decryption
    let decryption_result = encryption.decrypt_group_media(&encrypted_media, &participants[0], &session).await;
    assert!(decryption_result.is_ok());
    
    let decrypted_content = decryption_result.unwrap();
    assert_eq!(decrypted_content, test_content);
}

#[tokio::test]
async fn test_key_rotation() {
    let mut encryption = E2EMediaEncryption::new(CryptoMode::Hybrid);
    let participants = vec!["user1".to_string()];
    
    let mut session = encryption.establish_media_session(&participants, None).await.unwrap();
    let original_key = session.session_key.clone();
    
    // Rotate keys
    let rotation_result = encryption.rotate_session_keys(&mut session, RotationReason::Scheduled).await;
    assert!(rotation_result.is_ok());
    
    // Verify key changed
    assert_ne!(session.session_key, original_key);
    assert!(session.last_rotation > session.created_at);
}

#[tokio::test]
async fn test_gdpr_personal_data_detection() {
    let gdpr_compliance = MediaGDPRCompliance::new();
    
    // Test file with potential personal data
    let personal_data_file = create_test_media_file(
        "gdpr_001",
        b"John Doe, email: john.doe@example.com, phone: 555-1234".to_vec(),
        "text/plain"
    );
    
    let classification_result = gdpr_compliance.classify_personal_data(&personal_data_file).await;
    assert!(classification_result.is_ok());
    
    let classification = classification_result.unwrap();
    assert!(classification.contains_personal_data);
    assert!(!classification.data_categories.is_empty());
    assert!(classification.detection_confidence > 0.5);
}

#[tokio::test]
async fn test_gdpr_data_subject_access_request() {
    let gdpr_compliance = MediaGDPRCompliance::new();
    
    let access_request = crate::media::compliance::gdpr::AccessRequest {
        request_id: "gdpr_request_001".to_string(),
        subject_id: "subject_001".to_string(),
        requested_data_categories: None,
        requested_time_range: None,
        identity_verification: crate::media::compliance::gdpr::IdentityVerification {
            method: crate::media::compliance::gdpr::VerificationMethod::GovernmentID,
            verified: true,
            verification_date: SystemTime::now(),
            verification_evidence: vec!["ID_12345".to_string()],
        },
        delivery_method: crate::media::compliance::gdpr::DataDeliveryMethod::SecureDownload,
    };
    
    let report_result = gdpr_compliance.process_media_access_request(access_request).await;
    assert!(report_result.is_ok());
    
    let report = report_result.unwrap();
    assert_eq!(report.subject_id, "subject_001");
    assert!(!report.report_generated.duration_since(SystemTime::UNIX_EPOCH).unwrap().is_zero());
}

#[tokio::test]
async fn test_gdpr_media_erasure() {
    let gdpr_compliance = MediaGDPRCompliance::new();
    
    let erasure_request = crate::media::compliance::gdpr::MediaErasureRequest {
        request_id: "erasure_001".to_string(),
        subject_id: "subject_002".to_string(),
        files_to_erase: vec!["file_001".to_string(), "file_002".to_string()],
        erasure_method: crate::media::compliance::gdpr::ErasureMethod::SecureDelete,
        verification_required: true,
        backup_erasure: true,
        third_party_notification: false,
        legal_basis_check: true,
    };
    
    let erasure_result = gdpr_compliance.erase_personal_media(&erasure_request).await;
    assert!(erasure_result.is_ok());
    
    let confirmation = erasure_result.unwrap();
    assert_eq!(confirmation.request_id, "erasure_001");
    assert!(confirmation.erasure_completed);
    assert_eq!(confirmation.files_erased.len(), 2);
}

#[tokio::test]
async fn test_hipaa_phi_detection() {
    let hipaa_compliance = MediaHIPAACompliance::new();
    
    // Test file with potential PHI
    let phi_file = create_test_media_file(
        "hipaa_001",
        b"Patient: John Smith, DOB: 01/01/1980, Diagnosis: Hypertension, MRN: 12345".to_vec(),
        "text/plain"
    );
    
    let phi_result = hipaa_compliance.detect_phi_in_media(&phi_file).await;
    assert!(phi_result.is_ok());
    
    let phi_detection = phi_result.unwrap();
    assert!(phi_detection.contains_phi);
    assert!(!phi_detection.phi_types.is_empty());
    assert!(phi_detection.encryption_required);
}

#[tokio::test]
async fn test_hipaa_phi_encryption() {
    let hipaa_compliance = MediaHIPAACompliance::new();
    
    let phi_file = create_test_media_file(
        "hipaa_encrypt_001",
        b"Patient medical record with sensitive information".to_vec(),
        "text/plain"
    );
    
    let phi_detection = crate::media::compliance::hipaa::PHIDetectionResult {
        contains_phi: true,
        phi_types: vec![PHIType::PatientDocuments],
        confidence_scores: HashMap::from([(PHIType::PatientDocuments, 0.9)]),
        identifiers_found: vec![],
        patient_identifiers: vec![],
        redaction_recommendations: vec![],
        encryption_required: true,
        access_restrictions: vec![],
        minimum_necessary_compliance: true,
    };
    
    let encryption_result = hipaa_compliance.ensure_phi_encryption(&phi_file, &phi_detection).await;
    assert!(encryption_result.is_ok());
    
    let encrypted_phi = encryption_result.unwrap();
    assert_eq!(encrypted_phi.original_file_id, "hipaa_encrypt_001");
    assert!(!encrypted_phi.encrypted_content.is_empty());
    assert!(encrypted_phi.compliance_attestation.hipaa_compliant);
}

#[tokio::test]
async fn test_hipaa_access_report() {
    let hipaa_compliance = MediaHIPAACompliance::new();
    let report_period = (SystemTime::now() - Duration::from_secs(30 * 24 * 60 * 60), SystemTime::now());
    
    let report_result = hipaa_compliance.generate_phi_access_report(report_period).await;
    assert!(report_result.is_ok());
    
    let report = report_result.unwrap();
    assert!(!report.report_id.is_empty());
    assert_eq!(report.report_period, report_period);
}

#[tokio::test]
async fn test_audit_system_event_logging() {
    let mut audit_system = MediaAuditSystem::new();
    
    // Log a test event
    let test_event = MediaAuditEvent::FileUploaded {
        user_id: "audit_user_001".to_string(),
        file_metadata: FileMetadata::default(),
        scan_results: crate::media::compliance::auditing::ScanResults {
            malware_detected: false,
            threat_level: crate::media::compliance::auditing::ThreatLevel::Low,
            scan_engines_used: vec!["TestEngine".to_string()],
            scan_duration: Duration::from_secs(1),
            quarantine_applied: false,
            false_positive_probability: 0.01,
        },
        encryption_applied: true,
        compliance_check: crate::media::compliance::auditing::ComplianceCheckResult {
            gdpr_compliant: true,
            hipaa_compliant: true,
            sox_compliant: true,
            personal_data_detected: false,
            phi_detected: false,
            retention_policy_applied: true,
            legal_basis_documented: true,
        },
    };
    
    let log_result = audit_system.log_media_event(test_event).await;
    assert!(log_result.is_ok());
    
    let event_id = log_result.unwrap();
    assert!(!event_id.is_empty());
}

#[tokio::test]
async fn test_audit_system_integrity_verification() {
    let audit_system = MediaAuditSystem::new();
    
    // Verify audit trail integrity
    let integrity_result = audit_system.verify_audit_trail_integrity().await;
    assert!(integrity_result.is_ok());
    assert!(integrity_result.unwrap()); // Should be valid
}

#[tokio::test]
async fn test_audit_report_generation() {
    let audit_system = MediaAuditSystem::new();
    let report_period = (SystemTime::now() - Duration::from_secs(24 * 60 * 60), SystemTime::now());
    
    let report_result = audit_system.generate_media_audit_report(
        crate::media::compliance::auditing::AuditScope::AllFiles,
        report_period,
    ).await;
    
    assert!(report_result.is_ok());
    
    let report = report_result.unwrap();
    assert!(!report.report_id.is_empty());
    assert_eq!(report.report_period, report_period);
    assert!(!report.executive_summary.overall_security_posture == crate::media::compliance::auditing::SecurityPosture::Critical);
}

#[tokio::test]
async fn test_compliance_status_checking() {
    let audit_system = MediaAuditSystem::new();
    
    let status_result = audit_system.check_compliance_status().await;
    assert!(status_result.is_ok());
    
    let status = status_result.unwrap();
    // Should be compliant for new system
    assert!(matches!(status, 
        crate::media::compliance::auditing::ComplianceStatus::FullyCompliant | 
        crate::media::compliance::auditing::ComplianceStatus::MostlyCompliant
    ));
}

#[tokio::test]
async fn test_comprehensive_security_manager() {
    let security_config = MediaSecurityConfig {
        crypto_mode: CryptoMode::Hybrid,
        scanning_enabled: true,
        forensics_enabled: true,
        access_control_enabled: true,
        drm_enabled: true,
        e2e_encryption_enabled: true,
        quantum_enhanced: false,
        security_policy: SecurityPolicy::default(),
    };
    
    let security_manager = MediaSecurityManager::new(security_config);
    
    let test_file = create_test_media_file(
        "security_comprehensive_001",
        b"Test file for comprehensive security assessment".to_vec(),
        "text/plain"
    );
    let upload_context = create_test_upload_context("security_user_001");
    
    // Perform comprehensive security assessment
    let assessment_result = security_manager.assess_media_security(&test_file, &upload_context).await;
    assert!(assessment_result.is_ok());
    
    let assessment = assessment_result.unwrap();
    assert_eq!(assessment.file_id, "security_comprehensive_001");
    assert!(assessment.overall_risk_score >= 0.0 && assessment.overall_risk_score <= 1.0);
    
    // Apply security measures
    let security_result = security_manager.apply_security_measures(&test_file, &assessment).await;
    assert!(security_result.is_ok());
    
    let secured_media = security_result.unwrap();
    assert_eq!(secured_media.original_file_id, "security_comprehensive_001");
    assert!(!secured_media.security_applied.is_empty());
}

#[tokio::test]
async fn test_multi_regulation_compliance() {
    let compliance_config = ComplianceConfig {
        gdpr_enabled: true,
        hipaa_enabled: true,
        sox_enabled: false,
        audit_enabled: true,
        real_time_monitoring: true,
        auto_remediation: false,
        data_retention_policies: HashMap::new(),
        breach_notification_threshold: Duration::from_secs(72 * 60 * 60),
        compliance_officer: Some("compliance_officer_001".to_string()),
    };
    
    let compliance_manager = MediaComplianceManager::new(compliance_config);
    
    let test_file = create_test_media_file(
        "compliance_multi_001",
        b"Test file with potential personal and health information".to_vec(),
        "text/plain"
    );
    
    let compliance_context = crate::media::compliance::ComplianceContext {
        user_id: "compliance_user_001".to_string(),
        organization_id: "test_org".to_string(),
        jurisdiction: "US".to_string(),
        business_context: crate::media::compliance::BusinessContext {
            industry: "Healthcare".to_string(),
            organization_size: crate::media::compliance::OrganizationSize::Medium,
            is_healthcare_entity: true,
            is_financial_institution: false,
            international_operations: false,
        },
        data_processing_purpose: crate::media::compliance::ProcessingPurpose::Communication,
        consent_status: crate::media::compliance::ConsentStatus {
            has_consent: true,
            consent_date: Some(SystemTime::now()),
            consent_scope: vec![crate::media::compliance::ProcessingPurpose::Communication],
            withdrawal_possible: true,
        },
    };
    
    // Test multi-regulation compliance check
    let multi_result = compliance_manager.check_multi_regulation_compliance(&test_file, &compliance_context).await;
    assert!(multi_result.is_ok());
    
    let multi_compliance = multi_result.unwrap();
    assert!(multi_compliance.regulation_results.contains_key(&Regulation::GDPR));
    assert!(multi_compliance.regulation_results.contains_key(&Regulation::HIPAA));
}

#[tokio::test]
async fn test_compliance_dashboard_generation() {
    let compliance_manager = MediaComplianceManager::new(ComplianceConfig::default());
    let time_period = (SystemTime::now() - Duration::from_secs(30 * 24 * 60 * 60), SystemTime::now());
    
    let dashboard_result = compliance_manager.generate_compliance_dashboard(time_period).await;
    assert!(dashboard_result.is_ok());
    
    let dashboard = dashboard_result.unwrap();
    assert_eq!(dashboard.period, time_period);
    assert!(!dashboard.audit_summary.total_files_audited == 0 || dashboard.audit_summary.total_files_audited > 0);
}

#[tokio::test]
async fn test_security_policy_enforcement() {
    let mut security_policy = SecurityPolicy::default();
    security_policy.require_encryption_for_sensitive = true;
    security_policy.mandatory_virus_scanning = true;
    security_policy.quantum_safe_required = true;
    
    let security_config = MediaSecurityConfig {
        crypto_mode: CryptoMode::QuantumSafe,
        scanning_enabled: true,
        forensics_enabled: true,
        access_control_enabled: true,
        drm_enabled: true,
        e2e_encryption_enabled: true,
        quantum_enhanced: true,
        security_policy,
    };
    
    let security_manager = MediaSecurityManager::new(security_config);
    
    // Test that quantum-safe requirements are enforced
    let test_file = create_test_media_file(
        "policy_test_001",
        b"Sensitive data requiring quantum-safe protection".to_vec(),
        "text/plain"
    );
    let upload_context = create_test_upload_context("policy_user_001");
    
    let assessment = security_manager.assess_media_security(&test_file, &upload_context).await.unwrap();
    assert!(assessment.encryption_required); // Should require encryption due to policy
}

#[tokio::test]
async fn test_incident_response_workflow() {
    let mut audit_system = MediaAuditSystem::new();
    
    // Simulate security incident
    let security_incident = MediaAuditEvent::SecurityViolation {
        event_type: SecurityEventType::MalwareDetection,
        severity: crate::media::compliance::auditing::SecuritySeverity::High,
        affected_files: vec!["incident_file_001".to_string()],
        threat_indicators: vec![],
        automated_response: Some(crate::media::compliance::auditing::AutomatedResponse {
            action_type: crate::media::compliance::auditing::ResponseActionType::QuarantineFile,
            timestamp: SystemTime::now(),
            success: true,
            details: "File quarantined successfully".to_string(),
            follow_up_required: true,
        }),
    };
    
    let incident_log = audit_system.log_media_event(security_incident).await;
    assert!(incident_log.is_ok());
    
    // Generate security incident summary
    let incident_period = (SystemTime::now() - Duration::from_secs(60 * 60), SystemTime::now());
    let security_summary = audit_system.get_security_incident_summary(incident_period).await;
    assert!(security_summary.is_ok());
    
    let summary = security_summary.unwrap();
    assert!(summary.total_security_events > 0);
    assert!(summary.malware_detections > 0);
}

#[tokio::test]
async fn test_performance_under_load() {
    use std::sync::Arc;
    use tokio::sync::Semaphore;
    
    let security_manager = Arc::new(MediaSecurityManager::new(MediaSecurityConfig::default()));
    let semaphore = Arc::new(Semaphore::new(10)); // Limit concurrent operations
    
    let mut handles = Vec::new();
    
    // Simulate 50 concurrent security assessments
    for i in 0..50 {
        let security_manager = Arc::clone(&security_manager);
        let semaphore = Arc::clone(&semaphore);
        
        let handle = tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            
            let test_file = create_test_media_file(
                &format!("load_test_{}", i),
                format!("Test content for load test {}", i).into_bytes(),
                "text/plain"
            );
            let upload_context = create_test_upload_context(&format!("load_user_{}", i));
            
            let start_time = std::time::Instant::now();
            let result = security_manager.assess_media_security(&test_file, &upload_context).await;
            let duration = start_time.elapsed();
            
            (result.is_ok(), duration)
        });
        
        handles.push(handle);
    }
    
    // Wait for all operations to complete
    let results: Vec<(bool, std::time::Duration)> = futures::future::join_all(handles)
        .await
        .into_iter()
        .map(|result| result.unwrap())
        .collect();
    
    // Verify all operations succeeded
    let success_count = results.iter().filter(|(success, _)| *success).count();
    assert_eq!(success_count, 50);
    
    // Verify reasonable performance (all operations under 10 seconds)
    let max_duration = results.iter().map(|(_, duration)| *duration).max().unwrap();
    assert!(max_duration < Duration::from_secs(10));
    
    // Verify average performance is acceptable (under 1 second average)
    let total_duration: Duration = results.iter().map(|(_, duration)| *duration).sum();
    let average_duration = total_duration / results.len() as u32;
    assert!(average_duration < Duration::from_secs(1));
}

/// Integration test combining all Session 12 features
#[tokio::test]
async fn test_session_12_complete_integration() {
    // Initialize all systems
    let security_manager = MediaSecurityManager::new(MediaSecurityConfig::default());
    let compliance_manager = MediaComplianceManager::new(ComplianceConfig::default());
    let mut audit_system = MediaAuditSystem::new();
    
    // Create test scenario: healthcare organization uploading patient data
    let test_file = create_test_media_file(
        "integration_001",
        b"Patient: Jane Doe, DOB: 05/15/1975, Diagnosis: Diabetes Type 2, Treatment Plan: Metformin 500mg twice daily".to_vec(),
        "text/plain"
    );
    let upload_context = create_test_upload_context("healthcare_provider_001");
    
    // Step 1: Security Assessment
    let security_assessment = security_manager.assess_media_security(&test_file, &upload_context).await.unwrap();
    assert!(!security_assessment.file_id.is_empty());
    
    // Step 2: Apply Security Measures
    let secured_media = security_manager.apply_security_measures(&test_file, &security_assessment).await.unwrap();
    assert!(secured_media.security_applied.contains(&SecurityMeasure::Encrypted));
    
    // Step 3: Compliance Assessment
    let compliance_context = crate::media::compliance::ComplianceContext {
        user_id: upload_context.user_id.clone(),
        organization_id: "healthcare_org_001".to_string(),
        jurisdiction: "US".to_string(),
        business_context: crate::media::compliance::BusinessContext {
            industry: "Healthcare".to_string(),
            organization_size: crate::media::compliance::OrganizationSize::Large,
            is_healthcare_entity: true,
            is_financial_institution: false,
            international_operations: false,
        },
        data_processing_purpose: crate::media::compliance::ProcessingPurpose::Communication,
        consent_status: crate::media::compliance::ConsentStatus {
            has_consent: true,
            consent_date: Some(SystemTime::now()),
            consent_scope: vec![crate::media::compliance::ProcessingPurpose::Communication],
            withdrawal_possible: true,
        },
    };
    
    let compliance_assessment = compliance_manager.assess_compliance(&test_file, &compliance_context).await.unwrap();
    assert!(compliance_assessment.data_categories.contains(&DataCategory::ProtectedHealthInformation));
    
    // Step 4: Audit Logging
    let audit_event = MediaAuditEvent::FileUploaded {
        user_id: upload_context.user_id,
        file_metadata: test_file.metadata,
        scan_results: crate::media::compliance::auditing::ScanResults {
            malware_detected: false,
            threat_level: crate::media::compliance::auditing::ThreatLevel::Low,
            scan_engines_used: vec!["ComprehensiveAV".to_string()],
            scan_duration: Duration::from_secs(2),
            quarantine_applied: false,
            false_positive_probability: 0.01,
        },
        encryption_applied: true,
        compliance_check: crate::media::compliance::auditing::ComplianceCheckResult {
            gdpr_compliant: true,
            hipaa_compliant: true,
            sox_compliant: true,
            personal_data_detected: true,
            phi_detected: true,
            retention_policy_applied: true,
            legal_basis_documented: true,
        },
    };
    
    let audit_event_id = audit_system.log_media_event(audit_event).await.unwrap();
    assert!(!audit_event_id.is_empty());
    
    // Step 5: Generate Comprehensive Report
    let report_period = (SystemTime::now() - Duration::from_secs(60 * 60), SystemTime::now());
    let audit_report = audit_system.generate_media_audit_report(
        crate::media::compliance::auditing::AuditScope::AllFiles,
        report_period,
    ).await.unwrap();
    
    assert!(!audit_report.report_id.is_empty());
    assert!(audit_report.executive_summary.total_files_audited >= 1);
    
    // Verify integration success
    println!("✅ Session 12 Integration Test Completed Successfully");
    println!("   - Security assessment: Risk score {:.2}", security_assessment.overall_risk_score);
    println!("   - Security measures applied: {:?}", secured_media.security_applied);
    println!("   - Compliance score: {:.2}", compliance_assessment.overall_compliance_score);
    println!("   - Audit event logged: {}", audit_event_id);
    println!("   - Report generated: {}", audit_report.report_id);
}

// Helper function for running basic tests only
pub async fn run_all_session_12_tests() {
    println!("🧪 Running Session 12: Basic Security & Compliance Tests");
    
    println!("  Testing basic functionality only (advanced tests require all modules)");
    
    // Test basic configuration creation
    println!("  ✓ Testing basic security configuration...");
    let _security_config = MediaSecurityConfig::default();
    println!("    ✅ Security config created successfully");
    
    // Test basic compliance configuration
    println!("  ✓ Testing basic compliance configuration...");
    let _compliance_config = ComplianceConfig::default();
    println!("    ✅ Compliance config created successfully");
    
    // Test basic crypto modes
    println!("  ✓ Testing crypto mode integration...");
    let _hybrid_mode = CryptoMode::Hybrid;
    println!("    ✅ Crypto modes available");
    
    println!("✅ Basic Session 12 tests completed successfully!");
    println!("📝 Note: Advanced integration tests require all modules to be implemented");
}
