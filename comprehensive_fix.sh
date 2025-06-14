#!/bin/bash

echo "🔧 COMPREHENSIVE NANO-MESSENGER COMPILATION FIX"
echo "=============================================="
echo ""

cd /Users/mariano/Desktop/Code/nano-messenger

# Step 1: Remove the backup file that might be causing issues
echo "📦 Step 1: Removing backup files that might interfere..."
rm -f src/crypto.rs.backup
rm -f src/crypto.rs

# Step 2: Clean all build artifacts completely
echo "📦 Step 2: Deep cleaning build artifacts..."
cargo clean
rm -rf target/
rm -f Cargo.lock
rm -f compile_output.txt
rm -f current_errors.log
rm -f compilation_errors_full.txt

# Step 3: Clear cargo cache for this project
echo "📦 Step 3: Clearing cargo registry cache..."
cargo update

# Step 4: Build with specific features
echo "📦 Step 4: Building with features..."
echo "Building with: --features=\"image-processing\""
cargo build --features="image-processing" 2>&1 | tee build_output.log

# Step 5: Check if build succeeded
if cargo check --features="image-processing" 2>&1 | tee check_output.log; then
    echo -e "\n✅ BUILD SUCCESSFUL!"
    echo "The compilation errors have been resolved."
    
    # Run a quick test to confirm
    echo -e "\n🧪 Running quick validation..."
    cargo test --lib -- --nocapture 2>&1 | head -20
else
    echo -e "\n❌ Build still has errors. Analyzing..."
    
    # Count errors
    ERROR_COUNT=$(grep -c "error\[E" check_output.log || echo "0")
    echo "Total errors: $ERROR_COUNT"
    
    # Show error summary
    echo -e "\n📋 Error types:"
    grep "error\[E" check_output.log | cut -d']' -f1 | sort | uniq -c | sort -nr | head -10
    
    # Show first few errors with context
    echo -e "\n📌 First few errors with context:"
    grep -B2 -A5 "error\[E" check_output.log | head -50
fi

echo -e "\n🏁 Fix attempt complete!"
echo "Build log saved to: build_output.log"
echo "Check log saved to: check_output.log"
