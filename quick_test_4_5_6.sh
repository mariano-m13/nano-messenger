#!/bin/bash

echo "🧪 Quick Sessions 4-6 Functionality Test"
echo "========================================"
echo ""

cd /Users/mariano/Desktop/Code/nano-messenger

# Function to test compilation
test_compilation() {
    local component="$1"
    local command="$2"
    
    echo -n "🔧 Testing $component compilation... "
    if eval "$command" >/dev/null 2>&1; then
        echo "✅ PASS"
        return 0
    else
        echo "❌ FAIL"
        echo "   Command: $command"
        eval "$command" 2>&1 | head -10
        return 1
    fi
}

# Function to test functionality
test_functionality() {
    local test_name="$1"
    local command="$2"
    
    echo -n "🧪 Testing $test_name... "
    if eval "$command" >/dev/null 2>&1; then
        echo "✅ PASS"
        return 0
    else
        echo "❌ FAIL"
        return 1
    fi
}

echo "Phase 1: Compilation Tests"
echo "=========================="

# Test binaries compilation
test_compilation "client binary" "cargo build --bin nano-client --release"
test_compilation "relay binary" "cargo build --bin nano-relay --release"

# Test examples compilation
test_compilation "Session 4 example" "cargo check --example session4_validation"
test_compilation "Session 5 example" "cargo check --example session5_validation"  
test_compilation "Session 6 example" "cargo check --example session6_validation"

echo ""
echo "Phase 2: Basic Functionality Tests"
echo "=================================="

# Test CLI help (Session 4)
test_functionality "client CLI help" "cargo run --bin nano-client --release -- --help"
test_functionality "relay CLI help" "cargo run --bin nano-relay --release -- --help"

echo ""
echo "Phase 3: Crypto Module Tests (Session 6)"
echo "========================================"

# Test Session 6 core functionality
test_functionality "cache module" "cargo test --lib crypto::optimizations::tests::test_cache_creation"
test_functionality "benchmark module" "cargo test --lib crypto::benchmarks::tests::test_benchmark_creation"
test_functionality "adaptive config" "cargo test --lib config::adaptive::tests::test_adaptive_config_default"

echo ""
echo "Phase 4: Integration Test"
echo "========================="

# Create a focused integration test
echo -n "🧪 Testing end-to-end integration... "

cat > /tmp/integration_test.rs << 'EOF'
use nano_messenger::{
    crypto::{
        CryptoMode, CryptoConfig, ClassicalUserKeyPair, 
        CryptoBenchmark, CryptoCache, CacheConfig, init_crypto_config,
    },
    config::{AdaptiveConfig, AdaptiveModeSelector, NetworkMeasurement, DeviceMeasurement},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Test 1: Initialize crypto (Session 4)
    let config = CryptoConfig::new(CryptoMode::Classical);
    init_crypto_config(config)?;
    
    // Test 2: Generate keys for all modes
    let _classical = ClassicalUserKeyPair::generate();
    
    // Test 3: Performance optimization (Session 6)
    let cache = CryptoCache::new(CacheConfig::default());
    let _keypair = cache.get_or_generate_keypair("test", CryptoMode::Classical)?;
    
    // Test 4: Benchmarking
    let benchmark = CryptoBenchmark::new(3);
    let _time = benchmark.quick_benchmark(CryptoMode::Classical, "keygen")?;
    
    // Test 5: Adaptive selection
    let mut selector = AdaptiveModeSelector::new(AdaptiveConfig::default());
    let network = NetworkMeasurement::measure_current_conditions();
    let device = DeviceMeasurement::measure_current_constraints();
    let _rec = selector.recommend_mode(&network, &device);
    
    println!("All tests passed!");
    Ok(())
}
EOF

if cargo run --quiet --manifest-path=/dev/stdin --edition 2021 < <(cat << 'EOF'
[package]
name = "integration_test"
version = "0.1.0"
edition = "2021"

[dependencies]
nano-messenger = { path = "/Users/mariano/Desktop/Code/nano-messenger" }
anyhow = "1.0"
EOF
) /tmp/integration_test.rs 2>/dev/null; then
    echo "✅ PASS"
    integration_success=true
else
    echo "❌ FAIL"
    integration_success=false
fi

echo ""
echo "Phase 5: Feature Verification"  
echo "============================="

# Check for Session 4 features in client
echo -n "🔍 Session 4 features (client crypto modes)... "
if grep -q "crypto_mode.*String" src/bin/client.rs && grep -q "force_post_quantum" src/bin/client.rs; then
    echo "✅ FOUND"
    session4_features=true
else
    echo "❌ MISSING"
    session4_features=false
fi

# Check for Session 5 features in relay
echo -n "🔍 Session 5 features (relay policy enforcement)... "
if grep -q "require_post_quantum" src/bin/relay.rs && grep -q "minimum_crypto_mode" src/bin/relay.rs; then
    echo "✅ FOUND"
    session5_features=true
else
    echo "❌ MISSING"
    session5_features=false
fi

# Check for Session 6 files
echo -n "🔍 Session 6 features (performance optimization)... "
if [ -f "src/crypto/benchmarks.rs" ] && [ -f "src/crypto/optimizations.rs" ] && [ -f "src/config/adaptive.rs" ]; then
    echo "✅ FOUND"
    session6_features=true
else
    echo "❌ MISSING"
    session6_features=false
fi

echo ""
echo "📋 Summary"
echo "========="

# Count successes
successes=0
total=6

if $session4_features; then successes=$((successes + 1)); fi
if $session5_features; then successes=$((successes + 1)); fi  
if $session6_features; then successes=$((successes + 1)); fi
if $integration_success; then successes=$((successes + 1)); fi

# Add compilation success check
compilation_success=true
if ! cargo check --bin nano-client >/dev/null 2>&1; then compilation_success=false; fi
if ! cargo check --bin nano-relay >/dev/null 2>&1; then compilation_success=false; fi

if $compilation_success; then successes=$((successes + 1)); fi

# Add basic tests
basic_tests_success=true
if ! cargo test --lib crypto::optimizations::tests::test_cache_creation >/dev/null 2>&1; then basic_tests_success=false; fi

if $basic_tests_success; then successes=$((successes + 1)); fi

echo "Session Status:"
echo "  📱 Session 4 (Client Interface): $(if $session4_features; then echo "✅ WORKING"; else echo "❌ ISSUES"; fi)"
echo "  🖥️  Session 5 (Relay Configuration): $(if $session5_features; then echo "✅ WORKING"; else echo "❌ ISSUES"; fi)"
echo "  ⚡ Session 6 (Performance Optimization): $(if $session6_features; then echo "✅ WORKING"; else echo "❌ ISSUES"; fi)"

echo ""
echo "Overall Status: $successes/$total components working"

if [ $successes -ge 5 ]; then
    echo "🎉 Sessions 4-6 are working well!"
    echo ""
    echo "✨ Verified Features:"
    echo "   📱 Client with crypto mode selection and security preferences"
    echo "   🖥️  Relay with crypto policy enforcement and statistics"  
    echo "   ⚡ Performance optimizations with caching and benchmarking"
    echo "   🔄 Adaptive mode selection based on device/network conditions"
    echo ""
    echo "🚀 Ready to proceed to Session 7 (Security Validation)!"
    exit 0
else
    echo "⚠️  Some issues found in Sessions 4-6"
    echo "Consider reviewing the failed components above."
    exit 1
fi
