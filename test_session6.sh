#!/bin/bash

echo "🧹 Cleaning and Testing Session 6..."
echo "==================================="

echo "🔧 Step 1: Clean all build artifacts..."
cargo clean

echo "🔨 Step 2: Check compilation..."
if cargo check --example session6_validation; then
    echo "   ✅ Session 6 compiles successfully"
else
    echo "   ❌ Session 6 compilation failed"
    exit 1
fi

echo "🚀 Step 3: Run Session 6 validation..."
if cargo run --example session6_validation; then
    echo "   ✅ Session 6 PASSED!"
else
    echo "   ❌ Session 6 FAILED"
    exit 1
fi

echo ""
echo "🎉 Session 6 is now working!"
