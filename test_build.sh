#!/bin/bash

echo "🔧 Session 2: Post-Quantum Build Test"
echo "====================================="

cd /Users/mariano/Desktop/Code/nano-messenger

# Clear previous builds
echo "🧹 Cleaning previous builds..."
cargo clean > /dev/null 2>&1

# Check compilation
echo "🔍 Checking compilation..."
cargo check --lib 2>&1 | head -30

echo ""
echo "🏗️ Attempting to build..."
cargo build --lib 2>&1 | head -30

echo ""
echo "✅ Build test complete"
