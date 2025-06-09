use nano_messenger::{
    crypto::{CryptoMode, UserKeyPair},
    protocol::{MessageEnvelope, QuantumSafeEnvelope},
    network::RelayClient,
};
use tokio;
use anyhow::Result;

/// Session 5 Validation: Test relay crypto policy enforcement
#[tokio::main]
async fn main() -> Result<()> {
    println!("🛡️  Session 5 Validation: Relay Crypto Policy Enforcement");
    println!("============================================================");
    
    // Test configurations for different relay policies
    let test_configs = vec![
        ("Permissive Relay", "127.0.0.1:7733", CryptoMode::Classical),
        ("Security-Focused Relay", "127.0.0.1:7734", CryptoMode::Hybrid),
        ("High-Security Relay", "127.0.0.1:7735", CryptoMode::Quantum),
    ];
    
    // Generate test keypairs
    let alice_keypair = UserKeyPair::generate();
    let bob_keypair = UserKeyPair::generate();
    
    println!("\n🔑 Generated test keypairs:");
    println!("   Alice: {}", alice_keypair.public_key_string());
    println!("   Bob: {}", bob_keypair.public_key_string());
    
    // Test message scenarios
    let test_scenarios = vec![
        ("Classical Message", CryptoMode::Classical),
        ("Hybrid Message", CryptoMode::Hybrid),
        ("Quantum Message", CryptoMode::Quantum),
    ];
    
    for (config_name, relay_addr, min_mode) in test_configs {
        println!("\n🏢 Testing {} (minimum mode: {})", config_name, min_mode);
        println!("   Address: {}", relay_addr);
        
        let client = RelayClient::new(relay_addr.to_string());
        
        for (scenario_name, crypto_mode) in &test_scenarios {
            println!("\n  📨 Testing {}", scenario_name);
            
            match test_message_policy(&client, *crypto_mode, &alice_keypair).await {
                Ok(accepted) => {
                    if accepted {
                        println!("     ✅ Message accepted by relay policy");
                    } else {
                        println!("     🚫 Message rejected by relay policy");
                    }
                }
                Err(e) => {
                    println!("     ❌ Test failed: {}", e);
                }
            }
        }
    }
    
    println!("\n🧪 Policy Enforcement Validation:");
    test_policy_enforcement().await?;
    
    println!("\n📊 Performance Impact Analysis:");
    test_performance_impact().await?;
    
    println!("\n✅ Session 5 validation completed!");
    Ok(())
}

async fn test_message_policy(
    client: &RelayClient,
    crypto_mode: CryptoMode,
    _keypair: &UserKeyPair,
) -> Result<bool> {
    // Create a test message envelope
    let test_payload = b"Test message for policy validation";
    let inbox_id = "test_inbox_123".to_string();
    
    match crypto_mode {
        CryptoMode::Classical => {
            // Create legacy message envelope
            let envelope = MessageEnvelope::new(inbox_id, test_payload.to_vec());
            
            match client.send_envelope(envelope).await {
                Ok(()) => Ok(true),
                Err(e) => {
                    if e.to_string().contains("rejected by crypto policy") {
                        Ok(false)
                    } else {
                        Err(e.into())
                    }
                }
            }
        }
        CryptoMode::Hybrid | CryptoMode::Quantum => {
            // Create quantum-safe message envelope
            let envelope = QuantumSafeEnvelope::new(crypto_mode, inbox_id, test_payload.to_vec());
            
            match client.send_quantum_envelope(envelope).await {
                Ok(()) => Ok(true),
                Err(e) => {
                    if e.to_string().contains("rejected by crypto policy") {
                        Ok(false)
                    } else {
                        Err(e.into())
                    }
                }
            }
        }
    }
}

async fn test_policy_enforcement() -> Result<()> {
    println!("\n🔍 Testing Policy Enforcement Logic:");
    
    // Test 1: Classical message to post-quantum relay should be rejected
    println!("   Test 1: Classical message to quantum-only relay");
    let client = RelayClient::new("127.0.0.1:7735".to_string());
    let envelope = MessageEnvelope::new("test_inbox".to_string(), b"test".to_vec());
    
    match client.send_envelope(envelope).await {
        Ok(()) => println!("     ❌ FAIL: Classical message was accepted (should be rejected)"),
        Err(e) if e.to_string().contains("crypto policy") => {
            println!("     ✅ PASS: Classical message properly rejected");
        }
        Err(e) => println!("     ⚠️  UNKNOWN: Unexpected error: {}", e),
    }
    
    // Test 2: Quantum message to any relay should be accepted
    println!("   Test 2: Quantum message to permissive relay");
    let client = RelayClient::new("127.0.0.1:7733".to_string());
    let envelope = QuantumSafeEnvelope::new(
        CryptoMode::Quantum,
        "test_inbox".to_string(),
        b"quantum test".to_vec(),
    );
    
    match client.send_quantum_envelope(envelope).await {
        Ok(()) => println!("     ✅ PASS: Quantum message accepted"),
        Err(e) => println!("     ❌ FAIL: Quantum message rejected: {}", e),
    }
    
    Ok(())
}

async fn test_performance_impact() -> Result<()> {
    println!("\n⏱️  Performance Impact of Policy Enforcement:");
    
    let client = RelayClient::new("127.0.0.1:7733".to_string());
    let test_count = 10;
    
    // Test Classical messages
    let start = std::time::Instant::now();
    for i in 0..test_count {
        let envelope = MessageEnvelope::new(
            format!("perf_test_classical_{}", i),
            vec![0u8; 1024], // 1KB payload
        );
        let _ = client.send_envelope(envelope).await;
    }
    let classical_duration = start.elapsed();
    
    // Test Quantum messages
    let start = std::time::Instant::now();
    for i in 0..test_count {
        let envelope = QuantumSafeEnvelope::new(
            CryptoMode::Quantum,
            format!("perf_test_quantum_{}", i),
            vec![0u8; 1024], // 1KB payload
        );
        let _ = client.send_quantum_envelope(envelope).await;
    }
    let quantum_duration = start.elapsed();
    
    println!("   Classical messages: {:.2}ms average", 
             classical_duration.as_millis() as f64 / test_count as f64);
    println!("   Quantum messages: {:.2}ms average", 
             quantum_duration.as_millis() as f64 / test_count as f64);
    
    let overhead = (quantum_duration.as_millis() as f64 / classical_duration.as_millis() as f64 - 1.0) * 100.0;
    println!("   Policy enforcement overhead: {:.1}%", overhead);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_crypto_mode_validation() {
        // Test that crypto modes are correctly identified
        assert_eq!(CryptoMode::Classical.is_quantum_resistant(), false);
        assert_eq!(CryptoMode::Hybrid.is_quantum_resistant(), true);
        assert_eq!(CryptoMode::Quantum.is_quantum_resistant(), true);
        
        // Test mode transitions
        assert!(CryptoMode::Classical.can_transition_to(CryptoMode::Hybrid));
        assert!(!CryptoMode::Hybrid.can_transition_to(CryptoMode::Classical));
    }
    
    #[test]
    fn test_envelope_creation() {
        let envelope = QuantumSafeEnvelope::new(
            CryptoMode::Hybrid,
            "test_inbox".to_string(),
            b"test_data".to_vec(),
        );
        
        assert_eq!(envelope.version, "2.0-quantum");
        assert_eq!(envelope.crypto_mode, CryptoMode::Hybrid);
        assert_eq!(envelope.inbox_id, "test_inbox");
        assert!(!envelope.is_expired());
    }
}
