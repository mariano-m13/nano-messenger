#!/bin/bash

echo "🎯 Session 2: FINAL VALIDATION TEST"
echo "==================================="
echo ""

cd /Users/mariano/Desktop/Code/nano-messenger

# Function to check command result
check_result() {
    if [ $? -eq 0 ]; then
        echo "✅ PASS"
    else
        echo "❌ FAIL"
        FAILED=true
    fi
}

FAILED=false

echo "🔧 Step 1: Testing compilation..."
echo "--------------------------------"
cargo check --lib --quiet
check_result

echo ""
echo "🏗️ Step 2: Testing build..."
echo "--------------------------"
cargo build --lib --quiet
check_result

echo ""
echo "🧪 Step 3: Testing crypto fixes example..."
echo "-----------------------------------------"
cargo run --example test_crypto_fixes --quiet
check_result

echo ""
echo "🔬 Step 4: Running post-quantum tests..."
echo "---------------------------------------"
cargo test crypto::post_quantum --lib --quiet
check_result

echo ""
echo "🔄 Step 5: Running hybrid crypto tests..."
echo "----------------------------------------"
cargo test crypto::hybrid --lib --quiet
check_result

echo ""
echo "🚀 Step 6: Running Session 2 validation..."
echo "-----------------------------------------"
cargo run --example session2_validation --quiet
check_result

echo ""
echo "======================================"
if [ "$FAILED" = false ]; then
    echo "🎉 ALL TESTS PASSED!"
    echo "✅ Session 2 is FULLY COMPLETE and WORKING!"
    echo ""
    echo "🏆 Quantum-resistant nano-messenger is ready!"
    echo "   - Post-quantum cryptography: ✅ Working"
    echo "   - Hybrid cryptography: ✅ Working"
    echo "   - Classical compatibility: ✅ Maintained"
    echo "   - All tests passing: ✅ Verified"
    echo ""
    echo "🚀 Ready for Session 3: Message Format Evolution!"
else
    echo "❌ SOME TESTS FAILED"
    echo "Please check the errors above and fix them."
fi
echo "======================================"
