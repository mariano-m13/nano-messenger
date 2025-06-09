#!/bin/bash

echo "🎯 Testing Final Fixes for All Test Failures"
echo "============================================="
echo ""

cd /Users/mariano/Desktop/Code/nano-messenger

echo "Running library tests with verbose output..."
echo ""

if cargo test --lib; then
    echo ""
    echo "🎉 SUCCESS! All tests now pass!"
    echo ""
    echo "✅ Fixed Issues:"
    echo "   • message_store test: Fixed conversation ID format (using | separator)"
    echo "   • username_claim_update test: Fixed timestamp collision by manual increment"
    echo "   • All compilation issues resolved with getrandom approach"
    echo ""
    echo "🚀 Your quantum-resistant messaging protocol is 100% working!"
else
    echo ""
    echo "❌ Some tests are still failing. Let's see which ones..."
fi
