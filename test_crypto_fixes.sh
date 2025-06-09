#!/bin/bash

echo "🔧 Testing Post-Quantum Crypto Fixes"
echo "===================================="

cd /Users/mariano/Desktop/Code/nano-messenger

echo "🧪 Testing post-quantum crypto module..."
echo "---------------------------------------"
cargo test crypto::post_quantum --lib 2>&1 | head -20

echo ""
echo "🔄 Testing hybrid crypto module..."
echo "---------------------------------"
cargo test crypto::hybrid --lib 2>&1 | head -15

echo ""
echo "🚀 Running Session 2 validation example..."
echo "------------------------------------------"
cargo run --example session2_validation 2>&1 | head -15

echo ""
echo "✅ Test complete!"
