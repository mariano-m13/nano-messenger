#!/bin/bash

echo "🔧 FIXING DOC COMMENT SYNTAX ERRORS"
echo "==================================="

cd "$(dirname "$0")"

echo "Step 1: Getting specific error details..."

# Get the first few actual errors with file locations
cargo check --lib 2>&1 | grep -A 2 -B 1 "E0753" | head -20

echo ""
echo "Step 2: Checking for malformed doc comments..."

# Look for common doc comment issues
find src -name "*.rs" -exec grep -l "^///" {} \; | head -5 | while read file; do
    echo "Checking doc comments in $file..."
    # Check first few lines for malformed docs
    head -10 "$file"
    echo "---"
done

echo ""
echo "Step 3: Fixing common doc comment issues..."

# Fix files that might have import statements inserted incorrectly
for file in src/config/adaptive.rs src/production/mod.rs src/inbox.rs src/media/security/forensics.rs src/media/compliance/hipaa.rs; do
    if [ -f "$file" ]; then
        echo "Checking $file for syntax issues..."
        
        # Look for lines that start with 'use' followed by doc comments
        if grep -q "^use.*" "$file" && grep -q "^///" "$file"; then
            echo "Found potential doc comment conflict in $file"
            
            # Create a backup
            cp "$file" "${file}.backup"
            
            # Try to fix by ensuring imports come after any top-level doc comments
            # This is a simple fix - we'll extract doc comments and re-arrange
            
            # Get the file header (doc comments and attributes)
            grep -E "^(//!|///|#\[)" "$file" > "${file}.header" 2>/dev/null || touch "${file}.header"
            
            # Get imports
            grep "^use " "$file" > "${file}.imports" 2>/dev/null || touch "${file}.imports"
            
            # Get everything else
            grep -v -E "^(//!|///|#\[|use )" "$file" > "${file}.rest" 2>/dev/null || touch "${file}.rest"
            
            # Rebuild file: header, then imports, then rest
            cat "${file}.header" "${file}.imports" "${file}.rest" > "$file"
            
            # Clean up temp files
            rm -f "${file}.header" "${file}.imports" "${file}.rest"
            
            echo "Attempted fix for $file"
        fi
    fi
done

echo ""
echo "Step 4: Testing compilation..."

# Quick compile test
ERROR_COUNT=$(cargo check --lib 2>&1 | grep -c "error\[" || echo "0")
echo "Error count after fixes: $ERROR_COUNT"

if [ "$ERROR_COUNT" -lt "10" ]; then
    echo "🎉 Great progress! Down to $ERROR_COUNT errors"
    
    # Show remaining errors
    echo ""
    echo "Remaining errors:"
    cargo check --lib 2>&1 | grep "error\[" | head -5
    
elif [ "$ERROR_COUNT" -lt "50" ]; then
    echo "🚀 Significant improvement! Down to $ERROR_COUNT errors"
    echo ""
    echo "Top 3 remaining errors:"
    cargo check --lib 2>&1 | grep "error\[" | head -3
else
    echo "📋 Still need more fixes. Top error types:"
    cargo check --lib 2>&1 | grep "error\[" | cut -d: -f4 | sort | uniq -c | sort -nr | head -5
fi

echo ""
echo "🏁 Doc comment fix attempt complete."
