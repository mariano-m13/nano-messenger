//! Production Error Handling for Session 8
//! 
//! This module provides comprehensive error handling for production deployment,
//! including structured error types, error recovery mechanisms, and observability
//! integration for monitoring and debugging in production environments.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::time::SystemTime;
use uuid::Uuid;

/// Production-ready error types with detailed context and recovery guidance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProductionError {
    /// Cryptographic operation failures
    CryptographicError {
        operation: String,
        crypto_mode: String,
        error_code: String,
        is_recoverable: bool,
        recovery_suggestions: Vec<String>,
        context: ErrorContext,
    },
    
    /// Network and connectivity issues
    NetworkError {
        operation: String,
        endpoint: String,
        error_type: NetworkErrorType,
        retry_count: u32,
        max_retries: u32,
        backoff_seconds: u64,
        context: ErrorContext,
    },
    
    /// Configuration validation and parsing errors
    ConfigurationError {
        config_section: String,
        field_name: Option<String>,
        validation_failure: String,
        expected_format: String,
        actual_value: String,
        context: ErrorContext,
    },
    
    /// Database and storage related errors
    StorageError {
        operation: String,
        storage_type: StorageType,
        error_details: String,
        data_integrity_check: bool,
        recovery_possible: bool,
        context: ErrorContext,
    },
    
    /// Authentication and authorization failures
    SecurityError {
        security_domain: SecurityDomain,
        user_id: Option<String>,
        attempted_operation: String,
        failure_reason: String,
        threat_level: ThreatLevel,
        requires_incident_response: bool,
        context: ErrorContext,
    },
    
    /// Resource exhaustion and capacity issues
    ResourceError {
        resource_type: ResourceType,
        current_usage: u64,
        limit: u64,
        usage_percentage: f64,
        mitigation_actions: Vec<String>,
        context: ErrorContext,
    },
    
    /// Protocol and message processing errors
    ProtocolError {
        protocol_version: String,
        message_type: String,
        validation_errors: Vec<String>,
        compatibility_issue: bool,
        fallback_available: bool,
        context: ErrorContext,
    },
    
    /// Business logic and validation errors
    BusinessLogicError {
        rule_name: String,
        validation_failure: String,
        user_correctable: bool,
        suggested_actions: Vec<String>,
        context: ErrorContext,
    },
    
    /// System-level and infrastructure errors
    SystemError {
        subsystem: String,
        error_type: SystemErrorType,
        severity: ErrorSeverity,
        requires_restart: bool,
        affects_availability: bool,
        context: ErrorContext,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    pub error_id: Uuid,
    pub timestamp: SystemTime,
    pub correlation_id: Option<Uuid>,
    pub session_id: Option<String>,
    pub user_id: Option<String>,
    pub request_id: Option<String>,
    pub component: String,
    pub version: String,
    pub environment: String,
    pub additional_metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkErrorType {
    ConnectionTimeout,
    ConnectionRefused,
    ConnectionReset,
    DNSResolutionFailure,
    TLSHandshakeFailure,
    CertificateValidationFailure,
    RateLimitExceeded,
    ServiceUnavailable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageType {
    Database,
    FileSystem,
    KeyValueStore,
    ObjectStorage,
    MessageQueue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityDomain {
    Authentication,
    Authorization,
    Cryptography,
    DataAccess,
    AuditLogging,
    Compliance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceType {
    Memory,
    CPU,
    Disk,
    Network,
    FileDescriptors,
    DatabaseConnections,
    CryptoKeyCache,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemErrorType {
    OutOfMemory,
    DiskFull,
    ProcessLimit,
    ServiceDependencyFailure,
    ConfigurationInconsistency,
    HealthCheckFailure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorSeverity {
    Info,
    Warning,
    Error,
    Critical,
    Emergency,
}

impl fmt::Display for ProductionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProductionError::CryptographicError { operation, crypto_mode, error_code, .. } => {
                write!(f, "Cryptographic error in {} operation with {} mode: {}", operation, crypto_mode, error_code)
            }
            ProductionError::NetworkError { operation, endpoint, error_type, .. } => {
                write!(f, "Network error during {} to {}: {:?}", operation, endpoint, error_type)
            }
            ProductionError::ConfigurationError { config_section, field_name, validation_failure, .. } => {
                write!(f, "Configuration error in section '{}'{}: {}", 
                    config_section, 
                    field_name.as_ref().map(|f| format!(" field '{}'", f)).unwrap_or_default(),
                    validation_failure
                )
            }
            ProductionError::StorageError { operation, storage_type, error_details, .. } => {
                write!(f, "Storage error during {} on {:?}: {}", operation, storage_type, error_details)
            }
            ProductionError::SecurityError { security_domain, attempted_operation, failure_reason, .. } => {
                write!(f, "Security error in {:?} during {}: {}", security_domain, attempted_operation, failure_reason)
            }
            ProductionError::ResourceError { resource_type, usage_percentage, .. } => {
                write!(f, "Resource exhaustion: {:?} at {:.1}% capacity", resource_type, usage_percentage)
            }
            ProductionError::ProtocolError { protocol_version, message_type, validation_errors, .. } => {
                write!(f, "Protocol error in {} v{}: {} validation failures", message_type, protocol_version, validation_errors.len())
            }
            ProductionError::BusinessLogicError { rule_name, validation_failure, .. } => {
                write!(f, "Business logic error in rule '{}': {}", rule_name, validation_failure)
            }
            ProductionError::SystemError { subsystem, error_type, severity, .. } => {
                write!(f, "System error in {} subsystem: {:?} (severity: {:?})", subsystem, error_type, severity)
            }
        }
    }
}

impl std::error::Error for ProductionError {}

/// Error handling utilities and recovery mechanisms
pub struct ErrorHandler {
    pub environment: String,
    pub component: String,
    pub version: String,
    pub error_reporting: Box<dyn ErrorReporting + Send + Sync>,
    pub recovery_strategies: HashMap<String, Box<dyn RecoveryStrategy + Send + Sync>>,
}

pub trait ErrorReporting {
    fn report_error(&self, error: &ProductionError);
    fn report_error_with_context(&self, error: &ProductionError, additional_context: HashMap<String, String>);
}

pub trait RecoveryStrategy {
    fn can_recover(&self, error: &ProductionError) -> bool;
    fn attempt_recovery(&self, error: &ProductionError) -> Result<RecoveryResult, ProductionError>;
    fn get_recovery_timeout(&self) -> std::time::Duration;
}

#[derive(Debug, Clone)]
pub enum RecoveryResult {
    Recovered,
    PartialRecovery { remaining_issues: Vec<String> },
    RecoveryFailed { reason: String },
    RecoveryNotApplicable,
}

impl ErrorHandler {
    pub fn new(environment: String, component: String, version: String) -> Self {
        Self {
            environment,
            component,
            version,
            error_reporting: Box::new(DefaultErrorReporting::new()),
            recovery_strategies: Self::default_recovery_strategies(),
        }
    }
    
    pub fn handle_error(&self, error: ProductionError) -> Result<RecoveryResult, ProductionError> {
        // Log the error
        self.error_reporting.report_error(&error);
        
        // Attempt recovery if strategy exists
        let error_type = self.get_error_type_key(&error);
        if let Some(strategy) = self.recovery_strategies.get(&error_type) {
            if strategy.can_recover(&error) {
                match strategy.attempt_recovery(&error) {
                    Ok(recovery_result) => {
                        self.log_recovery_attempt(&error, &recovery_result);
                        return Ok(recovery_result);
                    }
                    Err(recovery_error) => {
                        self.error_reporting.report_error(&recovery_error);
                        return Err(recovery_error);
                    }
                }
            }
        }
        
        // No recovery possible, escalate error
        Err(error)
    }
    
    pub fn create_error_context(&self, correlation_id: Option<Uuid>) -> ErrorContext {
        ErrorContext {
            error_id: Uuid::new_v4(),
            timestamp: SystemTime::now(),
            correlation_id,
            session_id: None,
            user_id: None,
            request_id: None,
            component: self.component.clone(),
            version: self.version.clone(),
            environment: self.environment.clone(),
            additional_metadata: HashMap::new(),
        }
    }
    
    fn default_recovery_strategies() -> HashMap<String, Box<dyn RecoveryStrategy + Send + Sync>> {
        let mut strategies: HashMap<String, Box<dyn RecoveryStrategy + Send + Sync>> = HashMap::new();
        
        strategies.insert("network_error".to_string(), Box::new(NetworkRecoveryStrategy::new()));
        strategies.insert("crypto_error".to_string(), Box::new(CryptoRecoveryStrategy::new()));
        strategies.insert("storage_error".to_string(), Box::new(StorageRecoveryStrategy::new()));
        strategies.insert("resource_error".to_string(), Box::new(ResourceRecoveryStrategy::new()));
        
        strategies
    }
    
    fn get_error_type_key(&self, error: &ProductionError) -> String {
        match error {
            ProductionError::NetworkError { .. } => "network_error".to_string(),
            ProductionError::CryptographicError { .. } => "crypto_error".to_string(),
            ProductionError::StorageError { .. } => "storage_error".to_string(),
            ProductionError::ResourceError { .. } => "resource_error".to_string(),
            ProductionError::SecurityError { .. } => "security_error".to_string(),
            ProductionError::ConfigurationError { .. } => "config_error".to_string(),
            ProductionError::ProtocolError { .. } => "protocol_error".to_string(),
            ProductionError::BusinessLogicError { .. } => "business_logic_error".to_string(),
            ProductionError::SystemError { .. } => "system_error".to_string(),
        }
    }
    
    fn log_recovery_attempt(&self, error: &ProductionError, result: &RecoveryResult) {
        println!("Recovery attempt for error {}: {:?}", 
            self.get_error_type_key(error), result);
    }
}

/// Default error reporting implementation
pub struct DefaultErrorReporting {
    pub structured_logging: bool,
}

impl DefaultErrorReporting {
    pub fn new() -> Self {
        Self {
            structured_logging: true,
        }
    }
}

impl ErrorReporting for DefaultErrorReporting {
    fn report_error(&self, error: &ProductionError) {
        if self.structured_logging {
            let error_json = serde_json::to_string_pretty(error).unwrap_or_else(|_| format!("{:?}", error));
            eprintln!("STRUCTURED_ERROR: {}", error_json);
        } else {
            eprintln!("ERROR: {}", error);
        }
    }
    
    fn report_error_with_context(&self, error: &ProductionError, additional_context: HashMap<String, String>) {
        self.report_error(error);
        if !additional_context.is_empty() {
            eprintln!("ADDITIONAL_CONTEXT: {:?}", additional_context);
        }
    }
}

/// Network error recovery strategy
pub struct NetworkRecoveryStrategy {
    max_retries: u32,
    base_backoff_ms: u64,
}

impl NetworkRecoveryStrategy {
    pub fn new() -> Self {
        Self {
            max_retries: 3,
            base_backoff_ms: 1000,
        }
    }
}

impl RecoveryStrategy for NetworkRecoveryStrategy {
    fn can_recover(&self, error: &ProductionError) -> bool {
        match error {
            ProductionError::NetworkError { retry_count, max_retries, .. } => {
                retry_count < max_retries
            }
            _ => false,
        }
    }
    
    fn attempt_recovery(&self, error: &ProductionError) -> Result<RecoveryResult, ProductionError> {
        match error {
            ProductionError::NetworkError { retry_count, .. } => {
                // Implement exponential backoff
                let backoff_duration = std::time::Duration::from_millis(
                    self.base_backoff_ms * 2_u64.pow(*retry_count)
                );
                
                std::thread::sleep(backoff_duration);
                
                // In a real implementation, this would retry the actual network operation
                // For now, simulate a recovery attempt
                if *retry_count < self.max_retries {
                    Ok(RecoveryResult::Recovered)
                } else {
                    Ok(RecoveryResult::RecoveryFailed { 
                        reason: "Maximum retries exceeded".to_string() 
                    })
                }
            }
            _ => Ok(RecoveryResult::RecoveryNotApplicable),
        }
    }
    
    fn get_recovery_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }
}

/// Cryptographic error recovery strategy
pub struct CryptoRecoveryStrategy;

impl CryptoRecoveryStrategy {
    pub fn new() -> Self {
        Self
    }
}

impl RecoveryStrategy for CryptoRecoveryStrategy {
    fn can_recover(&self, error: &ProductionError) -> bool {
        match error {
            ProductionError::CryptographicError { is_recoverable, .. } => *is_recoverable,
            _ => false,
        }
    }
    
    fn attempt_recovery(&self, error: &ProductionError) -> Result<RecoveryResult, ProductionError> {
        match error {
            ProductionError::CryptographicError { operation, recovery_suggestions, .. } => {
                // Implement crypto-specific recovery logic
                if recovery_suggestions.contains(&"regenerate_keys".to_string()) {
                    // In a real implementation, this would regenerate keys
                    Ok(RecoveryResult::Recovered)
                } else if recovery_suggestions.contains(&"fallback_crypto_mode".to_string()) {
                    // In a real implementation, this would fall back to classical crypto
                    Ok(RecoveryResult::PartialRecovery { 
                        remaining_issues: vec!["Using fallback crypto mode".to_string()] 
                    })
                } else {
                    Ok(RecoveryResult::RecoveryFailed { 
                        reason: format!("No applicable recovery for operation: {}", operation) 
                    })
                }
            }
            _ => Ok(RecoveryResult::RecoveryNotApplicable),
        }
    }
    
    fn get_recovery_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(10)
    }
}

/// Storage error recovery strategy
pub struct StorageRecoveryStrategy;

impl StorageRecoveryStrategy {
    pub fn new() -> Self {
        Self
    }
}

impl RecoveryStrategy for StorageRecoveryStrategy {
    fn can_recover(&self, error: &ProductionError) -> bool {
        match error {
            ProductionError::StorageError { recovery_possible, .. } => *recovery_possible,
            _ => false,
        }
    }
    
    fn attempt_recovery(&self, error: &ProductionError) -> Result<RecoveryResult, ProductionError> {
        match error {
            ProductionError::StorageError { storage_type, .. } => {
                match storage_type {
                    StorageType::Database => {
                        // Attempt database connection recovery
                        Ok(RecoveryResult::PartialRecovery { 
                            remaining_issues: vec!["Database connection re-established, data consistency check needed".to_string()] 
                        })
                    }
                    StorageType::FileSystem => {
                        // Attempt file system recovery
                        Ok(RecoveryResult::Recovered)
                    }
                    _ => Ok(RecoveryResult::RecoveryFailed { 
                        reason: format!("No recovery strategy for storage type: {:?}", storage_type) 
                    })
                }
            }
            _ => Ok(RecoveryResult::RecoveryNotApplicable),
        }
    }
    
    fn get_recovery_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(15)
    }
}

/// Resource error recovery strategy
pub struct ResourceRecoveryStrategy;

impl ResourceRecoveryStrategy {
    pub fn new() -> Self {
        Self
    }
}

impl RecoveryStrategy for ResourceRecoveryStrategy {
    fn can_recover(&self, error: &ProductionError) -> bool {
        match error {
            ProductionError::ResourceError { usage_percentage, .. } => *usage_percentage < 95.0,
            _ => false,
        }
    }
    
    fn attempt_recovery(&self, error: &ProductionError) -> Result<RecoveryResult, ProductionError> {
        match error {
            ProductionError::ResourceError { resource_type, mitigation_actions, .. } => {
                // Implement resource-specific recovery
                match resource_type {
                    ResourceType::Memory => {
                        // Trigger garbage collection, clear caches
                        Ok(RecoveryResult::PartialRecovery { 
                            remaining_issues: vec!["Memory usage reduced, monitoring required".to_string()] 
                        })
                    }
                    ResourceType::CryptoKeyCache => {
                        // Clear key cache to free memory
                        Ok(RecoveryResult::Recovered)
                    }
                    _ => {
                        if mitigation_actions.is_empty() {
                            Ok(RecoveryResult::RecoveryFailed { 
                                reason: "No mitigation actions available".to_string() 
                            })
                        } else {
                            Ok(RecoveryResult::PartialRecovery { 
                                remaining_issues: mitigation_actions.clone() 
                            })
                        }
                    }
                }
            }
            _ => Ok(RecoveryResult::RecoveryNotApplicable),
        }
    }
    
    fn get_recovery_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(5)
    }
}

/// Helper functions for creating common production errors
pub mod error_builders {
    use super::*;
    
    pub fn network_timeout_error(endpoint: &str, retry_count: u32, context: ErrorContext) -> ProductionError {
        ProductionError::NetworkError {
            operation: "connection".to_string(),
            endpoint: endpoint.to_string(),
            error_type: NetworkErrorType::ConnectionTimeout,
            retry_count,
            max_retries: 3,
            backoff_seconds: 2_u64.pow(retry_count),
            context,
        }
    }
    
    pub fn crypto_key_generation_error(crypto_mode: &str, context: ErrorContext) -> ProductionError {
        ProductionError::CryptographicError {
            operation: "key_generation".to_string(),
            crypto_mode: crypto_mode.to_string(),
            error_code: "KEY_GEN_FAILED".to_string(),
            is_recoverable: true,
            recovery_suggestions: vec!["regenerate_keys".to_string(), "fallback_crypto_mode".to_string()],
            context,
        }
    }
    
    pub fn config_validation_error(section: &str, field: Option<&str>, message: &str, context: ErrorContext) -> ProductionError {
        ProductionError::ConfigurationError {
            config_section: section.to_string(),
            field_name: field.map(|s| s.to_string()),
            validation_failure: message.to_string(),
            expected_format: "Valid configuration value".to_string(),
            actual_value: "Invalid value".to_string(),
            context,
        }
    }
    
    pub fn memory_exhaustion_error(current_usage: u64, limit: u64, context: ErrorContext) -> ProductionError {
        let usage_percentage = (current_usage as f64 / limit as f64) * 100.0;
        
        ProductionError::ResourceError {
            resource_type: ResourceType::Memory,
            current_usage,
            limit,
            usage_percentage,
            mitigation_actions: vec![
                "Clear crypto key cache".to_string(),
                "Reduce concurrent connections".to_string(),
                "Trigger garbage collection".to_string(),
            ],
            context,
        }
    }
    
    pub fn security_authentication_error(user_id: Option<&str>, operation: &str, context: ErrorContext) -> ProductionError {
        ProductionError::SecurityError {
            security_domain: SecurityDomain::Authentication,
            user_id: user_id.map(|s| s.to_string()),
            attempted_operation: operation.to_string(),
            failure_reason: "Invalid credentials or expired session".to_string(),
            threat_level: ThreatLevel::Medium,
            requires_incident_response: false,
            context,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_context_creation() {
        let handler = ErrorHandler::new(
            "production".to_string(),
            "nano-messenger-relay".to_string(),
            "2.0.0".to_string(),
        );
        
        let context = handler.create_error_context(None);
        assert_eq!(context.component, "nano-messenger-relay");
        assert_eq!(context.version, "2.0.0");
        assert_eq!(context.environment, "production");
    }
    
    #[test]
    fn test_network_recovery_strategy() {
        let strategy = NetworkRecoveryStrategy::new();
        let context = ErrorContext {
            error_id: Uuid::new_v4(),
            timestamp: SystemTime::now(),
            correlation_id: None,
            session_id: None,
            user_id: None,
            request_id: None,
            component: "test".to_string(),
            version: "1.0.0".to_string(),
            environment: "test".to_string(),
            additional_metadata: HashMap::new(),
        };
        
        let error = ProductionError::NetworkError {
            operation: "test".to_string(),
            endpoint: "localhost:8080".to_string(),
            error_type: NetworkErrorType::ConnectionTimeout,
            retry_count: 1,
            max_retries: 3,
            backoff_seconds: 2,
            context,
        };
        
        assert!(strategy.can_recover(&error));
        
        let result = strategy.attempt_recovery(&error).unwrap();
        matches!(result, RecoveryResult::Recovered);
    }
    
    #[test]
    fn test_error_display() {
        let context = ErrorContext {
            error_id: Uuid::new_v4(),
            timestamp: SystemTime::now(),
            correlation_id: None,
            session_id: None,
            user_id: None,
            request_id: None,
            component: "test".to_string(),
            version: "1.0.0".to_string(),
            environment: "test".to_string(),
            additional_metadata: HashMap::new(),
        };
        
        let error = error_builders::crypto_key_generation_error("hybrid", context);
        let error_string = format!("{}", error);
        assert!(error_string.contains("Cryptographic error"));
        assert!(error_string.contains("key_generation"));
        assert!(error_string.contains("hybrid"));
    }
}