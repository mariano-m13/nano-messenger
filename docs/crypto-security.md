# Quantum-Resistant Nano-Messenger: Cryptographic Security

## Overview

The Quantum-Resistant Nano-Messenger implements hybrid cryptography to provide protection against both classical and quantum computer attacks. This document outlines the security assumptions, guarantees, and cryptographic design decisions.

## Security Model

### Threat Model

**Protected Against:**
- Classical cryptographic attacks (ECDLP, factoring)
- Post-quantum attacks (Shor's algorithm, Grover's algorithm)
- Man-in-the-middle attacks
- Message replay attacks
- Traffic analysis (metadata protection)
- Forward secrecy violations

**Not Protected Against:**
- Endpoint compromise (malware on user devices)
- Side-channel attacks on hardware
- Social engineering attacks
- Physical access to unlocked devices
- Quantum computers breaking both classical AND post-quantum crypto simultaneously

### Security Assumptions

1. **Cryptographic Assumptions:**
   - At least one of X25519 or ML-KEM-768 remains secure
   - At least one of Ed25519 or ML-DSA remains secure
   - ChaCha20Poly1305 provides semantic security
   - Random number generators are cryptographically secure

2. **Implementation Assumptions:**
   - Rust memory safety prevents buffer overflows
   - Constant-time operations prevent timing attacks
   - Secure key storage is maintained by the OS/hardware
   - Network layer provides basic transport security

3. **Operational Assumptions:**
   - Users maintain physical security of their devices
   - System clocks are reasonably synchronized
   - Software updates are applied in reasonable timeframes

## Cryptographic Algorithms

### Hybrid Key Exchange

**Algorithm:** X25519 + ML-KEM-768 (NIST FIPS 203)

```
Hybrid_KeyExchange():
  classical_shared = X25519(alice_private, bob_public)
  (pq_shared, pq_ciphertext) = ML-KEM-768.Encapsulate(bob_pq_public)
  
  final_shared = HKDF(classical_shared || pq_shared, "nano-messenger-v2")
  return (final_shared, pq_ciphertext)
```

**Security Properties:**
- Provides 128-bit classical security
- Provides NIST Level 1 post-quantum security
- Secure if either algorithm remains unbroken
- Forward secrecy through ephemeral key exchange

### Hybrid Digital Signatures

**Algorithm:** Ed25519 + ML-DSA (NIST FIPS 204)

```
Hybrid_Sign(message, classical_key, pq_key):
  classical_sig = Ed25519.Sign(message, classical_key)
  pq_sig = ML-DSA.Sign(message, pq_key)
  
  return (classical_sig, pq_sig)

Hybrid_Verify(message, signature, classical_pubkey, pq_pubkey):
  return Ed25519.Verify(message, classical_sig, classical_pubkey) AND
         ML-DSA.Verify(message, pq_sig, pq_pubkey)
```

**Security Properties:**
- Unforgeable if either signature scheme is secure
- Non-repudiation under hybrid security model
- 128-bit classical security + NIST Level 2 post-quantum security

### Symmetric Encryption

**Algorithm:** ChaCha20Poly1305 (RFC 8439)

**Security Properties:**
- Authenticated encryption with 256-bit keys
- Resistant to quantum attacks (symmetric crypto)
- Nonce-misuse resistance through proper generation
- 128-bit authentication tag

## Key Lifecycle Management

### Key Generation

1. **Entropy Sources:** System CSPRNG + hardware RNG when available
2. **Key Derivation:** HKDF-SHA256 for all derived keys
3. **Key Validation:** Public keys validated before use

### Key Distribution

1. **Initial Exchange:** Out-of-band verification recommended
2. **Key Verification:** Public key fingerprints for manual verification
3. **Trust Models:** TOFU (Trust On First Use) with manual verification option

### Key Storage

1. **Format:** PKCS#8 for private keys, X.509 SubjectPublicKeyInfo for public keys
2. **Protection:** OS keychain integration when available
3. **Backup:** Encrypted key export/import functionality

### Key Rotation

1. **Automatic:** Optional periodic key rotation (default: 30 days)
2. **Manual:** User-initiated key rotation
3. **Compromise Recovery:** Emergency key revocation and replacement

## Protocol Security

### Message Format

```json
{
  "version": "2.0-quantum",
  "crypto_mode": "hybrid",
  "envelope": {
    "recipient": "...",
    "payload": "...",  // ChaCha20Poly1305 encrypted
    "kem_ciphertext": "...",  // ML-KEM-768 ciphertext
    "signature": {
      "classical": "...",  // Ed25519 signature
      "post_quantum": "..."  // ML-DSA signature
    }
  }
}
```

### Forward Secrecy

- **Ephemeral Keys:** New key pair for each message exchange
- **Key Deletion:** Private keys securely wiped after use
- **Ratcheting:** Optional double ratchet for ongoing conversations

### Replay Protection

- **Nonces:** Cryptographically random nonces in all messages
- **Timestamps:** Message timestamps validated within acceptable window
- **Sequence Numbers:** Optional sequence numbering for ordered delivery

## Security Configurations

### Crypto Modes

1. **Classical Mode:**
   - Uses only X25519 + Ed25519
   - For legacy compatibility
   - **Warning:** Vulnerable to quantum attacks

2. **Hybrid Mode (Recommended):**
   - Uses X25519 + ML-KEM-768 and Ed25519 + ML-DSA
   - Provides classical and post-quantum security
   - Moderate performance impact

3. **Quantum Mode:**
   - Uses only ML-KEM-768 + ML-DSA
   - Maximum post-quantum security
   - Higher performance impact

### Security Policies

```rust
pub struct SecurityPolicy {
    pub minimum_crypto_mode: CryptoMode,
    pub require_perfect_forward_secrecy: bool,
    pub max_message_age_seconds: u64,
    pub require_manual_key_verification: bool,
    pub enable_metadata_protection: bool,
}
```

## Compliance Considerations

### FIPS 140-2 Compliance

- **Algorithms:** All algorithms selected from FIPS-approved lists
- **Implementation:** Rust memory safety provides some FIPS benefits
- **Validation:** Full FIPS validation requires certified module

### GDPR Compliance

- **Data Minimization:** Only necessary data collected
- **Purpose Limitation:** Data used only for messaging
- **Storage Limitation:** Configurable message retention policies
- **Right to Erasure:** Secure data deletion capabilities

### Export Controls

- **Cryptography:** Contains cryptographic software
- **Jurisdiction:** Check local export control laws
- **Documentation:** This software may be subject to export restrictions

## Audit and Monitoring

### Security Events

- Key generation and rotation events
- Authentication failures
- Protocol version mismatches
- Suspected replay attacks
- Configuration changes

### Audit Log Format

```json
{
  "timestamp": "2025-06-08T10:30:00Z",
  "event_type": "key_rotation",
  "user_id": "alice@example.com",
  "crypto_mode": "hybrid",
  "metadata": {
    "old_key_fingerprint": "...",
    "new_key_fingerprint": "...",
    "rotation_reason": "scheduled"
  }
}
```

## Incident Response

### Cryptographic Compromise

1. **Algorithm Compromise:**
   - Switch to secure algorithms immediately
   - Notify all users of required updates
   - Provide migration tools for key material

2. **Key Compromise:**
   - Revoke compromised keys
   - Generate new key pairs
   - Re-establish secure channels

3. **Implementation Vulnerability:**
   - Assess impact and affected versions
   - Deploy patches with priority based on severity
   - Consider temporary mitigations

### Breach Notification

- **Timeline:** Within 72 hours of discovery
- **Scope:** Affected users and data types
- **Mitigation:** Steps taken and user actions required
- **Prevention:** Measures to prevent recurrence

## Security Testing

### Continuous Testing

- **Unit Tests:** Cryptographic correctness validation
- **Integration Tests:** End-to-end security property verification
- **Fuzzing:** Protocol message fuzzing for edge cases
- **Performance Tests:** Cryptographic operation benchmarking

### External Validation

- **Penetration Testing:** Regular third-party security assessments
- **Code Audits:** Independent cryptographic implementation review
- **Compliance Audits:** FIPS, Common Criteria, or other standards

## Future Considerations

### Post-Quantum Evolution

- **Algorithm Agility:** Easy migration to new algorithms
- **Hybrid Combinations:** Support for multiple PQ algorithms
- **Performance Improvements:** Hardware acceleration adoption

### Quantum Key Distribution

- **QKD Integration:** Potential for quantum-secured key exchange
- **Hybrid QKD:** Classical backup for QKD failures
- **Infrastructure:** Requirements for QKD deployment

## References

- **NIST FIPS 203:** Module-Lattice-Based Key-Encapsulation Mechanism
- **NIST FIPS 204:** Module-Lattice-Based Digital Signature Standard  
- **RFC 8439:** ChaCha20 and Poly1305 for IETF Protocols
- **RFC 7748:** Elliptic Curves for Security (X25519)
- **RFC 8032:** Edwards-Curve Digital Signature Algorithm (Ed25519)

---

**Document Version:** 2.0  
**Last Updated:** June 2025  
**Classification:** Public Documentation