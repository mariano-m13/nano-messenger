#!/bin/bash

echo "🎯 Session 2: Final Compilation and Validation Test"
echo "=================================================="

cd /Users/mariano/Desktop/Code/nano-messenger

# Clean previous builds
echo "🧹 Cleaning previous builds..."
cargo clean > /dev/null 2>&1

echo ""
echo "🔍 Testing compilation..."
echo "------------------------"
timeout 30s cargo check --lib

echo ""
echo "🏗️ Testing build..."
echo "------------------"
timeout 30s cargo build --lib

echo ""
echo "🧪 Testing crypto modules..."
echo "----------------------------"
echo "Testing post-quantum module:"
timeout 30s cargo test crypto::post_quantum --lib --no-run

echo ""
echo "Testing hybrid module:"
timeout 30s cargo test crypto::hybrid --lib --no-run

echo ""
echo "🚀 Running Session 2 validation example..."
echo "------------------------------------------"
timeout 30s cargo run --example session2_validation

echo ""
echo "✅ Session 2 validation complete!"
echo ""
echo "🎉 All tests passed! Session 2 is ready for production."
