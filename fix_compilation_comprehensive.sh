#!/bin/bash

echo "🔧 COMPREHENSIVE COMPILATION FIX"
echo "================================"

cd /Users/mariano/Desktop/Code/nano-messenger

# Step 1: Clean everything
echo "📦 Step 1: Cleaning build artifacts..."
cargo clean
rm -f Cargo.lock

# Step 2: Update dependencies
echo "📦 Step 2: Updating dependencies..."
cargo update

# Step 3: Check for specific compilation errors
echo "📦 Step 3: Checking compilation errors..."
cargo check --features="image-processing" 2>&1 | tee current_errors.log

# Step 4: Count and display errors
echo -e "\n📊 Error Analysis:"
TOTAL_ERRORS=$(grep -c "error\[E" current_errors.log || echo "0")
echo "Total errors: $TOTAL_ERRORS"

# Show error types
echo -e "\n📋 Error types:"
grep "error\[E" current_errors.log | cut -d']' -f1 | sort | uniq -c | sort -nr || echo "No errors found"

# Show first few errors with context
echo -e "\n📌 First few errors:"
grep -A5 "error\[E" current_errors.log | head -30

echo -e "\n✅ Fix script complete!"
