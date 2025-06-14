#!/bin/bash

echo "🔍 CHECKING CURRENT COMPILATION STATUS"
echo "======================================"

cd "$(dirname "$0")"

echo "📊 Running cargo check on core library..."
cargo check --lib 2>&1 | head -20

echo ""
echo "📊 Testing basic session examples..."
echo "Testing session1_validation..."
cargo check --example session1_validation 2>&1 | head -10

echo ""
echo "Testing session12_basic_validation..."
cargo check --example session12_basic_validation 2>&1 | head -10

echo ""
echo "🏁 Current Status Summary"
echo "========================"
