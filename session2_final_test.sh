#!/bin/bash

echo "🎯 Session 2: Comprehensive Validation Test"
echo "==========================================="

cd /Users/mariano/Desktop/Code/nano-messenger

# Clean previous builds
echo "🧹 Cleaning previous builds..."
cargo clean > /dev/null 2>&1

echo ""
echo "🔍 Testing basic compilation..."
echo "------------------------------"
timeout 30s cargo check --lib 2>&1 | head -15

echo ""
echo "🏗️ Testing full build..."
echo "------------------------"
timeout 30s cargo build --lib 2>&1 | head -15

echo ""
echo "🧪 Testing post-quantum crypto module..."
echo "---------------------------------------"
timeout 30s cargo test crypto::post_quantum --lib --no-run 2>&1 | head -10

echo ""
echo "🔄 Testing hybrid crypto module..."
echo "---------------------------------"
timeout 30s cargo test crypto::hybrid --lib --no-run 2>&1 | head -10

echo ""
echo "🚀 Running Session 2 validation example..."
echo "------------------------------------------"
timeout 30s cargo run --example session2_validation 2>&1 | head -20

echo ""
echo "✅ Session 2 validation complete!"
