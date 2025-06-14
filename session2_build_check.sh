#!/bin/bash

# Session 2 Build Check Script
echo "🔧 Session 2: Post-Quantum Dependencies Build Check"
echo "=============================================="

cd /Users/mariano/Desktop/Code/nano-messenger

echo "📦 Checking if post-quantum dependencies are available..."
cargo check --lib 2>&1 | head -20

echo "🧪 Testing crypto modules specifically..."
cargo test crypto::post_quantum --lib --no-run 2>&1 | head -10

echo "🔍 Testing crypto modules compilation..."
cargo test crypto::hybrid --lib --no-run 2>&1 | head -10

echo "✅ Build check complete"
