#!/bin/bash

echo "Testing specific failing tests after Arc conversion fixes..."
cd /Users/mariano/Desktop/Code/nano-messenger

echo "Running just the media transfer tests..."
cargo test media::transfer::tests --lib

echo ""
echo "Running the media mod tests..."
cargo test media::tests --lib

echo ""
echo "If these pass, all memory safety and type issues have been resolved!"
