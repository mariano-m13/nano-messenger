#!/bin/bash

# Quick test script for critical sessions only - macOS compatible
echo "🚀 QUICK CRITICAL SESSION TEST"
echo "=============================="
echo "Testing most important sessions for quantum-resistant protocol"
echo ""

failed=0

echo "🔧 Testing Session 2 (Mode Transitions)..."
if cargo run --example session2_validation > /dev/null 2>&1; then
    echo "   ✅ Session 2 PASSED"
else
    echo "   ❌ Session 2 FAILED"
    failed=$((failed + 1))
fi

echo "🛡️  Testing Session 7 (Security Validation)..."
if cargo run --example session7_validation > /dev/null 2>&1; then
    echo "   ✅ Session 7 PASSED"
else
    echo "   ❌ Session 7 FAILED"
    failed=$((failed + 1))
fi

echo "⚡ Testing Session 6 (Performance)..."
if cargo run --example session6_validation > /dev/null 2>&1; then
    echo "   ✅ Session 6 PASSED"
else
    echo "   ❌ Session 6 FAILED"
    failed=$((failed + 1))
fi

echo ""
if [ $failed -eq 0 ]; then
    echo "🎉 ALL CRITICAL TESTS PASSED!"
    echo "✅ Your quantum-resistant protocol is working correctly"
    echo "🚀 Ready for full validation with: ./test_sessions_macos.sh"
else
    echo "⚠️  $failed critical test(s) failed"
    echo "🔧 Run individual sessions for detailed error output"
fi

echo ""
echo "📊 Quick Stats:"
echo "   Tested: 3 critical sessions"
echo "   Passed: $((3 - failed))"
echo "   Failed: $failed"
