#!/bin/bash

echo "🔍 ANALYZING E0753 ERRORS"
echo "========================"
echo ""

cd /Users/mariano/Desktop/Code/nano-messenger

# First, get fresh compilation errors
echo "Getting fresh compilation errors..."
cargo check --features="image-processing" 2>&1 > fresh_errors.log

# Extract E0753 errors with their file locations
echo -e "\n📍 Files with E0753 errors:"
grep -B5 "error\[E0753\]" fresh_errors.log | grep -E "^\s*-->|error\[E0753\]" | paste - - | while read line; do
    # Extract file path and line number
    if echo "$line" | grep -q "-->"; then
        file_info=$(echo "$line" | grep -oE "[^ ]+\.rs:[0-9]+:[0-9]+")
        if [ ! -z "$file_info" ]; then
            echo "  $file_info"
        fi
    fi
done | sort | uniq

# Count E0753 errors per file
echo -e "\n📊 E0753 error count by file:"
grep -B5 "error\[E0753\]" fresh_errors.log | grep -E "^\s*-->" | grep -oE "[^ ]+\.rs" | sort | uniq -c | sort -nr

# Show the actual error messages
echo -e "\n📝 Sample E0753 error messages:"
grep -A2 "error\[E0753\]" fresh_errors.log | head -20

# Try to identify the pattern
echo -e "\n🔍 Looking for the pattern causing E0753..."
# Get one full error with context
grep -B10 -A5 "error\[E0753\]" fresh_errors.log | head -20

echo -e "\n💡 Common causes of E0753:"
echo "  - Doc comments (///) inside function bodies"
echo "  - Doc comments after #[derive(...)] or other attributes"
echo "  - Misplaced module-level doc comments (//!)"
echo "  - Doc comments in match arms or other invalid locations"

echo -e "\n🏁 Analysis complete!"
echo "Full error log saved to: fresh_errors.log"
