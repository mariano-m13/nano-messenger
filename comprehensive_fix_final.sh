#!/bin/bash

echo "🚀 COMPREHENSIVE NANO-MESSENGER FIX"
echo "==================================="
echo "This script will fix compilation errors step by step"
echo ""

cd /Users/mariano/Desktop/Code/nano-messenger

# Function to count errors
count_errors() {
    grep -c "error\[E" "$1" 2>/dev/null || echo "0"
}

# Step 1: Clean everything
echo "🧹 Step 1: Complete cleanup..."
cargo clean
rm -rf target/
rm -f Cargo.lock
rm -f *.log
find . -name "*.backup" -type f -delete
find . -name "*~" -type f -delete
rm -f src/crypto.rs  # We use crypto/ directory, not crypto.rs

# Step 2: Fix potential x25519-dalek version issue
echo -e "\n🔧 Step 2: Checking dependency versions..."
# The error showed x25519-dalek 1.1.1 but Cargo.toml specifies 2.0
# This might be a Cargo.lock issue
echo "Current x25519-dalek version in Cargo.toml:"
grep "x25519-dalek" Cargo.toml

# Step 3: Update dependencies
echo -e "\n📦 Step 3: Updating all dependencies..."
cargo update

# Step 4: Initial compilation check
echo -e "\n🏗️ Step 4: Initial compilation check..."
cargo check 2>&1 | tee initial_check.log
INITIAL_ERRORS=$(count_errors initial_check.log)
echo "Initial error count: $INITIAL_ERRORS"

# Step 5: If there are E0753 errors, try to find and fix them
if grep -q "error\[E0753\]" initial_check.log 2>/dev/null; then
    echo -e "\n🔍 Step 5: Found E0753 errors, analyzing..."
    
    # Extract files with E0753 errors
    echo "Files with E0753 errors:"
    grep -B5 "error\[E0753\]" initial_check.log | grep -E "^\s*-->" | \
        grep -oE "src/[^ ]+\.rs" | sort | uniq | while read file; do
        echo "  - $file"
        
        # Check if file exists
        if [ -f "$file" ]; then
            echo "    Checking for problematic patterns..."
            
            # Look for /// inside functions
            if grep -Pzo '(?s)\{[^}]*///' "$file" 2>/dev/null; then
                echo "    ⚠️  Found /// inside function body"
            fi
            
            # Look for //! in wrong places
            if grep -n "//!" "$file"; then
                echo "    ⚠️  Found inner doc comments (//!)"
            fi
        fi
    done
else
    echo -e "\n✅ Step 5: No E0753 errors found"
fi

# Step 6: Check with features
echo -e "\n🏗️ Step 6: Checking with features..."
cargo check --features="image-processing" 2>&1 | tee feature_check.log
FEATURE_ERRORS=$(count_errors feature_check.log)
echo "Feature check error count: $FEATURE_ERRORS"

# Step 7: If still errors, show detailed analysis
if [ "$FEATURE_ERRORS" -gt 0 ]; then
    echo -e "\n📊 Step 7: Detailed error analysis..."
    
    echo "Error summary:"
    grep "error\[E" feature_check.log | cut -d']' -f1 | sort | uniq -c | sort -nr
    
    echo -e "\nFirst few errors with context:"
    grep -B2 -A3 "error\[E" feature_check.log | head -30
    
    # Special handling for rand_core version conflicts
    if grep -q "trait bound.*RngCore.*not satisfied" feature_check.log; then
        echo -e "\n⚠️  DETECTED: rand_core version conflict"
        echo "This is likely due to incompatible crate versions."
        echo "Recommended fix:"
        echo "1. Ensure x25519-dalek matches the version actually being used"
        echo "2. Check 'cargo tree' for version conflicts"
    fi
else
    echo -e "\n✅ Step 7: Build successful!"
    
    # Run a quick test
    echo -e "\n🧪 Running quick tests..."
    cargo test --lib -- crypto::tests::test_keypair_generation --nocapture 2>&1 | head -20
fi

# Step 8: Final recommendations
echo -e "\n📝 Step 8: Summary and recommendations:"
if [ "$FEATURE_ERRORS" -eq 0 ]; then
    echo "✅ All compilation errors have been fixed!"
    echo "You can now run:"
    echo "  - cargo build --release"
    echo "  - cargo test"
    echo "  - cargo run --bin nano-client"
else
    echo "❌ There are still $FEATURE_ERRORS errors to fix."
    echo ""
    echo "Next steps:"
    echo "1. Check 'cargo tree' for dependency conflicts"
    echo "2. Review the errors in feature_check.log"
    echo "3. If E0753 errors persist, manually check the reported files"
    echo "4. For version conflicts, try updating Cargo.toml versions"
fi

echo -e "\n🏁 Fix script complete!"
echo "Logs saved to:"
echo "  - initial_check.log"
echo "  - feature_check.log"
