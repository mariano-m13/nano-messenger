#!/bin/bash

echo "Testing nano-messenger after fixing test issues..."
cd /Users/mariano/Desktop/Code/nano-messenger

echo "Running cargo test..."
cargo test

echo ""
echo "Test compilation should now be successful!"
