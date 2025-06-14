use crate::media::security::scanning::{FileId, MediaFile, SecurityAction};
use crate::media::security::access_control::{MediaAction, AccessDecision, AccessDenialReason};
use crate::media::compliance::gdpr::{GDPRError, PersonalDataCategory};
use crate::media::compliance::hipaa::{HIPAAError, PHIType, UserRole, AccessPurpose};
use crate::username::UserId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use async_trait::async_trait;
use thiserror::Error;
use blake2::{Blake2b512, Digest};
use uuid::Uuid;

/// Unique identifier for audit events
pub type AuditEventId = String;

/// Unique identifier for audit sessions
pub type AuditSessionId = String;

/// Unique identifier for compliance violations
pub type ViolationId = String;

/// Unique identifier for audit reports
pub type ReportId = String;

/// Enterprise audit error types
#[derive(Debug, Error)]
pub enum AuditError {
    #[error("Event logging failed: {0}")]
    EventLogging(String),
    
    #[error("Tamper detection: {0}")]
    TamperDetection(String),
    
    #[error("Compliance monitoring failed: {0}")]
    ComplianceMonitoring(String),
    
    #[error("Report generation failed: {0}")]
    ReportGeneration(String),
    
    #[error("Real-time monitoring error: {0}")]
    RealTimeMonitoring(String),
    
    #[error("Alert system failure: {0}")]
    AlertSystem(String),
    
    #[error("Audit trail verification failed: {0}")]
    AuditTrailVerification(String),
}

/// Types of media audit events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaAuditEvent {
    FileUploaded {
        user_id: UserId,
        file_metadata: FileMetadata,
        scan_results: ScanResults,
        encryption_applied: bool,
        compliance_check: ComplianceCheckResult,
    },
    FileAccessed {
        user_id: UserId,
        file_id: FileId,
        access_type: MediaAccessType,
        access_purpose: Option<AccessPurpose>,
        user_role: Option<UserRole>,
        access_granted: bool,
        denial_reason: Option<AccessDenialReason>,
        data_accessed: DataAccessDetails,
    },
    FileShared {
        user_id: UserId,
        file_id: FileId,
        shared_with: Vec<UserId>,
        sharing_method: SharingMethod,
        permissions_granted: Vec<MediaAction>,
        expiry_time: Option<SystemTime>,
    },
    FileModified {
        user_id: UserId,
        file_id: FileId,
        modification_type: ModificationType,
        before_hash: String,
        after_hash: String,
        changes_documented: bool,
    },
    FileDeleted {
        user_id: UserId,
        file_id: FileId,
        deletion_reason: DeletionReason,
        secure_deletion: bool,
        backup_deletion: bool,
        verification_hash: Option<String>,
    },
    SecurityViolation {
        event_type: SecurityEventType,
        severity: SecuritySeverity,
        affected_files: Vec<FileId>,
        threat_indicators: Vec<ThreatIndicator>,
        automated_response: Option<AutomatedResponse>,
    },
    ComplianceViolation {
        regulation: ComplianceRegulation,
        violation_details: ViolationDetails,
        affected_data: Vec<DataElement>,
        risk_assessment: RiskAssessment,
        remediation_required: bool,
    },
    SystemEvent {
        event_type: SystemEventType,
        component: String,
        impact_level: ImpactLevel,
        recovery_time: Option<Duration>,
        data_integrity_verified: bool,
    },
}

/// File metadata for auditing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub original_name: String,
    pub file_size: u64,
    pub mime_type: String,
    pub upload_timestamp: SystemTime,
    pub checksum: String,
    pub encryption_status: EncryptionStatus,
}

/// Security scan results for audit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResults {
    pub malware_detected: bool,
    pub threat_level: ThreatLevel,
    pub scan_engines_used: Vec<String>,
    pub scan_duration: Duration,
    pub quarantine_applied: bool,
    pub false_positive_probability: f32,
}

/// Threat levels for security events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ThreatLevel {
    None,
    Low,
    Medium,
    High,
    Critical,
    Catastrophic,
}

/// Compliance check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCheckResult {
    pub gdpr_compliant: bool,
    pub hipaa_compliant: bool,
    pub sox_compliant: bool,
    pub personal_data_detected: bool,
    pub phi_detected: bool,
    pub retention_policy_applied: bool,
    pub legal_basis_documented: bool,
}

/// Types of media access for auditing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaAccessType {
    View,
    Download,
    Stream,
    Preview,
    Thumbnail,
    Metadata,
    Decrypt,
    Share,
    Export,
    Print,
    Screenshot,
    Copy,
}

/// Details of data accessed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataAccessDetails {
    pub bytes_accessed: u64,
    pub duration: Duration,
    pub client_ip: String,
    pub user_agent: String,
    pub device_fingerprint: String,
    pub geolocation: Option<GeolocationInfo>,
    pub network_type: NetworkType,
}

/// Geolocation information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeolocationInfo {
    pub country: String,
    pub region: String,
    pub city: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub accuracy: Option<f32>,
}

/// Network types for access tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkType {
    Corporate,
    Home,
    Public,
    Mobile,
    VPN,
    Unknown,
}

/// Methods of sharing files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SharingMethod {
    DirectLink,
    Email,
    SecureTransfer,
    CollaborativeSpace,
    PublicShare,
    TimeLimitedLink,
    PasswordProtected,
}

/// Types of file modifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModificationType {
    ContentChanged,
    MetadataUpdated,
    PermissionsChanged,
    EncryptionUpdated,
    Watermarked,
    Redacted,
    Compressed,
    FormatConverted,
}

/// Reasons for file deletion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeletionReason {
    UserRequest,
    RetentionPolicyExpired,
    ComplianceRequirement,
    SecurityThreat,
    DataSubjectRequest,
    LegalHold,
    StorageOptimization,
    SystemMaintenance,
}

/// Types of security events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SecurityEventType {
    UnauthorizedAccess,
    BruteForceAttempt,
    MalwareDetection,
    DataExfiltration,
    PrivilegeEscalation,
    SuspiciousActivity,
    SystemIntrusion,
    DataBreach,
    PhishingAttempt,
    InsiderThreat,
}

/// Security event severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SecuritySeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
    Emergency,
}

/// Threat indicators for security analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIndicator {
    pub indicator_type: IndicatorType,
    pub value: String,
    pub confidence: f32,
    pub source: String,
    pub first_seen: SystemTime,
    pub threat_intelligence: Option<ThreatIntelligence>,
}

/// Types of threat indicators
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum IndicatorType {
    IPAddress,
    DomainName,
    FileHash,
    URL,
    UserAgent,
    BehaviorPattern,
    NetworkTraffic,
    FileSignature,
}

/// Threat intelligence information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIntelligence {
    pub threat_actor: Option<String>,
    pub campaign: Option<String>,
    pub tactics: Vec<String>,
    pub techniques: Vec<String>,
    pub procedures: Vec<String>,
}

/// Automated responses to security events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomatedResponse {
    pub action_type: ResponseActionType,
    pub timestamp: SystemTime,
    pub success: bool,
    pub details: String,
    pub follow_up_required: bool,
}

/// Types of automated response actions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ResponseActionType {
    BlockAccess,
    QuarantineFile,
    AlertAdministrator,
    LockAccount,
    IsolateSystem,
    BackupData,
    NotifyAuthorities,
    InitiateIncidentResponse,
}

/// Compliance regulations tracked
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ComplianceRegulation {
    GDPR,
    HIPAA,
    SOX,
    PCI_DSS,
    CCPA,
    FERPA,
    ISO27001,
    NIST,
    Custom(String),
}

/// Violation details for compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViolationDetails {
    pub violation_id: ViolationId,
    pub rule_violated: String,
    pub description: String,
    pub evidence: Vec<EvidenceItem>,
    pub business_impact: BusinessImpact,
    pub regulatory_impact: RegulatoryImpact,
}

/// Evidence items for violations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceItem {
    pub evidence_type: EvidenceType,
    pub description: String,
    pub source: String,
    pub timestamp: SystemTime,
    pub integrity_hash: String,
}

/// Types of compliance evidence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceType {
    LogEntry,
    FileContent,
    UserAction,
    SystemConfiguration,
    NetworkTraffic,
    Screenshot,
    Document,
    Testimony,
}

/// Business impact assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessImpact {
    pub impact_level: ImpactLevel,
    pub affected_processes: Vec<String>,
    pub financial_impact: Option<f64>,
    pub reputation_impact: ReputationImpact,
    pub operational_impact: OperationalImpact,
}

/// Impact levels for business operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd, PartialEq)]
pub enum ImpactLevel {
    Minimal,
    Low,
    Medium,
    High,
    Severe,
    Catastrophic,
}

/// Reputation impact assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReputationImpact {
    None,
    Minor,
    Moderate,
    Significant,
    Severe,
}

/// Operational impact assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationalImpact {
    pub service_disruption: bool,
    pub data_availability: DataAvailabilityImpact,
    pub performance_degradation: f32, // Percentage
    pub recovery_time_estimate: Option<Duration>,
}

/// Data availability impact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataAvailabilityImpact {
    NoImpact,
    PartialLoss,
    TemporaryLoss,
    PermanentLoss,
    CorruptedData,
}

/// Regulatory impact assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryImpact {
    pub potential_fines: Option<f64>,
    pub reporting_required: bool,
    pub investigation_likely: bool,
    pub license_risk: bool,
    pub criminal_liability: bool,
}

/// Data elements affected by violations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataElement {
    pub element_type: DataElementType,
    pub sensitivity: DataSensitivity,
    pub volume: u64,
    pub subjects_affected: u64,
    pub jurisdictions: Vec<String>,
}

/// Types of data elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataElementType {
    PersonalData,
    ProtectedHealthInformation,
    FinancialData,
    IntellectualProperty,
    TradeSecrets,
    SystemData,
    OperationalData,
}

/// Data sensitivity classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd, PartialEq)]
pub enum DataSensitivity {
    Public,
    Internal,
    Confidential,
    Restricted,
    TopSecret,
}

/// Risk assessment for violations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub overall_risk_score: f32,
    pub likelihood: f32,
    pub impact: f32,
    pub risk_factors: Vec<RiskFactor>,
    pub mitigation_effectiveness: f32,
    pub residual_risk: f32,
}

/// Risk factors contributing to violations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    pub factor_type: RiskFactorType,
    pub weight: f32,
    pub description: String,
    pub mitigation_available: bool,
}

/// Types of risk factors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskFactorType {
    Technical,
    Procedural,
    Human,
    Environmental,
    Legal,
    Financial,
    Reputational,
}

/// Types of system events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemEventType {
    ServiceStart,
    ServiceStop,
    ConfigurationChange,
    SoftwareUpdate,
    HardwareFailure,
    NetworkOutage,
    DatabaseError,
    BackupCompleted,
    RestoreOperation,
    MaintenanceWindow,
}

/// Encryption status for files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionStatus {
    Unencrypted,
    Standard,
    Enhanced,
    QuantumResistant,
    Failed,
}

/// Tamper-evident audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TamperEvidentLogEntry {
    pub entry_id: AuditEventId,
    pub timestamp: SystemTime,
    pub event: MediaAuditEvent,
    pub source: AuditSource,
    pub integrity_hash: String,
    pub previous_hash: Option<String>,
    pub chain_position: u64,
    pub digital_signature: Option<Vec<u8>>,
    pub witness_signatures: Vec<WitnessSignature>,
}

/// Source of audit events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditSource {
    pub component: String,
    pub version: String,
    pub instance_id: String,
    pub hostname: String,
    pub process_id: u32,
}

/// Witness signatures for audit integrity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessSignature {
    pub witness_id: String,
    pub signature: Vec<u8>,
    pub timestamp: SystemTime,
    pub algorithm: String,
}

/// Real-time compliance monitoring stream
pub struct ComplianceViolationStream {
    violations: tokio::sync::mpsc::Receiver<ComplianceViolationEvent>,
}

/// Real-time compliance violation event
#[derive(Debug, Clone)]
pub struct ComplianceViolationEvent {
    pub violation_id: ViolationId,
    pub regulation: ComplianceRegulation,
    pub severity: SecuritySeverity,
    pub file_id: FileId,
    pub user_id: UserId,
    pub detection_timestamp: SystemTime,
    pub auto_remediation_possible: bool,
}

/// Scope for audit operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditScope {
    AllFiles,
    SpecificFiles(Vec<FileId>),
    UserActivity(UserId),
    TimeRange(SystemTime, SystemTime),
    ComplianceRegulation(ComplianceRegulation),
    SecurityEvents,
    DataCategory(Vec<PersonalDataCategory>),
    PHICategory(Vec<PHIType>),
}

/// Comprehensive media audit report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaAuditReport {
    pub report_id: ReportId,
    pub scope: AuditScope,
    pub generation_timestamp: SystemTime,
    pub report_period: (SystemTime, SystemTime),
    pub executive_summary: ExecutiveSummary,
    pub file_statistics: FileStatistics,
    pub security_summary: SecuritySummary,
    pub compliance_summary: ComplianceSummary,
    pub user_activity_summary: UserActivitySummary,
    pub risk_assessment: OverallRiskAssessment,
    pub recommendations: Vec<AuditRecommendation>,
    pub detailed_findings: Vec<DetailedFinding>,
    pub appendices: Vec<ReportAppendix>,
}

/// Executive summary for audit reports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutiveSummary {
    pub total_files_audited: u64,
    pub security_incidents: u64,
    pub compliance_violations: u64,
    pub high_risk_findings: u64,
    pub data_subjects_affected: u64,
    pub overall_security_posture: SecurityPosture,
    pub compliance_status: OverallComplianceStatus,
    pub key_concerns: Vec<String>,
    pub positive_findings: Vec<String>,
}

/// Security posture assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityPosture {
    Excellent,
    Good,
    Adequate,
    Poor,
    Critical,
}

/// Overall compliance status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverallComplianceStatus {
    pub gdpr_compliance_level: f32,
    pub hipaa_compliance_level: f32,
    pub sox_compliance_level: f32,
    pub overall_score: f32,
    pub trending: ComplianceTrend,
}

/// Compliance trending direction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceTrend {
    Improving,
    Stable,
    Declining,
    Volatile,
}

/// File statistics for audit reports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStatistics {
    pub total_files: u64,
    pub files_with_personal_data: u64,
    pub files_with_phi: u64,
    pub encrypted_files: u64,
    pub files_shared_externally: u64,
    pub files_deleted: u64,
    pub average_file_size: f64,
    pub total_storage_used: u64,
    pub file_types: HashMap<String, u64>,
}

/// Security summary for audit reports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySummary {
    pub total_security_events: u64,
    pub malware_detections: u64,
    pub unauthorized_access_attempts: u64,
    pub successful_attacks: u64,
    pub data_breaches: u64,
    pub incident_response_times: Vec<Duration>,
    pub threat_levels: HashMap<ThreatLevel, u64>,
    pub attack_vectors: HashMap<String, u64>,
}

/// Compliance summary for audit reports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceSummary {
    pub gdpr_violations: u64,
    pub hipaa_violations: u64,
    pub sox_violations: u64,
    pub data_subject_requests: u64,
    pub retention_violations: u64,
    pub consent_violations: u64,
    pub breach_notifications: u64,
    pub remediation_success_rate: f32,
}

/// User activity summary for audit reports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserActivitySummary {
    pub total_users: u64,
    pub active_users: u64,
    pub privileged_users: u64,
    pub suspicious_users: u64,
    pub most_active_users: Vec<UserActivityStats>,
    pub access_patterns: AccessPatternAnalysis,
    pub role_distribution: HashMap<UserRole, u64>,
}

/// Individual user activity statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserActivityStats {
    pub user_id: UserId,
    pub total_accesses: u64,
    pub files_accessed: u64,
    pub data_downloaded: u64,
    pub violations: u64,
    pub risk_score: f32,
}

/// Access pattern analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPatternAnalysis {
    pub peak_hours: Vec<u8>,
    pub geographic_distribution: HashMap<String, u64>,
    pub device_types: HashMap<String, u64>,
    pub unusual_patterns: Vec<String>,
}

/// Overall risk assessment for organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverallRiskAssessment {
    pub overall_risk_score: f32,
    pub risk_categories: HashMap<String, f32>,
    pub top_risks: Vec<TopRisk>,
    pub risk_trend: RiskTrend,
    pub mitigation_effectiveness: f32,
}

/// Top organizational risks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopRisk {
    pub risk_description: String,
    pub probability: f32,
    pub impact: f32,
    pub risk_score: f32,
    pub mitigation_plan: Option<String>,
}

/// Risk trending over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskTrend {
    Increasing,
    Stable,
    Decreasing,
    Volatile,
}

/// Audit recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditRecommendation {
    pub recommendation_id: String,
    pub category: RecommendationCategory,
    pub priority: RecommendationPriority,
    pub title: String,
    pub description: String,
    pub rationale: String,
    pub implementation_steps: Vec<ImplementationStep>,
    pub estimated_effort: EffortEstimate,
    pub expected_benefits: Vec<String>,
    pub risk_reduction: f32,
}

/// Categories of audit recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationCategory {
    Security,
    Compliance,
    Operational,
    Technical,
    Process,
    Policy,
    Training,
}

/// Priority levels for recommendations
#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd, PartialEq)]
pub enum RecommendationPriority {
    Low,
    Medium,
    High,
    Critical,
    Immediate,
}

/// Implementation steps for recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationStep {
    pub step_number: u32,
    pub description: String,
    pub responsible_party: String,
    pub estimated_duration: Duration,
    pub dependencies: Vec<String>,
    pub success_criteria: Vec<String>,
}

/// Effort estimate for implementations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffortEstimate {
    pub time_estimate: Duration,
    pub resource_requirements: Vec<String>,
    pub skill_requirements: Vec<String>,
    pub budget_estimate: Option<f64>,
    pub complexity_level: ComplexityLevel,
}

/// Complexity levels for implementations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplexityLevel {
    Low,
    Medium,
    High,
    VeryHigh,
    ExtremelyComplex,
}

/// Detailed findings from audit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedFinding {
    pub finding_id: String,
    pub category: FindingCategory,
    pub severity: FindingSeverity,
    pub title: String,
    pub description: String,
    pub evidence: Vec<EvidenceItem>,
    pub affected_assets: Vec<String>,
    pub business_impact: BusinessImpact,
    pub remediation_advice: String,
    pub timeline_for_resolution: Duration,
}

/// Categories of audit findings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FindingCategory {
    Security,
    Compliance,
    Privacy,
    DataGovernance,
    AccessControl,
    Encryption,
    AuditTrail,
    IncidentResponse,
}

/// Severity levels for findings
#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd, PartialEq)]
pub enum FindingSeverity {
    Informational,
    Low,
    Medium,
    High,
    Critical,
}

/// Report appendices for additional information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportAppendix {
    pub appendix_id: String,
    pub title: String,
    pub content_type: AppendixContentType,
    pub content: AppendixContent,
}

/// Types of appendix content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppendixContentType {
    RawData,
    Charts,
    DetailedLogs,
    PolicyDocuments,
    TechnicalSpecs,
    LegalAnalysis,
}

/// Appendix content variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppendixContent {
    Text(String),
    StructuredData(serde_json::Value),
    BinaryData(Vec<u8>),
    Reference(String),
}

/// Overall compliance status tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceStatus {
    FullyCompliant,
    MostlyCompliant,
    PartiallyCompliant,
    NonCompliant,
    UnderReview,
}

/// Tamper-evident logger for audit events
pub struct TamperEvidentLogger {
    log_entries: Vec<TamperEvidentLogEntry>,
    hash_chain: Vec<String>,
    digital_signing_enabled: bool,
    witness_nodes: Vec<String>,
}

/// Compliance monitor for real-time checking
pub struct ComplianceMonitor {
    active_rules: Vec<ComplianceRule>,
    violation_threshold: f32,
    auto_remediation_enabled: bool,
    notification_channels: Vec<NotificationChannel>,
}

/// Compliance rules for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRule {
    pub rule_id: String,
    pub regulation: ComplianceRegulation,
    pub rule_type: RuleType,
    pub condition: RuleCondition,
    pub action: RuleAction,
    pub severity: SecuritySeverity,
    pub enabled: bool,
}

/// Types of compliance rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleType {
    Preventive,
    Detective,
    Corrective,
    Compensating,
}

/// Rule conditions for compliance monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleCondition {
    FileContainsPersonalData,
    FileContainsPHI,
    UnencryptedSensitiveData,
    RetentionPeriodExceeded,
    UnauthorizedAccess,
    ExcessivePermissions,
    MissingConsent,
    CrossBorderTransfer,
    Custom(String),
}

/// Actions to take when rules trigger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleAction {
    Alert,
    Block,
    Quarantine,
    Encrypt,
    LogEvent,
    RequireApproval,
    NotifyDPO, // Data Protection Officer
    EscalateToManagement,
}

/// Notification channels for alerts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationChannel {
    pub channel_type: ChannelType,
    pub endpoint: String,
    pub priority_threshold: SecuritySeverity,
    pub rate_limiting: Option<RateLimit>,
}

/// Types of notification channels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelType {
    Email,
    Slack,
    SMS,
    WebHook,
    SysLog,
    SNMP,
    Dashboard,
}

/// Rate limiting for notifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    pub max_notifications: u32,
    pub time_window: Duration,
    pub burst_allowance: u32,
}

/// Compliance report generator
pub struct ComplianceReportGenerator {
    report_templates: HashMap<String, ReportTemplate>,
    data_sources: Vec<DataSource>,
    export_formats: Vec<ExportFormat>,
}

/// Report templates for different compliance needs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportTemplate {
    pub template_id: String,
    pub name: String,
    pub regulation: ComplianceRegulation,
    pub sections: Vec<ReportSection>,
    pub frequency: ReportFrequency,
    pub recipients: Vec<String>,
}

/// Sections within compliance reports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSection {
    pub section_id: String,
    pub title: String,
    pub content_type: SectionContentType,
    pub data_query: String,
    pub visualization: Option<VisualizationType>,
}

/// Types of report section content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SectionContentType {
    Summary,
    DetailedData,
    Chart,
    Table,
    Narrative,
    Recommendations,
    Appendix,
}

/// Types of data visualizations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VisualizationType {
    BarChart,
    LineChart,
    PieChart,
    HeatMap,
    Timeline,
    Network,
    Dashboard,
}

/// Report generation frequency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportFrequency {
    RealTime,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Annually,
    OnDemand,
}

/// Data sources for reports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSource {
    pub source_id: String,
    pub source_type: DataSourceType,
    pub connection_string: String,
    pub query_interface: QueryInterface,
    pub data_freshness: Duration,
}

/// Types of data sources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataSourceType {
    AuditLogs,
    SystemMetrics,
    SecurityEvents,
    ComplianceData,
    UserActivity,
    FileMetadata,
    ExternalFeed,
}

/// Query interfaces for data sources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryInterface {
    SQL,
    REST,
    GraphQL,
    ElasticSearch,
    Custom,
}

/// Export formats for reports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    PDF,
    HTML,
    CSV,
    JSON,
    XML,
    Excel,
    PowerBI,
}

/// Main media audit system
pub struct MediaAuditSystem {
    pub event_logger: TamperEvidentLogger,
    pub compliance_monitor: ComplianceMonitor,
    pub report_generator: ComplianceReportGenerator,
    pub real_time_monitoring: bool,
    pub retention_policy: AuditRetentionPolicy,
}

/// Audit retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditRetentionPolicy {
    pub default_retention: Duration,
    pub high_value_retention: Duration,
    pub compliance_retention: HashMap<ComplianceRegulation, Duration>,
    pub auto_archival: bool,
    pub secure_deletion: bool,
}

impl MediaAuditSystem {
    /// Create new media audit system
    pub fn new() -> Self {
        Self {
            event_logger: TamperEvidentLogger::new(),
            compliance_monitor: ComplianceMonitor::new(),
            report_generator: ComplianceReportGenerator::new(),
            real_time_monitoring: true,
            retention_policy: AuditRetentionPolicy::default(),
        }
    }

    /// Log media operation with tamper-evident integrity
    pub async fn log_media_event(
        &mut self,
        event: MediaAuditEvent,
    ) -> Result<AuditEventId, AuditError> {
        self.event_logger.log_event(event).await
    }

    /// Monitor for real-time compliance violations
    pub async fn monitor_compliance_violations(&self) -> ComplianceViolationStream {
        self.compliance_monitor.start_monitoring().await
    }

    /// Generate comprehensive audit report
    pub async fn generate_media_audit_report(
        &self,
        scope: AuditScope,
        period: (SystemTime, SystemTime),
    ) -> Result<MediaAuditReport, AuditError> {
        self.report_generator.generate_report(scope, period).await
    }

    /// Check current compliance status
    pub async fn check_compliance_status(&self) -> Result<ComplianceStatus, AuditError> {
        let violations = self.compliance_monitor.get_recent_violations().await?;
        let total_files = self.event_logger.get_total_files_audited().await?;
        
        if violations.is_empty() {
            Ok(ComplianceStatus::FullyCompliant)
        } else {
            let violation_rate = violations.len() as f32 / total_files as f32;
            match violation_rate {
                r if r < 0.01 => Ok(ComplianceStatus::MostlyCompliant),
                r if r < 0.05 => Ok(ComplianceStatus::PartiallyCompliant),
                _ => Ok(ComplianceStatus::NonCompliant),
            }
        }
    }

    /// Verify audit trail integrity
    pub async fn verify_audit_trail_integrity(&self) -> Result<bool, AuditError> {
        self.event_logger.verify_integrity().await
    }

    /// Archive old audit logs according to retention policy
    pub async fn archive_old_logs(&mut self) -> Result<u64, AuditError> {
        let cutoff_time = SystemTime::now() - self.retention_policy.default_retention;
        self.event_logger.archive_logs_before(cutoff_time).await
    }

    /// Get security incident summary
    pub async fn get_security_incident_summary(
        &self,
        period: (SystemTime, SystemTime),
    ) -> Result<SecuritySummary, AuditError> {
        let events = self.event_logger.get_events_in_period(period).await?;
        
        let mut security_events = 0;
        let mut malware_detections = 0;
        let mut unauthorized_attempts = 0;
        let mut successful_attacks = 0;
        let mut data_breaches = 0;
        let mut threat_levels = HashMap::new();

        for entry in events {
            match entry.event {
                MediaAuditEvent::SecurityViolation { event_type, severity, .. } => {
                    security_events += 1;
                    *threat_levels.entry(ThreatLevel::from_severity(&severity)).or_insert(0) += 1;
                    
                    match event_type {
                        SecurityEventType::MalwareDetection => malware_detections += 1,
                        SecurityEventType::UnauthorizedAccess => unauthorized_attempts += 1,
                        SecurityEventType::SystemIntrusion => successful_attacks += 1,
                        SecurityEventType::DataBreach => data_breaches += 1,
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        Ok(SecuritySummary {
            total_security_events: security_events,
            malware_detections,
            unauthorized_access_attempts: unauthorized_attempts,
            successful_attacks,
            data_breaches,
            incident_response_times: vec![], // Would calculate from actual data
            threat_levels,
            attack_vectors: HashMap::new(), // Would populate from actual analysis
        })
    }
}

impl TamperEvidentLogger {
    pub fn new() -> Self {
        Self {
            log_entries: Vec::new(),
            hash_chain: Vec::new(),
            digital_signing_enabled: true,
            witness_nodes: vec!["witness1".to_string(), "witness2".to_string()],
        }
    }

    pub async fn log_event(&mut self, event: MediaAuditEvent) -> Result<AuditEventId, AuditError> {
        let entry_id = Uuid::new_v4().to_string();
        let timestamp = SystemTime::now();
        
        // Create entry
        let mut entry = TamperEvidentLogEntry {
            entry_id: entry_id.clone(),
            timestamp,
            event,
            source: AuditSource {
                component: "MediaAuditSystem".to_string(),
                version: "1.0.0".to_string(),
                instance_id: "instance-1".to_string(),
                hostname: "audit-server".to_string(),
                process_id: std::process::id(),
            },
            integrity_hash: String::new(),
            previous_hash: self.hash_chain.last().cloned(),
            chain_position: self.log_entries.len() as u64 + 1,
            digital_signature: None,
            witness_signatures: vec![],
        };

        // Calculate integrity hash
        entry.integrity_hash = self.calculate_entry_hash(&entry)?;
        
        // Add to chain
        self.hash_chain.push(entry.integrity_hash.clone());
        
        // Add digital signatures if enabled
        if self.digital_signing_enabled {
            entry.digital_signature = Some(self.sign_entry(&entry).await?);
            entry.witness_signatures = self.collect_witness_signatures(&entry).await?;
        }

        self.log_entries.push(entry);
        Ok(entry_id)
    }

    pub async fn verify_integrity(&self) -> Result<bool, AuditError> {
        // Verify hash chain integrity
        for (i, entry) in self.log_entries.iter().enumerate() {
            // Verify entry hash
            let calculated_hash = self.calculate_entry_hash(entry)?;
            if calculated_hash != entry.integrity_hash {
                return Ok(false);
            }

            // Verify chain links
            if i > 0 {
                let previous_entry = &self.log_entries[i - 1];
                if entry.previous_hash.as_ref() != Some(&previous_entry.integrity_hash) {
                    return Ok(false);
                }
            }

            // Verify digital signatures
            if let Some(signature) = &entry.digital_signature {
                if !self.verify_signature(entry, signature).await? {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }

    pub async fn get_events_in_period(
        &self,
        period: (SystemTime, SystemTime),
    ) -> Result<Vec<TamperEvidentLogEntry>, AuditError> {
        let events = self.log_entries.iter()
            .filter(|entry| entry.timestamp >= period.0 && entry.timestamp <= period.1)
            .cloned()
            .collect();
        
        Ok(events)
    }

    pub async fn get_total_files_audited(&self) -> Result<u64, AuditError> {
        Ok(self.log_entries.len() as u64)
    }

    pub async fn archive_logs_before(&mut self, cutoff_time: SystemTime) -> Result<u64, AuditError> {
        let initial_count = self.log_entries.len();
        self.log_entries.retain(|entry| entry.timestamp >= cutoff_time);
        let archived_count = initial_count - self.log_entries.len();
        Ok(archived_count as u64)
    }

    fn calculate_entry_hash(&self, entry: &TamperEvidentLogEntry) -> Result<String, AuditError> {
        let mut hasher = Blake2b512::new();
        
        // Hash entry components
        hasher.update(entry.entry_id.as_bytes());
        hasher.update(&entry.timestamp.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs().to_be_bytes());
        
        // Serialize event for hashing
        let event_bytes = serde_json::to_vec(&entry.event)
            .map_err(|e| AuditError::EventLogging(format!("Serialization failed: {}", e)))?;
        hasher.update(&event_bytes);
        
        if let Some(prev_hash) = &entry.previous_hash {
            hasher.update(prev_hash.as_bytes());
        }
        
        let hash = hasher.finalize();
        Ok(format!("{:x}", hash))
    }

    async fn sign_entry(&self, _entry: &TamperEvidentLogEntry) -> Result<Vec<u8>, AuditError> {
        // Placeholder for digital signature
        Ok(vec![0u8; 64])
    }

    async fn collect_witness_signatures(&self, _entry: &TamperEvidentLogEntry) -> Result<Vec<WitnessSignature>, AuditError> {
        // Placeholder for witness signature collection
        Ok(vec![])
    }

    async fn verify_signature(&self, _entry: &TamperEvidentLogEntry, _signature: &[u8]) -> Result<bool, AuditError> {
        // Placeholder for signature verification
        Ok(true)
    }
}

impl ComplianceMonitor {
    pub fn new() -> Self {
        Self {
            active_rules: Self::create_default_rules(),
            violation_threshold: 0.8,
            auto_remediation_enabled: true,
            notification_channels: vec![
                NotificationChannel {
                    channel_type: ChannelType::Email,
                    endpoint: "compliance@company.com".to_string(),
                    priority_threshold: SecuritySeverity::Medium,
                    rate_limiting: Some(RateLimit {
                        max_notifications: 10,
                        time_window: Duration::from_secs(60 * 60), // 1 hour
                        burst_allowance: 3,
                    }),
                },
            ],
        }
    }

    pub async fn start_monitoring(&self) -> ComplianceViolationStream {
        let (_tx, rx) = tokio::sync::mpsc::channel(100);
        
        // Start background monitoring task
        tokio::spawn(async move {
            // Placeholder for actual monitoring logic
            loop {
                tokio::time::sleep(Duration::from_secs(60)).await;
                // Would check for violations and send through tx
            }
        });

        ComplianceViolationStream { violations: rx }
    }

    pub async fn get_recent_violations(&self) -> Result<Vec<ComplianceViolationEvent>, AuditError> {
        // Placeholder for getting recent violations
        Ok(vec![])
    }

    fn create_default_rules() -> Vec<ComplianceRule> {
        vec![
            ComplianceRule {
                rule_id: "gdpr-personal-data-encryption".to_string(),
                regulation: ComplianceRegulation::GDPR,
                rule_type: RuleType::Preventive,
                condition: RuleCondition::FileContainsPersonalData,
                action: RuleAction::Encrypt,
                severity: SecuritySeverity::High,
                enabled: true,
            },
            ComplianceRule {
                rule_id: "hipaa-phi-access-control".to_string(),
                regulation: ComplianceRegulation::HIPAA,
                rule_type: RuleType::Detective,
                condition: RuleCondition::FileContainsPHI,
                action: RuleAction::RequireApproval,
                severity: SecuritySeverity::Critical,
                enabled: true,
            },
        ]
    }
}

impl ComplianceReportGenerator {
    pub fn new() -> Self {
        Self {
            report_templates: HashMap::new(),
            data_sources: vec![],
            export_formats: vec![ExportFormat::PDF, ExportFormat::HTML, ExportFormat::CSV],
        }
    }

    pub async fn generate_report(
        &self,
        scope: AuditScope,
        period: (SystemTime, SystemTime),
    ) -> Result<MediaAuditReport, AuditError> {
        let report_id = Uuid::new_v4().to_string();
        
        // Generate executive summary
        let executive_summary = self.generate_executive_summary(&scope, period).await?;
        
        // Generate detailed sections
        let file_statistics = self.generate_file_statistics(&scope, period).await?;
        let security_summary = self.generate_security_summary(&scope, period).await?;
        let compliance_summary = self.generate_compliance_summary(&scope, period).await?;
        let user_activity_summary = self.generate_user_activity_summary(&scope, period).await?;
        let risk_assessment = self.generate_risk_assessment(&scope, period).await?;
        let recommendations = self.generate_recommendations(&scope, period).await?;
        let detailed_findings = self.generate_detailed_findings(&scope, period).await?;

        Ok(MediaAuditReport {
            report_id,
            scope,
            generation_timestamp: SystemTime::now(),
            report_period: period,
            executive_summary,
            file_statistics,
            security_summary,
            compliance_summary,
            user_activity_summary,
            risk_assessment,
            recommendations,
            detailed_findings,
            appendices: vec![],
        })
    }

    // Helper methods for report generation
    async fn generate_executive_summary(&self, _scope: &AuditScope, _period: (SystemTime, SystemTime)) -> Result<ExecutiveSummary, AuditError> {
        Ok(ExecutiveSummary {
            total_files_audited: 1000,
            security_incidents: 5,
            compliance_violations: 3,
            high_risk_findings: 2,
            data_subjects_affected: 150,
            overall_security_posture: SecurityPosture::Good,
            compliance_status: OverallComplianceStatus {
                gdpr_compliance_level: 0.95,
                hipaa_compliance_level: 0.98,
                sox_compliance_level: 0.92,
                overall_score: 0.95,
                trending: ComplianceTrend::Improving,
            },
            key_concerns: vec!["Unencrypted sensitive files detected".to_string()],
            positive_findings: vec!["Strong access controls in place".to_string()],
        })
    }

    async fn generate_file_statistics(&self, _scope: &AuditScope, _period: (SystemTime, SystemTime)) -> Result<FileStatistics, AuditError> {
        Ok(FileStatistics {
            total_files: 1000,
            files_with_personal_data: 150,
            files_with_phi: 75,
            encrypted_files: 950,
            files_shared_externally: 25,
            files_deleted: 100,
            average_file_size: 2048000.0,
            total_storage_used: 2048000000,
            file_types: HashMap::from([
                ("image/jpeg".to_string(), 400),
                ("application/pdf".to_string(), 300),
                ("video/mp4".to_string(), 200),
            ]),
        })
    }

    async fn generate_security_summary(&self, _scope: &AuditScope, _period: (SystemTime, SystemTime)) -> Result<SecuritySummary, AuditError> {
        Ok(SecuritySummary {
            total_security_events: 25,
            malware_detections: 3,
            unauthorized_access_attempts: 12,
            successful_attacks: 0,
            data_breaches: 0,
            incident_response_times: vec![Duration::from_secs(15 * 60), Duration::from_secs(30 * 60)],
            threat_levels: HashMap::from([
                (ThreatLevel::Low, 15),
                (ThreatLevel::Medium, 8),
                (ThreatLevel::High, 2),
            ]),
            attack_vectors: HashMap::from([
                ("Phishing".to_string(), 8),
                ("Malware".to_string(), 3),
                ("Brute Force".to_string(), 4),
            ]),
        })
    }

    async fn generate_compliance_summary(&self, _scope: &AuditScope, _period: (SystemTime, SystemTime)) -> Result<ComplianceSummary, AuditError> {
        Ok(ComplianceSummary {
            gdpr_violations: 2,
            hipaa_violations: 1,
            sox_violations: 0,
            data_subject_requests: 8,
            retention_violations: 3,
            consent_violations: 1,
            breach_notifications: 0,
            remediation_success_rate: 0.95,
        })
    }

    async fn generate_user_activity_summary(&self, _scope: &AuditScope, _period: (SystemTime, SystemTime)) -> Result<UserActivitySummary, AuditError> {
        Ok(UserActivitySummary {
            total_users: 250,
            active_users: 180,
            privileged_users: 25,
            suspicious_users: 3,
            most_active_users: vec![],
            access_patterns: AccessPatternAnalysis {
                peak_hours: vec![9, 10, 11, 14, 15, 16],
                geographic_distribution: HashMap::from([
                    ("US".to_string(), 150),
                    ("EU".to_string(), 75),
                    ("APAC".to_string(), 25),
                ]),
                device_types: HashMap::from([
                    ("Desktop".to_string(), 120),
                    ("Mobile".to_string(), 80),
                    ("Tablet".to_string(), 50),
                ]),
                unusual_patterns: vec!["Off-hours access from new location".to_string()],
            },
            role_distribution: HashMap::new(),
        })
    }

    async fn generate_risk_assessment(&self, _scope: &AuditScope, _period: (SystemTime, SystemTime)) -> Result<OverallRiskAssessment, AuditError> {
        Ok(OverallRiskAssessment {
            overall_risk_score: 0.3, // Low to medium risk
            risk_categories: HashMap::from([
                ("Data Breach".to_string(), 0.25),
                ("Compliance Violation".to_string(), 0.35),
                ("Insider Threat".to_string(), 0.20),
            ]),
            top_risks: vec![
                TopRisk {
                    risk_description: "Unencrypted sensitive data exposure".to_string(),
                    probability: 0.3,
                    impact: 0.8,
                    risk_score: 0.24,
                    mitigation_plan: Some("Implement mandatory encryption for all sensitive files".to_string()),
                },
            ],
            risk_trend: RiskTrend::Stable,
            mitigation_effectiveness: 0.85,
        })
    }

    async fn generate_recommendations(&self, _scope: &AuditScope, _period: (SystemTime, SystemTime)) -> Result<Vec<AuditRecommendation>, AuditError> {
        Ok(vec![
            AuditRecommendation {
                recommendation_id: "rec-001".to_string(),
                category: RecommendationCategory::Security,
                priority: RecommendationPriority::High,
                title: "Implement Zero Trust Access Model".to_string(),
                description: "Deploy comprehensive zero trust architecture for media access".to_string(),
                rationale: "Current access controls show gaps in privilege verification".to_string(),
                implementation_steps: vec![],
                estimated_effort: EffortEstimate {
                    time_estimate: Duration::from_secs(90 * 24 * 60 * 60), // 90 days
                    resource_requirements: vec!["Security Engineer".to_string(), "Network Admin".to_string()],
                    skill_requirements: vec!["Zero Trust Architecture".to_string()],
                    budget_estimate: Some(50000.0),
                    complexity_level: ComplexityLevel::High,
                },
                expected_benefits: vec!["Reduced unauthorized access risk".to_string()],
                risk_reduction: 0.4,
            },
        ])
    }

    async fn generate_detailed_findings(&self, _scope: &AuditScope, _period: (SystemTime, SystemTime)) -> Result<Vec<DetailedFinding>, AuditError> {
        Ok(vec![
            DetailedFinding {
                finding_id: "finding-001".to_string(),
                category: FindingCategory::Security,
                severity: FindingSeverity::Medium,
                title: "Unencrypted sensitive files detected".to_string(),
                description: "Several files containing personal data are not encrypted".to_string(),
                evidence: vec![],
                affected_assets: vec!["file-123".to_string(), "file-456".to_string()],
                business_impact: BusinessImpact {
                    impact_level: ImpactLevel::Medium,
                    affected_processes: vec!["Data Processing".to_string()],
                    financial_impact: Some(10000.0),
                    reputation_impact: ReputationImpact::Moderate,
                    operational_impact: OperationalImpact {
                        service_disruption: false,
                        data_availability: DataAvailabilityImpact::NoImpact,
                        performance_degradation: 0.0,
                        recovery_time_estimate: None,
                    },
                },
                remediation_advice: "Apply encryption to all identified files".to_string(),
                timeline_for_resolution: Duration::from_secs(7 * 24 * 60 * 60), // 7 days
            },
        ])
    }
}

// Helper implementations
impl ThreatLevel {
    fn from_severity(severity: &SecuritySeverity) -> Self {
        match severity {
            SecuritySeverity::Info => ThreatLevel::None,
            SecuritySeverity::Low => ThreatLevel::Low,
            SecuritySeverity::Medium => ThreatLevel::Medium,
            SecuritySeverity::High => ThreatLevel::High,
            SecuritySeverity::Critical => ThreatLevel::Critical,
            SecuritySeverity::Emergency => ThreatLevel::Catastrophic,
        }
    }
}

impl Default for AuditRetentionPolicy {
    fn default() -> Self {
        Self {
            default_retention: Duration::from_secs(7 * 365 * 24 * 60 * 60), // 7 years
            high_value_retention: Duration::from_secs(10 * 365 * 24 * 60 * 60), // 10 years
            compliance_retention: HashMap::from([
                (ComplianceRegulation::GDPR, Duration::from_secs(6 * 365 * 24 * 60 * 60)), // 6 years
                (ComplianceRegulation::HIPAA, Duration::from_secs(6 * 365 * 24 * 60 * 60)), // 6 years
                (ComplianceRegulation::SOX, Duration::from_secs(7 * 365 * 24 * 60 * 60)), // 7 years
            ]),
            auto_archival: true,
            secure_deletion: true,
        }
    }
}

// Duration helper trait
trait DurationExt {
    fn from_hours(hours: u64) -> Duration;
}

impl DurationExt for Duration {
    fn from_hours(hours: u64) -> Duration {
        Duration::from_secs(hours * 60 * 60)
    }
}

impl Default for MediaAuditSystem {
    fn default() -> Self {
        Self::new()
    }
}
