#!/bin/bash

# Session 7 Security Validation Test Script
# Comprehensive security testing for nano-messenger quantum-resistant protocol

echo "🛡️  NANO-MESSENGER SESSION 7: SECURITY VALIDATION"
echo "=================================================="
echo "Running comprehensive security test suite..."
echo

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to run a test and check result
run_test() {
    local test_name="$1"
    local test_command="$2"
    
    echo -e "${BLUE}🧪 Running: $test_name${NC}"
    echo "   Command: $test_command"
    
    if eval "$test_command" > /dev/null 2>&1; then
        echo -e "   ${GREEN}✅ PASSED${NC}"
        return 0
    else
        echo -e "   ${RED}❌ FAILED${NC}"
        return 1
    fi
}

# Function to run test with output
run_test_with_output() {
    local test_name="$1"
    local test_command="$2"
    
    echo -e "${BLUE}🧪 Running: $test_name${NC}"
    echo "   Command: $test_command"
    echo
    
    if eval "$test_command"; then
        echo -e "\n   ${GREEN}✅ PASSED${NC}"
        return 0
    else
        echo -e "\n   ${RED}❌ FAILED${NC}"
        return 1
    fi
}

# Counters
total_tests=0
passed_tests=0
failed_tests=0

# Start timer
start_time=$(date +%s)

echo "📋 Phase 1: Build and Compilation Tests"
echo "---------------------------------------"

total_tests=$((total_tests + 1))
if run_test "Cargo build (release)" "cargo build --release"; then
    passed_tests=$((passed_tests + 1))
else
    failed_tests=$((failed_tests + 1))
fi

total_tests=$((total_tests + 1))
if run_test "Cargo test compilation" "cargo test --no-run"; then
    passed_tests=$((passed_tests + 1))
else
    failed_tests=$((failed_tests + 1))
fi

echo
echo "📋 Phase 2: Core Security Test Suites"
echo "--------------------------------------"

# Run individual security test modules
total_tests=$((total_tests + 1))
if run_test "Cryptographic Correctness Tests" "cargo test crypto_correctness"; then
    passed_tests=$((passed_tests + 1))
else
    failed_tests=$((failed_tests + 1))
fi

total_tests=$((total_tests + 1))
if run_test "Protocol Security Tests" "cargo test protocol_security"; then
    passed_tests=$((passed_tests + 1))
else
    failed_tests=$((failed_tests + 1))
fi

total_tests=$((total_tests + 1))
if run_test "Attack Resistance Tests" "cargo test attack_resistance"; then
    passed_tests=$((passed_tests + 1))
else
    failed_tests=$((failed_tests + 1))
fi

total_tests=$((total_tests + 1))
if run_test "Interoperability Tests" "cargo test interoperability"; then
    passed_tests=$((passed_tests + 1))
else
    failed_tests=$((failed_tests + 1))
fi

echo
echo "📋 Phase 3: Integration and End-to-End Tests"
echo "---------------------------------------------"

total_tests=$((total_tests + 1))
if run_test "All Security Tests" "cargo test --test '*' security"; then
    passed_tests=$((passed_tests + 1))
else
    failed_tests=$((failed_tests + 1))
fi

total_tests=$((total_tests + 1))
if run_test "Session 7 Example Build" "cargo build --example session7_validation"; then
    passed_tests=$((passed_tests + 1))
else
    failed_tests=$((failed_tests + 1))
fi

echo
echo "📋 Phase 4: Comprehensive Validation Demo"
echo "------------------------------------------"

# Run the full Session 7 validation with output
total_tests=$((total_tests + 1))
if run_test_with_output "Session 7 Security Validation" "cargo run --example session7_validation"; then
    passed_tests=$((passed_tests + 1))
else
    failed_tests=$((failed_tests + 1))
    echo -e "${RED}💥 CRITICAL: Session 7 validation failed!${NC}"
fi

echo
echo "📋 Phase 5: Performance and Stress Tests"
echo "-----------------------------------------"

total_tests=$((total_tests + 1))
if run_test "Benchmark Compilation" "cargo test --release benchmarks --no-run"; then
    passed_tests=$((passed_tests + 1))
else
    failed_tests=$((failed_tests + 1))
fi

total_tests=$((total_tests + 1))
if run_test "Performance Test" "cargo test --release performance"; then
    passed_tests=$((passed_tests + 1))
else
    failed_tests=$((failed_tests + 1))
fi

# Calculate results
end_time=$(date +%s)
duration=$((end_time - start_time))
success_rate=$(echo "scale=1; $passed_tests * 100 / $total_tests" | bc -l)

echo
echo "🏁 SESSION 7 SECURITY VALIDATION RESULTS"
echo "========================================"
echo "⏱️  Total time: ${duration}s"
echo "🧪 Tests run: $total_tests"
echo "✅ Passed: $passed_tests"
echo "❌ Failed: $failed_tests"
echo "📊 Success rate: ${success_rate}%"

if [ $failed_tests -eq 0 ]; then
    echo
    echo -e "${GREEN}🎉 SESSION 7 VALIDATION: COMPLETE SUCCESS!${NC}"
    echo "=============================================="
    echo "✅ All security tests passed"
    echo "✅ Cryptographic implementations validated"
    echo "✅ Protocol security properties confirmed"
    echo "✅ Attack resistance verified"
    echo "✅ Cross-version compatibility ensured"
    echo
    echo -e "${GREEN}🎯 NANO-MESSENGER IS READY FOR PRODUCTION!${NC}"
    echo "   The quantum-resistant messaging protocol has passed"
    echo "   comprehensive security validation and is safe to deploy."
    
    exit 0
else
    echo
    echo -e "${RED}💥 SESSION 7 VALIDATION: FAILED${NC}"
    echo "==================================="
    echo "❌ $failed_tests security test(s) failed"
    echo "🚫 System is NOT ready for production"
    echo "⚠️  Review failed tests and fix issues before deployment"
    
    exit 1
fi
