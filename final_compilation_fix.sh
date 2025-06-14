#!/bin/bash

echo "🎯 FINAL COMPILATION ERROR FIX"
echo "==============================" 

cd "$(dirname "$0")"

echo "Identifying and fixing the last compilation error..."

# Get the specific error
echo "Getting detailed error output..."
ERROR_OUTPUT=$(cargo check --lib 2>&1)

echo "$ERROR_OUTPUT" > final_error.log

# Extract the most critical error
MAIN_ERROR=$(echo "$ERROR_OUTPUT" | grep "error\[" | head -1)
echo "Main error: $MAIN_ERROR"

# Check for common issues and fix them

# Issue 1: Missing uuid imports
if echo "$ERROR_OUTPUT" | grep -q "uuid::Uuid"; then
    echo "Fixing missing uuid imports..."
    
    # Add uuid imports where needed
    for file in $(find src -name "*.rs" -exec grep -l "Uuid::" {} \;); do
        if ! grep -q "use uuid" "$file"; then
            echo "Adding uuid import to $file"
            sed -i '' '1i\
use uuid;' "$file"
        fi
    done
fi

# Issue 2: Check for specific compilation problems in media modules
if echo "$ERROR_OUTPUT" | grep -q "src/media"; then
    echo "Found error in media module, applying fixes..."
    
    # Common fix: ensure all required imports are present
    for file in src/media/compliance/mod.rs src/media/security/mod.rs; do
        if [ -f "$file" ]; then
            echo "Checking imports in $file..."
            
            # Add standard imports if missing
            if grep -q "HashMap" "$file" && ! grep -q "use std::collections::HashMap" "$file"; then
                sed -i '' '1i\
use std::collections::HashMap;' "$file"
            fi
            
            if grep -q "SystemTime" "$file" && ! grep -q "use std::time::SystemTime" "$file"; then
                sed -i '' '1i\
use std::time::SystemTime;' "$file"
            fi
            
            if grep -q "Duration" "$file" && ! grep -q "use std::time::Duration" "$file"; then
                sed -i '' '1i\
use std::time::Duration;' "$file"
            fi
        fi
    done
fi

# Issue 3: Check for trait/type issues
if echo "$ERROR_OUTPUT" | grep -q "trait.*not found" || echo "$ERROR_OUTPUT" | grep -q "type.*not found"; then
    echo "Fixing missing trait/type definitions..."
    
    # Extract what's missing
    MISSING_ITEM=$(echo "$ERROR_OUTPUT" | grep -o "trait \`[^']*\`\|type \`[^']*\`" | head -1 | sed "s/[^']*'\([^']*\)'.*/\1/")
    echo "Missing item: $MISSING_ITEM"
    
    # Common fixes
    case "$MISSING_ITEM" in
        "Send" | "Sync")
            echo "Standard traits should be available - checking for import issues"
            ;;
        "Clone" | "Debug")
            echo "Derive traits issue - checking for missing derives"
            ;;
    esac
fi

# Issue 4: Fix specific known problematic patterns
echo "Applying specific pattern fixes..."

# Fix potential async-trait issues
if echo "$ERROR_OUTPUT" | grep -q "async-trait"; then
    echo "Checking async-trait usage..."
    for file in $(find src -name "*.rs" -exec grep -l "#\[async_trait\]" {} \;); do
        if ! grep -q "use async_trait::async_trait" "$file"; then
            echo "Adding async_trait import to $file"
            sed -i '' '1i\
use async_trait::async_trait;' "$file"
        fi
    done
fi

# Issue 5: Check for Cargo.toml dependency issues
echo "Verifying dependencies..."

# Ensure all required dependencies are in Cargo.toml
REQUIRED_DEPS="uuid tokio serde thiserror async-trait"
for dep in $REQUIRED_DEPS; do
    if ! grep -q "^$dep" Cargo.toml; then
        echo "Warning: $dep might be missing from Cargo.toml"
    fi
done

# Try a targeted build test
echo ""
echo "Testing compilation..."

# Test specific modules individually
echo "Testing crypto module..."
if cargo check --lib --message-format=short 2>&1 | grep -q "src/crypto.*error"; then
    echo "❌ Crypto module has issues"
else
    echo "✅ Crypto module OK"
fi

echo "Testing media module..."
if cargo check --lib --message-format=short 2>&1 | grep -q "src/media.*error"; then
    echo "❌ Media module has issues"
    
    # If media module has issues, try to disable problematic features temporarily
    echo "Attempting to isolate media module issue..."
    
    # Check if it's in specific submodules
    for submod in compliance security processing; do
        if cargo check --lib --message-format=short 2>&1 | grep -q "src/media/$submod"; then
            echo "Issue found in media/$submod"
            break
        fi
    done
else
    echo "✅ Media module OK"
fi

# Final comprehensive test
echo ""
echo "Final compilation test..."
FINAL_RESULT=$(cargo check --lib 2>&1)

if echo "$FINAL_RESULT" | grep -q "error:"; then
    echo "❌ Still has compilation errors:"
    echo "$FINAL_RESULT" | grep "error:" | head -3
    
    echo ""
    echo "Detailed error location:"
    ERROR_FILE=$(echo "$FINAL_RESULT" | grep -o "src/[^:]*:[0-9]*:[0-9]*" | head -1)
    if [ ! -z "$ERROR_FILE" ]; then
        echo "Primary error in: $ERROR_FILE"
        echo ""
        echo "🔧 Quick fix suggestion:"
        FILE_PATH=$(echo $ERROR_FILE | cut -d: -f1)
        LINE_NUM=$(echo $ERROR_FILE | cut -d: -f2)
        
        if [ -f "$FILE_PATH" ]; then
            echo "Error context in $FILE_PATH at line $LINE_NUM:"
            sed -n "$((LINE_NUM-2)),$((LINE_NUM+2))p" "$FILE_PATH" | nl -v$((LINE_NUM-2))
        fi
    fi
else
    echo "✅ All compilation errors fixed!"
    
    # Test the examples too
    echo ""
    echo "Testing example compilation..."
    if cargo check --example session1_validation 2>&1 | grep -q "error:"; then
        echo "❌ Session 1 example still has issues"
    else
        echo "✅ Session 1 example compiles!"
    fi
    
    if cargo check --example session12_basic_validation 2>&1 | grep -q "error:"; then
        echo "❌ Session 12 example still has issues"
    else
        echo "✅ Session 12 example compiles!"
    fi
fi

# Clean up
rm -f final_error.log

echo ""
echo "🏁 Final fix attempt complete!"
echo "If issues persist, the error details above will help identify the specific problem."
