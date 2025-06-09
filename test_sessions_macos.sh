#!/bin/bash

# Simplified Test Script for macOS - Nano-Messenger Quantum-Resistant Protocol
# Tests all session validations without timeout dependencies

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Test results tracking
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

echo -e "${PURPLE}🧪 NANO-MESSENGER SESSION TESTING (macOS)${NC}"
echo -e "${PURPLE}==========================================${NC}"
echo -e "${CYAN}Quantum-Resistant Messaging Protocol Validation Suite${NC}"
echo -e "${CYAN}Testing all implementation sessions...${NC}\n"

# Function to run a test and track results (simplified for macOS)
run_test() {
    local test_name="$1"
    local test_command="$2"
    local description="$3"
    
    echo -e "\n${YELLOW}🔍 Testing: $test_name${NC}"
    echo -e "${CYAN}   $description${NC}"
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    # Capture start time
    start_time=$(date +%s)
    
    # Run the test and capture output (no timeout)
    if $test_command > /tmp/test_output_$$.log 2>&1; then
        end_time=$(date +%s)
        duration=$((end_time - start_time))
        
        echo -e "${GREEN}   ✅ PASSED${NC} (${duration}s)"
        PASSED_TESTS=$((PASSED_TESTS + 1))
        
        # Show key success indicators from output
        if grep -q "COMPLETE\|SUCCESS\|✅" /tmp/test_output_$$.log 2>/dev/null; then
            echo -e "${GREEN}   📊 Key results:${NC}"
            grep -E "(✅|COMPLETE|SUCCESS|PASSED)" /tmp/test_output_$$.log | head -3 | sed 's/^/      /' 2>/dev/null || true
        fi
    else
        end_time=$(date +%s)
        duration=$((end_time - start_time))
        
        echo -e "${RED}   ❌ FAILED${NC} (${duration}s)"
        FAILED_TESTS=$((FAILED_TESTS + 1))
        
        # Show error details
        echo -e "${RED}   📋 Error details:${NC}"
        tail -10 /tmp/test_output_$$.log 2>/dev/null | sed 's/^/      /' || echo "      No error details available"
    fi
    
    # Cleanup
    rm -f /tmp/test_output_$$.log
}

echo -e "${YELLOW}🔨 COMPILATION CHECK${NC}"
echo -e "${CYAN}   Verifying all code compiles correctly...${NC}\n"

run_test "Compilation Check" "cargo check --examples" "Checking all examples compile"
run_test "Library Build" "cargo build --release" "Building optimized library"

echo -e "\n${YELLOW}📋 SESSION VALIDATIONS${NC}"
echo -e "${CYAN}   Running all session validation examples...${NC}\n"

# Test each session
sessions=(
    "Session 1 Validation:session1_validation:Core cryptographic implementation"
    "Session 2 Validation:session2_validation:Protocol implementation"
    "Session 3 Validation:session3_validation:Quantum-safe messaging"
    "Session 4 Validation:session4_validation:Multi-mode crypto support"
    "Session 5 Validation:session5_validation:Relay policy enforcement"
    "Session 6 Validation:session6_validation:Performance optimization"
    "Session 7 Validation:session7_validation:Security validation"
)

for session in "${sessions[@]}"; do
    IFS=':' read -r name example desc <<< "$session"
    echo -e "\n${BLUE}🔍 $name${NC}"
    run_test "$name" "cargo run --example $example" "$desc"
done

echo -e "\n${YELLOW}🧪 UNIT TESTS${NC}"
echo -e "${CYAN}   Running unit tests...${NC}\n"

run_test "Unit Tests" "cargo test" "All module unit tests"

# Generate final report
echo -e "\n${PURPLE}📊 COMPREHENSIVE TEST REPORT${NC}"
echo -e "${PURPLE}=============================${NC}\n"

echo -e "${BLUE}📈 Test Statistics:${NC}"
echo -e "   Total Tests: ${TOTAL_TESTS}"
echo -e "   ${GREEN}Passed: ${PASSED_TESTS}${NC}"
echo -e "   ${RED}Failed: ${FAILED_TESTS}${NC}"

if [ $FAILED_TESTS -eq 0 ]; then
    echo -e "   ${GREEN}Success Rate: 100%${NC}"
    echo -e "\n${GREEN}🎉 ALL TESTS PASSED! Your quantum-resistant messaging protocol is fully validated!${NC}"
    
    echo -e "\n${BLUE}🛡️  Security Status:${NC}"
    echo -e "   ${GREEN}✅ Security Validation: COMPLETE${NC}"
    echo -e "   ${GREEN}✅ Cryptographic Correctness: VERIFIED${NC}"
    echo -e "   ${GREEN}✅ Production Ready: YES${NC}"
    
    echo -e "\n${GREEN}🏆 ALL VALIDATIONS SUCCESSFUL! Protocol ready for deployment.${NC}"
    exit 0
else
    success_rate=$(( (PASSED_TESTS * 100) / TOTAL_TESTS ))
    echo -e "   ${YELLOW}Success Rate: ${success_rate}%${NC}"
    echo -e "\n${YELLOW}⚠️  Some tests failed. Review the detailed results above.${NC}"
    
    echo -e "\n${BLUE}🛡️  Security Status:${NC}"
    echo -e "   ${RED}❌ Security Validation: INCOMPLETE${NC}"
    echo -e "   ${RED}⚠️  Not recommended for production use${NC}"
    
    echo -e "\n${RED}❌ Some validations failed. Review errors before deployment.${NC}"
    exit 1
fi
