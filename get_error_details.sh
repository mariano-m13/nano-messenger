#!/bin/bash

echo "🔍 Getting Full Error Details for adaptive.rs"
echo "============================================="
echo ""

cd /Users/mariano/Desktop/Code/nano-messenger

echo "Full compilation errors from library:"
cargo check --lib 2>&1 | grep -A5 -B2 "src/config/adaptive.rs:512"

echo ""
echo "Context around the error:"
cargo check --lib 2>&1 | grep -A10 -B5 "mismatched types"
