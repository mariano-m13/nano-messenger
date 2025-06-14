#!/bin/bash

# Session 11 Functional Test Script
# Actually runs the Session 11 validation example to test functionality

echo "🧪 Session 11 Functional Test Runner"
echo "===================================="

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Check if we can compile first
echo -e "${BLUE}📦 Pre-flight compilation check...${NC}"
if ! cargo check --example session11_validation --features session11-full; then
    echo -e "${RED}❌ Session 11 example doesn't compile. Please fix compilation errors first.${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Compilation successful${NC}"
echo ""

# Set up environment
echo -e "${BLUE}🔧 Setting up test environment...${NC}"
export RUST_LOG=info
export RUST_BACKTRACE=1

# Create a timeout function for the test
timeout_duration=60  # 60 seconds timeout

echo -e "${BLUE}🚀 Running Session 11 validation example...${NC}"
echo -e "${YELLOW}⏱️ Timeout set to ${timeout_duration} seconds${NC}"
echo ""

# Run the actual test with timeout
if timeout ${timeout_duration} cargo run --example session11_validation --features session11-full; then
    echo ""
    echo -e "${GREEN}🎉 Session 11 functional test PASSED!${NC}"
    echo ""
    echo "✅ Validated features:"
    echo "  - Large file chunking and parallel processing"
    echo "  - File deduplication with space savings"
    echo "  - Real-time streaming capabilities"
    echo "  - Collaborative galleries and interactions"
    echo "  - Mobile device optimization"
    echo "  - Web browser compatibility"
    echo ""
    echo -e "${GREEN}🚀 Session 11 is ready for production use!${NC}"
    exit 0
else
    exit_code=$?
    echo ""
    if [ $exit_code -eq 124 ]; then
        echo -e "${YELLOW}⏱️ Test timed out after ${timeout_duration} seconds${NC}"
        echo "This might indicate:"
        echo "  - Long-running operations (normal for file processing)"
        echo "  - Potential deadlocks or infinite loops"
        echo "  - Network operations taking too long"
        echo ""
        echo "Try running manually with: cargo run --example session11_validation"
    else
        echo -e "${RED}❌ Session 11 functional test FAILED${NC}"
        echo ""
        echo "🔧 Troubleshooting steps:"
        echo "  1. Check the error output above"
        echo "  2. Ensure all dependencies are properly installed"
        echo "  3. Run with RUST_LOG=debug for more details"
        echo "  4. Try running individual unit tests first"
        echo ""
        echo "Debug commands:"
        echo "  cargo test media::chunking --lib"
        echo "  cargo test media::deduplication --lib"
        echo "  RUST_LOG=debug cargo run --example session11_validation"
    fi
    exit 1
fi
