#!/bin/bash

echo "🔧 Testing Fix for UnifiedPublicKeys::from_legacy"
echo "=============================================="
echo ""

cd /Users/mariano/Desktop/Code/nano-messenger

echo "Testing library compilation..."
if cargo check --lib 2>&1 | grep -q "error\["; then
    echo "❌ Still has compilation errors:"
    cargo check --lib 2>&1 | grep -A2 "error\[" | head -10
else
    echo "✅ Library compiles successfully!"
fi

echo ""
echo "Testing quick functionality..."
if cargo test --lib crypto::optimizations::tests::test_cache_creation --quiet >/dev/null 2>&1; then
    echo "✅ Cache tests pass"
else
    echo "❌ Cache tests fail"
fi

echo ""
echo "Testing client binary..."
if cargo check --bin nano-client 2>&1 | grep -q "error\["; then
    echo "❌ Client has compilation errors"
else
    echo "✅ Client compiles successfully!"
fi

echo ""
echo "Testing relay binary..."
if cargo check --bin nano-relay 2>&1 | grep -q "error\["; then
    echo "❌ Relay has compilation errors"
else
    echo "✅ Relay compiles successfully!"
fi
