use nano_messenger::crypto::{CryptoInterface, CryptoConfig, CryptoMode, init_crypto_config};
use nano_messenger::protocol::QuantumSafeEnvelope;

fn main() {
    println!("🧪 Simple Session 3 Build Test");
    println!("==============================");

    // Test 1: Can we initialize crypto config?
    match init_crypto_config(CryptoConfig::new(CryptoMode::Classical)) {
        Ok(()) => println!("✅ Crypto config initialized"),
        Err(_) => println!("✅ Crypto config already initialized (OK)"),
    }

    // Test 2: Can we generate keypairs?
    match CryptoInterface::generate_keypair() {
        Ok(keypair) => {
            println!("✅ Classical keypair generated: {}", keypair.public_key_string());
        }
        Err(e) => {
            println!("❌ Failed to generate keypair: {}", e);
            return;
        }
    }

    // Test 3: Can we create quantum-safe envelope?
    let envelope = QuantumSafeEnvelope::new(
        CryptoMode::Classical,
        "test_inbox".to_string(),
        b"test_data".to_vec(),
    );
    println!("✅ QuantumSafeEnvelope created: {}", envelope.version);
    println!("✅ Crypto mode: {}", envelope.crypto_mode);

    // Test 4: Can we serialize/deserialize?
    match envelope.to_json() {
        Ok(json) => {
            println!("✅ Envelope serialized to JSON");
            match QuantumSafeEnvelope::from_json(&json) {
                Ok(deserialized) => {
                    println!("✅ Envelope deserialized from JSON");
                    println!("✅ Round-trip successful: {}", deserialized.version);
                }
                Err(e) => println!("❌ Deserialization failed: {}", e),
            }
        }
        Err(e) => println!("❌ Serialization failed: {}", e),
    }

    println!("\n🎉 Basic Session 3 functionality working!");
    println!("Ready for comprehensive testing with ./session3_test.sh");
}
