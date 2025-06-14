#!/bin/bash

# Quick compilation diagnostic script
cd /Users/mariano/Desktop/Code/nano-messenger

echo "🔍 COMPILATION DIAGNOSTIC"
echo "========================"

echo "Checking core library compilation..."
cargo check --lib 2>&1 | head -30

echo ""
echo "Checking specific issues..."
echo "Looking for most common errors..."

# Try to identify specific patterns
cargo check --lib 2>&1 | grep -E "(error\[E[0-9]+\]|cannot find|undefined)" | head -10

echo ""
echo "Error count summary:"
cargo check --lib 2>&1 | grep -c "error:"

echo ""
echo "Warning count summary:"
cargo check --lib 2>&1 | grep -c "warning:"
