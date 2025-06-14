#!/bin/bash

echo "🔍 E0753 ERROR CHECKER"
echo "====================="
echo ""

cd /Users/mariano/Desktop/Code/nano-messenger

# First, let's see current state
echo "Current compilation status:"
cargo check --lib 2>&1 | grep -E "(error\[E0753\]|error: could not compile)" | head -5

echo -e "\n📍 Checking for E0753 patterns in source files..."

# Pattern 1: Files with //! after any code
echo -e "\n1. Files with module docs (//!) potentially after code:"
find src -name "*.rs" -type f | while read -r file; do
    # Check if //! appears after line 1 and after any non-comment line
    if awk 'NR>1 && /^\/\/!/ && found_code {print FILENAME ": line " NR; exit} 
            !/^\/\// && !/^[[:space:]]*$/ {found_code=1}' "$file"; then
        :
    fi
done

# Pattern 2: Check specific files mentioned in errors
echo -e "\n2. Checking files from error output:"
cargo check --lib 2>&1 | grep -B3 "error\[E0753\]" | grep "^ *-->" | while read -r line; do
    if [[ "$line" =~ src/([^:]+):([0-9]+):([0-9]+) ]]; then
        file="src/${BASH_REMATCH[1]}"
        linenum="${BASH_REMATCH[2]}"
        echo "  $file:$linenum"
        if [ -f "$file" ]; then
            echo "    Content: $(sed -n "${linenum}p" "$file")"
        fi
    fi
done

echo -e "\n3. Quick scan for problematic patterns:"
echo "Files with '//!' not at the start:"
grep -n "^//!" src/**/*.rs | while read -r match; do
    file=$(echo "$match" | cut -d: -f1)
    linenum=$(echo "$match" | cut -d: -f2)
    if [ "$linenum" -gt 1 ]; then
        # Check if there's code before this line
        if head -n $((linenum-1)) "$file" | grep -q "^[^/]"; then
            echo "  $file:$linenum - module doc after code"
        fi
    fi
done

echo -e "\n🏁 Check complete!"
