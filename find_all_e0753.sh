#!/bin/bash

echo "🔍 FINDING ALL E0753 ERROR LOCATIONS"
echo "===================================="
echo ""

cd /Users/mariano/Desktop/Code/nano-messenger

# Get fresh error output with file locations
echo "Getting current E0753 error locations..."
cargo check --lib 2>&1 | grep -B5 "error\[E0753\]" | grep -E "^\s*-->" > e0753_locations.txt

# Process each location
echo -e "\n📍 Files with E0753 errors:"
cat e0753_locations.txt | grep -oE "src/[^ ]+\.rs:[0-9]+:[0-9]+" | while read location; do
    file=$(echo "$location" | cut -d: -f1)
    line=$(echo "$location" | cut -d: -f2)
    col=$(echo "$location" | cut -d: -f3)
    
    echo -e "\n❌ $file at line $line, column $col"
    
    if [ -f "$file" ]; then
        # Show context around the error
        echo "Context:"
        start=$((line - 2))
        end=$((line + 2))
        sed -n "${start},${end}p" "$file" | nl -v $start
        
        # Check for common E0753 patterns
        echo "Checking for patterns..."
        
        # Pattern 1: use statement followed by //!
        if sed -n "1,${line}p" "$file" | grep -q "^use" && sed -n "${line}p" "$file" | grep -q "^//!"; then
            echo "  ⚠️  Pattern: Module doc comment (//!) after use statement"
        fi
        
        # Pattern 2: /// inside a function or after {
        if sed -n "$((line-5)),${line}p" "$file" | grep -q "{" && sed -n "${line}p" "$file" | grep -q "^[[:space:]]*///"; then
            echo "  ⚠️  Pattern: Doc comment (///) inside code block"
        fi
        
        # Pattern 3: //! not at the beginning of file
        if [ "$line" -gt 10 ] && sed -n "${line}p" "$file" | grep -q "^//!"; then
            echo "  ⚠️  Pattern: Inner doc comment (//!) not at file start"
        fi
    fi
done

# Create a fix script based on findings
echo -e "\n📝 Creating targeted fix script..."
cat > fix_e0753_targeted.sh << 'EOF'
#!/bin/bash

echo "🔧 TARGETED E0753 FIX"
echo "===================="

cd /Users/mariano/Desktop/Code/nano-messenger

# List of files that need fixing (from the error output)
FILES_TO_FIX=(
EOF

# Add files to the fix script
cat e0753_locations.txt | grep -oE "src/[^ ]+\.rs" | sort | uniq | while read file; do
    echo "    \"$file\"" >> fix_e0753_targeted.sh
done

cat >> fix_e0753_targeted.sh << 'EOF'
)

# Fix each file
for file in "${FILES_TO_FIX[@]}"; do
    if [ -f "$file" ]; then
        echo "Fixing: $file"
        
        # Create backup
        cp "$file" "${file}.bak"
        
        # Extract module doc comments (//!) if they exist
        module_docs=""
        if grep -q "^//!" "$file"; then
            module_docs=$(grep "^//!" "$file")
            
            # Remove these comments from their current location
            grep -v "^//!" "$file" > "${file}.tmp"
            
            # If there are module docs, put them at the very top
            if [ ! -z "$module_docs" ]; then
                {
                    echo "$module_docs"
                    echo ""
                    cat "${file}.tmp"
                } > "$file"
            else
                mv "${file}.tmp" "$file"
            fi
        fi
        
        echo "  ✅ Fixed $file"
    fi
done

echo -e "\n🏁 Targeted fix complete!"
echo "Run 'cargo check --lib' to verify fixes"
EOF

chmod +x fix_e0753_targeted.sh

echo -e "\n✅ Created fix_e0753_targeted.sh"
echo "Run './fix_e0753_targeted.sh' to fix all E0753 errors"

# Show summary
TOTAL_FILES=$(cat e0753_locations.txt | grep -oE "src/[^ ]+\.rs" | sort | uniq | wc -l)
echo -e "\n📊 Summary:"
echo "Total files with E0753 errors: $TOTAL_FILES"

echo -e "\n🏁 Analysis complete!"
