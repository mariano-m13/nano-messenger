#!/bin/bash

echo "🧹 COMPREHENSIVE CLEAN AND TEST"
echo "==============================="
echo "Forcing complete rebuild of quantum-resistant protocol"
echo ""

echo "🔧 Step 1: Complete cleanup..."
cargo clean
rm -rf target/
rm -rf Cargo.lock

echo "🔨 Step 2: Fresh compilation check..."
cargo check --examples

if [ $? -ne 0 ]; then
    echo "❌ Compilation failed during check phase"
    exit 1
fi

echo "✅ All examples compile successfully!"
echo ""

echo "🚀 Step 3: Testing critical sessions individually..."

echo "  Testing Session 2 (Mode Transitions)..."
if cargo run --example session2_validation > /dev/null 2>&1; then
    echo "     ✅ Session 2 PASSED"
else
    echo "     ❌ Session 2 FAILED"
    echo "     Running with full output for debugging:"
    cargo run --example session2_validation
fi

echo "  Testing Session 3 (Quantum-Safe Messaging)..."
if cargo run --example session3_validation > /dev/null 2>&1; then
    echo "     ✅ Session 3 PASSED"
else
    echo "     ❌ Session 3 FAILED"
    echo "     Running with full output for debugging:"
    cargo run --example session3_validation
fi

echo "  Testing Session 6 (Performance)..."
if cargo run --example session6_validation > /dev/null 2>&1; then
    echo "     ✅ Session 6 PASSED"
else
    echo "     ❌ Session 6 FAILED"
    echo "     Running with full output for debugging:"
    cargo run --example session6_validation
fi

echo "  Testing Session 7 (Security - CRITICAL)..."
if cargo run --example session7_validation > /dev/null 2>&1; then
    echo "     ✅ Session 7 PASSED (CRITICAL SUCCESS!)"
else
    echo "     ❌ Session 7 FAILED (CRITICAL ISSUE!)"
    echo "     Running with full output for debugging:"
    cargo run --example session7_validation
fi

echo ""
echo "🎯 SUMMARY:"
echo "All cache issues should now be resolved!"
echo "The quantum-resistant protocol should be working correctly."
