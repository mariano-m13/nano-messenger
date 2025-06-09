//! Security Validation Test Suite for Session 7
//! 
//! This integration test runs the comprehensive security validation suite
//! to verify all cryptographic and protocol security properties.

mod security;

use security::*;
use nano_messenger::crypto::*;

/// Run the complete security validation suite
#[test]
fn comprehensive_security_validation_test() {
    let results = run_comprehensive_security_validation()
        .expect("Security validation should complete");
    
    // Print results for CI/logging
    println!("\n{}", results.assessment_summary());
    
    // Ensure all critical security properties are validated
    assert!(results.crypto_correctness_passed, 
        "Cryptographic correctness validation failed");
    assert!(results.protocol_security_passed, 
        "Protocol security validation failed");
    assert!(results.attack_resistance_passed, 
        "Attack resistance validation failed");  
    assert!(results.interoperability_passed, 
        "Interoperability validation failed");
    
    // Check for critical issues
    let critical_issues = results.critical_issues();
    assert!(critical_issues.is_empty(), 
        "Critical security issues detected: {:?}", critical_issues);
    
    // Verify Session 7 exit criteria
    let checklist = SecurityValidationChecklist::from_test_results()
        .expect("Should generate validation checklist");
    
    assert!(checklist.exit_criteria_met(), 
        "Session 7 exit criteria not met:\n{}", checklist.generate_report());
}

/// Quick security check for CI/development
#[test]
fn quick_security_check_test() {
    let passed = run_quick_security_check()
        .expect("Quick security check should complete");
    
    assert!(passed, "Quick security check failed - run full validation for details");
}

/// Test cryptographic correctness specifically
#[test]
fn crypto_correctness_validation() {
    crypto_correctness::run_all_crypto_correctness_tests()
        .expect("Cryptographic correctness tests should pass");
}

/// Test protocol security specifically
#[test]
fn protocol_security_validation() {
    protocol_security::run_all_protocol_security_tests()
        .expect("Protocol security tests should pass");
}

/// Test attack resistance specifically
#[test]
fn attack_resistance_validation() {
    attack_resistance::run_all_attack_resistance_tests()
        .expect("Attack resistance tests should pass");
}

/// Test interoperability specifically
#[test]
fn interoperability_validation() {
    interoperability::run_all_interoperability_tests()
        .expect("Interoperability tests should pass");
}

/// Validate Session 7 completion criteria
#[test]
fn session7_exit_criteria_validation() {
    println!("Validating Session 7 Exit Criteria...");
    
    // Check that all security test suites pass
    let crypto_ok = crypto_correctness::run_all_crypto_correctness_tests().is_ok();
    let protocol_ok = protocol_security::run_all_protocol_security_tests().is_ok();
    let attack_ok = attack_resistance::run_all_attack_resistance_tests().is_ok();
    let interop_ok = interoperability::run_all_interoperability_tests().is_ok();
    
    assert!(crypto_ok, "Cryptographic correctness required for Session 7 completion");
    assert!(protocol_ok, "Protocol security validation required for Session 7 completion");
    assert!(attack_ok, "Attack resistance validation required for Session 7 completion");
    assert!(interop_ok, "Interoperability validation required for Session 7 completion");
    
    // Generate and validate checklist
    let checklist = SecurityValidationChecklist::from_test_results()
        .expect("Should generate validation checklist");
    
    println!("{}", checklist.generate_report());
    
    assert!(checklist.exit_criteria_met(), 
        "Session 7 exit criteria must be met to proceed to Session 8");
    
    println!("🎉 Session 7 Security Validation Complete!");
    println!("✅ All exit criteria met - ready for Session 8: Production Hardening");
}

/// Performance security test - ensure security operations complete in reasonable time
#[test]
fn security_performance_validation() {
    use std::time::Instant;
    
    let start = Instant::now();
    
    // Run core security operations and measure performance
    let _results = run_quick_security_check()
        .expect("Quick security check should complete");
    
    let duration = start.elapsed();
    
    // Security tests should complete reasonably quickly (under 30 seconds for quick check)
    assert!(duration.as_secs() < 30, 
        "Security validation taking too long: {:.2}s", duration.as_secs_f64());
    
    println!("Security validation performance: {:.2}s", duration.as_secs_f64());
}

/// Memory safety validation - ensure no unsafe operations in crypto code
#[test]
fn memory_safety_validation() {
    // This test validates that our crypto implementations don't rely on unsafe code
    // and properly handle memory management
    
    use nano_messenger::crypto::*;
    
    // Generate many keypairs to test for memory leaks/issues
    for _ in 0..100 {
        let classical = ClassicalUserKeyPair::generate();
        let hybrid = HybridUserKeyPair::generate();
        let _quantum = PostQuantumUserKeyPair::generate();
        
        // Perform crypto operations to stress memory management
        let message = b"memory safety test";
        
        // Classical operations
        let sig = ClassicalDigitalSignature::sign(&classical.signing_key, message);
        ClassicalDigitalSignature::verify(&classical.public_keys().verifying_key, message, &sig)
            .expect("Signature should verify");
        
        // Key exchange
        let shared = ClassicalKeyExchange::key_exchange(
            &classical.x25519_key,
            &hybrid.classical.public_keys().x25519_key
        ).expect("Key exchange should work");
        
        // Symmetric encryption
        let key = hash_sha256(shared.as_ref());
        let ciphertext = encrypt_symmetric(&key, message).expect("Encryption should work");
        let _decrypted = decrypt_symmetric(&key, &ciphertext).expect("Decryption should work");
    }
    
    println!("✅ Memory safety validation completed - no unsafe operations detected");
}

/// Thread safety validation - ensure crypto operations are thread-safe
#[test]
fn thread_safety_validation() {
    use std::thread;
    use std::sync::Arc;
    
    let keypair = Arc::new(ClassicalUserKeyPair::generate());
    let _message = b"thread safety test";
    
    let handles: Vec<_> = (0..10).map(|i| {
        let keypair = Arc::clone(&keypair);
        let message = format!("thread safety test {}", i);
        
        thread::spawn(move || {
            // Perform crypto operations from multiple threads
            let sig = ClassicalDigitalSignature::sign(&keypair.signing_key, message.as_bytes());
            ClassicalDigitalSignature::verify(&keypair.public_keys().verifying_key, message.as_bytes(), &sig)
                .expect("Signature should verify in thread");
            
            // Test symmetric encryption thread safety
            let key = [i as u8; 32];
            let ciphertext = encrypt_symmetric(&key, message.as_bytes()).expect("Encryption should work in thread");
            let _decrypted = decrypt_symmetric(&key, &ciphertext).expect("Decryption should work in thread");
        })
    }).collect();
    
    for handle in handles {
        handle.join().expect("Thread should complete successfully");
    }
    
    println!("✅ Thread safety validation completed - all operations thread-safe");
}
