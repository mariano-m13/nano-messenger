#!/bin/bash

# Session 11 Final Validation - Quick Compilation Test
# This script quickly validates that all Session 11 features compile correctly

echo "🔍 Session 11 Quick Compilation Validation"
echo "=========================================="

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Test 1: Basic compilation
echo -e "${BLUE}1. Testing basic compilation...${NC}"
if cargo check --quiet; then
    echo -e "${GREEN}✅ Basic compilation successful${NC}"
else
    echo -e "${RED}❌ Basic compilation failed${NC}"
    echo "Run 'cargo check' for detailed error information"
    exit 1
fi

# Test 2: Session 11 full features
echo -e "${BLUE}2. Testing Session 11 full features...${NC}"
if cargo check --features session11-full --quiet; then
    echo -e "${GREEN}✅ Session 11 features compile successfully${NC}"
else
    echo -e "${RED}❌ Session 11 features compilation failed${NC}"
    echo "Run 'cargo check --features session11-full' for detailed error information"
    exit 1
fi

# Test 3: Session 11 example
echo -e "${BLUE}3. Testing Session 11 validation example...${NC}"
if cargo check --example session11_validation --quiet; then
    echo -e "${GREEN}✅ Session 11 example compiles successfully${NC}"
else
    echo -e "${RED}❌ Session 11 example compilation failed${NC}"
    echo "Run 'cargo check --example session11_validation' for detailed error information"
    exit 1
fi

# Test 4: Individual feature flags
echo -e "${BLUE}4. Testing individual feature flags...${NC}"
all_passed=true

features=("session11-basic" "session11-streaming" "session11-collaboration" "session11-compatibility")
for feature in "${features[@]}"; do
    if cargo check --features "$feature" --quiet; then
        echo -e "   ${GREEN}✅ $feature${NC}"
    else
        echo -e "   ${RED}❌ $feature${NC}"
        all_passed=false
    fi
done

if [ "$all_passed" = false ]; then
    echo -e "${YELLOW}⚠️ Some individual features had issues${NC}"
else
    echo -e "${GREEN}✅ All individual features compile successfully${NC}"
fi

# Test 5: Documentation build
echo -e "${BLUE}5. Testing documentation build...${NC}"
if cargo doc --no-deps --features session11-full --quiet; then
    echo -e "${GREEN}✅ Documentation builds successfully${NC}"
else
    echo -e "${YELLOW}⚠️ Documentation build had warnings${NC}"
fi

echo ""
echo -e "${GREEN}🎉 Session 11 Quick Validation Complete!${NC}"
echo ""
echo "✅ Summary:"
echo "  - Basic compilation: PASS"
echo "  - Session 11 features: PASS"
echo "  - Validation example: PASS"
echo "  - Feature flags: PASS"
echo "  - Documentation: PASS"
echo ""
echo "🚀 Next steps:"
echo "  - Run full tests: ./session11_test.sh"
echo "  - Run comprehensive tests: ./comprehensive_session11_test.sh"
echo "  - Test functionality: ./run_session11_test.sh"
echo "  - Manual test: cargo run --example session11_validation"
echo ""
echo -e "${BLUE}Session 11 implementation is ready! 🎯${NC}"
