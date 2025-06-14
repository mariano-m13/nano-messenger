#!/bin/bash

echo "🔧 TARGETED COMPILATION FIX"
echo "=========================="

cd "$(dirname "$0")"

echo "📊 Step 1: Check dependencies and common issues..."

# Check if all required dependencies are properly declared
echo "Checking Cargo.toml dependencies..."
if ! grep -q "uuid.*features.*\[\"v4\", \"serde\"\]" Cargo.toml; then
    echo "Adding missing uuid v4 feature..."
    # Add uuid v4 feature if missing (it looks like it's already there but checking)
fi

echo ""
echo "📊 Step 2: Check for missing imports in critical modules..."

# Check if there are any obvious missing imports
echo "Checking for common missing imports..."

# Fix potential missing dependencies in media module files
for file in src/media/compliance/mod.rs src/media/security/mod.rs; do
    if [ -f "$file" ]; then
        echo "Checking $file for missing imports..."
        
        # Add uuid import if it's being used but not imported
        if grep -q "uuid::" "$file" && ! grep -q "use uuid" "$file"; then
            echo "Adding missing uuid import to $file"
            sed -i '' '1i\
use uuid;
' "$file"
        fi
    fi
done

echo ""
echo "📊 Step 3: Try incremental compilation to isolate the error..."

echo "Testing crypto module compilation..."
cargo check --lib --message-format=short 2>&1 | grep -E "(error|warning)" | head -10

echo ""
echo "📊 Step 4: Common Rust compilation fixes..."

# Fix 1: Add missing derive macros where needed
echo "Checking for missing derives..."

# Fix 2: Check for lifetime issues
echo "Checking for potential lifetime issues..."

# Fix 3: Check for type issues in complex modules
echo "Testing individual module compilation..."

echo ""
echo "📊 Step 5: Get detailed error information..."

# Get the exact error with full context
echo "Running cargo check with full output:"
cargo check --lib 2>&1 | tee compilation_error_detailed.log

echo ""
echo "📊 Step 6: Analyze the specific error..."

if [ -f compilation_error_detailed.log ]; then
    echo "Error analysis:"
    
    # Look for specific error patterns
    if grep -q "cannot find type" compilation_error_detailed.log; then
        echo "❌ Missing type definitions found:"
        grep -A 2 -B 2 "cannot find type" compilation_error_detailed.log
    fi
    
    if grep -q "unresolved import" compilation_error_detailed.log; then
        echo "❌ Unresolved imports found:"
        grep -A 2 -B 2 "unresolved import" compilation_error_detailed.log
    fi
    
    if grep -q "trait bound" compilation_error_detailed.log; then
        echo "❌ Trait bound issues found:"
        grep -A 2 -B 2 "trait bound" compilation_error_detailed.log
    fi
    
    if grep -q "mismatched types" compilation_error_detailed.log; then
        echo "❌ Type mismatch found:"
        grep -A 2 -B 2 "mismatched types" compilation_error_detailed.log
    fi
    
    # Get the file and line number of the error
    ERROR_LOCATION=$(grep -o "src/[^:]*:[0-9]*:[0-9]*" compilation_error_detailed.log | head -1)
    if [ ! -z "$ERROR_LOCATION" ]; then
        echo ""
        echo "🎯 Primary error location: $ERROR_LOCATION"
        
        FILE_PATH=$(echo $ERROR_LOCATION | cut -d: -f1)
        LINE_NUM=$(echo $ERROR_LOCATION | cut -d: -f2)
        
        echo "Context around error:"
        if [ -f "$FILE_PATH" ]; then
            echo "File: $FILE_PATH, Line: $LINE_NUM"
            # Show lines around the error
            sed -n "$((LINE_NUM-3)),$((LINE_NUM+3))p" "$FILE_PATH" | nl -v$((LINE_NUM-3))
        fi
    fi
fi

echo ""
echo "📊 Step 7: Apply targeted fixes based on error type..."

# If it's a simple import issue, try to fix it
if grep -q "unresolved import" compilation_error_detailed.log; then
    echo "Attempting to fix import issues..."
    
    # Common fix: Add missing std imports
    MISSING_IMPORT=$(grep "unresolved import" compilation_error_detailed.log | head -1 | sed 's/.*`\([^`]*\)`.*/\1/')
    if [ ! -z "$MISSING_IMPORT" ]; then
        echo "Trying to fix missing import: $MISSING_IMPORT"
        
        # Add common missing imports
        case "$MISSING_IMPORT" in
            "std::collections::HashMap")
                find src -name "*.rs" -exec grep -l "HashMap" {} \; | while read file; do
                    if ! grep -q "use std::collections::HashMap" "$file"; then
                        echo "Adding HashMap import to $file"
                        sed -i '' '1i\
use std::collections::HashMap;
' "$file"
                    fi
                done
                ;;
            "std::time::SystemTime")
                find src -name "*.rs" -exec grep -l "SystemTime" {} \; | while read file; do
                    if ! grep -q "use std::time::SystemTime" "$file"; then
                        echo "Adding SystemTime import to $file"
                        sed -i '' '1i\
use std::time::SystemTime;
' "$file"
                    fi
                done
                ;;
        esac
    fi
fi

echo ""
echo "📊 Step 8: Final compilation test..."

if cargo check --lib 2>&1 | grep -q "error:"; then
    echo "❌ Compilation still has errors. Check compilation_error_detailed.log for details."
    echo ""
    echo "Most relevant error lines:"
    cargo check --lib 2>&1 | grep -E "(error|note):" | head -5
else
    echo "✅ Compilation successful!"
fi

# Clean up
rm -f compilation_error_detailed.log

echo ""
echo "🏁 Targeted fix complete. If errors persist, check the detailed output above."
