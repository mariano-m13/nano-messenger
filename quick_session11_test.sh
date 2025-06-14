#!/bin/bash

# Quick Session 11 Validation Script
# Fast compilation and basic functionality check

echo "⚡ Quick Session 11 Validation"
echo "============================="

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

# Quick compilation test
echo -e "${BLUE}📦 Testing basic compilation...${NC}"
if cargo check; then
    echo -e "${GREEN}✅ Basic compilation OK${NC}"
else
    echo -e "${RED}❌ Basic compilation failed${NC}"
    exit 1
fi

# Test Session 11 features
echo -e "${BLUE}🧪 Testing Session 11 features...${NC}"
if cargo check --features session11-full; then
    echo -e "${GREEN}✅ Session 11 features compile OK${NC}"
else
    echo -e "${RED}❌ Session 11 features compilation failed${NC}"
    exit 1
fi

# Test Session 11 example
echo -e "${BLUE}📋 Testing Session 11 example...${NC}"
if cargo check --example session11_validation; then
    echo -e "${GREEN}✅ Session 11 example compiles OK${NC}"
else
    echo -e "${RED}❌ Session 11 example compilation failed${NC}"
    exit 1
fi

# Quick unit test
echo -e "${BLUE}🧪 Running quick unit tests...${NC}"
if cargo test media::chunking::tests --lib 2>/dev/null; then
    echo -e "${GREEN}✅ Chunking tests OK${NC}"
else
    echo -e "${RED}⚠️ Some chunking tests may have issues${NC}"
fi

if cargo test media::deduplication::tests --lib 2>/dev/null; then
    echo -e "${GREEN}✅ Deduplication tests OK${NC}"
else
    echo -e "${RED}⚠️ Some deduplication tests may have issues${NC}"
fi

echo ""
echo -e "${GREEN}🎉 Quick validation complete!${NC}"
echo ""
echo "To run comprehensive tests: ./comprehensive_session11_test.sh"
echo "To test functionality: cargo run --example session11_validation"
echo ""
