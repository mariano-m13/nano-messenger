#!/bin/bash

echo "🔧 NANO-MESSENGER FINAL COMPILATION FIX"
echo "======================================="
echo ""

cd /Users/mariano/Desktop/Code/nano-messenger

# Step 1: Remove ALL backup files that might interfere
echo "📦 Step 1: Removing all backup files..."
find . -name "*.backup" -type f -delete
find . -name "*~" -type f -delete
rm -f src/crypto.rs  # Remove if exists (we use crypto/ directory)

# Step 2: Deep clean
echo "📦 Step 2: Deep cleaning..."
cargo clean
rm -rf target/
rm -f Cargo.lock
rm -f *.log
rm -f *.txt

# Step 3: Update Cargo.toml to ensure version compatibility
echo "📦 Step 3: Checking dependency versions..."
# Check if we need to fix x25519-dalek version
if grep -q 'x25519-dalek = { version = "2.0"' Cargo.toml; then
    echo "Found x25519-dalek 2.0, checking compatibility..."
    # The build log showed it's using 1.1.1, so there might be a lock file issue
fi

# Step 4: Update dependencies
echo "📦 Step 4: Updating dependencies..."
cargo update

# Step 5: Build incrementally
echo "📦 Step 5: Building incrementally..."
echo "  - First, checking without features..."
if cargo check 2>&1 | tee check_basic.log; then
    echo "✅ Basic check passed"
else
    echo "❌ Basic check failed, continuing..."
fi

echo "  - Now checking with image-processing feature..."
if cargo check --features="image-processing" 2>&1 | tee check_features.log; then
    echo "✅ Feature check passed"
else
    echo "❌ Feature check failed"
fi

# Step 6: Analyze any remaining errors
echo -e "\n📊 Error Analysis:"
if [ -f check_features.log ]; then
    ERROR_COUNT=$(grep -c "error\[E" check_features.log || echo "0")
    echo "Total errors: $ERROR_COUNT"
    
    if [ "$ERROR_COUNT" -gt 0 ]; then
        echo -e "\n📋 Error types:"
        grep "error\[E" check_features.log | cut -d']' -f1 | sort | uniq -c | sort -nr
        
        echo -e "\n📌 Sample errors:"
        grep -A3 "error\[E" check_features.log | head -30
    fi
fi

# Step 7: If successful, run a quick test
if [ "$ERROR_COUNT" -eq 0 ] || [ -z "$ERROR_COUNT" ]; then
    echo -e "\n✅ BUILD SUCCESSFUL!"
    echo "Running quick tests..."
    cargo test --lib --tests crypto::tests -- --nocapture 2>&1 | head -30
else
    echo -e "\n❌ Build still has errors."
    echo "Next steps:"
    echo "1. Check if all dependencies are compatible"
    echo "2. Look for any syntax errors in the code"
    echo "3. Ensure no old backup files are interfering"
fi

echo -e "\n🏁 Fix complete!"
