# 🧪 Nano-Messenger Testing Suite

Comprehensive testing suite for the quantum-resistant messaging protocol. This suite validates all implementation sessions, ensures cryptographic correctness, and verifies production readiness.

## 🚀 Quick Start

### Option 1: Shell Script (Linux/macOS)
```bash
# Make script executable
chmod +x test_all_sessions.sh

# Run all tests
./test_all_sessions.sh

# Quick validation only
./test_all_sessions.sh --quick

# Help and options
./test_all_sessions.sh --help
```

### Option 2: Makefile (Linux/macOS)
```bash
# Run comprehensive test suite
make test

# Quick validation
make quick

# Only session validations
make sessions

# Only unit tests
make unit

# Security validation only
make security

# Help
make help
```

### Option 3: Windows Batch Script
```cmd
# Run all tests
test_all_sessions.bat
```

### Option 4: Cargo Integration Tests
```bash
# Run programmatic integration tests
cargo test --test comprehensive_session_tests

# Run specific test
cargo test --test comprehensive_session_tests test_all_sessions_comprehensive

# Run benchmarks
cargo test --test comprehensive_session_tests benchmark_all_sessions -- --ignored
```

## 📋 What Gets Tested

### 🔨 Compilation Tests
- ✅ All examples compile without errors
- ✅ Library builds in release mode
- ✅ All dependencies resolve correctly

### 📚 Session Validations
- **Session 1**: Core cryptographic implementation
- **Session 2**: Protocol implementation and message handling
- **Session 3**: Quantum-safe messaging integration
- **Session 4**: Multi-mode crypto support (Classical/Hybrid/Quantum)
- **Session 5**: Relay policy enforcement and compliance
- **Session 6**: Performance optimization and adaptive configuration
- **Session 7**: Comprehensive security validation ⭐ **Critical**

### 🧪 Unit & Integration Tests
- ✅ All module unit tests
- ✅ Integration test suite
- ✅ Documentation example tests
- ✅ Cross-session compatibility tests

### ⚡ Performance Tests
- ✅ Adaptive performance validation
- ✅ Benchmarking (optional)
- ✅ Timing validation for critical operations

## 🎯 Critical Success Criteria

For **production readiness**, these must pass:

1. **✅ Session 7 (Security Validation)** - Verifies:
   - Cryptographic correctness
   - Attack resistance (forgery, replay, quantum)
   - Protocol security properties
   - Cross-version interoperability

2. **✅ All Compilation Tests** - Ensures:
   - Clean builds without warnings
   - All dependencies available
   - Examples are executable

3. **✅ Core Session Validations** - Confirms:
   - All crypto modes work correctly
   - Protocol handles all message types
   - Policy enforcement functions

## 📊 Test Output

### Successful Run Example:
```
🧪 NANO-MESSENGER COMPREHENSIVE SESSION TESTING
=================================================

🔨 COMPILATION CHECK
✅ Compilation Check - PASSED (2s)
✅ Library Build - PASSED (8s)

📋 SESSION VALIDATIONS
✅ Session 1 Validation - PASSED (1s)
✅ Session 2 Validation - PASSED (1s)
✅ Session 3 Validation - PASSED (2s)
✅ Session 4 Validation - PASSED (1s)
✅ Session 5 Validation - PASSED (3s)
✅ Session 6 Validation - PASSED (2s)
✅ Session 7 Validation - PASSED (2s)

📊 COMPREHENSIVE TEST REPORT
Total Tests: 9, Passed: 9, Failed: 0
Success Rate: 100%

🛡️ Security Status:
✅ Security Validation: COMPLETE
✅ Cryptographic Correctness: VERIFIED
✅ Production Ready: YES

🏆 ALL VALIDATIONS SUCCESSFUL! Protocol ready for deployment.
```

## 🛠️ Troubleshooting

### Common Issues:

**Compilation Errors:**
```bash
# Clean and rebuild
cargo clean
cargo check --examples
```

**Session Test Failures:**
```bash
# Run individual session for detailed error
cargo run --example session7_validation

# Check specific functionality
cargo test crypto::tests
```

**Permission Issues (Linux/macOS):**
```bash
# Make script executable
chmod +x test_all_sessions.sh
```

### Getting Detailed Output:

**Verbose Session Output:**
```bash
# Run session directly to see full output
cargo run --example session7_validation
```

**Debug Mode:**
```bash
# Run with debug info
RUST_LOG=debug cargo run --example session7_validation
```

## 📁 Files in Testing Suite

```
nano-messenger/
├── test_all_sessions.sh      # Main test script (Linux/macOS)
├── test_all_sessions.bat     # Windows test script
├── Makefile                  # Make-based test runner
├── tests/
│   └── comprehensive_session_tests.rs  # Rust integration tests
└── examples/
    ├── session1_validation.rs
    ├── session2_validation.rs
    ├── session3_validation.rs
    ├── session4_validation.rs
    ├── session5_validation.rs
    ├── session6_validation.rs
    └── session7_validation.rs
```

## 🔍 Advanced Usage

### Running Specific Tests:
```bash
# Test only critical sessions
./test_all_sessions.sh --sessions

# Test specific session
make session SESSION=session7

# Run with timeout
timeout 300s ./test_all_sessions.sh
```

### Performance Benchmarking:
```bash
# Run performance benchmarks
make benchmark

# Time individual sessions
time cargo run --example session7_validation
```

### Continuous Integration:
```bash
# For CI/CD pipelines
./test_all_sessions.sh --quick && echo "✅ CI Tests Passed"
```

## 📈 Success Metrics

- **100% Pass Rate**: All tests must pass for production deployment
- **Session 7 Pass**: Security validation is mandatory
- **< 60s Total Time**: Complete test suite should finish quickly
- **Clean Output**: No compilation warnings or errors

## 🚨 Security Notice

**Session 7 (Security Validation) is CRITICAL** - this validates:
- Cryptographic algorithm correctness
- Attack resistance properties  
- Protocol security guarantees
- Production readiness

If Session 7 fails, **DO NOT deploy to production**.

## 🤝 Contributing

When adding new features:

1. **Add appropriate tests** to relevant session validations
2. **Update session examples** to demonstrate new functionality
3. **Run full test suite** before submitting changes
4. **Ensure Session 7 still passes** after modifications

## 📞 Support

If tests fail unexpectedly:

1. Check the generated test report: `test_report_YYYYMMDD_HHMMSS.txt`
2. Run individual sessions for detailed error output
3. Verify Rust/Cargo versions match requirements
4. Clean build artifacts: `cargo clean`

---

🏆 **A passing test suite means your quantum-resistant messaging protocol is cryptographically sound and production-ready!**
