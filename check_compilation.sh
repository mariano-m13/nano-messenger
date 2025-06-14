#!/bin/bash
cd /Users/mariano/Desktop/Code/nano-messenger
echo "🔧 Testing Session 1 Compilation Fixes..."
echo "=========================================="

echo "1. Checking library compilation..."
cargo check --lib 2>&1 | head -20

echo ""
echo "2. Testing if crypto tests compile..."
cargo test --no-run crypto::tests 2>&1 | head -15

echo ""
echo "3. Testing example compilation..."
cargo check --example session1_validation 2>&1 | head -10

echo ""
echo "4. Summary:"
if cargo check --lib >/dev/null 2>&1; then
    echo "✅ Library compiles successfully!"
else
    echo "❌ Library compilation failed"
fi

if cargo test --no-run crypto::tests >/dev/null 2>&1; then
    echo "✅ Crypto tests compile successfully!"
else
    echo "❌ Crypto tests compilation failed"
fi

if cargo check --example session1_validation >/dev/null 2>&1; then
    echo "✅ Session 1 validation example compiles successfully!"
else
    echo "❌ Session 1 validation example compilation failed"
fi
