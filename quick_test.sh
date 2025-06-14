#!/bin/bash

# Quick Test Script to Verify Core Functionality
cd /Users/mariano/Desktop/Code/nano-messenger

echo "🔧 NANO-MESSENGER QUICK TEST & FIX"
echo "=================================="

echo "1. Testing basic compilation..."
echo "------------------------------"

# Test if library compiles
cargo check --lib 2>/dev/null
if [ $? -eq 0 ]; then
    echo "✅ Library compiles successfully"
else
    echo "❌ Library compilation issues detected"
    echo "Checking specific errors..."
    cargo check --lib 2>&1 | head -10
fi

echo ""
echo "2. Testing simplified examples..."
echo "--------------------------------"

# Test Session 1 (should work)
echo "Testing Session 1..."
cargo check --example session1_validation 2>/dev/null
if [ $? -eq 0 ]; then
    echo "✅ Session 1 example compiles"
else
    echo "❌ Session 1 has issues"
fi

# Test our fixed Session 12
echo "Testing fixed Session 12..."
cargo check --example session12_validation_fixed 2>/dev/null
if [ $? -eq 0 ]; then
    echo "✅ Fixed Session 12 example compiles"
else
    echo "❌ Fixed Session 12 needs more work"
    echo "Error details:"
    cargo check --example session12_validation_fixed 2>&1 | head -5
fi

echo ""
echo "3. Running quick functional tests..."
echo "-----------------------------------"

# Try to run Session 1
echo "Running Session 1 validation..."
timeout 30s cargo run --example session1_validation 2>/dev/null
if [ $? -eq 0 ]; then
    echo "✅ Session 1 runs successfully"
else
    echo "⚠️  Session 1 execution issues"
fi

echo ""
echo "4. Making test scripts executable..."
echo "-----------------------------------"

chmod +x test_all_sessions_fixed.sh
chmod +x diagnostic.sh

echo "✅ Scripts made executable"

echo ""
echo "5. Testing fixed comprehensive script..."
echo "--------------------------------------"

# Run a quick test of our fixed script
echo "Testing basic functionality of fixed test script..."
if [ -f "test_all_sessions_fixed.sh" ]; then
    echo "✅ Fixed test script exists"
    # Test just the compilation check part
    ./test_all_sessions_fixed.sh --help > /dev/null 2>&1
    if [ $? -eq 0 ]; then
        echo "✅ Fixed test script help works"
    else
        echo "⚠️  Fixed test script may need adjustments"
    fi
else
    echo "❌ Fixed test script not found"
fi

echo ""
echo "🏁 Quick test complete!"
echo "Recommendations:"
echo "  - Use 'cargo check --example session1_validation' to test basic functionality"
echo "  - Use './test_all_sessions_fixed.sh --quick' for quick validation"
echo "  - Use 'cargo run --example session12_validation_fixed' for simplified Session 12 test"
