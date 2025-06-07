#!/bin/bash

echo "🔧 Testing nano-messenger build..."
echo

# Clean previous build artifacts
echo "🧹 Cleaning previous build..."
cargo clean

echo
echo "📦 Updating dependencies..."
cargo update

echo
echo "🔍 Checking code compilation..."
cargo check

echo
echo "🏗️ Building project..."
cargo build

echo
echo "✅ Build test complete!"
