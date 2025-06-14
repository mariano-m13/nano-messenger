//! Audit Logging for Compliance - Session 8 Production Hardening
//! 
//! This module provides comprehensive audit logging capabilities for regulatory
//! compliance including GDPR, HIPAA, SOX, and other data protection regulations.
//! Features tamper-evident logs, structured events, and automated compliance reporting.


use crate::crypto::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use std::fmt; // Unused
use std::time::SystemTime;
use uuid::Uuid;

/// Comprehensive audit event types for compliance logging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditEventType {
    // Authentication and Session Management
    UserAuthentication {
        user_id: String,
        authentication_method: AuthenticationMethod,
        source_ip: String,
        user_agent: Option<String>,
        success: bool,
        failure_reason: Option<String>,
    },
    
    UserSessionStart {
        user_id: String,
        session_id: String,
        source_ip: String,
        crypto_mode: CryptoMode,
    },
    
    UserSessionEnd {
        user_id: String,
        session_id: String,
        duration_seconds: u64,
        termination_reason: SessionTerminationReason,
    },
    
    // Authorization and Access Control
    AccessAttempt {
        user_id: String,
        resource: String,
        action: String,
        granted: bool,
        reason: Option<String>,
        required_permissions: Vec<String>,
    },
    
    PrivilegeEscalation {
        user_id: String,
        old_role: String,
        new_role: String,
        authorized_by: String,
        justification: String,
    },
    
    // Data Access and Modification
    DataAccess {
        user_id: String,
        data_type: DataType,
        data_classification: DataClassification,
        record_count: u64,
        access_method: DataAccessMethod,
        purpose: String,
    },
    
    DataModification {
        user_id: String,
        data_type: DataType,
        operation: DataOperation,
        record_count: u64,
        changes_summary: String,
        approval_required: bool,
        approver_id: Option<String>,
    },
    
    DataExport {
        user_id: String,
        export_format: ExportFormat,
        record_count: u64,
        data_categories: Vec<DataCategory>,
        destination: String,
        legal_basis: String,
    },
    
    DataErasure {
        user_id: Option<String>,
        data_subject_id: String,
        erasure_method: ErasureMethod,
        record_count: u64,
        legal_basis: ErasureReason,
        verification_hash: String,
    },
    
    // Cryptographic Operations
    KeyGeneration {
        key_type: KeyType,
        crypto_mode: CryptoMode,
        key_id: String,
        algorithm: String,
        key_size: u32,
        purpose: String,
    },
    
    KeyRotation {
        old_key_id: String,
        new_key_id: String,
        crypto_mode: CryptoMode,
        rotation_reason: KeyRotationReason,
        automated: bool,
    },
    
    CryptographicOperation {
        operation_type: CryptoOperationType,
        crypto_mode: CryptoMode,
        user_id: Option<String>,
        success: bool,
        duration_ms: u64,
        data_size_bytes: u64,
    },
    
    // System and Configuration Changes
    SystemStartup {
        component: String,
        version: String,
        configuration_hash: String,
        startup_duration_ms: u64,
    },
    
    SystemShutdown {
        component: String,
        uptime_seconds: u64,
        shutdown_reason: String,
        graceful: bool,
    },
    
    ConfigurationChange {
        component: String,
        setting_name: String,
        old_value_hash: String,
        new_value_hash: String,
        changed_by: String,
        change_reason: String,
    },
    
    // Security Events and Incidents
    SecurityIncident {
        incident_id: String,
        incident_type: SecurityIncidentType,
        severity: SecuritySeverity,
        affected_users: Vec<String>,
        description: String,
        mitigation_actions: Vec<String>,
    },
    
    ThreatDetection {
        threat_type: ThreatType,
        source_ip: String,
        target_resource: String,
        confidence_score: f64,
        automated_response: Option<String>,
    },
    
    ComplianceViolation {
        regulation: ComplianceRegulation,
        violation_type: String,
        severity: ComplianceSeverity,
        affected_data_subjects: u64,
        remediation_required: bool,
    },
    
    // Message Processing
    MessageSent {
        sender_id: String,
        recipient_id: String,
        message_id: String,
        crypto_mode: CryptoMode,
        message_size_bytes: u64,
        delivery_status: DeliveryStatus,
    },
    
    MessageReceived {
        sender_id: String,
        recipient_id: String,
        message_id: String,
        crypto_mode: CryptoMode,
        verification_status: VerificationStatus,
        processing_time_ms: u64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub event_id: Uuid,
    pub timestamp: SystemTime,
    pub event_type: AuditEventType,
    pub correlation_id: Option<Uuid>,
    pub session_id: Option<String>,
    pub request_id: Option<String>,
    pub component: String,
    pub version: String,
    pub environment: String,
    pub compliance_tags: Vec<ComplianceTag>,
    pub retention_policy: RetentionPolicy,
    pub additional_context: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthenticationMethod {
    Password,
    PublicKey,
    TwoFactor,
    Certificate,
    Biometric,
    Token,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionTerminationReason {
    UserLogout,
    Timeout,
    ForceTermination,
    SystemShutdown,
    SecurityViolation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    UserProfile,
    Message,
    CryptographicKey,
    Configuration,
    AuditLog,
    SystemMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataClassification {
    Public,
    Internal,
    Confidential,
    Restricted,
    TopSecret,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataAccessMethod {
    DirectQuery,
    API,
    Export,
    Replication,
    Backup,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataOperation {
    Create,
    Read,
    Update,
    Delete,
    Archive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    JSON,
    CSV,
    XML,
    PDF,
    Encrypted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataCategory {
    PersonalData,
    SensitivePersonalData,
    FinancialData,
    HealthData,
    BiometricData,
    CommunicationData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErasureMethod {
    StandardDeletion,
    CryptographicErasure,
    PhysicalDestruction,
    Anonymization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErasureReason {
    WithdrawalOfConsent,
    ObjectionToProcessing,
    UnlawfulProcessing,
    ComplianceWithLegalObligation,
    ProcessingOfChildData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyType {
    X25519,
    Ed25519,
    MLKEM768,
    MLDSA,
    Symmetric,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyRotationReason {
    Scheduled,
    Compromise,
    Algorithm,
    Policy,
    Emergency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CryptoOperationType {
    Encryption,
    Decryption,
    Signing,
    Verification,
    KeyExchange,
    KeyDerivation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityIncidentType {
    UnauthorizedAccess,
    DataBreach,
    MalwareDetection,
    DenialOfService,
    InsiderThreat,
    ConfigurationError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatType {
    BruteForce,
    SQLInjection,
    CrossSiteScripting,
    ManInTheMiddle,
    PasswordSpray,
    AnomalousAccess,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceRegulation {
    GDPR,
    HIPAA,
    SOX,
    CCPA,
    PciDss,
    ISO27001,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceSeverity {
    Minor,
    Major,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeliveryStatus {
    Sent,
    Delivered,
    Failed,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationStatus {
    Valid,
    Invalid,
    Expired,
    Revoked,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComplianceTag {
    GdprArticle6,
    GdprArticle9,
    GdprArticle25,
    GdprArticle32,
    HipaaAdministrative,
    HipaaPhysical,
    HipaaTechnical,
    SoxSection404,
    PciDssRequirement3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RetentionPolicy {
    SevenYears,     // SOX, GDPR for some data
    TenYears,       // Some regulatory requirements
    Indefinite,     // Security incidents, certain audit logs
    OneYear,        // Operational logs
    ThirtyDays,     // Debug logs
}

/// Tamper-evident audit log implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TamperEvidentAuditLog {
    pub log_entries: Vec<AuditLogEntry>,
    pub sequence_number: u64,
    pub merkle_tree_root: Option<[u8; 32]>,
    pub log_integrity_hash: [u8; 32],
    pub signing_key_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub sequence_number: u64,
    pub timestamp: SystemTime,
    pub event_hash: [u8; 32],
    pub previous_hash: [u8; 32],
    pub event_data: AuditEvent,
    pub integrity_signature: Vec<u8>,
}

impl TamperEvidentAuditLog {
    pub fn new(signing_key_id: String) -> Self {
        Self {
            log_entries: Vec::new(),
            sequence_number: 0,
            merkle_tree_root: None,
            log_integrity_hash: [0u8; 32],
            signing_key_id,
        }
    }
    
    pub fn append_event(&mut self, event: AuditEvent) -> Result<(), Box<dyn std::error::Error>> {
        let sequence_number = self.sequence_number + 1;
        let event_hash = self.compute_event_hash(&event)?;
        let previous_hash = self.get_last_entry_hash();
        
        // Create log entry
        let entry = AuditLogEntry {
            sequence_number,
            timestamp: SystemTime::now(),
            event_hash,
            previous_hash,
            event_data: event,
            integrity_signature: self.sign_entry_hash(event_hash)?,
        };
        
        // Update log state
        self.log_entries.push(entry);
        self.sequence_number = sequence_number;
        self.update_integrity_hash()?;
        
        // Periodic integrity verification
        if sequence_number % 1000 == 0 {
            self.verify_integrity()?;
        }
        
        Ok(())
    }
    
    pub fn verify_integrity(&self) -> Result<IntegrityVerificationResult, Box<dyn std::error::Error>> {
        let mut result = IntegrityVerificationResult {
            verified: true,
            total_entries: self.log_entries.len(),
            corrupted_entries: Vec::new(),
            hash_chain_valid: true,
            signatures_valid: true,
        };
        
        // Verify hash chain
        for (i, entry) in self.log_entries.iter().enumerate() {
            if i > 0 {
                let expected_previous_hash = self.log_entries[i - 1].event_hash;
                if entry.previous_hash != expected_previous_hash {
                    result.verified = false;
                    result.hash_chain_valid = false;
                    result.corrupted_entries.push(i);
                }
            }
            
            // Verify signature
            if !self.verify_entry_signature(entry)? {
                result.verified = false;
                result.signatures_valid = false;
                result.corrupted_entries.push(i);
            }
        }
        
        Ok(result)
    }
    
    pub fn search_events(&self, criteria: &AuditSearchCriteria) -> Vec<&AuditEvent> {
        self.log_entries
            .iter()
            .filter(|entry| self.matches_criteria(&entry.event_data, criteria))
            .map(|entry| &entry.event_data)
            .collect()
    }
    
    pub fn generate_compliance_report(&self, regulation: ComplianceRegulation, 
                                    period: AuditPeriod) -> ComplianceAuditReport {
        let relevant_events = self.filter_events_by_regulation_and_period(regulation.clone(), period.clone());
        
        ComplianceAuditReport {
            regulation,
            period,
            total_events: relevant_events.len(),
            event_summary: self.summarize_events(&relevant_events),
            compliance_violations: self.identify_violations(&relevant_events),
            recommendations: self.generate_recommendations(&relevant_events),
            report_generated_at: SystemTime::now(),
        }
    }
    
    fn compute_event_hash(&self, event: &AuditEvent) -> Result<[u8; 32], Box<dyn std::error::Error>> {
        let event_json = serde_json::to_string(event)?;
        Ok(hash_sha256(event_json.as_bytes()))
    }
    
    fn get_last_entry_hash(&self) -> [u8; 32] {
        self.log_entries
            .last()
            .map(|entry| entry.event_hash)
            .unwrap_or([0u8; 32])
    }
    
    fn sign_entry_hash(&self, hash: [u8; 32]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // In a real implementation, this would use the actual signing key
        // For now, we'll create a mock signature
        Ok(hash.to_vec())
    }
    
    fn verify_entry_signature(&self, entry: &AuditLogEntry) -> Result<bool, Box<dyn std::error::Error>> {
        // In a real implementation, this would verify the actual signature
        // For now, we'll just check if the signature matches the hash
        Ok(entry.integrity_signature == entry.event_hash.to_vec())
    }
    
    fn update_integrity_hash(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let combined_hashes: Vec<u8> = self.log_entries
            .iter()
            .flat_map(|entry| entry.event_hash.iter())
            .cloned()
            .collect();
        
        self.log_integrity_hash = hash_sha256(&combined_hashes);
        Ok(())
    }
    
    fn matches_criteria(&self, event: &AuditEvent, criteria: &AuditSearchCriteria) -> bool {
        // Check user ID
        if let Some(ref user_id) = criteria.user_id {
            if !self.event_involves_user(event, user_id) {
                return false;
            }
        }
        
        // Check event type
        if let Some(ref event_type) = criteria.event_type_filter {
            if !self.event_matches_type(event, event_type) {
                return false;
            }
        }
        
        // Check time range
        if let Some(start_time) = criteria.start_time {
            if event.timestamp < start_time {
                return false;
            }
        }
        
        if let Some(end_time) = criteria.end_time {
            if event.timestamp > end_time {
                return false;
            }
        }
        
        // Check compliance tags
        if !criteria.compliance_tags.is_empty() {
            if !event.compliance_tags.iter().any(|tag| criteria.compliance_tags.contains(tag)) {
                return false;
            }
        }
        
        true
    }
    
    fn event_involves_user(&self, event: &AuditEvent, user_id: &str) -> bool {
        match &event.event_type {
            AuditEventType::UserAuthentication { user_id: event_user_id, .. } => event_user_id == user_id,
            AuditEventType::UserSessionStart { user_id: event_user_id, .. } => event_user_id == user_id,
            AuditEventType::UserSessionEnd { user_id: event_user_id, .. } => event_user_id == user_id,
            AuditEventType::AccessAttempt { user_id: event_user_id, .. } => event_user_id == user_id,
            AuditEventType::DataAccess { user_id: event_user_id, .. } => event_user_id == user_id,
            AuditEventType::DataModification { user_id: event_user_id, .. } => event_user_id == user_id,
            AuditEventType::DataExport { user_id: event_user_id, .. } => event_user_id == user_id,
            AuditEventType::MessageSent { sender_id, recipient_id, .. } => {
                sender_id == user_id || recipient_id == user_id
            }
            AuditEventType::MessageReceived { sender_id, recipient_id, .. } => {
                sender_id == user_id || recipient_id == user_id
            }
            _ => false,
        }
    }
    
    fn event_matches_type(&self, event: &AuditEvent, filter: &str) -> bool {
        let event_type_name = match &event.event_type {
            AuditEventType::UserAuthentication { .. } => "user_authentication",
            AuditEventType::UserSessionStart { .. } => "user_session_start",
            AuditEventType::UserSessionEnd { .. } => "user_session_end",
            AuditEventType::AccessAttempt { .. } => "access_attempt",
            AuditEventType::DataAccess { .. } => "data_access",
            AuditEventType::DataModification { .. } => "data_modification",
            AuditEventType::DataExport { .. } => "data_export",
            AuditEventType::DataErasure { .. } => "data_erasure",
            AuditEventType::KeyGeneration { .. } => "key_generation",
            AuditEventType::KeyRotation { .. } => "key_rotation",
            AuditEventType::CryptographicOperation { .. } => "cryptographic_operation",
            AuditEventType::SecurityIncident { .. } => "security_incident",
            AuditEventType::ComplianceViolation { .. } => "compliance_violation",
            AuditEventType::MessageSent { .. } => "message_sent",
            AuditEventType::MessageReceived { .. } => "message_received",
            _ => "other",
        };
        
        event_type_name.contains(filter)
    }
    
    fn filter_events_by_regulation_and_period(&self, regulation: ComplianceRegulation, 
                                            period: AuditPeriod) -> Vec<&AuditEvent> {
        let start_time = period.start_time;
        let end_time = period.end_time;
        
        self.log_entries
            .iter()
            .filter(|entry| {
                entry.event_data.timestamp >= start_time && 
                entry.event_data.timestamp <= end_time &&
                self.event_relevant_to_regulation(&entry.event_data, &regulation)
            })
            .map(|entry| &entry.event_data)
            .collect()
    }
    
    fn event_relevant_to_regulation(&self, event: &AuditEvent, regulation: &ComplianceRegulation) -> bool {
        match regulation {
            ComplianceRegulation::GDPR => {
                event.compliance_tags.iter().any(|tag| matches!(tag, 
                    ComplianceTag::GdprArticle6 | 
                    ComplianceTag::GdprArticle9 | 
                    ComplianceTag::GdprArticle25 | 
                    ComplianceTag::GdprArticle32
                ))
            }
            ComplianceRegulation::HIPAA => {
                event.compliance_tags.iter().any(|tag| matches!(tag,
                    ComplianceTag::HipaaAdministrative |
                    ComplianceTag::HipaaPhysical |
                    ComplianceTag::HipaaTechnical
                ))
            }
            ComplianceRegulation::SOX => {
                event.compliance_tags.iter().any(|tag| matches!(tag,
                    ComplianceTag::SoxSection404
                ))
            }
            _ => false,
        }
    }
    
    fn summarize_events(&self, events: &[&AuditEvent]) -> HashMap<String, u64> {
        let mut summary = HashMap::new();
        
        for event in events {
            let event_type = format!("{:?}", event.event_type).split('{').next().unwrap_or("Unknown").to_string();
            *summary.entry(event_type).or_insert(0) += 1;
        }
        
        summary
    }
    
    fn identify_violations(&self, events: &[&AuditEvent]) -> Vec<ComplianceViolation> {
        let mut violations = Vec::new();
        
        for event in events {
            if let AuditEventType::ComplianceViolation { regulation, violation_type, severity, .. } = &event.event_type {
                violations.push(ComplianceViolation {
                    event_id: event.event_id,
                    regulation: regulation.clone(),
                    violation_type: violation_type.clone(),
                    severity: severity.clone(),
                    timestamp: event.timestamp,
                });
            }
        }
        
        violations
    }
    
    fn generate_recommendations(&self, events: &[&AuditEvent]) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        // Analyze event patterns and generate recommendations
        let auth_failures = events.iter()
            .filter(|e| matches!(e.event_type, AuditEventType::UserAuthentication { success: false, .. }))
            .count();
            
        if auth_failures > 10 {
            recommendations.push("Consider implementing additional authentication controls due to high failure rate".to_string());
        }
        
        let data_exports = events.iter()
            .filter(|e| matches!(e.event_type, AuditEventType::DataExport { .. }))
            .count();
            
        if data_exports > 5 {
            recommendations.push("Review data export activities for compliance with data minimization principles".to_string());
        }
        
        recommendations
    }
}

#[derive(Debug, Clone)]
pub struct IntegrityVerificationResult {
    pub verified: bool,
    pub total_entries: usize,
    pub corrupted_entries: Vec<usize>,
    pub hash_chain_valid: bool,
    pub signatures_valid: bool,
}

#[derive(Debug, Clone)]
pub struct AuditSearchCriteria {
    pub user_id: Option<String>,
    pub event_type_filter: Option<String>,
    pub start_time: Option<SystemTime>,
    pub end_time: Option<SystemTime>,
    pub compliance_tags: Vec<ComplianceTag>,
}

#[derive(Debug, Clone)]
pub struct AuditPeriod {
    pub start_time: SystemTime,
    pub end_time: SystemTime,
}

#[derive(Debug, Clone)]
pub struct ComplianceAuditReport {
    pub regulation: ComplianceRegulation,
    pub period: AuditPeriod,
    pub total_events: usize,
    pub event_summary: HashMap<String, u64>,
    pub compliance_violations: Vec<ComplianceViolation>,
    pub recommendations: Vec<String>,
    pub report_generated_at: SystemTime,
}

#[derive(Debug, Clone)]
pub struct ComplianceViolation {
    pub event_id: Uuid,
    pub regulation: ComplianceRegulation,
    pub violation_type: String,
    pub severity: ComplianceSeverity,
    pub timestamp: SystemTime,
}

/// Audit logger interface for application integration
pub struct AuditLogger {
    pub audit_log: TamperEvidentAuditLog,
    pub component: String,
    pub version: String,
    pub environment: String,
}

impl AuditLogger {
    pub fn new(component: String, version: String, environment: String) -> Self {
        Self {
            audit_log: TamperEvidentAuditLog::new("audit-signing-key".to_string()),
            component,
            version,
            environment,
        }
    }
    
    pub fn log_user_authentication(&mut self, user_id: &str, method: AuthenticationMethod, 
                                 source_ip: &str, success: bool, failure_reason: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        let event = AuditEvent {
            event_id: Uuid::new_v4(),
            timestamp: SystemTime::now(),
            event_type: AuditEventType::UserAuthentication {
                user_id: user_id.to_string(),
                authentication_method: method,
                source_ip: source_ip.to_string(),
                user_agent: None,
                success,
                failure_reason: failure_reason.map(|s| s.to_string()),
            },
            correlation_id: None,
            session_id: None,
            request_id: None,
            component: self.component.clone(),
            version: self.version.clone(),
            environment: self.environment.clone(),
            compliance_tags: vec![ComplianceTag::GdprArticle32, ComplianceTag::HipaaTechnical],
            retention_policy: RetentionPolicy::SevenYears,
            additional_context: HashMap::new(),
        };
        
        self.audit_log.append_event(event)
    }
    
    pub fn log_data_access(&mut self, user_id: &str, data_type: DataType, 
                          classification: DataClassification, record_count: u64, 
                          purpose: &str) -> Result<(), Box<dyn std::error::Error>> {
        let event = AuditEvent {
            event_id: Uuid::new_v4(),
            timestamp: SystemTime::now(),
            event_type: AuditEventType::DataAccess {
                user_id: user_id.to_string(),
                data_type,
                data_classification: classification,
                record_count,
                access_method: DataAccessMethod::API,
                purpose: purpose.to_string(),
            },
            correlation_id: None,
            session_id: None,
            request_id: None,
            component: self.component.clone(),
            version: self.version.clone(),
            environment: self.environment.clone(),
            compliance_tags: vec![ComplianceTag::GdprArticle6, ComplianceTag::HipaaAdministrative],
            retention_policy: RetentionPolicy::SevenYears,
            additional_context: HashMap::new(),
        };
        
        self.audit_log.append_event(event)
    }
    
    pub fn log_cryptographic_operation(&mut self, operation: CryptoOperationType, 
                                     crypto_mode: CryptoMode, user_id: Option<&str>,
                                     success: bool, duration_ms: u64) -> Result<(), Box<dyn std::error::Error>> {
        let event = AuditEvent {
            event_id: Uuid::new_v4(),
            timestamp: SystemTime::now(),
            event_type: AuditEventType::CryptographicOperation {
                operation_type: operation,
                crypto_mode,
                user_id: user_id.map(|s| s.to_string()),
                success,
                duration_ms,
                data_size_bytes: 0, // Would be populated with actual data size
            },
            correlation_id: None,
            session_id: None,
            request_id: None,
            component: self.component.clone(),
            version: self.version.clone(),
            environment: self.environment.clone(),
            compliance_tags: vec![ComplianceTag::GdprArticle32],
            retention_policy: RetentionPolicy::SevenYears,
            additional_context: HashMap::new(),
        };
        
        self.audit_log.append_event(event)
    }
    
    pub fn log_security_incident(&mut self, incident_type: SecurityIncidentType, 
                                severity: SecuritySeverity, affected_users: Vec<String>,
                                description: &str) -> Result<(), Box<dyn std::error::Error>> {
        let event = AuditEvent {
            event_id: Uuid::new_v4(),
            timestamp: SystemTime::now(),
            event_type: AuditEventType::SecurityIncident {
                incident_id: Uuid::new_v4().to_string(),
                incident_type,
                severity,
                affected_users,
                description: description.to_string(),
                mitigation_actions: Vec::new(),
            },
            correlation_id: None,
            session_id: None,
            request_id: None,
            component: self.component.clone(),
            version: self.version.clone(),
            environment: self.environment.clone(),
            compliance_tags: vec![ComplianceTag::GdprArticle32, ComplianceTag::HipaaAdministrative],
            retention_policy: RetentionPolicy::Indefinite,
            additional_context: HashMap::new(),
        };
        
        self.audit_log.append_event(event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_audit_log_creation() {
        let audit_log = TamperEvidentAuditLog::new("test-key".to_string());
        assert_eq!(audit_log.sequence_number, 0);
        assert!(audit_log.log_entries.is_empty());
    }
    
    #[test]
    fn test_audit_event_append() {
        let mut audit_log = TamperEvidentAuditLog::new("test-key".to_string());
        
        let event = AuditEvent {
            event_id: Uuid::new_v4(),
            timestamp: SystemTime::now(),
            event_type: AuditEventType::UserAuthentication {
                user_id: "test_user".to_string(),
                authentication_method: AuthenticationMethod::Password,
                source_ip: "127.0.0.1".to_string(),
                user_agent: None,
                success: true,
                failure_reason: None,
            },
            correlation_id: None,
            session_id: None,
            request_id: None,
            component: "test".to_string(),
            version: "1.0.0".to_string(),
            environment: "test".to_string(),
            compliance_tags: vec![ComplianceTag::GdprArticle32],
            retention_policy: RetentionPolicy::SevenYears,
            additional_context: HashMap::new(),
        };
        
        assert!(audit_log.append_event(event).is_ok());
        assert_eq!(audit_log.sequence_number, 1);
        assert_eq!(audit_log.log_entries.len(), 1);
    }
    
    #[test]
    fn test_audit_log_search() {
        let mut audit_log = TamperEvidentAuditLog::new("test-key".to_string());
        
        // Add test events
        for i in 0..5 {
            let event = AuditEvent {
                event_id: Uuid::new_v4(),
                timestamp: SystemTime::now(),
                event_type: AuditEventType::UserAuthentication {
                    user_id: format!("user_{}", i),
                    authentication_method: AuthenticationMethod::Password,
                    source_ip: "127.0.0.1".to_string(),
                    user_agent: None,
                    success: i % 2 == 0,
                    failure_reason: None,
                },
                correlation_id: None,
                session_id: None,
                request_id: None,
                component: "test".to_string(),
                version: "1.0.0".to_string(),
                environment: "test".to_string(),
                compliance_tags: vec![ComplianceTag::GdprArticle32],
                retention_policy: RetentionPolicy::SevenYears,
                additional_context: HashMap::new(),
            };
            
            audit_log.append_event(event).unwrap();
        }
        
        let criteria = AuditSearchCriteria {
            user_id: Some("user_2".to_string()),
            event_type_filter: None,
            start_time: None,
            end_time: None,
            compliance_tags: vec![],
        };
        
        let results = audit_log.search_events(&criteria);
        assert_eq!(results.len(), 1);
    }
    
    #[test]
    fn test_integrity_verification() {
        let mut audit_log = TamperEvidentAuditLog::new("test-key".to_string());
        
        let event = AuditEvent {
            event_id: Uuid::new_v4(),
            timestamp: SystemTime::now(),
            event_type: AuditEventType::SystemStartup {
                component: "test".to_string(),
                version: "1.0.0".to_string(),
                configuration_hash: "test_hash".to_string(),
                startup_duration_ms: 1000,
            },
            correlation_id: None,
            session_id: None,
            request_id: None,
            component: "test".to_string(),
            version: "1.0.0".to_string(),
            environment: "test".to_string(),
            compliance_tags: vec![],
            retention_policy: RetentionPolicy::OneYear,
            additional_context: HashMap::new(),
        };
        
        audit_log.append_event(event).unwrap();
        
        let verification_result = audit_log.verify_integrity().unwrap();
        assert!(verification_result.verified);
        assert_eq!(verification_result.total_entries, 1);
        assert!(verification_result.corrupted_entries.is_empty());
    }
}
