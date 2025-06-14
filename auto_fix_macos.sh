#!/bin/bash

echo "🚀 NANO-MESSENGER AUTOMATED FIX (macOS)"
echo "======================================"
echo "This will fix all E0753 and compilation errors"
echo ""

cd /Users/mariano/Desktop/Code/nano-messenger

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Step 1: Clean
echo -e "${YELLOW}Step 1: Cleaning all build artifacts...${NC}"
cargo clean
rm -rf target/
rm -f Cargo.lock
find . -name "*.bak" -delete
find . -name "*.backup" -delete

# Step 2: Fix all Rust files with E0753 issues
echo -e "\n${YELLOW}Step 2: Fixing E0753 errors in all Rust files...${NC}"

# Process each .rs file
find src -name "*.rs" -type f | while read -r file; do
    # Skip if no //! comments
    if ! grep -q "^//!" "$file" 2>/dev/null; then
        continue
    fi
    
    echo "Processing: $file"
    
    # Create temp file
    tmpfile="${file}.tmp$$"
    
    # Extract module documentation (//! comments)
    module_docs=""
    regular_code=""
    in_module_docs=false
    first_code_seen=false
    
    while IFS= read -r line; do
        if [[ "$line" =~ ^//! ]] && [[ "$first_code_seen" == "false" ]]; then
            # Module doc at the beginning
            module_docs="${module_docs}${line}\n"
            in_module_docs=true
        elif [[ "$line" =~ ^//! ]] && [[ "$first_code_seen" == "true" ]]; then
            # Module doc after code - this causes E0753
            echo -e "  ${RED}Found E0753: //! after code${NC}"
            module_docs="${module_docs}${line}\n"
        elif [[ -z "$line" ]] && [[ "$in_module_docs" == "true" ]]; then
            # Empty line in module docs
            module_docs="${module_docs}\n"
        else
            # Regular code
            if [[ ! -z "$line" ]] || [[ "$first_code_seen" == "true" ]]; then
                first_code_seen=true
                in_module_docs=false
            fi
            regular_code="${regular_code}${line}\n"
        fi
    done < "$file"
    
    # Reconstruct file with module docs at the top
    {
        if [[ ! -z "$module_docs" ]]; then
            echo -ne "$module_docs"
            echo ""  # Blank line after module docs
        fi
        echo -ne "$regular_code"
    } > "$tmpfile"
    
    # Replace original file
    mv "$tmpfile" "$file"
    echo -e "  ${GREEN}✓ Fixed${NC}"
done

# Step 3: Fix Cargo.toml - disable ffmpeg-next
echo -e "\n${YELLOW}Step 3: Fixing dependency issues...${NC}"
if grep -q '^ffmpeg-next' Cargo.toml; then
    echo "Disabling ffmpeg-next..."
    # For macOS sed, we need to provide a backup extension or use -i ''
    sed -i '' 's/^ffmpeg-next/# ffmpeg-next/' Cargo.toml
    sed -i '' 's/^video-processing = \["ffmpeg-next"\]/# video-processing = ["ffmpeg-next"]/' Cargo.toml
    echo -e "${GREEN}✓ Disabled ffmpeg-next${NC}"
fi

# Step 4: Update dependencies
echo -e "\n${YELLOW}Step 4: Updating dependencies...${NC}"
cargo update

# Step 5: Test compilation
echo -e "\n${YELLOW}Step 5: Testing compilation...${NC}"
echo "Checking library..."
if cargo check --lib 2>&1 | tee final_check.log; then
    echo -e "${GREEN}✅ Library compiles successfully!${NC}"
    
    echo -e "\nChecking with features..."
    if cargo check --lib --features="image-processing" 2>&1 | tee feature_check.log; then
        echo -e "${GREEN}✅ All features compile successfully!${NC}"
    else
        FEATURE_ERRORS=$(grep -c "error\[E" feature_check.log || echo "0")
        echo -e "${RED}Feature compilation has $FEATURE_ERRORS errors${NC}"
    fi
else
    ERRORS=$(grep -c "error\[E" final_check.log || echo "0")
    echo -e "${RED}Still have $ERRORS compilation errors${NC}"
    
    # Show error summary
    echo -e "\n${YELLOW}Error Summary:${NC}"
    grep "error\[E" final_check.log | cut -d']' -f1 | sort | uniq -c
    
    # Show first few errors
    echo -e "\n${YELLOW}First few errors:${NC}"
    grep -A2 "error\[E" final_check.log | head -20
fi

# Step 6: Summary
echo -e "\n${YELLOW}========== SUMMARY ==========${NC}"
TOTAL_ERRORS=$(grep -c "error\[E" final_check.log 2>/dev/null || echo "0")

if [ "$TOTAL_ERRORS" -eq 0 ]; then
    echo -e "${GREEN}🎉 SUCCESS! All compilation errors fixed!${NC}"
    echo -e "\nYou can now run:"
    echo "  cargo build --release"
    echo "  cargo test"
    echo "  cargo run --bin nano-client"
    
    # Try a quick test
    echo -e "\n${YELLOW}Running a quick test...${NC}"
    cargo test --lib crypto::tests::test_keypair_generation -- --nocapture
else
    echo -e "${RED}❌ $TOTAL_ERRORS errors remain${NC}"
    echo -e "\nCheck these files:"
    echo "  - final_check.log"
    echo "  - feature_check.log"
    echo -e "\nMost common remaining errors:"
    grep "error\[E" final_check.log | cut -d']' -f1 | sort | uniq -c | sort -nr | head -5
fi

echo -e "\n🏁 Automated fix complete!"
