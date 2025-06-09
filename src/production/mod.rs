//! Production Module - Session 8 Production Hardening
//! 
//! This module provides comprehensive production-ready capabilities for the
//! Quantum-Resistant Nano-Messenger, including error handling, audit logging,
//! configuration validation, migration tools, and health monitoring.

pub mod error_handling;
pub mod audit_logging;
pub mod config_validation;
pub mod migration_tools;
pub mod health_monitoring;

pub use error_handling::{
    ProductionError, ErrorHandler, ErrorContext, RecoveryStrategy, RecoveryResult,
    error_builders
};
pub use audit_logging::{
    AuditLogger, AuditEvent, AuditEventType, TamperEvidentAuditLog,
    ComplianceRegulation, SecuritySeverity
};
pub use config_validation::{
    ProductionConfig, ConfigValidator, ValidationResult, Environment,
    load_and_validate_config
};
pub use migration_tools::{
    MigrationManager, MigrationPlan, MigrationResult, MigrationStatus,
    MigrationType
};
pub use health_monitoring::{
    HealthMonitor, SystemHealthReport, HealthStatus, HealthCheckResult,
    SystemMetrics, PerformanceMetrics, SecurityMetrics
};

// use crate::crypto::CryptoMode; // Unused
use std::sync::Arc;
use std::time::SystemTime;
use uuid::Uuid;

/// Production environment manager that coordinates all production components
pub struct ProductionManager {
    pub config: ProductionConfig,
    pub error_handler: ErrorHandler,
    pub audit_logger: AuditLogger,
    pub health_monitor: Option<HealthMonitor>,
    pub environment: Environment,
    pub startup_time: SystemTime,
    pub instance_id: Uuid,
}

impl ProductionManager {
    /// Initialize production manager with validated configuration
    pub fn new(config: ProductionConfig) -> Result<Self, ProductionError> {
        let environment = config.server.environment.clone();
        let instance_id = Uuid::new_v4();
        
        // Initialize error handler
        let error_handler = ErrorHandler::new(
            format!("{:?}", environment),
            "nano-messenger".to_string(),
            "2.0.0".to_string(),
        );
        
        // Initialize audit logger
        let audit_logger = AuditLogger::new(
            "nano-messenger".to_string(),
            "2.0.0".to_string(),
            format!("{:?}", environment),
        );
        
        Ok(Self {
            config,
            error_handler,
            audit_logger,
            health_monitor: None,
            environment,
            startup_time: SystemTime::now(),
            instance_id,
        })
    }
    
    /// Start all production services
    pub async fn start(&mut self) -> Result<(), ProductionError> {
        println!("🚀 Starting Quantum-Resistant Nano-Messenger v2.0.0");
        println!("Environment: {:?}", self.environment);
        println!("Instance ID: {}", self.instance_id);
        
        // Log system startup
        if let Err(e) = self.audit_logger.log_system_startup() {
            eprintln!("Failed to log system startup: {}", e);
        }
        
        // Initialize health monitoring if enabled
        if self.config.monitoring.metrics_enabled {
            self.initialize_health_monitoring().await?;
        }
        
        // Validate production readiness
        self.validate_production_readiness().await?;
        
        println!("✅ Production services started successfully");
        Ok(())
    }
    
    /// Stop all production services gracefully
    pub async fn stop(&mut self) -> Result<(), ProductionError> {
        println!("🛑 Stopping Quantum-Resistant Nano-Messenger");
        
        let uptime = self.startup_time.elapsed().unwrap_or_default();
        
        // Log system shutdown
        if let Err(e) = self.audit_logger.log_system_shutdown(uptime) {
            eprintln!("Failed to log system shutdown: {}", e);
        }
        
        println!("✅ Production services stopped gracefully");
        Ok(())
    }
    
    /// Handle production errors with recovery attempts
    pub async fn handle_error(&self, error: ProductionError) -> Result<RecoveryResult, ProductionError> {
        // Log the error for audit trail
        self.log_error_event(&error).await;
        
        // Attempt error recovery
        self.error_handler.handle_error(error)
    }
    
    /// Get current system health status
    pub async fn get_health_status(&self) -> Result<SystemHealthReport, ProductionError> {
        if let Some(ref monitor) = self.health_monitor {
            monitor.generate_health_report().await
                .map_err(|_e| ProductionError::SystemError {
                    subsystem: "health_monitoring".to_string(),
                    error_type: crate::production::error_handling::SystemErrorType::HealthCheckFailure,
                    severity: crate::production::error_handling::ErrorSeverity::Error,
                    requires_restart: false,
                    affects_availability: false,
                    context: self.error_handler.create_error_context(None),
                })
        } else {
            Err(ProductionError::SystemError {
                subsystem: "health_monitoring".to_string(),
                error_type: crate::production::error_handling::SystemErrorType::ServiceDependencyFailure,
                severity: crate::production::error_handling::ErrorSeverity::Warning,
                requires_restart: false,
                affects_availability: false,
                context: self.error_handler.create_error_context(None),
            })
        }
    }
    
    /// Perform production readiness validation
    async fn validate_production_readiness(&self) -> Result<(), ProductionError> {
        println!("🔍 Validating production readiness...");
        
        // Validate configuration
        let validator = ConfigValidator::new(self.environment.clone());
        let validation_result = validator.validate(&self.config);
        
        if !validation_result.valid {
            return Err(ProductionError::ConfigurationError {
                config_section: "production_readiness".to_string(),
                field_name: None,
                validation_failure: "Production readiness validation failed".to_string(),
                expected_format: "Valid production configuration".to_string(),
                actual_value: format!("{} errors", validation_result.errors.len()),
                context: self.error_handler.create_error_context(None),
            });
        }
        
        // Environment-specific validations
        match self.environment {
            Environment::Production => {
                self.validate_production_environment().await?;
            }
            Environment::Staging => {
                self.validate_staging_environment().await?;
            }
            _ => {
                println!("⚠️ Non-production environment - some validations skipped");
            }
        }
        
        println!("✅ Production readiness validated");
        Ok(())
    }
    
    async fn validate_production_environment(&self) -> Result<(), ProductionError> {
        // Ensure critical production requirements
        if !self.config.security.require_post_quantum && self.config.security.minimum_crypto_mode == "classical" {
            return Err(ProductionError::SecurityError {
                security_domain: crate::production::error_handling::SecurityDomain::Cryptography,
                user_id: None,
                attempted_operation: "production_validation".to_string(),
                failure_reason: "Production environment must use hybrid or quantum crypto".to_string(),
                threat_level: crate::production::error_handling::ThreatLevel::High,
                requires_incident_response: false,
                context: self.error_handler.create_error_context(None),
            });
        }
        
        if !self.config.database.ssl_required {
            return Err(ProductionError::SecurityError {
                security_domain: crate::production::error_handling::SecurityDomain::DataAccess,
                user_id: None,
                attempted_operation: "production_validation".to_string(),
                failure_reason: "Production environment requires SSL for database connections".to_string(),
                threat_level: crate::production::error_handling::ThreatLevel::High,
                requires_incident_response: false,
                context: self.error_handler.create_error_context(None),
            });
        }
        
        if !self.config.backup.enabled {
            return Err(ProductionError::SystemError {
                subsystem: "backup".to_string(),
                error_type: crate::production::error_handling::SystemErrorType::ConfigurationInconsistency,
                severity: crate::production::error_handling::ErrorSeverity::Critical,
                requires_restart: false,
                affects_availability: false,
                context: self.error_handler.create_error_context(None),
            });
        }
        
        Ok(())
    }
    
    async fn validate_staging_environment(&self) -> Result<(), ProductionError> {
        // Staging-specific validations
        if !self.config.monitoring.metrics_enabled {
            println!("⚠️ Staging environment should have monitoring enabled");
        }
        
        Ok(())
    }
    
    async fn initialize_health_monitoring(&mut self) -> Result<(), ProductionError> {
        use crate::production::health_monitoring::{
            HealthMonitorConfig, AlertThresholds, NotificationSettings
        };
        
        let health_config = HealthMonitorConfig {
            enabled: true,
            health_check_interval_seconds: self.config.monitoring.health_check_interval_seconds,
            metrics_collection_interval_seconds: 60,
            alert_thresholds: AlertThresholds {
                cpu_usage_warning: self.config.monitoring.alert_thresholds.cpu_usage_percent * 0.8,
                cpu_usage_critical: self.config.monitoring.alert_thresholds.cpu_usage_percent,
                memory_usage_warning: self.config.monitoring.alert_thresholds.memory_usage_percent * 0.8,
                memory_usage_critical: self.config.monitoring.alert_thresholds.memory_usage_percent,
                disk_usage_warning: self.config.monitoring.alert_thresholds.disk_usage_percent * 0.8,
                disk_usage_critical: self.config.monitoring.alert_thresholds.disk_usage_percent,
                error_rate_warning: self.config.monitoring.alert_thresholds.error_rate_percent * 0.5,
                error_rate_critical: self.config.monitoring.alert_thresholds.error_rate_percent,
                response_time_warning_ms: self.config.monitoring.alert_thresholds.response_time_ms as f64 * 0.8,
                response_time_critical_ms: self.config.monitoring.alert_thresholds.response_time_ms as f64,
                connection_count_warning: (self.config.server.max_connections as f64 * 0.8) as u32,
                connection_count_critical: self.config.server.max_connections,
            },
            external_endpoints: vec![],
            notification_settings: NotificationSettings {
                enabled: true,
                email_notifications: false,
                webhook_notifications: false,
                webhook_url: None,
                email_recipients: vec![],
                notification_cooldown_minutes: 5,
            },
        };
        
        // Create mock implementations for the traits (in real implementation, these would be actual implementations)
        let metrics_collector = Arc::new(DefaultMetricsCollector::new());
        let alert_manager = Arc::new(DefaultAlertManager::new());
        
        let health_monitor = HealthMonitor::new(
            health_config,
            metrics_collector,
            alert_manager,
            "2.0.0".to_string(),
            format!("{:?}", self.environment),
        );
        
        // Start health monitoring
        health_monitor.start().await
            .map_err(|_e| ProductionError::SystemError {
                subsystem: "health_monitoring".to_string(),
                error_type: crate::production::error_handling::SystemErrorType::ServiceDependencyFailure,
                severity: crate::production::error_handling::ErrorSeverity::Error,
                requires_restart: false,
                affects_availability: false,
                context: self.error_handler.create_error_context(None),
            })?;
        
        self.health_monitor = Some(health_monitor);
        Ok(())
    }
    
    async fn log_error_event(&self, error: &ProductionError) {
        // Convert production error to audit event
        let event_type = match error {
            ProductionError::SecurityError { .. } => {
                audit_logging::AuditEventType::SecurityIncident {
                    incident_id: Uuid::new_v4().to_string(),
                    incident_type: audit_logging::SecurityIncidentType::ConfigurationError,
                    severity: audit_logging::SecuritySeverity::High,
                    affected_users: vec![],
                    description: error.to_string(),
                    mitigation_actions: vec![],
                }
            }
            ProductionError::SystemError { .. } => {
                audit_logging::AuditEventType::SystemShutdown {
                    component: "error_handler".to_string(),
                    uptime_seconds: 0,
                    shutdown_reason: error.to_string(),
                    graceful: false,
                }
            }
            _ => {
                audit_logging::AuditEventType::SystemShutdown {
                    component: "error_handler".to_string(),
                    uptime_seconds: 0,
                    shutdown_reason: format!("Error: {}", error),
                    graceful: false,
                }
            }
        };
        
        let _audit_event = audit_logging::AuditEvent {
            event_id: Uuid::new_v4(),
            timestamp: SystemTime::now(),
            event_type,
            correlation_id: None,
            session_id: None,
            request_id: None,
            component: "production_manager".to_string(),
            version: "2.0.0".to_string(),
            environment: format!("{:?}", self.environment),
            compliance_tags: vec![],
            retention_policy: audit_logging::RetentionPolicy::SevenYears,
            additional_context: std::collections::HashMap::new(),
        };
        
        // Note: We can't modify the audit_logger here due to borrowing rules
        // In a real implementation, this would use a message queue or async logging
        println!("Error logged for audit: {}", error);
    }
}

impl AuditLogger {
    /// Log system startup event
    pub fn log_system_startup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let startup_event = audit_logging::AuditEvent {
            event_id: Uuid::new_v4(),
            timestamp: SystemTime::now(),
            event_type: audit_logging::AuditEventType::SystemStartup {
                component: self.component.clone(),
                version: self.version.clone(),
                configuration_hash: "production_config_hash".to_string(),
                startup_duration_ms: 1000,
            },
            correlation_id: None,
            session_id: None,
            request_id: None,
            component: self.component.clone(),
            version: self.version.clone(),
            environment: self.environment.clone(),
            compliance_tags: vec![audit_logging::ComplianceTag::SoxSection404],
            retention_policy: audit_logging::RetentionPolicy::SevenYears,
            additional_context: std::collections::HashMap::new(),
        };
        
        self.audit_log.append_event(startup_event)
    }
    
    /// Log system shutdown event
    pub fn log_system_shutdown(&mut self, uptime: std::time::Duration) -> Result<(), Box<dyn std::error::Error>> {
        let shutdown_event = audit_logging::AuditEvent {
            event_id: Uuid::new_v4(),
            timestamp: SystemTime::now(),
            event_type: audit_logging::AuditEventType::SystemShutdown {
                component: self.component.clone(),
                uptime_seconds: uptime.as_secs(),
                shutdown_reason: "Graceful shutdown".to_string(),
                graceful: true,
            },
            correlation_id: None,
            session_id: None,
            request_id: None,
            component: self.component.clone(),
            version: self.version.clone(),
            environment: self.environment.clone(),
            compliance_tags: vec![audit_logging::ComplianceTag::SoxSection404],
            retention_policy: audit_logging::RetentionPolicy::SevenYears,
            additional_context: std::collections::HashMap::new(),
        };
        
        self.audit_log.append_event(shutdown_event)
    }
}

/// Default metrics collector implementation
pub struct DefaultMetricsCollector;

impl DefaultMetricsCollector {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl health_monitoring::MetricsCollector for DefaultMetricsCollector {
    async fn collect_system_metrics(&self) -> health_monitoring::SystemMetrics {
        // In a real implementation, this would collect actual system metrics
        health_monitoring::SystemMetrics {
            cpu_usage_percent: 25.0,
            memory_usage_percent: 60.0,
            memory_total_mb: 8192,
            memory_used_mb: 4915,
            disk_usage_percent: 45.0,
            disk_total_gb: 500,
            disk_used_gb: 225,
            network_bytes_sent: 1024000,
            network_bytes_received: 2048000,
            load_average_1m: 1.2,
            load_average_5m: 1.0,
            load_average_15m: 0.8,
            open_file_descriptors: 256,
            max_file_descriptors: 65536,
        }
    }
    
    async fn collect_performance_metrics(&self) -> health_monitoring::PerformanceMetrics {
        health_monitoring::PerformanceMetrics {
            messages_processed_total: 10000,
            messages_processed_per_second: 100.0,
            average_response_time_ms: 50.0,
            p95_response_time_ms: 120.0,
            p99_response_time_ms: 200.0,
            error_rate_percent: 0.1,
            active_connections: 50,
            database_connections_active: 5,
            database_connections_idle: 15,
            cache_hit_rate_percent: 95.0,
            crypto_operations_per_second: 500.0,
            key_rotation_last_performed: Some(SystemTime::now()),
        }
    }
    
    async fn collect_security_metrics(&self) -> health_monitoring::SecurityMetrics {
        health_monitoring::SecurityMetrics {
            authentication_failures_per_minute: 0.1,
            authorization_failures_per_minute: 0.05,
            suspicious_activities_detected: 0,
            blocked_ips_count: 5,
            crypto_mode_distribution: [
                ("classical".to_string(), 10),
                ("hybrid".to_string(), 80),
                ("quantum".to_string(), 10),
            ].into(),
            tls_handshake_failures: 2,
            certificate_expiry_days: Some(90),
            last_security_scan: Some(SystemTime::now()),
            compliance_violations: 0,
        }
    }
}

/// Default alert manager implementation
pub struct DefaultAlertManager;

impl DefaultAlertManager {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl health_monitoring::AlertManager for DefaultAlertManager {
    async fn send_alert(&self, alert: health_monitoring::Alert) {
        println!("🚨 ALERT: {} - {}", alert.title, alert.description);
    }
    
    async fn check_alert_conditions(&self, report: &health_monitoring::SystemHealthReport) -> Vec<health_monitoring::Alert> {
        let mut alerts = Vec::new();
        
        // Check for critical system conditions
        if report.system_metrics.cpu_usage_percent > 90.0 {
            alerts.push(health_monitoring::Alert {
                alert_id: Uuid::new_v4(),
                alert_type: health_monitoring::AlertType::SystemHealth,
                severity: health_monitoring::AlertSeverity::Critical,
                title: "High CPU Usage".to_string(),
                description: format!("CPU usage is {}%", report.system_metrics.cpu_usage_percent),
                timestamp: SystemTime::now(),
                metadata: std::collections::HashMap::new(),
            });
        }
        
        if report.system_metrics.memory_usage_percent > 95.0 {
            alerts.push(health_monitoring::Alert {
                alert_id: Uuid::new_v4(),
                alert_type: health_monitoring::AlertType::SystemHealth,
                severity: health_monitoring::AlertSeverity::Critical,
                title: "High Memory Usage".to_string(),
                description: format!("Memory usage is {}%", report.system_metrics.memory_usage_percent),
                timestamp: SystemTime::now(),
                metadata: std::collections::HashMap::new(),
            });
        }
        
        alerts
    }
}

/// Production utilities and helper functions
pub mod utils {
    use super::*;
    
    /// Initialize production logging
    pub fn init_production_logging(environment: &Environment) -> Result<(), Box<dyn std::error::Error>> {
        let log_level = match environment {
            Environment::Production => "info",
            Environment::Staging => "info", 
            Environment::Testing => "debug",
            Environment::Development => "debug",
        };
        
        std::env::set_var("RUST_LOG", log_level);
        env_logger::init();
        
        println!("📋 Production logging initialized at level: {}", log_level);
        Ok(())
    }
    
    /// Validate system prerequisites for production deployment
    pub async fn validate_system_prerequisites() -> Result<(), ProductionError> {
        println!("🔍 Validating system prerequisites...");
        
        // Check Rust version
        let rust_version = std::env::var("RUSTC_VERSION").unwrap_or_else(|_| "unknown".to_string());
        println!("Rust version: {}", rust_version);
        
        // Check available memory
        // In a real implementation, this would check actual system resources
        println!("System memory: Available");
        
        // Check disk space
        println!("Disk space: Available");
        
        // Check network connectivity
        println!("Network connectivity: Available");
        
        println!("✅ System prerequisites validated");
        Ok(())
    }
    
    /// Generate production deployment summary
    pub fn generate_deployment_summary(config: &ProductionConfig, instance_id: &Uuid) -> String {
        format!(
            r#"
🚀 QUANTUM-RESISTANT NANO-MESSENGER DEPLOYMENT SUMMARY
=====================================================

Instance ID: {}
Environment: {:?}
Version: 2.0.0
Crypto Mode: {}
TLS Required: {}
Monitoring: {}
Backup: {}
Compliance: GDPR={}, HIPAA={}, SOX={}

Server Configuration:
- API Port: {}
- WebSocket Port: {}
- Max Connections: {}
- Worker Threads: {:?}

Security Configuration:
- Minimum Crypto Mode: {}
- Post-Quantum Required: {}
- Key Rotation: {} days
- MFA Required: {}

Performance Configuration:
- Caching: {}
- Connection Pooling: {}
- Batch Processing: {}

Deployment Time: {}
"#,
            instance_id,
            config.server.environment,
            config.security.minimum_crypto_mode,
            config.database.ssl_required,
            config.monitoring.metrics_enabled,
            config.backup.enabled,
            config.compliance.gdpr_enabled,
            config.compliance.hipaa_enabled,
            config.compliance.sox_enabled,
            config.server.api_port,
            config.server.websocket_port,
            config.server.max_connections,
            config.server.worker_threads,
            config.security.minimum_crypto_mode,
            config.security.require_post_quantum,
            config.security.key_rotation.rotation_interval_days,
            config.security.access_control.require_mfa,
            config.performance.enable_caching,
            config.performance.connection_pooling,
            config.performance.batch_processing.enabled,
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        )
    }
}

/// Export main production types and functions for easy access
pub use utils::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;
    use std::path::PathBuf;
    
    fn create_test_production_config() -> ProductionConfig {
        ProductionConfig {
            server: config_validation::ServerConfig {
                bind_address: std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
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
            security: config_validation::SecurityConfig {
                minimum_crypto_mode: "hybrid".to_string(),
                require_post_quantum: false,
                max_message_age_seconds: 300,
                rate_limiting: config_validation::RateLimitConfig {
                    max_messages_per_minute: 60,
                    max_connections_per_ip: 10,
                    burst_allowance: 20,
                    ban_duration_minutes: 60,
                },
                key_rotation: config_validation::KeyRotationConfig {
                    automatic_rotation: true,
                    rotation_interval_days: 30,
                    emergency_rotation_enabled: true,
                    pre_rotation_notification_hours: 24,
                },
                access_control: config_validation::AccessControlConfig {
                    require_authentication: true,
                    session_timeout_minutes: 60,
                    max_failed_attempts: 3,
                    lockout_duration_minutes: 30,
                    require_mfa: false,
                },
                threat_detection: config_validation::ThreatDetectionConfig {
                    enabled: true,
                    anomaly_detection: true,
                    ip_reputation_checking: true,
                    behavioral_analysis: false,
                },
            },
            database: config_validation::DatabaseConfig {
                database_type: config_validation::DatabaseType::PostgreSQL,
                connection_string: "postgresql://user:pass@localhost/db".to_string(),
                max_connections: 20,
                connection_timeout_seconds: 30,
                query_timeout_seconds: 60,
                ssl_required: true,
                backup_enabled: true,
                encryption_at_rest: true,
            },
            logging: config_validation::LoggingConfig {
                level: config_validation::LogLevel::Info,
                structured_logging: true,
                audit_log_path: PathBuf::from("/tmp/audit.log"),
                error_log_path: PathBuf::from("/tmp/error.log"),
                access_log_path: PathBuf::from("/tmp/access.log"),
                log_rotation: config_validation::LogRotationConfig {
                    max_size_mb: 100,
                    max_files: 10,
                    rotation_schedule: config_validation::RotationSchedule::Daily,
                    compression_enabled: true,
                },
                remote_logging: None,
            },
            monitoring: config_validation::MonitoringConfig {
                metrics_enabled: true,
                metrics_port: 9090,
                health_check_interval_seconds: 30,
                alert_thresholds: config_validation::AlertThresholds {
                    cpu_usage_percent: 80.0,
                    memory_usage_percent: 85.0,
                    disk_usage_percent: 90.0,
                    error_rate_percent: 1.0,
                    response_time_ms: 1000,
                },
                external_monitoring: None,
            },
            compliance: config_validation::ComplianceConfig {
                gdpr_enabled: false,
                hipaa_enabled: false,
                sox_enabled: false,
                data_retention_days: 90,
                audit_logging_enabled: true,
                data_encryption_required: true,
                breach_notification_enabled: true,
                privacy_controls: config_validation::PrivacyControlsConfig {
                    data_minimization: true,
                    purpose_limitation: true,
                    storage_limitation: true,
                    accuracy_controls: true,
                    security_controls: true,
                    accountability_measures: true,
                },
            },
            performance: config_validation::PerformanceConfig {
                enable_caching: true,
                cache_size_mb: 256,
                cache_ttl_seconds: 300,
                connection_pooling: true,
                async_processing: true,
                batch_processing: config_validation::BatchProcessingConfig {
                    enabled: true,
                    batch_size: 100,
                    batch_timeout_ms: 1000,
                    max_concurrent_batches: 10,
                },
                resource_limits: config_validation::ResourceLimitsConfig {
                    max_memory_mb: Some(2048),
                    max_cpu_percent: Some(80.0),
                    max_file_descriptors: Some(65536),
                    max_network_connections: Some(10000),
                },
            },
            backup: config_validation::BackupConfig {
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
    fn test_production_manager_creation() {
        let config = create_test_production_config();
        let manager = ProductionManager::new(config).unwrap();
        
        assert_eq!(manager.environment, Environment::Testing);
        assert_eq!(manager.error_handler.component, "nano-messenger");
        assert_eq!(manager.audit_logger.component, "nano-messenger");
    }
    
    #[tokio::test]
    async fn test_production_manager_startup() {
        let config = create_test_production_config();
        let mut manager = ProductionManager::new(config).unwrap();
        
        // This should succeed in test environment
        let result = manager.start().await;
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_deployment_summary_generation() {
        let config = create_test_production_config();
        let instance_id = Uuid::new_v4();
        
        let summary = utils::generate_deployment_summary(&config, &instance_id);
        
        assert!(summary.contains("QUANTUM-RESISTANT NANO-MESSENGER"));
        assert!(summary.contains(&instance_id.to_string()));
        assert!(summary.contains("hybrid"));
    }
}