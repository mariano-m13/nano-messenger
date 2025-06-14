#!/bin/bash

echo "🔧 Session 1: Testing Crypto Foundation Implementation"
echo "======================================================="

cd /Users/mariano/Desktop/Code/nano-messenger

echo "1. Checking if project compiles..."
cargo check 2>&1 | head -20

echo ""
echo "2. Running crypto tests..."
cargo test crypto::tests 2>&1 | head -30

echo ""
echo "3. Running protocol tests (should use new crypto module)..."
cargo test protocol::tests 2>&1 | head -30

echo ""
echo "4. Testing that classical crypto mode works..."
cargo test --lib 2>&1 | head -20

echo ""
echo "Session 1 Status Check Complete!"
