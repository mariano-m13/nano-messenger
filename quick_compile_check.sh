#!/bin/bash
cd /Users/mariano/Desktop/Code/nano-messenger
echo "Running cargo check to identify remaining compilation errors..."
cargo check 2>&1 | head -50
