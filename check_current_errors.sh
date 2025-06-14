#!/bin/bash

echo "🔍 Checking current compilation errors..."
echo "======================================="

cd /Users/mariano/Desktop/Code/nano-messenger

# Clean and build to get fresh errors
echo "Running cargo clean and build..."
cargo clean
cargo build 2>&1 | tee current_errors.log

# Count errors
ERROR_COUNT=$(grep -c "error\[E" current_errors.log || echo "0")
echo -e "\n📊 Total errors: $ERROR_COUNT"

# Show E0753 errors specifically
echo -e "\n🔍 E0753 (doc comment) errors:"
grep -A 5 "error\[E0753\]" current_errors.log | head -30

# Show files with E0753 errors
echo -e "\n📁 Files with doc comment errors:"
grep -B 2 "error\[E0753\]" current_errors.log | grep "src/" | grep -o "src/[^:]*\.rs" | sort | uniq

echo -e "\n✅ Check complete!"
