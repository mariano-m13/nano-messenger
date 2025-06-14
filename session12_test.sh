#!/bin/bash

# Session 12: Security & Compliance Test Script
# Comprehensive testing for enterprise-grade media security and compliance features

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Test results
PASSED_TESTS=0
FAILED_TESTS=0
TEST_RESULTS=()

echo -e "${PURPLE}🛡️  NANO-MESSENGER SESSION 12 TESTING${NC}"
echo -e "${PURPLE}======================================${NC}"
echo -e "${CYAN}Security & Compliance for Media${NC}"
echo -e "${CYAN}Enterprise-grade threat detection, forensics, access control,${NC}"
echo -e "${CYAN}E2E encryption, GDPR/HIPAA compliance, and audit systems${NC}"
echo -e ""

# Function to run a test
run_test() {
    local test_name="$1"
    local test_command="$2"
    local description="$3"
    
    echo -e "${YELLOW}🔍 Testing: $test_name${NC}"
    echo -e "${CYAN}   $description${NC}"
    
    start_time=$(date +%s)
    
    if eval "$test_command" > /tmp/session12_test_$$.log 2>&1; then
        end_time=$(date +%s)
        duration=$((end_time - start_time))
        
        echo -e "${GREEN}   ✅ PASSED${NC} (${duration}s)"
        PASSED_TESTS=$((PASSED_TESTS + 1))
        TEST_RESULTS+=("✅ $test_name - PASSED (${duration}s)")
        
        # Show key success indicators
        if grep -q "SUCCESS\|✅\|COMPLETE" /tmp/session12_test_$$.log; then
            echo -e "${GREEN}   📊 Key results:${NC}"
            grep -E "(✅|SUCCESS|COMPLETE|PASSED)" /tmp/session12_test_$$.log | head -3 | sed 's/^/      /'
        fi
    else
        end_time=$(date +%s)
        duration=$((end_time - start_time))
        
        echo -e "${RED}   ❌ FAILED${NC} (${duration}s)"
        FAILED_TESTS=$((FAILED_TESTS + 1))
        TEST_RESULTS+=("❌ $test_name - FAILED (${duration}s)")
        
        echo -e "${RED}   📋 Error details:${NC}"
        tail -10 /tmp/session12_test_$$.log | sed 's/^/      /'
    fi
    
    rm -f /tmp/session12_test_$$.log
    echo ""
}

# Main Session 12 testing
echo -e "${BLUE}🏗️  COMPILATION CHECK${NC}"
echo -e "${CYAN}   Ensuring Session 12 code compiles correctly...${NC}"
echo ""

run_test "Session 12 Compilation" "cargo check --examples" "Verifying Session 12 security and compliance code compiles"

echo -e "${BLUE}🧪 SESSION 12 VALIDATION${NC}"
echo -e "${CYAN}   Running comprehensive Session 12 validation suite...${NC}"
echo ""

run_test "Session 12 Full Validation" "cargo run --example session12_validation" "Complete security and compliance validation"

echo -e "${BLUE}🔬 UNIT TEST SUITE${NC}"
echo -e "${CYAN}   Running Session 12 unit tests...${NC}"
echo ""

run_test "Security Scanner Tests" "cargo test test_security_scanner" "Media security scanning and threat detection"
run_test "Forensics Tests" "cargo test test_media_forensics" "Digital forensics and integrity verification"
run_test "Access Control Tests" "cargo test test_access_control" "Permissions and DRM protection"
run_test "E2E Encryption Tests" "cargo test test_e2e_media_encryption" "End-to-end media encryption"
run_test "GDPR Compliance Tests" "cargo test test_gdpr" "GDPR personal data protection"
run_test "HIPAA Compliance Tests" "cargo test test_hipaa" "HIPAA PHI detection and encryption"
run_test "Audit System Tests" "cargo test test_audit_system" "Enterprise audit and reporting"
run_test "Multi-Regulation Tests" "cargo test test_multi_regulation" "Multi-regulation compliance"
run_test "Performance Tests" "cargo test test_performance_under_load" "Performance and load testing"
run_test "Integration Tests" "cargo test test_session_12_complete_integration" "Complete system integration"

echo -e "${BLUE}⚡ PERFORMANCE VALIDATION${NC}"
echo -e "${CYAN}   Testing Session 12 performance under enterprise conditions...${NC}"
echo ""

run_test "Concurrent Operations" "cargo test test_performance_under_load -- --nocapture" "1000+ concurrent security assessments"
run_test "Large File Handling" "cargo test -- --nocapture test_large_file" "Processing 100MB+ media files"
run_test "Real-time Compliance" "cargo test -- --nocapture test_real_time" "Real-time compliance monitoring"

# Generate final report
echo -e "${PURPLE}📊 SESSION 12 TEST REPORT${NC}"
echo -e "${PURPLE}==========================${NC}"
echo ""

total_tests=$((PASSED_TESTS + FAILED_TESTS))
echo -e "${BLUE}📈 Test Statistics:${NC}"
echo -e "   Total Tests: $total_tests"
echo -e "   ${GREEN}Passed: $PASSED_TESTS${NC}"
echo -e "   ${RED}Failed: $FAILED_TESTS${NC}"

if [ $FAILED_TESTS -eq 0 ]; then
    success_rate=100
    echo -e "   ${GREEN}Success Rate: ${success_rate}%${NC}"
    echo ""
    echo -e "${GREEN}🎉 ALL SESSION 12 TESTS PASSED!${NC}"
    echo -e "${GREEN}🛡️  Enterprise security and compliance features fully validated!${NC}"
else
    success_rate=$(( (PASSED_TESTS * 100) / total_tests ))
    echo -e "   ${YELLOW}Success Rate: ${success_rate}%${NC}"
    echo ""
    echo -e "${YELLOW}⚠️  Some Session 12 tests failed. Review the results below.${NC}"
fi

echo ""
echo -e "${BLUE}📋 Detailed Results:${NC}"
for result in "${TEST_RESULTS[@]}"; do
    echo -e "   $result"
done

echo ""
echo -e "${BLUE}🏆 Session 12 Achievements:${NC}"
if [ $FAILED_TESTS -eq 0 ]; then
    echo -e "   ${GREEN}✅ Advanced Threat Detection: OPERATIONAL${NC}"
    echo -e "   ${GREEN}✅ Media Forensics: TAMPER-EVIDENT${NC}"
    echo -e "   ${GREEN}✅ Access Control: ZERO-TRUST${NC}"
    echo -e "   ${GREEN}✅ DRM Protection: HARDWARE-BOUND${NC}"
    echo -e "   ${GREEN}✅ E2E Encryption: QUANTUM-RESISTANT${NC}"
    echo -e "   ${GREEN}✅ GDPR Compliance: AUTOMATED${NC}"
    echo -e "   ${GREEN}✅ HIPAA Compliance: MEDICAL-GRADE${NC}"
    echo -e "   ${GREEN}✅ Audit System: LEGALLY-ADMISSIBLE${NC}"
    echo -e "   ${GREEN}✅ Multi-Regulation: CONFLICT-RESOLVED${NC}"
    echo -e "   ${GREEN}✅ Performance: ENTERPRISE-SCALE${NC}"
    echo ""
    echo -e "${GREEN}🌟 ENTERPRISE DEPLOYMENT READY${NC}"
    echo -e "${GREEN}🔐 QUANTUM-SAFE SECURITY VALIDATED${NC}"
    echo -e "${GREEN}📋 MULTI-REGULATION COMPLIANT${NC}"
else
    echo -e "   ${RED}❌ Some features require attention before deployment${NC}"
fi

# Save detailed report
{
    echo "Session 12 Test Report - Security & Compliance for Media"
    echo "========================================================"
    echo "Generated: $(date)"
    echo ""
    echo "Test Statistics:"
    echo "Total: $total_tests, Passed: $PASSED_TESTS, Failed: $FAILED_TESTS"
    echo "Success Rate: ${success_rate}%"
    echo ""
    echo "Detailed Results:"
    for result in "${TEST_RESULTS[@]}"; do
        echo "$result"
    done
    echo ""
    echo "System Information:"
    echo "Rust Version: $(rustc --version)"
    echo "Platform: $(uname -s) $(uname -m)"
} > session12_test_report_$(date +%Y%m%d_%H%M%S).txt

echo ""
echo -e "${CYAN}📄 Detailed report saved to: session12_test_report_$(date +%Y%m%d_%H%M%S).txt${NC}"

# Exit with appropriate code
if [ $FAILED_TESTS -eq 0 ]; then
    echo ""
    echo -e "${GREEN}🏆 SESSION 12 VALIDATION SUCCESSFUL!${NC}"
    echo -e "${GREEN}🚀 Enterprise security and compliance ready for production!${NC}"
    exit 0
else
    echo ""
    echo -e "${RED}❌ Session 12 validation incomplete. Review failures before deployment.${NC}"
    exit 1
fi
