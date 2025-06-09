#!/bin/bash

echo "🎯 Session 2: Comprehensive Test After Crypto Fixes"
echo "=================================================="

cd /Users/mariano/Desktop/Code/nano-messenger

echo "🔧 Building project..."
echo "---------------------"
cargo build --lib --quiet

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
else
    echo "❌ Build failed!"
    exit 1
fi

echo ""
echo "🧪 Running focused crypto fixes test..."
echo "--------------------------------------"
cargo run --example test_crypto_fixes

echo ""
echo "🔬 Running post-quantum unit tests..."
echo "------------------------------------"
cargo test crypto::post_quantum --lib

echo ""
echo "🔄 Running hybrid crypto unit tests..."
echo "-------------------------------------"
cargo test crypto::hybrid --lib

echo ""
echo "🚀 Running Session 2 validation example..."
echo "------------------------------------------"
cargo run --example session2_validation

echo ""
echo "✅ All tests complete!"
echo ""
echo "🎉 Session 2 validation finished!"
