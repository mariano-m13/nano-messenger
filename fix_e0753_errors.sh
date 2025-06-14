#!/bin/bash

echo "🔧 FIXING E0753 DOC COMMENT ERRORS"
echo "=================================="
echo ""

cd /Users/mariano/Desktop/Code/nano-messenger

# Function to fix doc comments in a file
fix_doc_comments() {
    local file="$1"
    echo "Checking: $file"
    
    # Create a temporary file
    local tmpfile="${file}.tmp"
    
    # Check if file starts with a use statement followed by //!
    if head -10 "$file" | grep -q "^use.*" && head -10 "$file" | grep -q "^//!"; then
        echo "  ⚠️  Found potential E0753 issue in $file"
        
        # Extract module doc comments (//!)
        local doc_comments=""
        local in_doc_block=false
        local found_doc=false
        
        while IFS= read -r line; do
            if [[ "$line" =~ ^//! ]]; then
                doc_comments="${doc_comments}${line}\n"
                found_doc=true
                in_doc_block=true
            elif [[ "$in_doc_block" == true && -z "$line" ]]; then
                doc_comments="${doc_comments}\n"
            elif [[ "$in_doc_block" == true && ! "$line" =~ ^//! ]]; then
                in_doc_block=false
            fi
        done < "$file"
        
        if [[ "$found_doc" == true ]]; then
            # Rewrite the file with doc comments at the top
            {
                # First, write doc comments
                echo -e "${doc_comments}"
                
                # Then write the rest of the file, skipping doc comments
                while IFS= read -r line; do
                    if [[ ! "$line" =~ ^//! ]]; then
                        echo "$line"
                    fi
                done < "$file"
            } > "$tmpfile"
            
            # Replace the original file
            mv "$tmpfile" "$file"
            echo "  ✅ Fixed doc comment placement in $file"
        fi
    fi
    
    # Clean up any temp files
    rm -f "$tmpfile"
}

# Find all Rust files and check them
echo "🔍 Finding and fixing E0753 errors in Rust files..."
find src -name "*.rs" -type f | while read -r file; do
    fix_doc_comments "$file"
done

# Fix the ffmpeg-next issue by making it optional
echo -e "\n🔧 Fixing ffmpeg-next dependency issue..."
if grep -q 'ffmpeg-next = { version = "6.0"' Cargo.toml; then
    echo "Making ffmpeg-next optional..."
    sed -i.bak 's/ffmpeg-next = { version = "6.0", optional = true }/ffmpeg-next = { version = "6.0", optional = true, default-features = false }/' Cargo.toml
    echo "✅ Updated ffmpeg-next configuration"
fi

# Clean and rebuild
echo -e "\n🧹 Cleaning build artifacts..."
cargo clean
rm -f Cargo.lock

# Check compilation again
echo -e "\n🏗️ Checking compilation..."
cargo check --features="image-processing" 2>&1 | tee e0753_fix_check.log

# Count remaining errors
REMAINING_ERRORS=$(grep -c "error\[E" e0753_fix_check.log 2>/dev/null || echo "0")
echo -e "\n📊 Results:"
echo "Remaining errors: $REMAINING_ERRORS"

if [ "$REMAINING_ERRORS" -gt 0 ]; then
    echo -e "\n❌ Still have errors. Checking for other E0753 patterns..."
    
    # Look for other patterns that might cause E0753
    echo -e "\nFiles with potential issues:"
    grep -B3 "error\[E0753\]" e0753_fix_check.log | grep -E "^\s*-->" | \
        grep -oE "src/[^ ]+\.rs:[0-9]+" | sort | uniq | while read location; do
        file=$(echo "$location" | cut -d: -f1)
        line=$(echo "$location" | cut -d: -f2)
        echo "  - $file at line $line"
        
        # Show the problematic line
        if [ -f "$file" ]; then
            echo "    Line $line: $(sed -n "${line}p" "$file")"
        fi
    done
else
    echo -e "\n✅ All E0753 errors have been fixed!"
fi

echo -e "\n🏁 Fix complete!"
