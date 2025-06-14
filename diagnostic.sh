#!/bin/bash

# Quick Diagnostic Script for Nano-Messenger
# Check basic compilation and identify specific issues

cd /Users/mariano/Desktop/Code/nano-messenger

echo "🔍 NANO-MESSENGER DIAGNOSTIC SCRIPT"
echo "===================================="

echo "1. Testing basic compilation..."
echo "------------------------------"

# Test basic library compilation
echo "Checking if library compiles..."
cargo check --lib 2>&1 | head -20
echo ""

echo "2. Testing specific examples..."
echo "------------------------------"

# Test a simple example first
echo "Testing Session 1 example..."
cargo check --example session1_validation 2>&1 | head -10
echo ""

echo "Testing Session 12 example..."
cargo check --example session12_validation 2>&1 | head -15
echo ""

echo "3. Checking dependencies..."
echo "-------------------------"
echo "Checking for missing modules..."

# Check for specific missing modules
if [ ! -f "src/media/security/mod.rs" ]; then
    echo "❌ Missing: src/media/security/mod.rs"
else
    echo "✅ Found: src/media/security/mod.rs"
fi

if [ ! -f "src/media/compliance/mod.rs" ]; then
    echo "❌ Missing: src/media/compliance/mod.rs"
else
    echo "✅ Found: src/media/compliance/mod.rs"
fi

echo ""
echo "4. Quick fixes..."
echo "----------------"

# Try to identify and suggest quick fixes
echo "Checking for common compilation issues..."

# Check if all needed trait implementations exist
echo "Testing basic crypto compilation..."
cargo check --example session1_validation 2>/dev/null
if [ $? -eq 0 ]; then
    echo "✅ Session 1 compiles successfully"
else
    echo "❌ Session 1 has compilation issues"
fi

echo ""
echo "🏁 Diagnostic complete!"
