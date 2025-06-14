#!/bin/bash

echo "🔍 Getting error locations..."
cd /Users/mariano/Desktop/Code/nano-messenger

# Get the first 100 lines of errors to see the file locations
cargo check --features="image-processing" 2>&1 | head -100 | grep -E "(error\[E0753\]|-->)"
