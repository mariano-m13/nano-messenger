#!/bin/bash

# Test script to check compilation
echo "Testing nano-messenger compilation..."

cd /Users/mariano/Desktop/Code/nano-messenger

echo "Running cargo check..."
cargo check 2>&1 | tee compile_output.txt

echo ""
echo "Compilation test completed. Check the output above for any remaining issues."
