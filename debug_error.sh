#!/bin/bash

echo "🔍 Getting Exact Error Details"
echo "============================="
echo ""

cd /Users/mariano/Desktop/Code/nano-messenger

echo "Full compilation error output:"
cargo check --lib 2>&1 | head -30

echo ""
echo "Specifically around line 512:"
sed -n '510,515p' src/config/adaptive.rs | nl -v510

echo ""
echo "More context around the error:"
cargo check --lib 2>&1 | grep -A10 -B5 "adaptive.rs:512"
