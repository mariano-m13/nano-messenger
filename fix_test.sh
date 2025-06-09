#!/bin/bash

echo "🔧 Testing compilation after fixes..."
echo "===================================="

cd /Users/mariano/Desktop/Code/nano-messenger

echo "1. Testing library compilation..."
if cargo check --lib >/dev/null 2>&1; then
    echo "   ✅ Library compiles successfully!"
else
    echo "   ❌ Library compilation failed:"
    cargo check --lib 2>&1 | head -10
fi

echo ""
echo "2. Testing client binary..."
if cargo check --bin nano-client >/dev/null 2>&1; then
    echo "   ✅ Client binary compiles!"
else
    echo "   ❌ Client binary failed:"
    cargo check --bin nano-client 2>&1 | head -10
fi

echo ""
echo "3. Testing relay binary..."
if cargo check --bin nano-relay >/dev/null 2>&1; then
    echo "   ✅ Relay binary compiles!"
else
    echo "   ❌ Relay binary failed:"
    cargo check --bin nano-relay 2>&1 | head -10
fi

echo ""
echo "4. Testing Session 4 example..."
if cargo check --example session4_validation >/dev/null 2>&1; then
    echo "   ✅ Session 4 example compiles!"
else
    echo "   ❌ Session 4 example failed:"
    cargo check --example session4_validation 2>&1 | head -10
fi

echo ""
echo "5. Testing Session 6 example..."
if cargo check --example session6_validation >/dev/null 2>&1; then
    echo "   ✅ Session 6 example compiles!"
else
    echo "   ❌ Session 6 example failed:"
    cargo check --example session6_validation 2>&1 | head -10
fi

echo ""
echo "🧪 Quick functionality test..."
if cargo test --lib crypto::optimizations::tests::test_cache_creation >/dev/null 2>&1; then
    echo "   ✅ Cache tests pass!"
else
    echo "   ❌ Cache tests failed"
fi

echo ""
echo "🎯 Final status:"
if cargo check --lib >/dev/null 2>&1 && cargo check --bins >/dev/null 2>&1; then
    echo "   🎉 All compilation successful! Ready to run full tests."
    echo ""
    echo "   Next steps:"
    echo "     ./quick_test_4_5_6.sh     # Quick functionality test"
    echo "     ./demo_sessions_4_5_6.sh  # Live demo"
else
    echo "   ⚠️  Some compilation issues remain"
fi
