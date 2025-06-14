#!/bin/bash

# Quick setup script for Session 12 testing
echo "🛡️  Setting up Session 12 testing environment..."

# Make test scripts executable
chmod +x session12_test.sh
chmod +x test_all_sessions.sh
chmod +x *.sh

echo "✅ Test scripts are now executable"

# Quick compilation check
echo "🔨 Checking Session 12 compilation..."
if cargo check --example session12_validation; then
    echo "✅ Session 12 code compiles successfully"
else
    echo "❌ Compilation issues detected"
    exit 1
fi

echo ""
echo "🎯 Session 12 Testing Ready!"
echo "=========================="
echo ""
echo "Quick test options:"
echo "  ./session12_test.sh                    # Full Session 12 test suite"
echo "  cargo run --example session12_validation  # Session 12 validation example"
echo "  ./test_all_sessions.sh                 # All sessions including Session 12"
echo ""
echo "Individual test categories:"
echo "  cargo test test_security_scanner        # Security scanning tests"
echo "  cargo test test_gdpr                    # GDPR compliance tests" 
echo "  cargo test test_hipaa                   # HIPAA compliance tests"
echo "  cargo test test_audit_system            # Audit system tests"
echo "  cargo test test_performance_under_load  # Performance tests"
echo ""
echo "🚀 Ready to test enterprise-grade security and compliance features!"
