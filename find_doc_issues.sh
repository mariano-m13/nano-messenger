#!/bin/bash

echo "🔍 SEARCHING FOR DOC COMMENT ISSUES"
echo "==================================="
echo ""

cd /Users/mariano/Desktop/Code/nano-messenger/src

# Search for various documentation comment patterns that might cause issues
echo "1. Searching for doc comments inside functions (/// inside { })..."
find . -name "*.rs" -type f -exec grep -l "///" {} \; | while read file; do
    if grep -Pzo '(?s)\{[^}]*///' "$file" 2>/dev/null; then
        echo "Found /// inside braces in: $file"
    fi
done

echo -e "\n2. Searching for inner doc comments (//!)..."
find . -name "*.rs" -type f -exec grep -Hn "//!" {} \;

echo -e "\n3. Searching for attribute-style doc comments (#[doc = ])..."
find . -name "*.rs" -type f -exec grep -Hn "#\[doc" {} \;

echo -e "\n4. Searching for misplaced module-level doc comments..."
find . -name "*.rs" -type f -exec grep -B1 -Hn "^///" {} \; | head -20

echo -e "\n5. Checking for syntax issues that might trigger E0753..."
# Look for common patterns that cause this error
find . -name "*.rs" -type f | while read file; do
    # Check for doc comments after attributes
    if grep -B1 "^///" "$file" | grep -q "^#\["; then
        echo "Potential issue in $file: doc comment after attribute"
    fi
    
    # Check for doc comments in wrong places
    if grep -A1 "^[[:space:]]*///" "$file" | grep -q "^[[:space:]]*{"; then
        echo "Potential issue in $file: doc comment before opening brace"
    fi
done

echo -e "\n6. Files with the most comments (potential problem areas)..."
find . -name "*.rs" -type f -exec bash -c 'echo -n "$1: "; grep -c "^[[:space:]]*//" "$1"' _ {} \; | sort -t: -k2 -nr | head -10

echo -e "\n🏁 Search complete!"
