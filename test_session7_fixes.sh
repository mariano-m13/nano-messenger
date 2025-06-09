#!/bin/bash

echo "🔧 Testing Session 7 Security Validation Fixes"
echo "==============================================="

cd /Users/mariano/Desktop/Code/nano-messenger

echo "Building project..."
cargo build 2>&1 | head -20

echo ""
echo "Testing security validation compilation..."
cargo test --test security_validation --no-run 2>&1 | head -30

echo ""
echo "Testing crypto correctness compilation..."
cargo test crypto_correctness --no-run 2>&1 | head -20

echo ""
echo "Fix verification complete."
