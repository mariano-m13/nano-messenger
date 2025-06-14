#!/bin/bash

echo "Checking compilation after initial fixes..."
cd /Users/mariano/Desktop/Code/nano-messenger

# Run cargo check and capture output
cargo check --all-targets 2>&1 | head -200
