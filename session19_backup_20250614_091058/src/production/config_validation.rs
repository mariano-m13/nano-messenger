//! Configuration Validation for Production - Session 8
//! 
//! This module provides comprehensive configuration validation for production
//! deployments, ensuring all settings are valid, secure, and compliant with
//! organizational policies before the system starts.


use serde::{Deserialize, Serialize};
// use std::collections::HashMap; // Unused
use std::fmt;
use std::net::IpAddr;
use std::path::PathBuf;
// use std::time::Duration; // Unused

/// Production configuration with comprehensive validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionConfig {
    pub server: ServerConfig,
    pub security: SecurityConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
    pub monitoring: MonitoringConfig,
    pub compliance: ComplianceConfig,
    pub performance: PerformanceConfig,
    pub backup: BackupConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub bind_address: IpAddr,
    pub api_port: u16,
    pub websocket_port: u16,
    pub worker_threads: Option<usize>,
    pub max_connections: u32,
    pub connection_timeout_seconds: u64,
    pub keep_alive_seconds: u64,
    pub tls_cert_path: PathBuf,
    pub tls_key_path: PathBuf,
    pub server_name: String,
    pub environment: Environment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub minimum_crypto_mode: String,
    pub require_post_quantum: bool,
    pub max_message_age_seconds: u64,
    pub rate_limiting: RateLimitConfig,
    pub key_rotation: KeyRotationConfig,
    pub access_control: AccessControlConfig,
    pub threat_detection: ThreatDetectionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub max_messages_per_minute: u32,
    pub max_connections_per_ip: u32,
    pub burst_allowance: u32,
    pub ban_duration_minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyRotationConfig {
    pub automatic_rotation: bool,
    pub rotation_interval_days: u32,
    pub emergency_rotation_enabled: bool,
    pub pre_rotation_notification_hours: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlConfig {
    pub require_authentication: bool,
    pub session_timeout_minutes: u32,
    pub max_failed_attempts: u32,
    pub lockout_duration_minutes: u32,
    pub require_mfa: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetectionConfig {
    pub enabled: bool,
    pub anomaly_detection: bool,
    pub ip_reputation_checking: bool,
    pub behavioral_analysis: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub database_type: DatabaseType,
    pub connection_string: String,
    pub max_connections: u32,
    pub connection_timeout_seconds: u64,
    pub query_timeout_seconds: u64,
    pub ssl_required: bool,
    pub backup_enabled: bool,
    pub encryption_at_rest: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: LogLevel,
    pub structured_logging: bool,
    pub audit_log_path: PathBuf,
    pub error_log_path: PathBuf,
    pub access_log_path: PathBuf,
    pub log_rotation: LogRotationConfig,
    pub remote_logging: Option<RemoteLoggingConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRotationConfig {
    pub max_size_mb: u64,
    pub max_files: u32,
    pub rotation_schedule: RotationSchedule,
    pub compression_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteLoggingConfig {
    pub enabled: bool,
    pub endpoint: String,
    pub api_key: String,
    pub buffer_size: u32,
    pub flush_interval_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub metrics_enabled: bool,
    pub metrics_port: u16,
    pub health_check_interval_seconds: u64,
    pub alert_thresholds: AlertThresholds,
    pub external_monitoring: Option<ExternalMonitoringConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    pub cpu_usage_percent: f64,
    pub memory_usage_percent: f64,
    pub disk_usage_percent: f64,
    pub error_rate_percent: f64,
    pub response_time_ms: f64, // Fixed: Changed from u64 to f64 for consistency
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalMonitoringConfig {
    pub prometheus_enabled: bool,
    pub prometheus_endpoint: String,
    pub grafana_enabled: bool,
    pub grafana_endpoint: String,
    pub custom_metrics: Vec<CustomMetric>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomMetric {
    pub name: String,
    pub description: String,
    pub metric_type: MetricType,
    pub labels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceConfig {
    pub gdpr_enabled: bool,
    pub hipaa_enabled: bool,
    pub sox_enabled: bool,
    pub data_retention_days: u32,
    pub audit_logging_enabled: bool,
    pub data_encryption_required: bool,
    pub breach_notification_enabled: bool,
    pub privacy_controls: PrivacyControlsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyControlsConfig {
    pub data_minimization: bool,
    pub purpose_limitation: bool,
    pub storage_limitation: bool,
    pub accuracy_controls: bool,
    pub security_controls: bool,
    pub accountability_measures: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub enable_caching: bool,
    pub cache_size_mb: u64,
    pub cache_ttl_seconds: u64,
    pub connection_pooling: bool,
    pub async_processing: bool,
    pub batch_processing: BatchProcessingConfig,
    pub resource_limits: ResourceLimitsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchProcessingConfig {
    pub enabled: bool,
    pub batch_size: u32,
    pub batch_timeout_ms: u64,
    pub max_concurrent_batches: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimitsConfig {
    pub max_memory_mb: Option<u64>,
    pub max_cpu_percent: Option<f64>,
    pub max_file_descriptors: Option<u32>,
    pub max_network_connections: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    pub enabled: bool,
    pub backup_interval_hours: u32,
    pub retention_days: u32,
    pub backup_location: PathBuf,
    pub encryption_enabled: bool,
    pub compression_enabled: bool,
    pub remote_backup: Option<RemoteBackupConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteBackupConfig {
    pub provider: BackupProvider,
    pub endpoint: String,
    pub credentials_path: PathBuf,
    pub bucket_name: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Environment {
    Development,
    Testing,
    Staging,
    Production,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DatabaseType {
    SQLite,
    PostgreSQL,
    MySQL,
    Redis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RotationSchedule {
    Daily,
    Weekly,
    Monthly,
    Size,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackupProvider {
    AwsS3,
    AzureBlob,
    GoogleCloud,
    Local,
}

/// Configuration validation errors
#[derive(Debug, Clone)]
pub enum ConfigValidationError {
    InvalidValue {
        field: String,
        value: String,
        expected: String,
        reason: String,
    },
    MissingRequired {
        field: String,
        context: String,
    },
    SecurityViolation {
        field: String,
        violation: String,
        recommendation: String,
    },
    EnvironmentMismatch {
        environment: Environment,
        incompatible_settings: Vec<String>,
    },
    PathError {
        path: PathBuf,
        error_type: PathErrorType,
    },
    NetworkError {
        address: String,
        error_type: NetworkErrorType,
    },
    DependencyError {
        dependency: String,
        required_version: String,
        available_version: Option<String>,
    },
}

#[derive(Debug, Clone)]
pub enum PathErrorType {
    NotFound,
    PermissionDenied,
    NotWritable,
    NotReadable,
}

#[derive(Debug, Clone)]
pub enum NetworkErrorType {
    InvalidAddress,
    PortInUse,
    UnreachableHost,
    DNSResolutionFailure,
}

impl fmt::Display for ConfigValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigValidationError::InvalidValue { field, value, expected, reason } => {
                write!(f, "Invalid value for '{}': '{}' (expected: {}) - {}", field, value, expected, reason)
            }
            ConfigValidationError::MissingRequired { field, context } => {
                write!(f, "Missing required field '{}' in context: {}", field, context)
            }
            ConfigValidationError::SecurityViolation { field, violation, recommendation } => {
                write!(f, "Security violation in '{}': {} - Recommendation: {}", field, violation, recommendation)
            }
            ConfigValidationError::EnvironmentMismatch { environment, incompatible_settings } => {
                write!(f, "Environment '{}' incompatible with settings: {}", 
                    format!("{:?}", environment), incompatible_settings.join(", "))
            }
            ConfigValidationError::PathError { path, error_type } => {
                write!(f, "Path error for '{}': {:?}", path.display(), error_type)
            }
            ConfigValidationError::NetworkError { address, error_type } => {
                write!(f, "Network error for '{}': {:?}", address, error_type)
            }
            ConfigValidationError::DependencyError { dependency, required_version, available_version } => {
                write!(f, "Dependency error: {} requires version {} but {} is available", 
                    dependency, required_version, 
                    available_version.as_ref().unwrap_or(&"none".to_string()))
            }
        }
    }
}

impl std::error::Error for ConfigValidationError {}

/// Configuration validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<ConfigValidationError>,
    pub warnings: Vec<String>,
    pub recommendations: Vec<String>,
    pub security_score: u32,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            recommendations: Vec::new(),
            security_score: 100,
        }
    }
    
    pub fn add_error(&mut self, error: ConfigValidationError) {
        self.valid = false;
        self.errors.push(error);
        self.security_score = self.security_score.saturating_sub(10);
    }
    
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
        self.security_score = self.security_score.saturating_sub(5);
    }
    
    pub fn add_recommendation(&mut self, recommendation: String) {
        self.recommendations.push(recommendation);
    }
}

/// Configuration validator with production-grade validation rules
pub struct ConfigValidator {
    pub environment: Environment,
    pub strict_mode: bool,
    pub security_baseline: SecurityBaseline,
}

#[derive(Debug, Clone)]
pub struct SecurityBaseline {
    pub min_tls_version: String,
    pub required_cipher_suites: Vec<String>,
    pub min_key_size: u32,
    pub require_perfect_forward_secrecy: bool,
    pub max_session_timeout_minutes: u32,
    pub require_audit_logging: bool,
}

impl ConfigValidator {
    pub fn new(environment: Environment) -> Self {
        Self {
            environment: environment.clone(),
            strict_mode: matches!(environment, Environment::Production),
            security_baseline: SecurityBaseline::for_environment(&environment),
        }
    }
    
    pub fn validate(&self, config: &ProductionConfig) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        self.validate_server_config(&config.server, &mut result);
        self.validate_security_config(&config.security, &mut result);
        self.validate_database_config(&config.database, &mut result);
        self.validate_logging_config(&config.logging, &mut result);
        self.validate_monitoring_config(&config.monitoring, &mut result);
        self.validate_compliance_config(&config.compliance, &mut result);
        self.validate_performance_config(&config.performance, &mut result);
        self.validate_backup_config(&config.backup, &mut result);
        
        // Cross-configuration validation
        self.validate_environment_consistency(config, &mut result);
        self.validate_security_consistency(config, &mut result);
        self.validate_resource_allocation(config, &mut result);
        
        result
    }
    
    fn validate_server_config(&self, config: &ServerConfig, result: &mut ValidationResult) {
        // Validate ports
        if config.api_port == config.websocket_port {
            result.add_error(ConfigValidationError::InvalidValue {
                field: "server.websocket_port".to_string(),
                value: config.websocket_port.to_string(),
                expected: "different from api_port".to_string(),
                reason: "API and WebSocket ports must be different".to_string(),
            });
        }
        
        if config.api_port < 1024 && self.environment == Environment::Production {
            result.add_warning("Using privileged port for API - ensure proper permissions".to_string());
        }
        
        // Validate connection limits
        if config.max_connections > 10000 && self.environment == Environment::Production {
            result.add_warning("High connection limit - ensure adequate system resources".to_string());
        }
        
        // Validate timeouts
        if config.connection_timeout_seconds > 300 {
            result.add_warning("Long connection timeout may impact resource usage".to_string());
        }
        
        // Validate TLS configuration
        if !config.tls_cert_path.exists() {
            result.add_error(ConfigValidationError::PathError {
                path: config.tls_cert_path.clone(),
                error_type: PathErrorType::NotFound,
            });
        }
        
        if !config.tls_key_path.exists() {
            result.add_error(ConfigValidationError::PathError {
                path: config.tls_key_path.clone(),
                error_type: PathErrorType::NotFound,
            });
        }
        
        // Environment-specific validation
        if self.environment == Environment::Production {
            if config.bind_address.to_string() == "127.0.0.1" {
                result.add_error(ConfigValidationError::SecurityViolation {
                    field: "server.bind_address".to_string(),
                    violation: "Localhost binding in production".to_string(),
                    recommendation: "Use 0.0.0.0 or specific interface address".to_string(),
                });
            }
        }
    }
    
    fn validate_security_config(&self, config: &SecurityConfig, result: &mut ValidationResult) {
        // Validate crypto mode
        let valid_modes = ["classical", "hybrid", "quantum"];
        if !valid_modes.contains(&config.minimum_crypto_mode.as_str()) {
            result.add_error(ConfigValidationError::InvalidValue {
                field: "security.minimum_crypto_mode".to_string(),
                value: config.minimum_crypto_mode.clone(),
                expected: "classical, hybrid, or quantum".to_string(),
                reason: "Invalid cryptographic mode".to_string(),
            });
        }
        
        // Production should use at least hybrid mode
        if self.environment == Environment::Production && config.minimum_crypto_mode == "classical" {
            result.add_warning("Production environment should use hybrid or quantum crypto mode".to_string());
        }
        
        // Validate rate limiting
        if config.rate_limiting.max_messages_per_minute < 10 {
            result.add_warning("Very low rate limit may impact user experience".to_string());
        }
        
        if config.rate_limiting.max_connections_per_ip > 100 {
            result.add_warning("High connection limit per IP may allow abuse".to_string());
        }
        
        // Validate key rotation
        if config.key_rotation.rotation_interval_days > 365 {
            result.add_warning("Key rotation interval exceeds recommended maximum of 1 year".to_string());
        }
        
        // Validate access control
        if config.access_control.session_timeout_minutes > self.security_baseline.max_session_timeout_minutes {
            result.add_error(ConfigValidationError::SecurityViolation {
                field: "security.access_control.session_timeout_minutes".to_string(),
                violation: "Session timeout exceeds security baseline".to_string(),
                recommendation: format!("Set to {} minutes or less", self.security_baseline.max_session_timeout_minutes),
            });
        }
        
        if self.environment == Environment::Production && !config.access_control.require_mfa {
            result.add_recommendation("Enable multi-factor authentication for production environment".to_string());
        }
    }
    
    fn validate_database_config(&self, config: &DatabaseConfig, result: &mut ValidationResult) {
        // Validate connection limits
        if config.max_connections < 10 {
            result.add_warning("Low database connection limit may impact performance".to_string());
        }
        
        if config.max_connections > 1000 {
            result.add_warning("High database connection limit may exhaust database resources".to_string());
        }
        
        // Validate timeouts
        if config.query_timeout_seconds > 60 {
            result.add_warning("Long query timeout may indicate performance issues".to_string());
        }
        
        // Security validation
        if !config.ssl_required && self.environment == Environment::Production {
            result.add_error(ConfigValidationError::SecurityViolation {
                field: "database.ssl_required".to_string(),
                violation: "SSL not required for database connections in production".to_string(),
                recommendation: "Enable SSL for all database connections".to_string(),
            });
        }
        
        if !config.encryption_at_rest && self.environment == Environment::Production {
            result.add_error(ConfigValidationError::SecurityViolation {
                field: "database.encryption_at_rest".to_string(),
                violation: "Database encryption at rest not enabled in production".to_string(),
                recommendation: "Enable encryption at rest for sensitive data".to_string(),
            });
        }
        
        // Validate connection string (basic check)
        if config.connection_string.contains("password=") && !config.connection_string.contains("sslmode=require") {
            result.add_warning("Database connection string may expose credentials without SSL".to_string());
        }
    }
    
    fn validate_logging_config(&self, config: &LoggingConfig, result: &mut ValidationResult) {
        // Validate log paths
        for (_name, path) in [
            ("audit_log", &config.audit_log_path),
            ("error_log", &config.error_log_path),
            ("access_log", &config.access_log_path),
        ] {
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    result.add_error(ConfigValidationError::PathError {
                        path: parent.to_path_buf(),
                        error_type: PathErrorType::NotFound,
                    });
                }
            }
        }
        
        // Validate log levels
        if matches!(config.level, LogLevel::Trace | LogLevel::Debug) && self.environment == Environment::Production {
            result.add_warning("Debug logging enabled in production may impact performance and expose sensitive information".to_string());
        }
        
        // Validate log rotation
        if config.log_rotation.max_size_mb > 1000 {
            result.add_warning("Large log file size may impact disk I/O performance".to_string());
        }
        
        if config.log_rotation.max_files > 100 {
            result.add_warning("Large number of log files may consume significant disk space".to_string());
        }
        
        // Audit logging requirement
        if self.security_baseline.require_audit_logging && !config.structured_logging {
            result.add_error(ConfigValidationError::SecurityViolation {
                field: "logging.structured_logging".to_string(),
                violation: "Structured logging required for audit compliance".to_string(),
                recommendation: "Enable structured logging for better audit trail".to_string(),
            });
        }
    }
    
    fn validate_monitoring_config(&self, config: &MonitoringConfig, result: &mut ValidationResult) {
        // Production should have monitoring enabled
        if !config.metrics_enabled && self.environment == Environment::Production {
            result.add_error(ConfigValidationError::MissingRequired {
                field: "monitoring.metrics_enabled".to_string(),
                context: "Production environment requires monitoring".to_string(),
            });
        }
        
        // Validate alert thresholds
        if config.alert_thresholds.cpu_usage_percent > 95.0 {
            result.add_warning("CPU usage threshold very high - consider lowering for proactive alerts".to_string());
        }
        
        if config.alert_thresholds.memory_usage_percent > 90.0 {
            result.add_warning("Memory usage threshold high - consider lowering to prevent OOM conditions".to_string());
        }
        
        if config.alert_thresholds.error_rate_percent > 5.0 {
            result.add_warning("Error rate threshold high - consider lowering for better reliability".to_string());
        }
        
        // Validate health check interval
        if config.health_check_interval_seconds > 300 {
            result.add_warning("Long health check interval may delay problem detection".to_string());
        }
    }
    
    fn validate_compliance_config(&self, config: &ComplianceConfig, result: &mut ValidationResult) {
        // Validate data retention
        if config.data_retention_days < 30 {
            result.add_warning("Short data retention period may not meet regulatory requirements".to_string());
        }
        
        if config.data_retention_days > 2555 { // 7 years
            result.add_warning("Long data retention period may increase storage costs and privacy risks".to_string());
        }
        
        // GDPR validation
        if config.gdpr_enabled {
            if !config.audit_logging_enabled {
                result.add_error(ConfigValidationError::SecurityViolation {
                    field: "compliance.audit_logging_enabled".to_string(),
                    violation: "GDPR requires comprehensive audit logging".to_string(),
                    recommendation: "Enable audit logging for GDPR compliance".to_string(),
                });
            }
            
            if !config.data_encryption_required {
                result.add_error(ConfigValidationError::SecurityViolation {
                    field: "compliance.data_encryption_required".to_string(),
                    violation: "GDPR requires data encryption by design".to_string(),
                    recommendation: "Enable data encryption for GDPR compliance".to_string(),
                });
            }
        }
        
        // HIPAA validation
        if config.hipaa_enabled {
            if !config.data_encryption_required {
                result.add_error(ConfigValidationError::SecurityViolation {
                    field: "compliance.data_encryption_required".to_string(),
                    violation: "HIPAA requires encryption of PHI".to_string(),
                    recommendation: "Enable data encryption for HIPAA compliance".to_string(),
                });
            }
            
            if !config.breach_notification_enabled {
                result.add_error(ConfigValidationError::SecurityViolation {
                    field: "compliance.breach_notification_enabled".to_string(),
                    violation: "HIPAA requires breach notification procedures".to_string(),
                    recommendation: "Enable breach notification for HIPAA compliance".to_string(),
                });
            }
        }
    }
    
    fn validate_performance_config(&self, config: &PerformanceConfig, result: &mut ValidationResult) {
        // Validate cache configuration
        if config.enable_caching && config.cache_size_mb > 10000 {
            result.add_warning("Large cache size may consume significant memory".to_string());
        }
        
        if config.cache_ttl_seconds < 60 {
            result.add_warning("Short cache TTL may reduce cache effectiveness".to_string());
        }
        
        // Validate batch processing
        if config.batch_processing.enabled {
            if config.batch_processing.batch_size > 1000 {
                result.add_warning("Large batch size may increase memory usage and latency".to_string());
            }
            
            if config.batch_processing.batch_timeout_ms > 10000 {
                result.add_warning("Long batch timeout may increase response time".to_string());
            }
        }
        
        // Validate resource limits
        if let Some(max_memory) = config.resource_limits.max_memory_mb {
            if max_memory < 256 {
                result.add_warning("Low memory limit may cause performance issues".to_string());
            }
        }
        
        if let Some(max_cpu) = config.resource_limits.max_cpu_percent {
            if max_cpu > 90.0 {
                result.add_warning("High CPU limit may impact system stability".to_string());
            }
        }
    }
    
    fn validate_backup_config(&self, config: &BackupConfig, result: &mut ValidationResult) {
        if !config.enabled && self.environment == Environment::Production {
            result.add_error(ConfigValidationError::MissingRequired {
                field: "backup.enabled".to_string(),
                context: "Production environment requires backup configuration".to_string(),
            });
        }
        
        if config.enabled {
            // Validate backup intervals
            if config.backup_interval_hours > 24 {
                result.add_warning("Long backup interval may increase data loss risk".to_string());
            }
            
            // Validate retention
            if config.retention_days < 7 {
                result.add_warning("Short backup retention may not provide adequate recovery options".to_string());
            }
            
            // Validate backup location
            if !config.backup_location.exists() {
                result.add_error(ConfigValidationError::PathError {
                    path: config.backup_location.clone(),
                    error_type: PathErrorType::NotFound,
                });
            }
            
            // Security validation
            if !config.encryption_enabled {
                result.add_error(ConfigValidationError::SecurityViolation {
                    field: "backup.encryption_enabled".to_string(),
                    violation: "Backup encryption not enabled".to_string(),
                    recommendation: "Enable backup encryption to protect sensitive data".to_string(),
                });
            }
        }
    }
    
    fn validate_environment_consistency(&self, config: &ProductionConfig, result: &mut ValidationResult) {
        let mut incompatible_settings = Vec::new();
        
        match self.environment {
            Environment::Development => {
                if config.security.require_post_quantum {
                    incompatible_settings.push("require_post_quantum enabled in development".to_string());
                }
                if config.compliance.gdpr_enabled || config.compliance.hipaa_enabled {
                    incompatible_settings.push("compliance features enabled in development".to_string());
                }
            }
            Environment::Production => {
                if !config.monitoring.metrics_enabled {
                    incompatible_settings.push("monitoring disabled in production".to_string());
                }
                if !config.logging.structured_logging {
                    incompatible_settings.push("structured logging disabled in production".to_string());
                }
                if !config.backup.enabled {
                    incompatible_settings.push("backups disabled in production".to_string());
                }
            }
            _ => {}
        }
        
        if !incompatible_settings.is_empty() {
            result.add_error(ConfigValidationError::EnvironmentMismatch {
                environment: self.environment.clone(),
                incompatible_settings,
            });
        }
    }
    
    fn validate_security_consistency(&self, config: &ProductionConfig, result: &mut ValidationResult) {
        // Check if security settings are internally consistent
        if config.security.require_post_quantum && config.security.minimum_crypto_mode == "classical" {
            result.add_error(ConfigValidationError::InvalidValue {
                field: "security.minimum_crypto_mode".to_string(),
                value: config.security.minimum_crypto_mode.clone(),
                expected: "hybrid or quantum".to_string(),
                reason: "Post-quantum requirement conflicts with classical crypto mode".to_string(),
            });
        }
        
        if config.compliance.gdpr_enabled && !config.security.access_control.require_authentication {
            result.add_error(ConfigValidationError::SecurityViolation {
                field: "security.access_control.require_authentication".to_string(),
                violation: "GDPR compliance requires authentication".to_string(),
                recommendation: "Enable authentication for GDPR compliance".to_string(),
            });
        }
    }
    
    fn validate_resource_allocation(&self, config: &ProductionConfig, result: &mut ValidationResult) {
        // Check for resource conflicts
        let estimated_memory_usage = self.estimate_memory_usage(config);
        
        if let Some(max_memory) = config.performance.resource_limits.max_memory_mb {
            if estimated_memory_usage > max_memory {
                result.add_warning(format!(
                    "Estimated memory usage ({} MB) exceeds configured limit ({} MB)",
                    estimated_memory_usage, max_memory
                ));
            }
        }
        
        // Check connection limits consistency
        let total_connections = config.server.max_connections + config.database.max_connections;
        if total_connections > 50000 {
            result.add_warning("Very high total connection count may exhaust system resources".to_string());
        }
    }
    
    fn estimate_memory_usage(&self, config: &ProductionConfig) -> u64 {
        let mut estimated_mb = 0u64;
        
        // Base application memory
        estimated_mb += 128;
        
        // Cache memory
        if config.performance.enable_caching {
            estimated_mb += config.performance.cache_size_mb;
        }
        
        // Connection overhead (estimate 1KB per connection)
        estimated_mb += config.server.max_connections as u64 / 1024;
        
        // Database connection overhead
        estimated_mb += config.database.max_connections as u64 / 10;
        
        estimated_mb
    }
}

impl SecurityBaseline {
    pub fn for_environment(environment: &Environment) -> Self {
        match environment {
            Environment::Production => Self {
                min_tls_version: "1.3".to_string(),
                required_cipher_suites: vec![
                    "TLS_AES_256_GCM_SHA384".to_string(),
                    "TLS_CHACHA20_POLY1305_SHA256".to_string(),
                ],
                min_key_size: 256,
                require_perfect_forward_secrecy: true,
                max_session_timeout_minutes: 60,
                require_audit_logging: true,
            },
            Environment::Staging => Self {
                min_tls_version: "1.2".to_string(),
                required_cipher_suites: vec!["TLS_AES_256_GCM_SHA384".to_string()],
                min_key_size: 256,
                require_perfect_forward_secrecy: true,
                max_session_timeout_minutes: 120,
                require_audit_logging: true,
            },
            _ => Self {
                min_tls_version: "1.2".to_string(),
                required_cipher_suites: vec![],
                min_key_size: 128,
                require_perfect_forward_secrecy: false,
                max_session_timeout_minutes: 240,
                require_audit_logging: false,
            },
        }
    }
}

/// Load and validate configuration from file
pub fn load_and_validate_config(config_path: &std::path::Path, environment: Environment) -> Result<ProductionConfig, Box<dyn std::error::Error>> {
    // Load configuration from file
    let config_content = std::fs::read_to_string(config_path)?;
    let config: ProductionConfig = if config_path.extension().unwrap_or_default() == "toml" {
        toml::from_str(&config_content)?
    } else {
        serde_json::from_str(&config_content)?
    };
    
    // Validate configuration
    let validator = ConfigValidator::new(environment);
    let validation_result = validator.validate(&config);
    
    if !validation_result.valid {
        let error_messages: Vec<String> = validation_result.errors
            .iter()
            .map(|e| e.to_string())
            .collect();
        
        return Err(format!("Configuration validation failed:
{}", error_messages.join("
")).into());
    }
    
    // Print warnings and recommendations
    for warning in &validation_result.warnings {
        eprintln!("WARNING: {}", warning);
    }
    
    for recommendation in &validation_result.recommendations {
        eprintln!("RECOMMENDATION: {}", recommendation);
    }
    
    eprintln!("Configuration security score: {}/100", validation_result.security_score);
    
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;
    
    fn create_test_config() -> ProductionConfig {
        ProductionConfig {
            server: ServerConfig {
                bind_address: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
                api_port: 8080,
                websocket_port: 8443,
                worker_threads: Some(4),
                max_connections: 1000,
                connection_timeout_seconds: 30,
                keep_alive_seconds: 300,
                tls_cert_path: PathBuf::from("/tmp/cert.pem"),
                tls_key_path: PathBuf::from("/tmp/key.pem"),
                server_name: "test-server".to_string(),
                environment: Environment::Testing,
            },
            security: SecurityConfig {
                minimum_crypto_mode: "hybrid".to_string(),
                require_post_quantum: false,
                max_message_age_seconds: 300,
                rate_limiting: RateLimitConfig {
                    max_messages_per_minute: 60,
                    max_connections_per_ip: 10,
                    burst_allowance: 20,
                    ban_duration_minutes: 60,
                },
                key_rotation: KeyRotationConfig {
                    automatic_rotation: true,
                    rotation_interval_days: 30,
                    emergency_rotation_enabled: true,
                    pre_rotation_notification_hours: 24,
                },
                access_control: AccessControlConfig {
                    require_authentication: true,
                    session_timeout_minutes: 60,
                    max_failed_attempts: 3,
                    lockout_duration_minutes: 30,
                    require_mfa: false,
                },
                threat_detection: ThreatDetectionConfig {
                    enabled: true,
                    anomaly_detection: true,
                    ip_reputation_checking: true,
                    behavioral_analysis: false,
                },
            },
            database: DatabaseConfig {
                database_type: DatabaseType::PostgreSQL,
                connection_string: "postgresql://user:pass@localhost/db".to_string(),
                max_connections: 20,
                connection_timeout_seconds: 30,
                query_timeout_seconds: 60,
                ssl_required: true,
                backup_enabled: true,
                encryption_at_rest: true,
            },
            logging: LoggingConfig {
                level: LogLevel::Info,
                structured_logging: true,
                audit_log_path: PathBuf::from("/tmp/audit.log"),
                error_log_path: PathBuf::from("/tmp/error.log"),
                access_log_path: PathBuf::from("/tmp/access.log"),
                log_rotation: LogRotationConfig {
                    max_size_mb: 100,
                    max_files: 10,
                    rotation_schedule: RotationSchedule::Daily,
                    compression_enabled: true,
                },
                remote_logging: None,
            },
            monitoring: MonitoringConfig {
                metrics_enabled: true,
                metrics_port: 9090,
                health_check_interval_seconds: 30,
                alert_thresholds: AlertThresholds {
                    cpu_usage_percent: 80.0,
                    memory_usage_percent: 85.0,
                    disk_usage_percent: 90.0,
                    error_rate_percent: 1.0,
                    response_time_ms: 1000.0, // Fixed: Use f64 instead of u64
                },
                external_monitoring: None,
            },
            compliance: ComplianceConfig {
                gdpr_enabled: false,
                hipaa_enabled: false,
                sox_enabled: false,
                data_retention_days: 90,
                audit_logging_enabled: true,
                data_encryption_required: true,
                breach_notification_enabled: true,
                privacy_controls: PrivacyControlsConfig {
                    data_minimization: true,
                    purpose_limitation: true,
                    storage_limitation: true,
                    accuracy_controls: true,
                    security_controls: true,
                    accountability_measures: true,
                },
            },
            performance: PerformanceConfig {
                enable_caching: true,
                cache_size_mb: 256,
                cache_ttl_seconds: 300,
                connection_pooling: true,
                async_processing: true,
                batch_processing: BatchProcessingConfig {
                    enabled: true,
                    batch_size: 100,
                    batch_timeout_ms: 1000,
                    max_concurrent_batches: 10,
                },
                resource_limits: ResourceLimitsConfig {
                    max_memory_mb: Some(2048),
                    max_cpu_percent: Some(80.0),
                    max_file_descriptors: Some(65536),
                    max_network_connections: Some(10000),
                },
            },
            backup: BackupConfig {
                enabled: true,
                backup_interval_hours: 6,
                retention_days: 30,
                backup_location: PathBuf::from("/tmp/backups"),
                encryption_enabled: true,
                compression_enabled: true,
                remote_backup: None,
            },
        }
    }
    
    #[test]
    fn test_config_validation_valid() {
        let config = create_test_config();
        let validator = ConfigValidator::new(Environment::Testing);
        let result = validator.validate(&config);
        
        assert!(result.valid, "Valid configuration should pass validation");
        assert!(result.errors.is_empty(), "No errors should be present for valid config");
    }
    
    #[test]
    fn test_config_validation_port_conflict() {
        let mut config = create_test_config();
        config.server.websocket_port = config.server.api_port;
        
        let validator = ConfigValidator::new(Environment::Testing);
        let result = validator.validate(&config);
        
        assert!(!result.valid, "Configuration with port conflict should fail");
        assert!(!result.errors.is_empty(), "Should have validation errors");
    }
    
    #[test]
    fn test_production_security_requirements() {
        let mut config = create_test_config();
        config.server.environment = Environment::Production;
        config.security.minimum_crypto_mode = "classical".to_string();
        
        let validator = ConfigValidator::new(Environment::Production);
        let result = validator.validate(&config);
        
        assert!(!result.warnings.is_empty(), "Production with classical crypto should generate warnings");
    }
    
    #[test]
    fn test_security_baseline() {
        let prod_baseline = SecurityBaseline::for_environment(&Environment::Production);
        let dev_baseline = SecurityBaseline::for_environment(&Environment::Development);
        
        assert_eq!(prod_baseline.min_tls_version, "1.3");
        assert_eq!(dev_baseline.min_tls_version, "1.2");
        assert!(prod_baseline.require_audit_logging);
        assert!(!dev_baseline.require_audit_logging);
    }
}
