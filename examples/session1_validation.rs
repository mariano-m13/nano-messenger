// Session 1 Validation Test
// This test demonstrates the pluggable crypto architecture is working

use nano_messenger::crypto::{
    CryptoConfig, CryptoMode, CryptoInterface, 
    init_crypto_config, get_crypto_config,
    generate_keypair, encrypt_symmetric, decrypt_symmetric
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Session 1: Crypto Foundation Validation");
    println!("==========================================");
    
    // Test 1: Initialize crypto configuration
    println!("1. Testing crypto configuration...");
    let config = CryptoConfig::new(CryptoMode::Classical);
    init_crypto_config(config)?;
    
    let current_config = get_crypto_config();
    println!("   ✓ Current mode: {}", current_config.mode);
    println!("   ✓ Description: {}", current_config.mode.description());
    
    // Test 2: Generate keypair using new interface
    println!("\n2. Testing unified keypair generation...");
    let keypair = CryptoInterface::generate_keypair()?;
    println!("   ✓ Generated keypair with mode: {}", keypair.mode());
    println!("   ✓ Public key string: {}", keypair.public_key_string());
    
    // Test 3: Test backwards compatibility
    println!("\n3. Testing backwards compatibility...");
    let old_style_keypair = generate_keypair();
    let _old_public_keys = old_style_keypair.public_keys();
    println!("   ✓ Old API still works");
    println!("   ✓ Old style public key: {}", old_style_keypair.public_key_string());
    
    // Test 4: Test crypto operations
    println!("\n4. Testing crypto operations...");
    let key = [42u8; 32];
    let plaintext = b"Session 1: Pluggable crypto architecture working!";
    
    let ciphertext = encrypt_symmetric(&key, plaintext)?;
    let decrypted = decrypt_symmetric(&key, &ciphertext)?;
    
    assert_eq!(decrypted, plaintext);
    println!("   ✓ Symmetric encryption/decryption working");
    
    // Test 5: Test performance info
    println!("\n5. Testing performance monitoring...");
    let perf_info = CryptoInterface::performance_info();
    println!("   ✓ Mode: {}", perf_info.mode);
    println!("   ✓ Relative cost: {:.1}x", perf_info.relative_cost);
    println!("   ✓ Size overhead: {} bytes", perf_info.size_overhead);
    println!("   ✓ Quantum resistant: {}", perf_info.quantum_resistant);
    
    // Test 6: Test mode acceptance
    println!("\n6. Testing mode acceptance...");
    println!("   ✓ Accepts classical: {}", CryptoInterface::accepts_mode(CryptoMode::Classical));
    println!("   ✓ Accepts hybrid: {}", CryptoInterface::accepts_mode(CryptoMode::Hybrid));
    println!("   ✓ Accepts quantum: {}", CryptoInterface::accepts_mode(CryptoMode::Quantum));
    
    println!("\n🎉 Session 1 Implementation: SUCCESS!");
    println!("✓ Pluggable cryptography architecture is working");
    println!("✓ Backwards compatibility maintained");
    println!("✓ Ready for Session 2: Post-Quantum Dependencies");
    
    Ok(())
}
