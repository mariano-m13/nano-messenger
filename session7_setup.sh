#!/bin/bash

# Quick Session 7 Setup and Test Script
# Makes security validation script executable and runs basic validation

echo "🔒 Session 7: Security Validation Setup"
echo "======================================="

# Make the security test script executable
chmod +x session7_security_test.sh

echo "✅ Made session7_security_test.sh executable"

# Quick compilation check
echo ""
echo "🔨 Running quick compilation check..."
if cargo check > /dev/null 2>&1; then
    echo "✅ Project compiles successfully"
else
    echo "❌ Compilation issues detected - please check code"
    exit 1
fi

# Run a quick security check
echo ""
echo "🔍 Running quick security validation..."
echo "Note: This runs essential tests only. For full validation, use:"
echo "      ./session7_security_test.sh"
echo ""

# Run just the core tests to verify implementation
if cargo test quick_security_check_test --test security_validation > /dev/null 2>&1; then
    echo "✅ Quick security check passed!"
    echo ""
    echo "🎉 SESSION 7 READY!"
    echo ""
    echo "Next steps:"
    echo "1. Run full security validation: ./session7_security_test.sh"
    echo "2. Review results in test_results/ directory"
    echo "3. Proceed to Session 8 if all tests pass"
else
    echo "⚠️  Quick security check had issues"
    echo "Run full validation for detailed analysis: ./session7_security_test.sh"
fi

echo ""
echo "📁 Session 7 Files Created:"
echo "   tests/security/               - Security test suites"
echo "   tests/security_validation.rs  - Main test runner"
echo "   session7_security_test.sh     - Comprehensive validation script"
echo "   SESSION7_COMPLETED.md         - Implementation documentation"
echo ""
echo "🔒 Security Validation Features:"
echo "   • 37 comprehensive security tests"
echo "   • Cryptographic correctness validation"
echo "   • Protocol security verification"
echo "   • Attack resistance testing"
echo "   • Cross-version compatibility"
echo "   • Automated reporting and exit criteria"
