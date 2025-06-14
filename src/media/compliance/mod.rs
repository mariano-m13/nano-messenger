pub mod gdpr;
pub mod hipaa;
pub mod auditing;

// Re-export key types and traits for easier access
pub use gdpr::{
    MediaGDPRCompliance, PersonalDataClassification, PersonalDataCategory, 
    DataSensitivityLevel, DataSubjectRequest, MediaErasureRequest, ErasureConfirmation,
    GDPRComplianceReport, LegalBasis, RetentionRequirements, PersonalDataProcessor
};

pub use hipaa::{
    MediaHIPAACompliance, PHIDetectionResult, PHIType, EncryptedPHIMedia,
    HIPAAAccessReport, UserRole, AccessPurpose, RedactionArea, RedactionType,
    ComplianceAttestation, PHIDetector, HIPAAEncryptionEnforcer
};

pub use auditing::{
    MediaAuditSystem, MediaAuditEvent, MediaAuditReport, TamperEvidentLogEntry,
    ComplianceViolationStream, AuditScope, SecuritySummary, ComplianceSummary,
    ExecutiveSummary, AuditRecommendation, ComplianceStatus, SecurityEventType
};

use crate::media::security::scanning::{FileId, MediaFile};
use crate::username::UserId;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use thiserror::Error;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

/// Unified compliance error type
#[derive(Debug, Error)]
pub enum ComplianceError {
    #[error("GDPR compliance failed: {0}")]
    GDPRFailed(#[from] gdpr::GDPRError),
    
    #[error("HIPAA compliance failed: {0}")]
    HIPAAFailed(#[from] hipaa::HIPAAError),
    
    #[error("Audit system failed: {0}")]
    AuditFailed(#[from] auditing::AuditError),
    
    #[error("Compliance policy violation: {0}")]
    PolicyViolation(String),
    
    #[error("Multi-regulation conflict: {0}")]
    RegulationConflict(String),
    
    #[error("Compliance configuration error: {0}")]
    ConfigurationError(String),
}

/// Compliance framework configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceConfig {
    pub gdpr_enabled: bool,
    pub hipaa_enabled: bool,
    pub sox_enabled: bool,
    pub audit_enabled: bool,
    pub real_time_monitoring: bool,
    pub auto_remediation: bool,
    pub data_retention_policies: HashMap<DataCategory, Duration>,
    pub breach_notification_threshold: Duration,
    pub compliance_officer: Option<UserId>,
}

/// Data categories for compliance classification
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataCategory {
    PersonalData,
    ProtectedHealthInformation,
    FinancialData,
    BiometricData,
    LocationData,
    CommunicationData,
    SystemData,
    OperationalData,
}

/// Unified compliance assessment result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceAssessment {
    pub file_id: FileId,
    pub assessment_timestamp: SystemTime,
    pub data_categories: Vec<DataCategory>,
    pub gdpr_assessment: Option<gdpr::PersonalDataClassification>,
    pub hipaa_assessment: Option<hipaa::PHIDetectionResult>,
    pub compliance_requirements: Vec<ComplianceRequirement>,
    pub violations: Vec<ComplianceViolation>,
    pub remediation_actions: Vec<RemediationAction>,
    pub overall_compliance_score: f32,
    pub risk_level: ComplianceRiskLevel,
}

/// Compliance requirements for media
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRequirement {
    pub regulation: Regulation,
    pub requirement_type: RequirementType,
    pub description: String,
    pub mandatory: bool,
    pub deadline: Option<SystemTime>,
    pub responsible_party: Option<UserId>,
}

/// Supported regulations
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Regulation {
    GDPR,
    HIPAA,
    SOX,
    CCPA,
    PIPEDA,
    LGPD,
    Custom(String),
}

/// Types of compliance requirements
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RequirementType {
    DataProtection,
    AccessControl,
    Encryption,
    AuditLogging,
    RetentionPolicy,
    ConsentManagement,
    BreachNotification,
    DataPortability,
    RightToErasure,
    DataMinimization,
}

/// Compliance violations detected
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceViolation {
    pub violation_id: String,
    pub regulation: Regulation,
    pub violation_type: ViolationType,
    pub severity: ViolationSeverity,
    pub description: String,
    pub detected_at: SystemTime,
    pub affected_data: DataAffected,
    pub potential_impact: ImpactAssessment,
    pub auto_remediation_possible: bool,
}

/// Types of compliance violations
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ViolationType {
    UnauthorizedProcessing,
    InsufficientConsent,
    DataRetentionViolation,
    EncryptionMissing,
    AccessControlBreach,
    AuditTrailMissing,
    BreachNotificationDelay,
    DataSubjectRightsViolation,
    CrossBorderTransferViolation,
    DataMinimizationViolation,
}

/// Severity levels for violations
#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Serialize, Deserialize)]
pub enum ViolationSeverity {
    Low,
    Medium,
    High,
    Critical,
    Catastrophic,
}

/// Data affected by violations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataAffected {
    pub file_ids: Vec<FileId>,
    pub data_subjects: u64,
    pub data_categories: Vec<DataCategory>,
    pub geographic_scope: Vec<String>,
    pub time_range: (SystemTime, SystemTime),
}

/// Impact assessment for violations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAssessment {
    pub privacy_impact: PrivacyImpact,
    pub financial_impact: FinancialImpact,
    pub reputational_impact: ReputationalImpact,
    pub operational_impact: OperationalImpact,
    pub regulatory_impact: RegulatoryImpact,
}

/// Privacy impact levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrivacyImpact {
    None,
    Minimal,
    Moderate,
    Significant,
    Severe,
}

/// Financial impact assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialImpact {
    pub estimated_fines: Option<f64>,
    pub remediation_costs: Option<f64>,
    pub business_disruption_costs: Option<f64>,
    pub legal_costs: Option<f64>,
    pub total_estimated_cost: f64,
}

/// Reputational impact levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReputationalImpact {
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
    pub data_availability_impact: bool,
    pub process_changes_required: bool,
    pub staff_training_required: bool,
    pub system_changes_required: bool,
}

/// Regulatory impact assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryImpact {
    pub investigation_likely: bool,
    pub enforcement_action_risk: f32,
    pub license_implications: bool,
    pub reporting_requirements: Vec<String>,
    pub certification_impact: bool,
}

/// Remediation actions for compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationAction {
    pub action_id: String,
    pub action_type: ActionType,
    pub description: String,
    pub priority: ActionPriority,
    pub estimated_effort: Duration,
    pub responsible_party: Option<UserId>,
    pub deadline: SystemTime,
    pub automated: bool,
    pub dependencies: Vec<String>,
}

/// Types of remediation actions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionType {
    DataEncryption,
    AccessRestriction,
    DataDeletion,
    ConsentObtaining,
    PolicyUpdate,
    SystemConfiguration,
    UserTraining,
    AuditImplementation,
    NotificationSending,
    DataAnonymization,
}

/// Priority levels for actions
#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub enum ActionPriority {
    Low,
    Medium,
    High,
    Critical,
    Immediate,
}

/// Compliance risk levels
#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Serialize, Deserialize)]
pub enum ComplianceRiskLevel {
    Low,
    Medium,
    High,
    Critical,
    Extreme,
}

/// Unified compliance manager
pub struct MediaComplianceManager {
    pub gdpr_compliance: MediaGDPRCompliance,
    pub hipaa_compliance: MediaHIPAACompliance,
    pub audit_system: MediaAuditSystem,
    pub config: ComplianceConfig,
    pub active_violations: Vec<ComplianceViolation>,
    pub remediation_queue: Vec<RemediationAction>,
}

/// Multi-regulation compliance result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiRegulationResult {
    pub overall_compliant: bool,
    pub regulation_results: HashMap<Regulation, RegulationResult>,
    pub conflicts: Vec<RegulationConflict>,
    pub unified_requirements: Vec<ComplianceRequirement>,
    pub priority_actions: Vec<RemediationAction>,
}

/// Individual regulation compliance result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulationResult {
    pub regulation: Regulation,
    pub compliant: bool,
    pub compliance_score: f32,
    pub violations: Vec<ComplianceViolation>,
    pub requirements_met: u32,
    pub requirements_total: u32,
}

/// Conflicts between regulations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulationConflict {
    pub conflict_id: String,
    pub regulations: Vec<Regulation>,
    pub conflict_type: ConflictType,
    pub description: String,
    pub resolution_strategy: ConflictResolution,
}

/// Types of regulation conflicts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictType {
    RetentionPeriod,
    DataTransfer,
    ConsentRequirements,
    AccessRights,
    DeletionRights,
    EncryptionStandards,
    AuditRequirements,
}

/// Conflict resolution strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictResolution {
    MostRestrictive,
    RegulationPriority(Regulation),
    ManualReview,
    ComplianceOfficerDecision,
    LegalAdvice,
}

impl MediaComplianceManager {
    /// Create new compliance manager
    pub fn new(config: ComplianceConfig) -> Self {
        Self {
            gdpr_compliance: MediaGDPRCompliance::new(),
            hipaa_compliance: MediaHIPAACompliance::new(),
            audit_system: MediaAuditSystem::new(),
            config,
            active_violations: Vec::new(),
            remediation_queue: Vec::new(),
        }
    }

    /// Perform comprehensive compliance assessment
    pub async fn assess_compliance(
        &mut self,
        media: &MediaFile,
        context: &ComplianceContext,
    ) -> Result<ComplianceAssessment, ComplianceError> {
        let mut assessment = ComplianceAssessment {
            file_id: media.file_id.clone(),
            assessment_timestamp: SystemTime::now(),
            data_categories: vec![],
            gdpr_assessment: None,
            hipaa_assessment: None,
            compliance_requirements: vec![],
            violations: vec![],
            remediation_actions: vec![],
            overall_compliance_score: 1.0,
            risk_level: ComplianceRiskLevel::Low,
        };

        // GDPR assessment if enabled
        if self.config.gdpr_enabled {
            match self.gdpr_compliance.classify_personal_data(media).await {
                Ok(gdpr_result) => {
                    if gdpr_result.contains_personal_data {
                        assessment.data_categories.extend(
                            gdpr_result.data_categories.iter()
                                .map(|cat| self.map_gdpr_to_data_category(cat))
                        );
                        assessment.gdpr_assessment = Some(gdpr_result);
                    }
                }
                Err(e) => {
                    eprintln!("Warning: GDPR assessment failed: {}", e);
                    // Continue with assessment even if GDPR check fails
                }
            }
        }

        // HIPAA assessment if enabled
        if self.config.hipaa_enabled {
            match self.hipaa_compliance.detect_phi_in_media(media).await {
                Ok(hipaa_result) => {
                    if hipaa_result.contains_phi {
                        assessment.data_categories.extend(
                            hipaa_result.phi_types.iter()
                                .map(|phi| self.map_phi_to_data_category(phi))
                        );
                        assessment.hipaa_assessment = Some(hipaa_result);
                    }
                }
                Err(e) => {
                    eprintln!("Warning: HIPAA assessment failed: {}", e);
                    // Continue with assessment even if HIPAA check fails
                }
            }
        }

        // Generate compliance requirements
        assessment.compliance_requirements = self.generate_compliance_requirements(&assessment, context)?;

        // Detect violations
        assessment.violations = self.detect_violations(&assessment, context).await?;

        // Generate remediation actions
        assessment.remediation_actions = self.generate_remediation_actions(&assessment.violations)?;

        // Calculate overall compliance score
        assessment.overall_compliance_score = self.calculate_compliance_score(&assessment)?;
        assessment.risk_level = self.assess_risk_level(&assessment)?;

        // Log assessment as system event
        if let Err(e) = self.audit_system.log_media_event(
            auditing::MediaAuditEvent::SystemEvent {
                event_type: auditing::SystemEventType::ConfigurationChange,
                component: "ComplianceAssessment".to_string(),
                impact_level: if assessment.violations.is_empty() { 
                    auditing::ImpactLevel::Minimal 
                } else { 
                    auditing::ImpactLevel::Medium 
                },
                recovery_time: None,
                data_integrity_verified: true,
            }
        ).await {
            // Non-fatal error - log warning but continue
            eprintln!("Warning: Failed to log compliance assessment: {}", e);
        }

        Ok(assessment)
    }

    /// Apply multi-regulation compliance check
    pub async fn check_multi_regulation_compliance(
        &mut self,
        media: &MediaFile,
        context: &ComplianceContext,
    ) -> Result<MultiRegulationResult, ComplianceError> {
        let mut regulation_results = HashMap::new();
        let mut conflicts = vec![];
        let mut all_requirements = vec![];

        // Check each enabled regulation
        if self.config.gdpr_enabled {
            let gdpr_result = self.check_gdpr_compliance(media, context).await?;
            regulation_results.insert(Regulation::GDPR, gdpr_result);
        }

        if self.config.hipaa_enabled {
            let hipaa_result = self.check_hipaa_compliance(media, context).await?;
            regulation_results.insert(Regulation::HIPAA, hipaa_result);
        }

        // Detect conflicts between regulations
        conflicts = self.detect_regulation_conflicts(&regulation_results)?;

        // Generate unified requirements
        all_requirements = self.unify_requirements(&regulation_results, &conflicts)?;

        // Determine overall compliance
        let overall_compliant = regulation_results.values().all(|result| result.compliant);

        // Generate priority actions
        let priority_actions = self.prioritize_remediation_actions(&regulation_results)?;

        Ok(MultiRegulationResult {
            overall_compliant,
            regulation_results,
            conflicts,
            unified_requirements: all_requirements,
            priority_actions,
        })
    }

    /// Execute automatic remediation where possible
    pub async fn execute_auto_remediation(
        &mut self,
        violations: &[ComplianceViolation],
    ) -> Result<RemediationResult, ComplianceError> {
        let mut remediated = vec![];
        let mut failed = vec![];
        let mut manual_required = vec![];

        for violation in violations {
            if !violation.auto_remediation_possible || !self.config.auto_remediation {
                manual_required.push(violation.clone());
                continue;
            }

            match self.execute_remediation_action(violation).await {
                Ok(_) => remediated.push(violation.clone()),
                Err(e) => {
                    failed.push((violation.clone(), e.to_string()));
                }
            }
        }

        Ok(RemediationResult {
            total_violations: violations.len(),
            auto_remediated: remediated.len(),
            failed_remediations: failed.len(),
            manual_required: manual_required.len(),
            remediated_violations: remediated,
            failed_violations: failed,
            manual_violations: manual_required,
        })
    }

    /// Generate compliance dashboard data
    pub async fn generate_compliance_dashboard(
        &self,
        time_period: (SystemTime, SystemTime),
    ) -> Result<ComplianceDashboard, ComplianceError> {
        // Generate comprehensive audit report
        let audit_report = self.audit_system.generate_media_audit_report(
            auditing::AuditScope::AllFiles,
            time_period,
        ).await?;

        // Get current compliance status
        let compliance_status = self.audit_system.check_compliance_status().await?;

        // Get recent violations
        let recent_violations = self.get_recent_violations(time_period).await?;

        // Calculate compliance trends
        let compliance_trends = self.calculate_compliance_trends(time_period).await?;

        Ok(ComplianceDashboard {
            period: time_period,
            overall_status: compliance_status,
            audit_summary: audit_report.executive_summary,
            recent_violations,
            compliance_trends,
            risk_indicators: self.get_risk_indicators().await?,
            action_items: self.get_priority_action_items().await?,
            regulatory_updates: vec![], // Would fetch from external sources
        })
    }

    // Helper methods
    fn map_gdpr_to_data_category(&self, gdpr_category: &PersonalDataCategory) -> DataCategory {
        // Safe mapping that handles all possible enum variants
        match gdpr_category {
            // Map biometric-related categories
            cat if format!("{:?}", cat).contains("Biometric") || 
                 format!("{:?}", cat).contains("Facial") => DataCategory::BiometricData,
            // Map location data
            cat if format!("{:?}", cat).contains("Location") => DataCategory::LocationData,
            // Map health information
            cat if format!("{:?}", cat).contains("Health") || 
                 format!("{:?}", cat).contains("Medical") => DataCategory::ProtectedHealthInformation,
            // Default to personal data
            _ => DataCategory::PersonalData,
        }
    }

    fn map_phi_to_data_category(&self, phi_type: &PHIType) -> DataCategory {
        // Safe mapping that handles all possible PHI types
        match phi_type {
            // Map financial/billing information
            phi if format!("{:?}", phi).contains("Billing") || 
                  format!("{:?}", phi).contains("Insurance") || 
                  format!("{:?}", phi).contains("Financial") => DataCategory::FinancialData,
            // All other PHI types map to protected health information
            _ => DataCategory::ProtectedHealthInformation,
        }
    }

    async fn check_gdpr_compliance(&self, _media: &MediaFile, _context: &ComplianceContext) -> Result<RegulationResult, ComplianceError> {
        // Placeholder implementation
        Ok(RegulationResult {
            regulation: Regulation::GDPR,
            compliant: true,
            compliance_score: 0.95,
            violations: vec![],
            requirements_met: 18,
            requirements_total: 20,
        })
    }

    async fn check_hipaa_compliance(&self, _media: &MediaFile, _context: &ComplianceContext) -> Result<RegulationResult, ComplianceError> {
        // Placeholder implementation
        Ok(RegulationResult {
            regulation: Regulation::HIPAA,
            compliant: true,
            compliance_score: 0.98,
            violations: vec![],
            requirements_met: 15,
            requirements_total: 16,
        })
    }

    fn generate_compliance_requirements(&self, _assessment: &ComplianceAssessment, _context: &ComplianceContext) -> Result<Vec<ComplianceRequirement>, ComplianceError> {
        // Placeholder implementation
        Ok(vec![])
    }

    async fn detect_violations(&self, _assessment: &ComplianceAssessment, _context: &ComplianceContext) -> Result<Vec<ComplianceViolation>, ComplianceError> {
        // Placeholder implementation
        Ok(vec![])
    }

    fn generate_remediation_actions(&self, _violations: &[ComplianceViolation]) -> Result<Vec<RemediationAction>, ComplianceError> {
        // Placeholder implementation
        Ok(vec![])
    }

    fn calculate_compliance_score(&self, assessment: &ComplianceAssessment) -> Result<f32, ComplianceError> {
        if assessment.violations.is_empty() {
            Ok(1.0)
        } else {
            let violation_impact: f32 = assessment.violations.iter()
                .map(|v| match v.severity {
                    ViolationSeverity::Low => 0.05,
                    ViolationSeverity::Medium => 0.15,
                    ViolationSeverity::High => 0.3,
                    ViolationSeverity::Critical => 0.5,
                    ViolationSeverity::Catastrophic => 0.8,
                })
                .sum();
            Ok((1.0 - violation_impact).max(0.0))
        }
    }

    fn assess_risk_level(&self, assessment: &ComplianceAssessment) -> Result<ComplianceRiskLevel, ComplianceError> {
        let score = assessment.overall_compliance_score;
        Ok(match score {
            s if s >= 0.9 => ComplianceRiskLevel::Low,
            s if s >= 0.7 => ComplianceRiskLevel::Medium,
            s if s >= 0.5 => ComplianceRiskLevel::High,
            s if s >= 0.3 => ComplianceRiskLevel::Critical,
            _ => ComplianceRiskLevel::Extreme,
        })
    }

    fn detect_regulation_conflicts(&self, _results: &HashMap<Regulation, RegulationResult>) -> Result<Vec<RegulationConflict>, ComplianceError> {
        // Placeholder implementation
        Ok(vec![])
    }

    fn unify_requirements(&self, _results: &HashMap<Regulation, RegulationResult>, _conflicts: &[RegulationConflict]) -> Result<Vec<ComplianceRequirement>, ComplianceError> {
        // Placeholder implementation
        Ok(vec![])
    }

    fn prioritize_remediation_actions(&self, _results: &HashMap<Regulation, RegulationResult>) -> Result<Vec<RemediationAction>, ComplianceError> {
        // Placeholder implementation
        Ok(vec![])
    }

    async fn execute_remediation_action(&self, _violation: &ComplianceViolation) -> Result<(), ComplianceError> {
        // Placeholder implementation
        Ok(())
    }

    async fn get_recent_violations(&self, _period: (SystemTime, SystemTime)) -> Result<Vec<ComplianceViolation>, ComplianceError> {
        // Placeholder implementation
        Ok(vec![])
    }

    async fn calculate_compliance_trends(&self, _period: (SystemTime, SystemTime)) -> Result<ComplianceTrends, ComplianceError> {
        // Placeholder implementation
        Ok(ComplianceTrends {
            gdpr_trend: TrendDirection::Stable,
            hipaa_trend: TrendDirection::Improving,
            overall_trend: TrendDirection::Improving,
            violation_trend: TrendDirection::Declining,
        })
    }

    async fn get_risk_indicators(&self) -> Result<Vec<RiskIndicator>, ComplianceError> {
        // Placeholder implementation
        Ok(vec![])
    }

    async fn get_priority_action_items(&self) -> Result<Vec<ActionItem>, ComplianceError> {
        // Placeholder implementation
        Ok(vec![])
    }

    /// Get list of currently enabled regulations
    fn get_enabled_regulations(&self) -> Vec<String> {
        let mut regulations = Vec::new();
        if self.config.gdpr_enabled {
            regulations.push("GDPR".to_string());
        }
        if self.config.hipaa_enabled {
            regulations.push("HIPAA".to_string());
        }
        if self.config.sox_enabled {
            regulations.push("SOX".to_string());
        }
        regulations
    }
}

/// Context for compliance assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceContext {
    pub user_id: UserId,
    pub organization_id: String,
    pub jurisdiction: String,
    pub business_context: BusinessContext,
    pub data_processing_purpose: ProcessingPurpose,
    pub consent_status: ConsentStatus,
}

/// Business context for compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessContext {
    pub industry: String,
    pub organization_size: OrganizationSize,
    pub is_healthcare_entity: bool,
    pub is_financial_institution: bool,
    pub international_operations: bool,
}

/// Organization size categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrganizationSize {
    Small,      // <250 employees
    Medium,     // 250-1000 employees  
    Large,      // >1000 employees
    Enterprise, // >10000 employees
}

/// Data processing purposes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessingPurpose {
    Communication,
    Storage,
    Sharing,
    Analysis,
    Backup,
    Archive,
    Marketing,
    Research,
    LegalCompliance,
}

/// Consent status for processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentStatus {
    pub has_consent: bool,
    pub consent_date: Option<SystemTime>,
    pub consent_scope: Vec<ProcessingPurpose>,
    pub withdrawal_possible: bool,
}

/// Remediation execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationResult {
    pub total_violations: usize,
    pub auto_remediated: usize,
    pub failed_remediations: usize,
    pub manual_required: usize,
    pub remediated_violations: Vec<ComplianceViolation>,
    pub failed_violations: Vec<(ComplianceViolation, String)>,
    pub manual_violations: Vec<ComplianceViolation>,
}

/// Compliance dashboard data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceDashboard {
    pub period: (SystemTime, SystemTime),
    pub overall_status: ComplianceStatus,
    pub audit_summary: ExecutiveSummary,
    pub recent_violations: Vec<ComplianceViolation>,
    pub compliance_trends: ComplianceTrends,
    pub risk_indicators: Vec<RiskIndicator>,
    pub action_items: Vec<ActionItem>,
    pub regulatory_updates: Vec<RegulatoryUpdate>,
}

/// Compliance trends over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceTrends {
    pub gdpr_trend: TrendDirection,
    pub hipaa_trend: TrendDirection,
    pub overall_trend: TrendDirection,
    pub violation_trend: TrendDirection,
}

/// Trend direction indicators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Improving,
    Stable,
    Declining,
    Volatile,
}

/// Risk indicators for compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskIndicator {
    pub indicator_type: RiskIndicatorType,
    pub current_value: f32,
    pub threshold: f32,
    pub trend: TrendDirection,
    pub alert_level: AlertLevel,
}

/// Types of risk indicators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskIndicatorType {
    ViolationRate,
    DataExposure,
    AccessAnomalies,
    EncryptionGaps,
    ConsentExpiry,
    RetentionOverage,
}

/// Alert levels for risk indicators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertLevel {
    Green,
    Yellow,
    Orange,
    Red,
    Critical,
}

/// Action items for compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionItem {
    pub item_id: String,
    pub title: String,
    pub description: String,
    pub priority: ActionPriority,
    pub due_date: SystemTime,
    pub assigned_to: Option<UserId>,
    pub regulation: Regulation,
    pub status: ActionStatus,
}

/// Status of action items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionStatus {
    Open,
    InProgress,
    Completed,
    Overdue,
    Cancelled,
}

/// Regulatory updates and changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryUpdate {
    pub update_id: String,
    pub regulation: Regulation,
    pub title: String,
    pub description: String,
    pub effective_date: SystemTime,
    pub impact_level: ImpactLevel,
    pub action_required: bool,
}

/// Impact levels for regulatory updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactLevel {
    None,
    Low,
    Medium,
    High,
    Critical,
}

impl Default for ComplianceConfig {
    fn default() -> Self {
        Self {
            gdpr_enabled: true,
            hipaa_enabled: false,
            sox_enabled: false,
            audit_enabled: true,
            real_time_monitoring: true,
            auto_remediation: false,
            data_retention_policies: HashMap::new(),
            breach_notification_threshold: Duration::from_secs(72 * 60 * 60), // 72 hours
            compliance_officer: None,
        }
    }
}
