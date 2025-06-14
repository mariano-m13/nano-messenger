#!/bin/bash

cd /Users/mariano/Desktop/Code/nano-messenger

echo "Capturing current compilation errors..."
cargo build 2>&1 | head -200 > current_build_errors.txt

echo "First 50 lines of errors:"
head -50 current_build_errors.txt
