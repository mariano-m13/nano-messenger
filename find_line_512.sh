#!/bin/bash

echo "🎯 Finding Exact Line 512 Content"
echo "================================"
echo ""

cd /Users/mariano/Desktop/Code/nano-messenger

echo "Content at line 512:"
sed -n '512p' src/config/adaptive.rs

echo ""
echo "Context around line 512 (lines 510-515):"
sed -n '510,515p' src/config/adaptive.rs | nl -v510

echo ""
echo "Looking for the exact error with more context:"
cargo check --lib 2>&1 | grep -A5 -B5 "adaptive.rs:512:55"
