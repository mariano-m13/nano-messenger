#!/bin/bash

echo "🔧 Testing Clone Fix for HybridSharedSecret"
echo "=========================================="

cd /Users/mariano/Desktop/Code/nano-messenger

echo "Testing compilation..."
cargo check --lib 2>&1 | head -20

echo ""
echo "Done!"
