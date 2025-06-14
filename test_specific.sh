#!/bin/bash

echo "Testing specific failing tests after memory safety fixes..."
cd /Users/mariano/Desktop/Code/nano-messenger

echo "Running just the media transfer tests..."
cargo test media::transfer::tests --lib

echo ""
echo "If these pass, the memory safety issue has been resolved!"
