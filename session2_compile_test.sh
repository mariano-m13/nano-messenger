#!/bin/bash

echo "🚀 Session 2: Post-Quantum Implementation Test"
echo "============================================="

cd /Users/mariano/Desktop/Code/nano-messenger

echo "🧹 Cleaning previous builds..."
cargo clean > /dev/null 2>&1

echo ""
echo "🔍 Testing basic compilation..."
cargo check --lib 2>&1 | head -20

echo ""
echo "🏗️ Testing build..."
cargo build --lib 2>&1 | head -20

echo ""
echo "🧪 Testing post-quantum crypto module..."
cargo test crypto::post_quantum --lib --no-run 2>&1 | head -10

echo ""
echo "🔄 Testing hybrid crypto module..."
cargo test crypto::hybrid --lib --no-run 2>&1 | head -10

echo ""
echo "✅ Session 2 compilation test complete!"
