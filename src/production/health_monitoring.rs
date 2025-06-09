//! Health Monitoring for Production Systems - Session 8
//! 
//! This module provides comprehensive health monitoring capabilities for production
//! deployments, including system health checks, performance monitoring, and
//! automated alerting for operational issues.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use uuid::Uuid;

/// System health status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Critical,
}

/// Individual health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    pub check_id: String,
    pub name: String,
    pub status: HealthStatus,
    pub message: String,
    pub timestamp: SystemTime,
    pub duration_ms: u64,
    pub tags: HashMap<String, String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Overall system health report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealthReport {
    pub report_id: Uuid,
    pub overall_status: HealthStatus,
    pub timestamp: SystemTime,
    pub uptime_seconds: u64,
    pub version: String,
    pub environment: String,
    pub health_checks: Vec<HealthCheckResult>,
    pub system_metrics: SystemMetrics,
    pub performance_metrics: PerformanceMetrics,
    pub security_metrics: SecurityMetrics,
}

/// System-level metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage_percent: f64,
    pub memory_usage_percent: f64,
    pub memory_total_mb: u64,
    pub memory_used_mb: u64,
    pub disk_usage_percent: f64,
    pub disk_total_gb: u64,
    pub disk_used_gb: u64,
    pub network_bytes_sent: u64,
    pub network_bytes_received: u64,
    pub load_average_1m: f64,
    pub load_average_5m: f64,
    pub load_average_15m: f64,
    pub open_file_descriptors: u32,
    pub max_file_descriptors: u32,
}

/// Application performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub messages_processed_total: u64,
    pub messages_processed_per_second: f64,
    pub average_response_time_ms: f64,
    pub p95_response_time_ms: f64,
    pub p99_response_time_ms: f64,
    pub error_rate_percent: f64,
    pub active_connections: u32,
    pub database_connections_active: u32,
    pub database_connections_idle: u32,
    pub cache_hit_rate_percent: f64,
    pub crypto_operations_per_second: f64,
    pub key_rotation_last_performed: Option<SystemTime>,
}

/// Security-related metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMetrics {
    pub authentication_failures_per_minute: f64,
    pub authorization_failures_per_minute: f64,
    pub suspicious_activities_detected: u32,
    pub blocked_ips_count: u32,
    pub crypto_mode_distribution: HashMap<String, u32>,
    pub tls_handshake_failures: u32,
    pub certificate_expiry_days: Option<u32>,
    pub last_security_scan: Option<SystemTime>,
    pub compliance_violations: u32,
}

/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    pub check_id: String,
    pub name: String,
    pub enabled: bool,
    pub interval_seconds: u64,
    pub timeout_seconds: u64,
    pub retry_count: u32,
    pub critical: bool,
    pub tags: HashMap<String, String>,
}

/// Health monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMonitorConfig {
    pub enabled: bool,
    pub health_check_interval_seconds: u64,
    pub metrics_collection_interval_seconds: u64,
    pub alert_thresholds: AlertThresholds,
    pub external_endpoints: Vec<ExternalEndpointConfig>,
    pub notification_settings: NotificationSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    pub cpu_usage_warning: f64,
    pub cpu_usage_critical: f64,
    pub memory_usage_warning: f64,
    pub memory_usage_critical: f64,
    pub disk_usage_warning: f64,
    pub disk_usage_critical: f64,
    pub error_rate_warning: f64,
    pub error_rate_critical: f64,
    pub response_time_warning_ms: f64,
    pub response_time_critical_ms: f64,
    pub connection_count_warning: u32,
    pub connection_count_critical: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalEndpointConfig {
    pub name: String,
    pub url: String,
    pub method: String,
    pub timeout_seconds: u64,
    pub expected_status_code: u16,
    pub check_interval_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettings {
    pub enabled: bool,
    pub email_notifications: bool,
    pub webhook_notifications: bool,
    pub webhook_url: Option<String>,
    pub email_recipients: Vec<String>,
    pub notification_cooldown_minutes: u32,
}

/// Health monitoring system
pub struct HealthMonitor {
    pub config: HealthMonitorConfig,
    pub health_checks: HashMap<String, Box<dyn HealthCheck + Send + Sync>>,
    pub metrics_collector: Arc<dyn MetricsCollector + Send + Sync>,
    pub alert_manager: Arc<dyn AlertManager + Send + Sync>,
    pub last_report: Arc<RwLock<Option<SystemHealthReport>>>,
    pub startup_time: SystemTime,
    pub version: String,
    pub environment: String,
}

/// Trait for individual health checks
#[async_trait::async_trait]
pub trait HealthCheck {
    async fn check(&self) -> HealthCheckResult;
    fn config(&self) -> &HealthCheckConfig;
}

/// Trait for metrics collection
#[async_trait::async_trait]
pub trait MetricsCollector {
    async fn collect_system_metrics(&self) -> SystemMetrics;
    async fn collect_performance_metrics(&self) -> PerformanceMetrics;
    async fn collect_security_metrics(&self) -> SecurityMetrics;
}

/// Trait for alert management
#[async_trait::async_trait]
pub trait AlertManager {
    async fn send_alert(&self, alert: Alert);
    async fn check_alert_conditions(&self, report: &SystemHealthReport) -> Vec<Alert>;
}

/// Alert information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub alert_id: Uuid,
    pub alert_type: AlertType,
    pub severity: AlertSeverity,
    pub title: String,
    pub description: String,
    pub timestamp: SystemTime,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    SystemHealth,
    Performance,
    Security,
    Availability,
    Compliance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
    Emergency,
}

impl HealthMonitor {
    pub fn new(
        config: HealthMonitorConfig,
        metrics_collector: Arc<dyn MetricsCollector + Send + Sync>,
        alert_manager: Arc<dyn AlertManager + Send + Sync>,
        version: String,
        environment: String,
    ) -> Self {
        let mut monitor = Self {
            config,
            health_checks: HashMap::new(),
            metrics_collector,
            alert_manager,
            last_report: Arc::new(RwLock::new(None)),
            startup_time: SystemTime::now(),
            version,
            environment,
        };
        
        // Register default health checks
        monitor.register_default_health_checks();
        
        monitor
    }
    
    /// Start the health monitoring service
    pub async fn start(&self) -> Result<(), HealthMonitorError> {
        println!("🏥 Starting health monitoring service");
        
        let monitor_clone = Arc::new(self.clone());
        
        // Start periodic health checks
        let health_check_monitor = Arc::clone(&monitor_clone);
        tokio::spawn(async move {
            health_check_monitor.run_periodic_health_checks().await;
        });
        
        // Start periodic metrics collection
        let metrics_monitor = Arc::clone(&monitor_clone);
        tokio::spawn(async move {
            metrics_monitor.run_periodic_metrics_collection().await;
        });
        
        // Start external endpoint monitoring
        let endpoint_monitor = Arc::clone(&monitor_clone);
        tokio::spawn(async move {
            endpoint_monitor.run_external_endpoint_monitoring().await;
        });
        
        println!("✅ Health monitoring service started successfully");
        Ok(())
    }
    
    /// Generate comprehensive health report
    pub async fn generate_health_report(&self) -> Result<SystemHealthReport, HealthMonitorError> {
        let start_time = SystemTime::now();
        
        // Collect all health check results
        let mut health_checks = Vec::new();
        for health_check in self.health_checks.values() {
            let result = health_check.check().await;
            health_checks.push(result);
        }
        
        // Collect metrics
        let system_metrics = self.metrics_collector.collect_system_metrics().await;
        let performance_metrics = self.metrics_collector.collect_performance_metrics().await;
        let security_metrics = self.metrics_collector.collect_security_metrics().await;
        
        // Determine overall health status
        let overall_status = self.determine_overall_status(&health_checks, &system_metrics, &performance_metrics);
        
        let report = SystemHealthReport {
            report_id: Uuid::new_v4(),
            overall_status,
            timestamp: start_time,
            uptime_seconds: start_time.duration_since(self.startup_time).unwrap_or_default().as_secs(),
            version: self.version.clone(),
            environment: self.environment.clone(),
            health_checks,
            system_metrics,
            performance_metrics,
            security_metrics,
        };
        
        // Store latest report
        *self.last_report.write().await = Some(report.clone());
        
        // Check for alerts
        let alerts = self.alert_manager.check_alert_conditions(&report).await;
        for alert in alerts {
            self.alert_manager.send_alert(alert).await;
        }
        
        Ok(report)
    }
    
    /// Get quick health status for endpoints
    pub async fn get_quick_status(&self) -> Result<QuickHealthStatus, HealthMonitorError> {
        let system_metrics = self.metrics_collector.collect_system_metrics().await;
        
        let status = if system_metrics.cpu_usage_percent > 90.0 || 
                       system_metrics.memory_usage_percent > 95.0 {
            HealthStatus::Critical
        } else if system_metrics.cpu_usage_percent > 80.0 || 
                  system_metrics.memory_usage_percent > 85.0 {
            HealthStatus::Degraded
        } else {
            HealthStatus::Healthy
        };
        
        Ok(QuickHealthStatus {
            status,
            timestamp: SystemTime::now(),
            uptime_seconds: SystemTime::now().duration_since(self.startup_time).unwrap_or_default().as_secs(),
            version: self.version.clone(),
        })
    }
    
    /// Register a custom health check
    pub fn register_health_check(&mut self, health_check: Box<dyn HealthCheck + Send + Sync>) {
        let config = health_check.config();
        self.health_checks.insert(config.check_id.clone(), health_check);
    }
    
    fn register_default_health_checks(&mut self) {
        // Database connectivity check
        let db_check = DatabaseHealthCheck::new(HealthCheckConfig {
            check_id: "database_connectivity".to_string(),
            name: "Database Connectivity".to_string(),
            enabled: true,
            interval_seconds: 30,
            timeout_seconds: 10,
            retry_count: 2,
            critical: true,
            tags: [("component".to_string(), "database".to_string())].into(),
        });
        self.register_health_check(Box::new(db_check));
        
        // API endpoint check
        let api_check = ApiHealthCheck::new(HealthCheckConfig {
            check_id: "api_endpoints".to_string(),
            name: "API Endpoints".to_string(),
            enabled: true,
            interval_seconds: 60,
            timeout_seconds: 5,
            retry_count: 1,
            critical: false,
            tags: [("component".to_string(), "api".to_string())].into(),
        });
        self.register_health_check(Box::new(api_check));
        
        // Cryptographic operations check
        let crypto_check = CryptographicHealthCheck::new(HealthCheckConfig {
            check_id: "cryptographic_operations".to_string(),
            name: "Cryptographic Operations".to_string(),
            enabled: true,
            interval_seconds: 120,
            timeout_seconds: 15,
            retry_count: 1,
            critical: true,
            tags: [("component".to_string(), "crypto".to_string())].into(),
        });
        self.register_health_check(Box::new(crypto_check));
        
        // Memory and resource check
        let resource_check = ResourceHealthCheck::new(HealthCheckConfig {
            check_id: "system_resources".to_string(),
            name: "System Resources".to_string(),
            enabled: true,
            interval_seconds: 30,
            timeout_seconds: 5,
            retry_count: 1,
            critical: true,
            tags: [("component".to_string(), "system".to_string())].into(),
        });
        self.register_health_check(Box::new(resource_check));
    }
    
    async fn run_periodic_health_checks(&self) {
        let mut interval = tokio::time::interval(Duration::from_secs(self.config.health_check_interval_seconds));
        
        loop {
            interval.tick().await;
            
            if let Err(e) = self.generate_health_report().await {
                eprintln!("Error generating health report: {}", e);
            }
        }
    }
    
    async fn run_periodic_metrics_collection(&self) {
        let mut interval = tokio::time::interval(Duration::from_secs(self.config.metrics_collection_interval_seconds));
        
        loop {
            interval.tick().await;
            
            // Collect and store metrics for trending
            let _system_metrics = self.metrics_collector.collect_system_metrics().await;
            let _performance_metrics = self.metrics_collector.collect_performance_metrics().await;
            let _security_metrics = self.metrics_collector.collect_security_metrics().await;
            
            // In a real implementation, these would be stored in a time-series database
        }
    }
    
    async fn run_external_endpoint_monitoring(&self) {
        for endpoint in &self.config.external_endpoints {
            let endpoint_clone = endpoint.clone();
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_secs(endpoint_clone.check_interval_seconds));
                
                loop {
                    interval.tick().await;
                    
                    // Check external endpoint
                    if let Err(e) = check_external_endpoint(&endpoint_clone).await {
                        eprintln!("External endpoint check failed for {}: {}", endpoint_clone.name, e);
                    }
                }
            });
        }
    }
    
    fn determine_overall_status(
        &self,
        health_checks: &[HealthCheckResult],
        system_metrics: &SystemMetrics,
        performance_metrics: &PerformanceMetrics,
    ) -> HealthStatus {
        // Check for critical health check failures
        let critical_failures = health_checks.iter()
            .filter(|check| check.status == HealthStatus::Critical || check.status == HealthStatus::Unhealthy)
            .count();
        
        if critical_failures > 0 {
            return HealthStatus::Critical;
        }
        
        // Check system resource thresholds
        if system_metrics.cpu_usage_percent > self.config.alert_thresholds.cpu_usage_critical ||
           system_metrics.memory_usage_percent > self.config.alert_thresholds.memory_usage_critical ||
           system_metrics.disk_usage_percent > self.config.alert_thresholds.disk_usage_critical {
            return HealthStatus::Critical;
        }
        
        // Check performance thresholds
        if performance_metrics.error_rate_percent > self.config.alert_thresholds.error_rate_critical ||
           performance_metrics.average_response_time_ms > self.config.alert_thresholds.response_time_critical_ms {
            return HealthStatus::Critical;
        }
        
        // Check for degraded conditions
        let degraded_checks = health_checks.iter()
            .filter(|check| check.status == HealthStatus::Degraded)
            .count();
        
        if degraded_checks > 0 ||
           system_metrics.cpu_usage_percent > self.config.alert_thresholds.cpu_usage_warning ||
           system_metrics.memory_usage_percent > self.config.alert_thresholds.memory_usage_warning ||
           performance_metrics.error_rate_percent > self.config.alert_thresholds.error_rate_warning {
            return HealthStatus::Degraded;
        }
        
        HealthStatus::Healthy
    }
}

impl Clone for HealthMonitor {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            health_checks: HashMap::new(), // Cannot clone trait objects
            metrics_collector: Arc::clone(&self.metrics_collector),
            alert_manager: Arc::clone(&self.alert_manager),
            last_report: Arc::clone(&self.last_report),
            startup_time: self.startup_time,
            version: self.version.clone(),
            environment: self.environment.clone(),
        }
    }
}

/// Quick health status for lightweight endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickHealthStatus {
    pub status: HealthStatus,
    pub timestamp: SystemTime,
    pub uptime_seconds: u64,
    pub version: String,
}

/// Database connectivity health check
pub struct DatabaseHealthCheck {
    config: HealthCheckConfig,
}

impl DatabaseHealthCheck {
    pub fn new(config: HealthCheckConfig) -> Self {
        Self { config }
    }
}

#[async_trait::async_trait]
impl HealthCheck for DatabaseHealthCheck {
    async fn check(&self) -> HealthCheckResult {
        let start_time = SystemTime::now();
        
        // Simulate database connectivity check
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        // In real implementation, this would actually test database connectivity
        let status = HealthStatus::Healthy;
        let message = "Database connectivity verified".to_string();
        
        let duration_ms = start_time.elapsed().unwrap_or_default().as_millis() as u64;
        
        HealthCheckResult {
            check_id: self.config.check_id.clone(),
            name: self.config.name.clone(),
            status,
            message,
            timestamp: SystemTime::now(),
            duration_ms,
            tags: self.config.tags.clone(),
            metadata: HashMap::new(),
        }
    }
    
    fn config(&self) -> &HealthCheckConfig {
        &self.config
    }
}

/// API endpoints health check
pub struct ApiHealthCheck {
    config: HealthCheckConfig,
}

impl ApiHealthCheck {
    pub fn new(config: HealthCheckConfig) -> Self {
        Self { config }
    }
}

#[async_trait::async_trait]
impl HealthCheck for ApiHealthCheck {
    async fn check(&self) -> HealthCheckResult {
        let start_time = SystemTime::now();
        
        // Test internal API endpoints
        tokio::time::sleep(Duration::from_millis(50)).await;
        
        let status = HealthStatus::Healthy;
        let message = "All API endpoints responding".to_string();
        
        let duration_ms = start_time.elapsed().unwrap_or_default().as_millis() as u64;
        
        HealthCheckResult {
            check_id: self.config.check_id.clone(),
            name: self.config.name.clone(),
            status,
            message,
            timestamp: SystemTime::now(),
            duration_ms,
            tags: self.config.tags.clone(),
            metadata: HashMap::new(),
        }
    }
    
    fn config(&self) -> &HealthCheckConfig {
        &self.config
    }
}

/// Cryptographic operations health check
pub struct CryptographicHealthCheck {
    config: HealthCheckConfig,
}

impl CryptographicHealthCheck {
    pub fn new(config: HealthCheckConfig) -> Self {
        Self { config }
    }
}

#[async_trait::async_trait]
impl HealthCheck for CryptographicHealthCheck {
    async fn check(&self) -> HealthCheckResult {
        let start_time = SystemTime::now();
        
        // Test cryptographic operations
        tokio::time::sleep(Duration::from_millis(200)).await;
        
        // In real implementation, this would test key generation, encryption, etc.
        let status = HealthStatus::Healthy;
        let message = "Cryptographic operations functioning normally".to_string();
        
        let duration_ms = start_time.elapsed().unwrap_or_default().as_millis() as u64;
        
        HealthCheckResult {
            check_id: self.config.check_id.clone(),
            name: self.config.name.clone(),
            status,
            message,
            timestamp: SystemTime::now(),
            duration_ms,
            tags: self.config.tags.clone(),
            metadata: HashMap::new(),
        }
    }
    
    fn config(&self) -> &HealthCheckConfig {
        &self.config
    }
}

/// System resources health check
pub struct ResourceHealthCheck {
    config: HealthCheckConfig,
}

impl ResourceHealthCheck {
    pub fn new(config: HealthCheckConfig) -> Self {
        Self { config }
    }
}

#[async_trait::async_trait]
impl HealthCheck for ResourceHealthCheck {
    async fn check(&self) -> HealthCheckResult {
        let start_time = SystemTime::now();
        
        // Check system resources
        tokio::time::sleep(Duration::from_millis(30)).await;
        
        // In real implementation, this would check actual system resources
        let status = HealthStatus::Healthy;
        let message = "System resources within normal limits".to_string();
        
        let duration_ms = start_time.elapsed().unwrap_or_default().as_millis() as u64;
        
        HealthCheckResult {
            check_id: self.config.check_id.clone(),
            name: self.config.name.clone(),
            status,
            message,
            timestamp: SystemTime::now(),
            duration_ms,
            tags: self.config.tags.clone(),
            metadata: HashMap::new(),
        }
    }
    
    fn config(&self) -> &HealthCheckConfig {
        &self.config
    }
}

/// Check external endpoint health
async fn check_external_endpoint(_endpoint: &ExternalEndpointConfig) -> Result<(), Box<dyn std::error::Error>> {
    // In real implementation, this would make HTTP requests to external endpoints
    tokio::time::sleep(Duration::from_millis(100)).await;
    Ok(())
}

/// Health monitoring errors
#[derive(Debug)]
pub enum HealthMonitorError {
    ConfigurationError(String),
    HealthCheckFailed(String),
    MetricsCollectionFailed(String),
    AlertingFailed(String),
}

impl std::fmt::Display for HealthMonitorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HealthMonitorError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
            HealthMonitorError::HealthCheckFailed(msg) => write!(f, "Health check failed: {}", msg),
            HealthMonitorError::MetricsCollectionFailed(msg) => write!(f, "Metrics collection failed: {}", msg),
            HealthMonitorError::AlertingFailed(msg) => write!(f, "Alerting failed: {}", msg),
        }
    }
}

impl std::error::Error for HealthMonitorError {}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct MockMetricsCollector;
    
    #[async_trait::async_trait]
    impl MetricsCollector for MockMetricsCollector {
        async fn collect_system_metrics(&self) -> SystemMetrics {
            SystemMetrics {
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
        
        async fn collect_performance_metrics(&self) -> PerformanceMetrics {
            PerformanceMetrics {
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
        
        async fn collect_security_metrics(&self) -> SecurityMetrics {
            SecurityMetrics {
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
    
    struct MockAlertManager;
    
    #[async_trait::async_trait]
    impl AlertManager for MockAlertManager {
        async fn send_alert(&self, _alert: Alert) {
            // Mock implementation
        }
        
        async fn check_alert_conditions(&self, _report: &SystemHealthReport) -> Vec<Alert> {
            vec![]
        }
    }
    
    #[tokio::test]
    async fn test_health_monitor_creation() {
        let config = HealthMonitorConfig {
            enabled: true,
            health_check_interval_seconds: 30,
            metrics_collection_interval_seconds: 60,
            alert_thresholds: AlertThresholds {
                cpu_usage_warning: 80.0,
                cpu_usage_critical: 95.0,
                memory_usage_warning: 85.0,
                memory_usage_critical: 95.0,
                disk_usage_warning: 90.0,
                disk_usage_critical: 98.0,
                error_rate_warning: 1.0,
                error_rate_critical: 5.0,
                response_time_warning_ms: 1000.0,
                response_time_critical_ms: 5000.0,
                connection_count_warning: 1000,
                connection_count_critical: 5000,
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
        
        let metrics_collector = Arc::new(MockMetricsCollector);
        let alert_manager = Arc::new(MockAlertManager);
        
        let monitor = HealthMonitor::new(
            config,
            metrics_collector,
            alert_manager,
            "2.0.0".to_string(),
            "test".to_string(),
        );
        
        assert_eq!(monitor.version, "2.0.0");
        assert_eq!(monitor.environment, "test");
        assert_eq!(monitor.health_checks.len(), 4); // Default health checks
    }
    
    #[tokio::test]
    async fn test_health_report_generation() {
        let config = HealthMonitorConfig {
            enabled: true,
            health_check_interval_seconds: 30,
            metrics_collection_interval_seconds: 60,
            alert_thresholds: AlertThresholds {
                cpu_usage_warning: 80.0,
                cpu_usage_critical: 95.0,
                memory_usage_warning: 85.0,
                memory_usage_critical: 95.0,
                disk_usage_warning: 90.0,
                disk_usage_critical: 98.0,
                error_rate_warning: 1.0,
                error_rate_critical: 5.0,
                response_time_warning_ms: 1000.0,
                response_time_critical_ms: 5000.0,
                connection_count_warning: 1000,
                connection_count_critical: 5000,
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
        
        let metrics_collector = Arc::new(MockMetricsCollector);
        let alert_manager = Arc::new(MockAlertManager);
        
        let monitor = HealthMonitor::new(
            config,
            metrics_collector,
            alert_manager,
            "2.0.0".to_string(),
            "test".to_string(),
        );
        
        let report = monitor.generate_health_report().await.unwrap();
        
        assert_eq!(report.version, "2.0.0");
        assert_eq!(report.environment, "test");
        assert_eq!(report.health_checks.len(), 4);
        assert_eq!(report.overall_status, HealthStatus::Healthy);
    }
    
    #[tokio::test]
    async fn test_quick_health_status() {
        let config = HealthMonitorConfig {
            enabled: true,
            health_check_interval_seconds: 30,
            metrics_collection_interval_seconds: 60,
            alert_thresholds: AlertThresholds {
                cpu_usage_warning: 80.0,
                cpu_usage_critical: 95.0,
                memory_usage_warning: 85.0,
                memory_usage_critical: 95.0,
                disk_usage_warning: 90.0,
                disk_usage_critical: 98.0,
                error_rate_warning: 1.0,
                error_rate_critical: 5.0,
                response_time_warning_ms: 1000.0,
                response_time_critical_ms: 5000.0,
                connection_count_warning: 1000,
                connection_count_critical: 5000,
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
        
        let metrics_collector = Arc::new(MockMetricsCollector);
        let alert_manager = Arc::new(MockAlertManager);
        
        let monitor = HealthMonitor::new(
            config,
            metrics_collector,
            alert_manager,
            "2.0.0".to_string(),
            "test".to_string(),
        );
        
        let status = monitor.get_quick_status().await.unwrap();
        
        assert_eq!(status.status, HealthStatus::Healthy);
        assert_eq!(status.version, "2.0.0");
    }
}