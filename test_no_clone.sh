#!/bin/bash

echo "🔧 Testing After Removing Clone Implementation"
echo "============================================="

cd /Users/mariano/Desktop/Code/nano-messenger

echo "Testing compilation..."
cargo check --lib 2>&1 | head -25

echo ""
echo "Done!"
