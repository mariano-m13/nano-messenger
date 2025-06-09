#!/bin/bash

echo "🔧 Testing Fixed Session Files"
echo "=============================="
echo ""

echo "🧹 Step 1: Clean compilation cache..."
cargo clean

echo "✅ Step 2: Test compilation..."
echo "  Checking session3_validation..."
if cargo check --example session3_validation; then
    echo "    ✅ Session 3 compiles"
else
    echo "    ❌ Session 3 compilation failed"
    exit 1
fi

echo "  Checking session5_validation..."
if cargo check --example session5_validation; then
    echo "    ✅ Session 5 compiles"
else
    echo "    ❌ Session 5 compilation failed"
    exit 1
fi

echo "  Checking session6_validation..."
if cargo check --example session6_validation; then
    echo "    ✅ Session 6 compiles"
else
    echo "    ❌ Session 6 compilation failed"
    exit 1
fi

echo "  Checking all examples..."
if cargo check --examples; then
    echo "    ✅ All examples compile successfully!"
else
    echo "    ❌ Some examples still have issues"
    exit 1
fi

echo ""
echo "🎉 All compilation issues fixed!"
echo "🚀 Ready to run the full test suite"
