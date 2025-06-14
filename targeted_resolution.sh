#!/bin/bash

echo "🎯 TARGETED ERROR RESOLUTION"
echo "============================"

cd "$(dirname "$0")"

echo "Step 1: Getting first 20 compilation errors for analysis..."

# Get first 20 errors with context
cargo check --lib 2>&1 | grep -A 2 -B 1 "error\[" | head -40 > errors_sample.log

echo "First few errors:"
cat errors_sample.log

echo ""
echo "Step 2: Applying targeted fixes based on common patterns..."

# Fix 1: Add missing imports systematically
echo "Fixing import issues..."

# Add missing regex import to HIPAA module
if grep -q "regex::" src/media/compliance/hipaa.rs && ! grep -q "use regex" src/media/compliance/hipaa.rs; then
    echo "Adding regex import to HIPAA module"
    sed -i '' '/use thiserror::Error;/a\
use regex::Regex;
' src/media/compliance/hipaa.rs
fi

# Add missing md5 import to HIPAA module
if grep -q "md5::" src/media/compliance/hipaa.rs && ! grep -q "use md5" src/media/compliance/hipaa.rs; then
    echo "Adding md5 import to HIPAA module"
    sed -i '' '/use thiserror::Error;/a\
use md5;
' src/media/compliance/hipaa.rs
fi

# Fix 2: Ensure all modules that need UUID have it
echo "Fixing UUID imports..."
for file in src/media/compliance/auditing.rs src/media/security/forensics.rs src/media/security/encryption.rs; do
    if [ -f "$file" ] && grep -q "Uuid::" "$file" && ! grep -q "use uuid" "$file"; then
        echo "Adding UUID import to $file"
        sed -i '' '1i\
use uuid::Uuid;
' "$file"
    fi
done

# Fix 3: Check for missing standard library imports
echo "Fixing standard library imports..."

# Add Duration import where needed
for file in $(find src -name "*.rs" -exec grep -l "Duration::" {} \;); do
    if ! grep -q "use std::time::Duration\|use std::time::{.*Duration" "$file"; then
        echo "Adding Duration import to $file"
        sed -i '' '1i\
use std::time::Duration;
' "$file"
    fi
done

# Fix 4: Check async-trait usage
echo "Fixing async-trait issues..."
for file in $(find src -name "*.rs" -exec grep -l "#\[async_trait\]" {} \;); do
    if ! grep -q "use async_trait::async_trait" "$file"; then
        echo "Adding async_trait import to $file"
        sed -i '' '/use thiserror::Error;/a\
use async_trait::async_trait;
' "$file"
    fi
done

echo ""
echo "Step 3: Testing core modules individually..."

# Test crypto module
echo "Testing crypto module..."
CRYPTO_RESULT=$(cargo check --lib 2>&1 | grep "src/crypto" | head -2)
if [ -n "$CRYPTO_RESULT" ]; then
    echo "❌ Crypto module issues:"
    echo "$CRYPTO_RESULT"
else
    echo "✅ Crypto module: OK"
fi

# Test media module
echo "Testing media module..."
MEDIA_RESULT=$(cargo check --lib 2>&1 | grep "src/media" | head -2)
if [ -n "$MEDIA_RESULT" ]; then
    echo "❌ Media module issues:"
    echo "$MEDIA_RESULT"
else
    echo "✅ Media module: OK"
fi

echo ""
echo "Step 4: Final compilation test..."

ERROR_COUNT=$(cargo check --lib 2>&1 | grep -c "error\[" || echo "0")

if [ "$ERROR_COUNT" -gt "0" ]; then
    echo "❌ Still $ERROR_COUNT errors remaining"
    
    echo ""
    echo "Showing top 5 errors with solutions:"
    
    # Get specific error types and suggest fixes
    cargo check --lib 2>&1 | grep -A 1 "error\[" | head -10 | while read line; do
        if echo "$line" | grep -q "unresolved import"; then
            echo "🔧 Import Error: $line"
            echo "   Solution: Add missing import statement"
        elif echo "$line" | grep -q "cannot find type"; then
            echo "🔧 Type Error: $line"
            echo "   Solution: Define type or add use statement"
        elif echo "$line" | grep -q "trait.*not found"; then
            echo "🔧 Trait Error: $line"
            echo "   Solution: Import trait or add dependency"
        fi
    done
    
else
    echo "✅ All compilation errors resolved!"
    
    # Test examples
    echo ""
    echo "Testing examples..."
    
    if cargo check --example session1_validation 2>&1 | grep -q "error:"; then
        echo "❌ Session 1 example needs fixes"
    else
        echo "✅ Session 1 example: OK"
    fi
    
    if cargo check --example session12_basic_validation 2>&1 | grep -q "error:"; then
        echo "❌ Session 12 example needs fixes"
    else
        echo "✅ Session 12 example: OK"
    fi
fi

# Clean up
rm -f errors_sample.log

echo ""
echo "🏁 Targeted resolution complete."
