use nano_messenger::crypto::{
    PostQuantumKeyExchange, PostQuantumDigitalSignature, PostQuantumAsymmetricEncryption,
    PostQuantumUserKeyPair
};
use nano_messenger::crypto::traits::{DigitalSignature, AsymmetricEncryption};

fn main() {
    println!("🔍 Testing Post-Quantum Crypto Fixes");
    println!("=====================================\n");
    
    // Test 1: Key Encapsulation/Decapsulation
    println!("📦 Testing Key Encapsulation...");
    let keypair = PostQuantumUserKeyPair::generate();
    let public_keys = keypair.public_keys();
    
    match PostQuantumKeyExchange::encapsulate(&public_keys.public_key) {
        Ok((shared_secret1, ciphertext)) => {
            match PostQuantumKeyExchange::decapsulate(&keypair.private_key, &ciphertext) {
                Ok(shared_secret2) => {
                    if shared_secret1.as_bytes() == shared_secret2.as_bytes() {
                        println!("  ✅ Key encapsulation/decapsulation: PASS");
                    } else {
                        println!("  ❌ Key encapsulation/decapsulation: FAIL - secrets don't match");
                        println!("     Secret 1: {:?}", shared_secret1.as_bytes());
                        println!("     Secret 2: {:?}", shared_secret2.as_bytes());
                    }
                }
                Err(e) => println!("  ❌ Decapsulation failed: {}", e),
            }
        }
        Err(e) => println!("  ❌ Encapsulation failed: {}", e),
    }
    
    // Test 2: Digital Signatures
    println!("\n✍️ Testing Digital Signatures...");
    let signing_private = PostQuantumDigitalSignature::generate_private_key();
    let verifying_public = PostQuantumDigitalSignature::derive_public_key(&signing_private);
    
    let test_data = b"test message for signature verification";
    let signature = PostQuantumDigitalSignature::sign(&signing_private, test_data);
    
    match PostQuantumDigitalSignature::verify(&verifying_public, test_data, &signature) {
        Ok(()) => println!("  ✅ Signature verification: PASS"),
        Err(e) => println!("  ❌ Signature verification: FAIL - {}", e),
    }
    
    // Test 3: Asymmetric Encryption
    println!("\n🔐 Testing Asymmetric Encryption...");
    let plaintext = b"Hello, post-quantum world!";
    
    match PostQuantumAsymmetricEncryption::encrypt(&public_keys.public_key, plaintext) {
        Ok(ciphertext) => {
            match PostQuantumAsymmetricEncryption::decrypt_pq_direct(&keypair.private_key, &ciphertext) {
                Ok(decrypted) => {
                    if decrypted == plaintext {
                        println!("  ✅ Asymmetric encryption/decryption: PASS");
                    } else {
                        println!("  ❌ Asymmetric encryption/decryption: FAIL - data mismatch");
                    }
                }
                Err(e) => println!("  ❌ Asymmetric decryption failed: {}", e),
            }
        }
        Err(e) => println!("  ❌ Asymmetric encryption failed: {}", e),
    }
    
    println!("\n🎯 Test complete!");
}
