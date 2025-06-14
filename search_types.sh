#!/bin/bash

echo "🔍 Searching for Type Issues in adaptive.rs"
echo "=========================================="
echo ""

cd /Users/mariano/Desktop/Code/nano-messenger

echo "Searching for potential type mismatch patterns:"
echo ""

echo "1. Any remaining .sum() operations:"
grep -n "\.sum" src/config/adaptive.rs || echo "   No .sum() found"

echo ""
echo "2. Looking for iterator chains with potential issues:"
grep -n "\.iter()" src/config/adaptive.rs | head -5

echo ""
echo "3. Looking for type annotations:"
grep -n ": f64" src/config/adaptive.rs | head -5

echo ""
echo "4. Getting the exact error with line numbers:"
cargo check --lib 2>&1 | grep -A3 -B3 "512"
