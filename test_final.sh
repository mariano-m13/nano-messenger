#!/bin/bash

echo "Testing nano-messenger after fixing session9 validation..."
cd /Users/mariano/Desktop/Code/nano-messenger

echo "Running cargo test..."
cargo test

echo ""
echo "Test should now compile and run successfully!"
