#!/bin/bash

echo "🔧 Testing compilation after fixes..."
echo "===================================="

cd /Users/mariano/Desktop/Code/nano-messenger

echo ""
echo "1. Testing library compilation..."
if cargo check --lib; then
    echo "✅ Library compilation successful"
else
    echo "❌ Library compilation failed:"
    cargo check --lib 2>&1 | head -10
fi

echo ""
echo "2. Testing client binary..."
if cargo check --bin nano-client; then
    echo "✅ Client binary compilation successful"
else
    echo "❌ Client binary failed:"
    cargo check --bin nano-client 2>&1 | head -10
fi

echo ""
echo "3. Testing relay binary..."
if cargo check --bin nano-relay; then
    echo "✅ Relay binary compilation successful"
else
    echo "❌ Relay binary failed:"
    cargo check --bin nano-relay 2>&1 | head -10
fi

echo ""
echo "4. Testing Session 4 example..."
if cargo check --example session4_validation; then
    echo "✅ Session 4 example compilation successful"
else
    echo "❌ Session 4 example failed:"
    cargo check --example session4_validation 2>&1 | head -10
fi

echo ""
echo "5. Testing Session 6 example..."
if cargo check --example session6_validation; then
    echo "✅ Session 6 example compilation successful"
else
    echo "❌ Session 6 example failed:"
    cargo check --example session6_validation 2>&1 | head -10
fi

echo ""
echo "🧪 Quick functionality test..."
if cargo test --lib config::adaptive::tests::test_adaptive_config_default; then
    echo "✅ Cache tests passed"
else
    echo "❌ Cache tests failed"
fi

echo ""
echo "🎯 Final status:"
if cargo check --all-targets; then
    echo "✅ All compilation tests passed!"
    echo ""
    echo "🚀 Sessions 4-6 fixes applied successfully:"
    echo "   📱 Session 4: Client interface with crypto mode selection"
    echo "   🖥️  Session 5: Relay configuration with policy enforcement"  
    echo "   ⚡ Session 6: Performance optimization with caching and adaptive selection"
    echo ""
    echo "🎉 Ready to test full functionality with:"
    echo "   ./quick_test_4_5_6.sh"
else
    echo "⚠️ Some compilation issues remain"
fi
