#!/bin/bash

echo "Testing nano-messenger compilation after fixes..."
cd /Users/mariano/Desktop/Code/nano-messenger

echo "Running cargo check..."
cargo check

echo ""
echo "If successful, the project should compile without errors or warnings!"
