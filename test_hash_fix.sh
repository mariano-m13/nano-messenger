#!/bin/bash

echo "🔧 Testing Fix for CryptoMode Hash Trait"
echo "======================================="
echo ""

cd /Users/mariano/Desktop/Code/nano-messenger

echo "Testing library compilation..."
if cargo check --lib 2>&1 | grep -q "error\["; then
    echo "❌ Still has compilation errors:"
    cargo check --lib 2>&1 | grep -A1 "error\[" | head -8
    echo ""
    echo "Checking for specific Hash trait error..."
    if cargo check --lib 2>&1 | grep -q "Hash"; then
        echo "⚠️  Still seeing Hash trait issues"
    else
        echo "✅ Hash trait issue resolved"
    fi
else
    echo "✅ Library compiles successfully!"
    
    echo ""
    echo "Testing some basic functionality..."
    
    echo -n "   🧪 Basic crypto test: "
    if cargo test --lib crypto::config::tests::test_crypto_mode_parsing --quiet >/dev/null 2>&1; then
        echo "✅ PASS"
    else
        echo "❌ FAIL"
    fi
    
    echo -n "   📊 Benchmark test: "
    if cargo test --lib crypto::benchmarks::tests::test_benchmark_creation --quiet >/dev/null 2>&1; then
        echo "✅ PASS"
    else
        echo "❌ FAIL"
    fi
    
    echo -n "   🏃 Cache test: "
    if cargo test --lib crypto::optimizations::tests::test_cache_creation --quiet >/dev/null 2>&1; then
        echo "✅ PASS"
    else
        echo "❌ FAIL"
    fi
fi

echo ""
echo "Testing binaries..."
echo -n "   📱 Client: "
if cargo check --bin nano-client --quiet >/dev/null 2>&1; then
    echo "✅ COMPILES"
else
    echo "❌ FAILS"
fi

echo -n "   🖥️  Relay: "
if cargo check --bin nano-relay --quiet >/dev/null 2>&1; then
    echo "✅ COMPILES"
else
    echo "❌ FAILS"
fi

echo ""
echo "Testing examples..."
echo -n "   📱 Session 4: "
if cargo check --example session4_validation --quiet >/dev/null 2>&1; then
    echo "✅ COMPILES"
else
    echo "❌ FAILS"
fi

echo -n "   ⚡ Session 6: "
if cargo check --example session6_validation --quiet >/dev/null 2>&1; then
    echo "✅ COMPILES"
else
    echo "❌ FAILS"
fi
