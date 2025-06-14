#!/bin/bash

echo "🔍 Capturing compilation errors..."
cd /Users/mariano/Desktop/Code/nano-messenger

# Capture all errors to a file
cargo check --features="image-processing" 2>&1 > compilation_errors_full.txt

# Show the first error with context
echo "First error with context:"
head -20 compilation_errors_full.txt | grep -A 5 -B 5 "error\[E0753\]"

# Count error types
echo -e "\n📊 Error summary:"
grep "error\[E" compilation_errors_full.txt | cut -d']' -f1 | sort | uniq -c | sort -nr

# Show files with E0753 errors
echo -e "\n📁 Files with E0753 errors:"
grep -B2 "error\[E0753\]" compilation_errors_full.txt | grep "^error:" -A2 | head -20
