#!/bin/bash

echo "🎯 Running Session 2 Validation Example"
echo "======================================="

cd /Users/mariano/Desktop/Code/nano-messenger

echo "Building and running Session 2 validation..."
cargo run --example session2_validation 2>&1 | head -30

echo ""
echo "Done!"
