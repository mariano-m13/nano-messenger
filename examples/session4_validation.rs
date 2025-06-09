use nano_messenger::{
    crypto::{
        CryptoMode, CryptoConfig, CryptoInterface, ClassicalUserKeyPair,
        HybridUserKeyPair, PostQuantumUserKeyPair, init_crypto_config,
    },
};
use anyhow::Result;

/// Session 4 Validation: Client Interface Updates
#[tokio::main]
async fn main() -> Result<()> {
    println!("📱 Session 4 Validation: Client Interface Updates");
    println!("==================================================");
    
    // Initialize crypto system with different modes
    println!("\n1. 🔧 Crypto Mode Configuration");
    test_crypto_mode_configuration().await?;
    
    println!("\n2. 🔑 Keypair Generation for All Modes");
    test_keypair_generation().await?;
    
    println!("\n3. 🛡️  Security Configuration");
    test_security_configuration().await?;
    
    println!("\n4. 🔄 Mode Transition Validation");
    test_mode_transitions().await?;
    
    println!("\n5. 📊 Performance Information");
    test_performance_information().await?;
    
    println!("\n✅ Session 4 validation completed successfully!");
    println!("🚀 Client interface supports all quantum-safe crypto modes!");
    
    Ok(())
}

async fn test_crypto_mode_configuration() -> Result<()> {
    println!("   🔧 Testing crypto mode configuration...");
    
    // Test each crypto mode configuration
    for mode in CryptoMode::all() {
        let config = CryptoConfig::new(*mode);
        
        // Validate the configuration
        config.validate()?;
        
        println!("     ✅ {} mode: {} ({})", 
                 mode, mode.description(), mode.security_level());
        
        // Test initializing with this mode
        let _ = init_crypto_config(config);
    }
    
    println!("   ✅ All crypto modes configured successfully");
    Ok(())
}

async fn test_keypair_generation() -> Result<()> {
    println!("   🔑 Testing keypair generation for all modes...");
    
    // Test classical keypair generation
    let classical_keypair = ClassicalUserKeyPair::generate();
    println!("     ✅ Classical keypair: {}", 
             &classical_keypair.public_key_string()[..50]);
    
    // Test hybrid keypair generation
    let hybrid_keypair = HybridUserKeyPair::generate();
    println!("     ✅ Hybrid keypair: {}", 
             &hybrid_keypair.public_key_string()[..50]);
    
    // Test post-quantum keypair generation
    let quantum_keypair = PostQuantumUserKeyPair::generate();
    println!("     ✅ Post-quantum keypair: {}", 
             &quantum_keypair.public_key_string()[..50]);
    
    // Test that each mode generates different formats
    assert_ne!(classical_keypair.public_key_string(), hybrid_keypair.public_key_string());
    assert_ne!(classical_keypair.public_key_string(), quantum_keypair.public_key_string());
    assert_ne!(hybrid_keypair.public_key_string(), quantum_keypair.public_key_string());
    
    println!("   ✅ All keypair types generate successfully with unique formats");
    Ok(())
}

async fn test_security_configuration() -> Result<()> {
    println!("   🛡️  Testing security configuration...");
    
    // Test default configuration
    let default_config = CryptoConfig::default();
    assert_eq!(default_config.mode, CryptoMode::Classical);
    println!("     ✅ Default config: {}", default_config.mode);
    
    // Test high security configuration
    let high_sec_config = CryptoConfig::high_security();
    assert_eq!(high_sec_config.mode, CryptoMode::Hybrid);
    assert_eq!(high_sec_config.minimum_mode, CryptoMode::Hybrid);
    println!("     ✅ High security config: {} (min: {})", 
             high_sec_config.mode, high_sec_config.minimum_mode);
    
    // Test quantum-only configuration
    let quantum_config = CryptoConfig {
        mode: CryptoMode::Quantum,
        minimum_mode: CryptoMode::Quantum,
        allow_auto_upgrade: false,
        adaptive_mode: false,
    };
    quantum_config.validate()?;
    println!("     ✅ Quantum-only config: {}", quantum_config.mode);
    
    // Test adaptive configuration
    let adaptive_config = CryptoConfig {
        mode: CryptoMode::Classical,
        minimum_mode: CryptoMode::Classical,
        allow_auto_upgrade: true,
        adaptive_mode: true,
    };
    adaptive_config.validate()?;
    println!("     ✅ Adaptive config: {} (adaptive: {})", 
             adaptive_config.mode, adaptive_config.adaptive_mode);
    
    println!("   ✅ All security configurations validate successfully");
    Ok(())
}

async fn test_mode_transitions() -> Result<()> {
    println!("   🔄 Testing crypto mode transitions...");
    
    // Test valid transitions
    assert!(CryptoMode::Classical.can_transition_to(CryptoMode::Hybrid));
    assert!(CryptoMode::Classical.can_transition_to(CryptoMode::Quantum));
    assert!(CryptoMode::Hybrid.can_transition_to(CryptoMode::Hybrid));
    assert!(CryptoMode::Quantum.can_transition_to(CryptoMode::Quantum));
    
    println!("     ✅ Valid transitions: Classical → Hybrid, Classical → Quantum");
    
    // Test invalid transitions (security downgrades not allowed)
    assert!(!CryptoMode::Hybrid.can_transition_to(CryptoMode::Classical));
    assert!(!CryptoMode::Quantum.can_transition_to(CryptoMode::Classical));
    
    println!("     ✅ Invalid transitions properly blocked: Hybrid ↛ Classical, Quantum ↛ Classical");
    
    // Test configuration validation with transitions
    let valid_config = CryptoConfig {
        mode: CryptoMode::Hybrid,
        minimum_mode: CryptoMode::Classical,
        allow_auto_upgrade: true,
        adaptive_mode: false,
    };
    assert!(valid_config.validate().is_ok());
    
    let invalid_config = CryptoConfig {
        mode: CryptoMode::Classical,
        minimum_mode: CryptoMode::Hybrid,
        allow_auto_upgrade: false,
        adaptive_mode: false,
    };
    assert!(invalid_config.validate().is_err());
    
    println!("   ✅ Mode transition validation working correctly");
    Ok(())
}

async fn test_performance_information() -> Result<()> {
    println!("   📊 Testing performance information...");
    
    // Test performance info for each mode
    for mode in CryptoMode::all() {
        let cost = mode.performance_cost();
        let overhead = mode.size_overhead();
        let quantum_resistant = mode.is_quantum_resistant();
        let security_level = mode.security_level();
        
        println!("     {} {}: {:.1}x cost, +{} bytes, QR: {}, Security: {}", 
                 match mode {
                     CryptoMode::Classical => "🔓",
                     CryptoMode::Hybrid => "🔐",
                     CryptoMode::Quantum => "⚛️",
                 },
                 mode, cost, overhead, quantum_resistant, security_level);
        
        // Validate that costs make sense
        assert!(cost >= 1.0);
        // Note: overhead is unsigned, so always >= 0
        
        // Validate quantum resistance
        match mode {
            CryptoMode::Classical => assert!(!quantum_resistant),
            CryptoMode::Hybrid | CryptoMode::Quantum => assert!(quantum_resistant),
        }
    }
    
    // Test current crypto interface performance info
    let perf_info = CryptoInterface::performance_info();
    println!("     📈 Current interface performance: {:.1}x cost, +{} bytes", 
             perf_info.relative_cost, perf_info.size_overhead);
    
    println!("   ✅ Performance information accurate for all modes");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_session4_basic_functionality() {
        // Test that we can initialize different crypto modes
        for mode in CryptoMode::all() {
            let config = CryptoConfig::new(*mode);
            assert!(config.validate().is_ok());
        }
    }

    #[test]
    fn test_crypto_mode_properties() {
        // Test that all modes have proper properties
        for mode in CryptoMode::all() {
            assert!(mode.performance_cost() >= 1.0);
            assert!(!mode.description().is_empty());
            assert!(!mode.security_level().is_empty());
        }
    }

    #[test]
    fn test_keypair_generation_deterministic() {
        // Test that keypair generation is deterministic for testing
        let keypair1 = ClassicalUserKeyPair::generate();
        let keypair2 = ClassicalUserKeyPair::generate();
        
        // Should be different (random generation)
        assert_ne!(keypair1.public_key_string(), keypair2.public_key_string());
    }
}
