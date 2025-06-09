use serde::{Deserialize, Serialize};
use std::str::FromStr;
use crate::error::{NanoError, Result};

/// Available cryptographic modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CryptoMode {
    /// Classical cryptography (X25519 + Ed25519 + ChaCha20Poly1305)
    Classical,
    /// Hybrid mode (Classical + Post-Quantum for forward security)
    Hybrid,
    /// Pure post-quantum cryptography
    Quantum,
}

impl Default for CryptoMode {
    fn default() -> Self {
        CryptoMode::Classical
    }
}

impl FromStr for CryptoMode {
    type Err = NanoError;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "classical" | "classic" => Ok(CryptoMode::Classical),
            "hybrid" => Ok(CryptoMode::Hybrid),
            "quantum" | "postquantum" | "post-quantum" | "pq" => Ok(CryptoMode::Quantum),
            _ => Err(NanoError::Crypto(format!(
                "Invalid crypto mode: {}. Valid options: classical, hybrid, quantum",
                s
            ))),
        }
    }
}

impl std::fmt::Display for CryptoMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoMode::Classical => write!(f, "classical"),
            CryptoMode::Hybrid => write!(f, "hybrid"),
            CryptoMode::Quantum => write!(f, "quantum"),
        }
    }
}

impl CryptoMode {
    /// Returns all available crypto modes
    pub fn all() -> &'static [CryptoMode] {
        &[CryptoMode::Classical, CryptoMode::Hybrid, CryptoMode::Quantum]
    }

    /// Returns a human-readable description of the crypto mode
    pub fn description(&self) -> &'static str {
        match self {
            CryptoMode::Classical => {
                "Classical cryptography using X25519 key exchange, Ed25519 signatures, and ChaCha20Poly1305 encryption"
            }
            CryptoMode::Hybrid => {
                "Hybrid security combining classical algorithms with post-quantum ML-KEM-768 and ML-DSA"
            }
            CryptoMode::Quantum => {
                "Pure post-quantum cryptography using ML-KEM-768 and ML-DSA algorithms"
            }
        }
    }

    /// Returns the numeric security level (higher is more secure)
    pub fn security_level(&self) -> u8 {
        match self {
            CryptoMode::Classical => 1,  // Generation 1: Classical cryptography
            CryptoMode::Hybrid => 2,     // Generation 2: Transitional (classical + PQ)
            CryptoMode::Quantum => 3,    // Generation 3: Pure post-quantum
        }
    }

    /// Returns the security level description
    pub fn security_description(&self) -> &'static str {
        match self {
            CryptoMode::Classical => "Strong against classical attacks, vulnerable to quantum attacks",
            CryptoMode::Hybrid => "Strong against both classical and quantum attacks (best security)",
            CryptoMode::Quantum => "Strong against quantum attacks, standard classical security",
        }
    }

    /// Returns whether this mode is quantum-resistant
    pub fn is_quantum_resistant(&self) -> bool {
        matches!(self, CryptoMode::Hybrid | CryptoMode::Quantum)
    }

    /// Returns the relative performance cost (1.0 = classical baseline)
    pub fn performance_cost(&self) -> f32 {
        match self {
            CryptoMode::Classical => 1.0,
            CryptoMode::Hybrid => 1.8,    // Hybrid is slower due to dual operations
            CryptoMode::Quantum => 1.4,   // Pure PQ is faster than hybrid but slower than classical
        }
    }

    /// Returns the approximate message size overhead in bytes
    pub fn size_overhead(&self) -> usize {
        match self {
            CryptoMode::Classical => 0,     // Baseline
            CryptoMode::Hybrid => 2048,     // Additional PQ key material
            CryptoMode::Quantum => 1536,    // PQ-only overhead
        }
    }

    /// Check if transition from one mode to another is safe
    pub fn can_transition_to(&self, other: CryptoMode) -> bool {
        match (self, other) {
            // Same mode is always ok
            (CryptoMode::Classical, CryptoMode::Classical) => true,
            (CryptoMode::Hybrid, CryptoMode::Hybrid) => true,
            (CryptoMode::Quantum, CryptoMode::Quantum) => true,
            // Always allow upgrading to higher security levels
            (CryptoMode::Classical, CryptoMode::Hybrid) => true,
            (CryptoMode::Classical, CryptoMode::Quantum) => true,
            (CryptoMode::Hybrid, CryptoMode::Quantum) => true,
            // Don't allow downgrades (going to lower security levels)
            (CryptoMode::Hybrid, CryptoMode::Classical) => false,
            (CryptoMode::Quantum, CryptoMode::Classical) => false,
            (CryptoMode::Quantum, CryptoMode::Hybrid) => false,
        }
    }

    /// Get the recommended mode based on threat model
    pub fn recommended_for_threat_model(quantum_threat: bool, performance_critical: bool) -> CryptoMode {
        match (quantum_threat, performance_critical) {
            (true, true) => CryptoMode::Quantum,   // Quantum threat + need performance
            (true, false) => CryptoMode::Hybrid,   // Quantum threat + max security
            (false, _) => CryptoMode::Classical,   // No quantum threat
        }
    }
}

/// Global configuration for cryptography
#[derive(Debug, Clone, Copy)]
pub struct CryptoConfig {
    /// Current cryptographic mode
    pub mode: CryptoMode,
    /// Whether to allow automatic upgrades for security
    pub allow_auto_upgrade: bool,
    /// Whether to adapt crypto mode based on network conditions
    pub adaptive_mode: bool,
    /// Minimum acceptable crypto mode for incoming messages
    pub minimum_mode: CryptoMode,
}

impl Default for CryptoConfig {
    fn default() -> Self {
        Self {
            mode: CryptoMode::Classical,
            allow_auto_upgrade: true,
            adaptive_mode: false,
            minimum_mode: CryptoMode::Classical,
        }
    }
}

impl CryptoConfig {
    /// Create a new configuration with the specified mode
    pub fn new(mode: CryptoMode) -> Self {
        Self {
            mode,
            ..Default::default()
        }
    }

    /// Create a configuration optimized for high security
    pub fn high_security() -> Self {
        Self {
            mode: CryptoMode::Hybrid,
            allow_auto_upgrade: true,
            adaptive_mode: false,
            minimum_mode: CryptoMode::Hybrid,
        }
    }

    /// Create a configuration optimized for performance
    pub fn performance_optimized() -> Self {
        Self {
            mode: CryptoMode::Classical,
            allow_auto_upgrade: false,
            adaptive_mode: true,
            minimum_mode: CryptoMode::Classical,
        }
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        if !self.minimum_mode.can_transition_to(self.mode) {
            return Err(NanoError::Crypto(format!(
                "Invalid config: current mode {} is weaker than minimum mode {}",
                self.mode, self.minimum_mode
            )));
        }
        Ok(())
    }

    /// Check if an incoming message with the given mode is acceptable
    pub fn accepts_mode(&self, incoming_mode: CryptoMode) -> bool {
        self.minimum_mode.can_transition_to(incoming_mode) || incoming_mode == self.minimum_mode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto_mode_parsing() {
        assert_eq!("classical".parse::<CryptoMode>().unwrap(), CryptoMode::Classical);
        assert_eq!("hybrid".parse::<CryptoMode>().unwrap(), CryptoMode::Hybrid);
        assert_eq!("quantum".parse::<CryptoMode>().unwrap(), CryptoMode::Quantum);
        assert_eq!("pq".parse::<CryptoMode>().unwrap(), CryptoMode::Quantum);
        
        assert!("invalid".parse::<CryptoMode>().is_err());
    }

    #[test]
    fn test_mode_transitions() {
        // Classical can upgrade to anything
        assert!(CryptoMode::Classical.can_transition_to(CryptoMode::Hybrid));
        assert!(CryptoMode::Classical.can_transition_to(CryptoMode::Quantum));
        
        // Hybrid can upgrade to quantum
        assert!(CryptoMode::Hybrid.can_transition_to(CryptoMode::Quantum));
        
        // Quantum modes cannot downgrade
        assert!(!CryptoMode::Hybrid.can_transition_to(CryptoMode::Classical));
        assert!(!CryptoMode::Quantum.can_transition_to(CryptoMode::Classical));
        assert!(!CryptoMode::Quantum.can_transition_to(CryptoMode::Hybrid));
        
        // Same mode is ok
        assert!(CryptoMode::Hybrid.can_transition_to(CryptoMode::Hybrid));
    }

    #[test]
    fn test_config_validation() {
        let valid_config = CryptoConfig {
            mode: CryptoMode::Hybrid,
            minimum_mode: CryptoMode::Classical,
            ..Default::default()
        };
        assert!(valid_config.validate().is_ok());

        let invalid_config = CryptoConfig {
            mode: CryptoMode::Classical,
            minimum_mode: CryptoMode::Hybrid,
            ..Default::default()
        };
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_recommended_modes() {
        assert_eq!(
            CryptoMode::recommended_for_threat_model(false, true),
            CryptoMode::Classical
        );
        assert_eq!(
            CryptoMode::recommended_for_threat_model(true, true),
            CryptoMode::Quantum
        );
        assert_eq!(
            CryptoMode::recommended_for_threat_model(true, false),
            CryptoMode::Hybrid
        );
    }
}
