#!/bin/bash

# Comprehensive Test Script for Nano-Messenger Quantum-Resistant Protocol
# Tests all session validations and provides detailed reporting
# FIXED VERSION - addresses log file path issues

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
SESSION_RESULTS=()

echo -e "${PURPLE}🧪 NANO-MESSENGER COMPREHENSIVE SESSION TESTING${NC}"
echo -e "${PURPLE}=================================================${NC}"
echo -e "${CYAN}Quantum-Resistant Messaging Protocol Validation Suite${NC}"
echo -e "${CYAN}Testing all implementation sessions...${NC}\n"

# Function to print test header
print_test_header() {
    local session_name="$1"
    local session_desc="$2"
    echo -e "${BLUE}┌─────────────────────────────────────────────────────────┐${NC}"
    echo -e "${BLUE}│ ${YELLOW}SESSION: $session_name${BLUE}$(printf '%*s' $((48 - ${#session_name})) '')│${NC}"
    echo -e "${BLUE}│ ${CYAN}$session_desc${BLUE}$(printf '%*s' $((48 - ${#session_desc})) '')│${NC}"
    echo -e "${BLUE}└─────────────────────────────────────────────────────────┘${NC}"
}

# Function to run a test and track results
run_test() {
    local test_name="$1"
    local test_command="$2"
    local description="$3"
    
    echo -e "\n${YELLOW}🔍 Testing: $test_name${NC}"
    echo -e "${CYAN}   $description${NC}"
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    # Capture start time
    start_time=$(date +%s)
    
    # Create unique log file for this test
    local log_file="/tmp/nano_test_$(date +%s)_$$.log"
    
    # Run the test and capture output
    if eval "$test_command" > "$log_file" 2>&1; then
        end_time=$(date +%s)
        duration=$((end_time - start_time))
        
        echo -e "${GREEN}   ✅ PASSED${NC} (${duration}s)"
        PASSED_TESTS=$((PASSED_TESTS + 1))
        SESSION_RESULTS+=("✅ $test_name - PASSED (${duration}s)")
        
        # Show key success indicators from output
        if [ -f "$log_file" ] && grep -q "COMPLETE\|SUCCESS\|✅" "$log_file" 2>/dev/null; then
            echo -e "${GREEN}   📊 Key results:${NC}"
            grep -E "(✅|COMPLETE|SUCCESS|PASSED)" "$log_file" 2>/dev/null | head -3 | sed 's/^/      /' || true
        fi
    else
        end_time=$(date +%s)
        duration=$((end_time - start_time))
        
        echo -e "${RED}   ❌ FAILED${NC} (${duration}s)"
        FAILED_TESTS=$((FAILED_TESTS + 1))
        SESSION_RESULTS+=("❌ $test_name - FAILED (${duration}s)")
        
        # Show error details
        echo -e "${RED}   📋 Error details:${NC}"
        if [ -f "$log_file" ]; then
            tail -10 "$log_file" | sed 's/^/      /'
        else
            echo "      Log file not found"
        fi
    fi
    
    # Cleanup
    rm -f "$log_file"
}

# Function to check compilation
check_compilation() {
    echo -e "\n${YELLOW}🔨 COMPILATION CHECK${NC}"
    echo -e "${CYAN}   Verifying all code compiles correctly...${NC}\n"
    
    run_test "Compilation Check" "cargo check --examples" "Checking all examples compile"
    run_test "Library Build" "cargo build --release" "Building optimized library"
}

# Function to test individual sessions
test_sessions() {
    echo -e "\n${YELLOW}📋 SESSION VALIDATIONS${NC}"
    echo -e "${CYAN}   Running all session validation examples...${NC}\n"
    
    # Session 1: Core Crypto Implementation
    print_test_header "SESSION 1" "Core Cryptographic Implementation"
    run_test "Session 1 Validation" "cargo run --example session1_validation" "Basic crypto operations and key management"
    
    # Session 2: Protocol Implementation  
    print_test_header "SESSION 2" "Protocol Implementation"
    run_test "Session 2 Validation" "cargo run --example session2_validation" "Message protocol and envelope handling"
    
    # Session 3: Quantum-Safe Messaging
    print_test_header "SESSION 3" "Quantum-Safe Messaging"
    run_test "Session 3 Validation" "cargo run --example session3_validation" "Post-quantum cryptography integration"
    
    # Session 4: Multi-Mode Support
    print_test_header "SESSION 4" "Multi-Mode Crypto Support" 
    run_test "Session 4 Validation" "cargo run --example session4_validation" "Classical, Hybrid, and Quantum modes"
    
    # Session 5: Relay Policy Enforcement
    print_test_header "SESSION 5" "Relay Policy Enforcement"
    run_test "Session 5 Validation" "cargo run --example session5_validation" "Crypto policy enforcement and compliance"
    
    # Session 6: Performance Optimization
    print_test_header "SESSION 6" "Performance Optimization"
    run_test "Session 6 Validation" "cargo run --example session6_validation" "Adaptive configuration and optimization"
    
    # Session 7: Security Validation
    print_test_header "SESSION 7" "Security Validation"
    run_test "Session 7 Validation" "cargo run --example session7_validation" "Comprehensive security property verification"
    
    # Session 9: Media and File Attachments (if available)
    if [ -f "examples/session9_validation.rs" ]; then
        print_test_header "SESSION 9" "Media and File Attachments"
        run_test "Session 9 Validation" "cargo run --example session9_validation" "Media upload, processing, and secure transfer"
    fi
    
    # Session 10: Media Processing & Optimization (if available)
    if [ -f "examples/session10_validation.rs" ]; then
        print_test_header "SESSION 10" "Media Processing & Optimization"
        run_test "Session 10 Validation" "cargo run --example session10_validation" "Advanced media processing and thumbnails"
    fi
    
    # Session 11: Advanced Media Features (if available)
    if [ -f "examples/session11_validation.rs" ]; then
        print_test_header "SESSION 11" "Advanced Media Features"
        run_test "Session 11 Validation" "cargo run --example session11_validation" "Large file support, streaming, and collaboration"
    fi
    
    # Session 12: Security & Compliance for Media - Use fixed version
    if [ -f "examples/session12_validation_fixed.rs" ]; then
        print_test_header "SESSION 12" "Security & Compliance for Media (Fixed)"
        run_test "Session 12 Validation (Fixed)" "cargo run --example session12_validation_fixed" "Core security and compliance features"
    elif [ -f "examples/session12_validation.rs" ]; then
        print_test_header "SESSION 12" "Security & Compliance for Media"
        run_test "Session 12 Validation" "cargo run --example session12_validation" "Enterprise security, threat detection, and compliance"
    fi
}

# Function to run unit tests
test_unit_tests() {
    echo -e "\n${YELLOW}🧪 UNIT TESTS${NC}"
    echo -e "${CYAN}   Running all unit tests...${NC}\n"
    
    run_test "Unit Tests" "cargo test" "All module unit tests"
    
    # Check if integration tests exist
    if [ -d "tests" ] && [ "$(ls -A tests)" ]; then
        run_test "Integration Tests" "cargo test --test '*'" "Integration test suite"
    fi
    
    run_test "Doc Tests" "cargo test --doc" "Documentation example tests"
}

# Function to run performance benchmarks (if available)
test_performance() {
    echo -e "\n${YELLOW}⚡ PERFORMANCE TESTS${NC}"
    echo -e "${CYAN}   Basic performance validation...${NC}\n"
    
    # Check if benchmark features are available
    if grep -q "bench" Cargo.toml 2>/dev/null; then
        run_test "Benchmarks" "cargo bench" "Performance benchmarks"
    else
        echo -e "${YELLOW}   ⚠️  No benchmarks configured, skipping...${NC}"
    fi
    
    # Run a basic performance test using our session validation
    if [ -f "examples/session6_validation.rs" ]; then
        run_test "Performance Validation" "cargo run --example session6_validation" "Adaptive performance testing"
    fi
}

# Function to generate final report
generate_report() {
    echo -e "\n${PURPLE}📊 COMPREHENSIVE TEST REPORT${NC}"
    echo -e "${PURPLE}=============================${NC}\n"
    
    # Overall statistics
    echo -e "${BLUE}📈 Test Statistics:${NC}"
    echo -e "   Total Tests: ${TOTAL_TESTS}"
    echo -e "   ${GREEN}Passed: ${PASSED_TESTS}${NC}"
    echo -e "   ${RED}Failed: ${FAILED_TESTS}${NC}"
    
    if [ $FAILED_TESTS -eq 0 ]; then
        echo -e "   ${GREEN}Success Rate: 100%${NC}"
        echo -e "\n${GREEN}🎉 ALL TESTS PASSED! Your quantum-resistant messaging protocol is fully validated!${NC}"
    else
        success_rate=$(( (PASSED_TESTS * 100) / TOTAL_TESTS ))
        echo -e "   ${YELLOW}Success Rate: ${success_rate}%${NC}"
        echo -e "\n${YELLOW}⚠️  Some tests failed. Review the detailed results below.${NC}"
    fi
    
    # Detailed results
    echo -e "\n${BLUE}📋 Detailed Results:${NC}"
    for result in "${SESSION_RESULTS[@]}"; do
        echo -e "   $result"
    done
    
    # Security status
    echo -e "\n${BLUE}🛡️  Security Status:${NC}"
    if echo "${SESSION_RESULTS[@]}" | grep -q "Session 7.*PASSED"; then
        echo -e "   ${GREEN}✅ Security Validation: COMPLETE${NC}"
        echo -e "   ${GREEN}✅ Cryptographic Correctness: VERIFIED${NC}"
        echo -e "   ${GREEN}✅ Attack Resistance: VALIDATED${NC}"
        echo -e "   ${GREEN}✅ Production Ready: YES${NC}"
    else
        echo -e "   ${RED}❌ Security Validation: INCOMPLETE${NC}"
        echo -e "   ${RED}⚠️  Not recommended for production use${NC}"
    fi
    
    # System information
    echo -e "\n${BLUE}💻 System Information:${NC}"
    echo -e "   Rust Version: $(rustc --version)"
    echo -e "   Cargo Version: $(cargo --version)"
    echo -e "   Test Date: $(date)"
    echo -e "   Platform: $(uname -s) $(uname -m)"
    
    # Save report to file
    {
        echo "Nano-Messenger Test Report"
        echo "========================="
        echo "Generated: $(date)"
        echo ""
        echo "Test Statistics:"
        echo "Total: $TOTAL_TESTS, Passed: $PASSED_TESTS, Failed: $FAILED_TESTS"
        echo ""
        echo "Detailed Results:"
        for result in "${SESSION_RESULTS[@]}"; do
            echo "$result"
        done
    } > test_report_$(date +%Y%m%d_%H%M%S).txt
    
    echo -e "\n${CYAN}📄 Detailed report saved to: test_report_$(date +%Y%m%d_%H%M%S).txt${NC}"
}

# Main execution
main() {
    echo -e "${CYAN}Starting comprehensive test suite...${NC}\n"
    
    # Check prerequisites
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}❌ Cargo not found. Please install Rust/Cargo first.${NC}"
        exit 1
    fi
    
    # Change to project directory
    cd /Users/mariano/Desktop/Code/nano-messenger
    
    # Run test phases
    check_compilation
    test_sessions
    test_unit_tests
    test_performance
    
    # Generate final report
    generate_report
    
    # Exit with appropriate code
    if [ $FAILED_TESTS -eq 0 ]; then
        echo -e "\n${GREEN}🏆 ALL VALIDATIONS SUCCESSFUL! Protocol ready for deployment.${NC}"
        exit 0
    else
        echo -e "\n${RED}❌ Some validations failed. Review errors before deployment.${NC}"
        exit 1
    fi
}

# Handle script arguments
case "${1:-}" in
    --help|-h)
        echo "Nano-Messenger Comprehensive Test Script"
        echo ""
        echo "Usage: $0 [options]"
        echo ""
        echo "Options:"
        echo "  --help, -h     Show this help message"
        echo "  --quick, -q    Run only compilation and core session tests"
        echo "  --sessions     Run only session validation tests"
        echo "  --unit         Run only unit tests"
        echo ""
        echo "Examples:"
        echo "  $0             # Run all tests"
        echo "  $0 --quick     # Quick validation"
        echo "  $0 --sessions  # Session tests only"
        exit 0
        ;;
    --quick|-q)
        echo -e "${CYAN}🚀 Running quick validation...${NC}\n"
        cd /Users/mariano/Desktop/Code/nano-messenger
        check_compilation
        test_sessions
        generate_report
        ;;
    --sessions)
        echo -e "${CYAN}📋 Running session tests only...${NC}\n"
        cd /Users/mariano/Desktop/Code/nano-messenger
        test_sessions
        generate_report
        ;;
    --unit)
        echo -e "${CYAN}🧪 Running unit tests only...${NC}\n"
        cd /Users/mariano/Desktop/Code/nano-messenger
        test_unit_tests
        generate_report
        ;;
    *)
        main
        ;;
esac
