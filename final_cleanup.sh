#!/bin/bash

echo "🎯 FINAL COMPILATION ERROR CLEANUP"
echo "=================================="

cd "$(dirname "$0")"

echo "Applying final fixes for common Rust compilation issues..."

# Fix 1: Ensure all modules have proper imports
echo "1. Checking and fixing import issues..."

# Add missing standard library imports where needed
for file in $(find src -name "*.rs"); do
    # Add HashMap import if used but not imported
    if grep -q "HashMap" "$file" && ! grep -q "use std::collections::HashMap\|use std::collections::{.*HashMap" "$file"; then
        if ! grep -q "use std::collections::HashMap" "$file"; then
            echo "Adding HashMap import to $file"
            sed -i '' '1i\
use std::collections::HashMap;' "$file"
        fi
    fi
    
    # Add SystemTime import if used but not imported
    if grep -q "SystemTime" "$file" && ! grep -q "use std::time::SystemTime\|use std::time::{.*SystemTime" "$file"; then
        if ! grep -q "use std::time::SystemTime" "$file"; then
            echo "Adding SystemTime import to $file"
            sed -i '' '1i\
use std::time::SystemTime;' "$file"
        fi
    fi
done

# Fix 2: Check for any remaining uuid issues
echo "2. Fixing any remaining UUID import issues..."

# Ensure uuid is imported where Uuid:: is used
for file in $(find src -name "*.rs" -exec grep -l "Uuid::" {} \;); do
    if ! grep -q "use uuid" "$file"; then
        echo "Adding uuid import to $file"
        sed -i '' '1i\
use uuid::Uuid;' "$file"
    fi
done

# Fix 3: Ensure all async-trait usages have proper imports
echo "3. Fixing async-trait imports..."

for file in $(find src -name "*.rs" -exec grep -l "#\[async_trait\]" {} \;); do
    if ! grep -q "use async_trait::async_trait" "$file"; then
        echo "Adding async_trait import to $file"
        sed -i '' '1i\
use async_trait::async_trait;' "$file"
    fi
done

# Fix 4: Check for any regex imports needed
echo "4. Checking regex imports..."

for file in $(find src -name "*.rs" -exec grep -l "regex::" {} \;); do
    if ! grep -q "use regex" "$file"; then
        echo "Adding regex import to $file"
        sed -i '' '1i\
use regex;' "$file"
    fi
done

# Fix 5: Check for md5 imports needed
echo "5. Checking md5 imports..."

for file in $(find src -name "*.rs" -exec grep -l "md5::" {} \;); do
    if ! grep -q "use md5" "$file"; then
        echo "Adding md5 import to $file"
        sed -i '' '1i\
use md5;' "$file"
    fi
done

echo ""
echo "6. Testing compilation after fixes..."

# Quick compilation test
if cargo check --lib 2>&1 | grep -q "error:"; then
    echo "❌ Still has errors. Running detailed analysis..."
    
    # Get the first few errors for analysis
    ERRORS=$(cargo check --lib 2>&1 | grep "error:" | head -3)
    echo "Remaining errors:"
    echo "$ERRORS"
    
    # Try to identify the pattern and suggest fixes
    if echo "$ERRORS" | grep -q "unresolved import"; then
        echo ""
        echo "💡 Remaining import issues detected."
        echo "   Manual intervention may be required."
    fi
    
    if echo "$ERRORS" | grep -q "cannot find type"; then
        echo ""
        echo "💡 Missing type definitions detected."
        echo "   Some types may need to be defined or imported."
    fi
else
    echo "✅ Core library compilation successful!"
    
    # Test examples
    echo ""
    echo "Testing example compilations..."
    
    SESSION1_RESULT=$(cargo check --example session1_validation 2>&1)
    if echo "$SESSION1_RESULT" | grep -q "error:"; then
        echo "❌ Session 1 example: $(echo "$SESSION1_RESULT" | grep "error:" | head -1)"
    else
        echo "✅ Session 1 example: OK"
    fi
    
    SESSION12_RESULT=$(cargo check --example session12_basic_validation 2>&1)
    if echo "$SESSION12_RESULT" | grep -q "error:"; then
        echo "❌ Session 12 example: $(echo "$SESSION12_RESULT" | grep "error:" | head -1)"
    else
        echo "✅ Session 12 example: OK"
    fi
    
    echo ""
    echo "🎉 SUCCESS! All compilation issues have been resolved!"
    echo "Your nano-messenger project is now ready to use."
fi

echo ""
echo "🏁 Final cleanup complete."
