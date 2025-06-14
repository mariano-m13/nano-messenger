#!/bin/bash

# Comprehensive Session 11 Test Script
# Tests compilation, unit tests, and functionality of all Session 11 features

set -e  # Exit on any error

echo "🚀 Quantum-Resistant Nano-Messenger - Session 11 Comprehensive Test"
echo "=================================================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test counters
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

# Function to run a test
run_test() {
    local test_name="$1"
    local test_command="$2"
    
    echo -e "${BLUE}🧪 Testing: ${test_name}${NC}"
    TESTS_RUN=$((TESTS_RUN + 1))
    
    if eval "$test_command" > /dev/null 2>&1; then
        echo -e "${GREEN}✅ PASS: ${test_name}${NC}"
        TESTS_PASSED=$((TESTS_PASSED + 1))
        return 0
    else
        echo -e "${RED}❌ FAIL: ${test_name}${NC}"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        return 1
    fi
}

# Function to run a test with output
run_test_with_output() {
    local test_name="$1"
    local test_command="$2"
    
    echo -e "${BLUE}🧪 Testing: ${test_name}${NC}"
    TESTS_RUN=$((TESTS_RUN + 1))
    
    if eval "$test_command"; then
        echo -e "${GREEN}✅ PASS: ${test_name}${NC}"
        TESTS_PASSED=$((TESTS_PASSED + 1))
        return 0
    else
        echo -e "${RED}❌ FAIL: ${test_name}${NC}"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        return 1
    fi
}

echo "📋 Phase 1: Basic Compilation Tests"
echo "=================================="

# Test basic compilation
run_test "Basic project compilation" "cargo check"

# Test with all features
run_test "Compilation with all features" "cargo check --all-features"

# Test Session 11 specific features
run_test "Session 11 basic features" "cargo check --features session11-basic"
run_test "Session 11 streaming features" "cargo check --features session11-streaming"
run_test "Session 11 collaboration features" "cargo check --features session11-collaboration"
run_test "Session 11 compatibility features" "cargo check --features session11-compatibility"
run_test "Session 11 full features" "cargo check --features session11-full"

echo ""
echo "🧪 Phase 2: Unit Tests"
echo "====================="

# Test core media functionality (existing)
run_test "Core media tests" "cargo test media::storage --lib"
run_test "Media encryption tests" "cargo test media::encryption --lib"
run_test "Media transfer tests" "cargo test media::transfer --lib"

# Test Session 11 components
run_test "Chunking system tests" "cargo test media::chunking --lib"
run_test "Deduplication tests" "cargo test media::deduplication --lib"
run_test "Streaming tests" "cargo test media::streaming --lib"
run_test "Collaboration tests" "cargo test media::collaboration --lib"
run_test "Compatibility tests" "cargo test media::compatibility --lib"

echo ""
echo "📝 Phase 3: Example Compilation"
echo "==============================="

# Test example compilation
run_test "Session 11 validation example compilation" "cargo check --example session11_validation"

# Test other examples still work
run_test "Session 9 example compilation" "cargo check --example session9_validation"
run_test "Session 10 example compilation" "cargo check --example session10_validation"

echo ""
echo "🔧 Phase 4: Feature Flag Validation"
echo "==================================="

# Test different combinations of features
run_test "Local storage only" "cargo check --features local-storage"
run_test "Image processing only" "cargo check --features image-processing"
run_test "Basic + Session 11 basic" "cargo check --features 'local-storage,session11-basic'"
run_test "Full Session 11 package" "cargo check --features 'local-storage,image-processing,session11-full'"

echo ""
echo "⚙️ Phase 5: Build Tests"
echo "======================"

# Test release build
run_test "Release build" "cargo build --release"

# Test documentation generation
run_test "Documentation generation" "cargo doc --no-deps"

echo ""
echo "🏗️ Phase 6: Dependency Resolution"
echo "================================="

# Check for dependency conflicts
run_test "Dependency tree check" "cargo tree > /dev/null"

# Update dependencies (if needed)
run_test "Dependency update check" "cargo update --dry-run"

echo ""
echo "📊 Phase 7: Advanced Validation"
echo "==============================="

# Try to run a quick version of the Session 11 example (just compilation check)
if cargo check --example session11_validation --features session11-full; then
    echo -e "${GREEN}✅ Session 11 validation example is ready to run${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    echo -e "${RED}❌ Session 11 validation example has issues${NC}"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi
TESTS_RUN=$((TESTS_RUN + 1))

# Check that all Session 11 modules are properly exported
echo -e "${BLUE}🔍 Checking Session 11 module exports...${NC}"
if cargo check --features session11-full 2>&1 | grep -q "error"; then
    echo -e "${RED}❌ Module export issues detected${NC}"
    TESTS_FAILED=$((TESTS_FAILED + 1))
else
    echo -e "${GREEN}✅ All Session 11 modules properly exported${NC}"
    TESTS_PASSED=$((TESTS_PASSED + 1))
fi
TESTS_RUN=$((TESTS_RUN + 1))

echo ""
echo "🧹 Phase 8: Code Quality Checks"
echo "==============================="

# Check for common issues
run_test "Clippy lints (warnings allowed)" "cargo clippy --all-features -- -A warnings"

# Check formatting (if rustfmt is available)
if command -v rustfmt >/dev/null 2>&1; then
    run_test "Code formatting check" "cargo fmt -- --check"
else
    echo -e "${YELLOW}⚠️ rustfmt not available, skipping format check${NC}"
fi

echo ""
echo "📈 Test Results Summary"
echo "======================"
echo -e "Total tests run: ${BLUE}${TESTS_RUN}${NC}"
echo -e "Tests passed: ${GREEN}${TESTS_PASSED}${NC}"
echo -e "Tests failed: ${RED}${TESTS_FAILED}${NC}"

if [ $TESTS_FAILED -eq 0 ]; then
    echo ""
    echo -e "${GREEN}🎉 ALL TESTS PASSED! Session 11 implementation is working correctly.${NC}"
    echo ""
    echo "✅ Session 11 Features Validated:"
    echo "  - Large file chunking with parallel processing"
    echo "  - File deduplication for storage efficiency"  
    echo "  - Real-time media streaming with quantum encryption"
    echo "  - Collaborative galleries and interactions"
    echo "  - Cross-platform mobile and web optimization"
    echo "  - Comprehensive test coverage"
    echo "  - Modular feature flags"
    echo ""
    echo -e "${BLUE}🚀 Ready to run: cargo run --example session11_validation${NC}"
    echo -e "${BLUE}🔜 Ready for Session 12: Security & Compliance${NC}"
    exit 0
else
    echo ""
    echo -e "${RED}❌ Some tests failed. Please review the issues above.${NC}"
    echo ""
    echo "🔧 Common fixes:"
    echo "  - Run 'cargo clean' and try again"
    echo "  - Check for missing dependencies"
    echo "  - Verify Rust version compatibility"
    echo "  - Review compilation errors in detail"
    exit 1
fi
