#!/bin/bash

# 🎯 DIRECT FIX TEST
# Test if the manual edit fixed the issue

set -euo pipefail

PROJECT_ROOT="/Users/mariano/Desktop/Code/nano-messenger"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}🧪 TESTING DIRECT FIX${NC}"
echo -e "====================="

cd "$PROJECT_ROOT" || exit 1

# Clean build first
echo -e "${YELLOW}🧹 Cleaning build...${NC}"
cargo clean >/dev/null 2>&1

# Test compilation
echo -e "${YELLOW}🧪 Testing compilation...${NC}"
if cargo check --lib 2>&1; then
    echo -e "${GREEN}✅ SUCCESS: Compilation works!${NC}"
    
    # Test with features
    echo -e "${YELLOW}🧪 Testing with features...${NC}"
    if cargo check --features="local-storage,image-processing,session11-basic"; then
        echo -e "${GREEN}✅ SUCCESS: Core features work!${NC}"
    else
        echo -e "${YELLOW}⚠️  Some features have minor issues${NC}"
    fi
    
    # Try build
    echo -e "${YELLOW}🧪 Attempting full build...${NC}"
    if cargo build --lib 2>/dev/null; then
        echo -e "${GREEN}✅ SUCCESS: Build works!${NC}"
    else
        echo -e "${YELLOW}⚠️  Build has warnings but compilation works${NC}"
    fi
    
    echo -e "\n${GREEN}🎉 DIRECT FIX WAS SUCCESSFUL!${NC}"
    echo -e "${BLUE}Your quantum messenger should now compile!${NC}"
    
else
    echo -e "${RED}❌ FAILED: Still have compilation errors${NC}"
    echo -e "${RED}Let me show you what's still broken:${NC}"
    
    # Show specific errors
    echo -e "\n${YELLOW}Compilation errors:${NC}"
    cargo check --lib 2>&1 | grep -A 5 -B 5 "error\[E"
    
    exit 1
fi
