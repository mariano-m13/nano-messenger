# Nano-Messenger Test Runner
# Quick commands for testing the quantum-resistant messaging protocol

# Default target - run comprehensive tests
.PHONY: all test quick sessions unit compile benchmark clean help

all: test

# Run comprehensive test suite (all tests)
test:
	@echo "🧪 Running comprehensive test suite..."
	@./test_all_sessions.sh

# Quick validation (compilation + critical sessions)
quick:
	@echo "🚀 Running quick validation..."
	@./test_all_sessions.sh --quick

# Run only session validation tests
sessions:
	@echo "📋 Running session validations..."
	@./test_all_sessions.sh --sessions

# Run only unit tests
unit:
	@echo "🧪 Running unit tests..."
	@cargo test

# Test only compilation
compile:
	@echo "🔨 Testing compilation..."
	@cargo check --examples
	@cargo build

# Run integration tests (programmatic)
integration:
	@echo "🔧 Running integration tests..."
	@cargo test --test comprehensive_session_tests

# Run performance benchmarks
benchmark:
	@echo "📊 Running performance benchmarks..."
	@cargo test --test comprehensive_session_tests benchmark_all_sessions -- --ignored

# Run security validation only (Session 7)
security:
	@echo "🛡️  Running security validation..."
	@cargo run --example session7_validation

# Run specific session (usage: make session SESSION=session1)
session:
	@echo "🔍 Running $(SESSION)_validation..."
	@cargo run --example $(SESSION)_validation

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	@cargo clean
	@rm -f test_report_*.txt

# Setup - make test script executable
setup:
	@echo "⚙️  Setting up test environment..."
	@chmod +x test_all_sessions.sh
	@echo "✅ Test script is now executable"

# Show help
help:
	@echo "Nano-Messenger Test Runner"
	@echo "=========================="
	@echo ""
	@echo "Available commands:"
	@echo "  make test        - Run comprehensive test suite (all tests)"
	@echo "  make quick       - Quick validation (compilation + critical sessions)"
	@echo "  make sessions    - Run only session validation tests"
	@echo "  make unit        - Run only unit tests"
	@echo "  make integration - Run integration tests (programmatic)"
	@echo "  make compile     - Test compilation only"
	@echo "  make benchmark   - Run performance benchmarks"
	@echo "  make security    - Run security validation (Session 7)"
	@echo "  make session SESSION=session1 - Run specific session"
	@echo "  make clean       - Clean build artifacts"
	@echo "  make setup       - Setup test environment"
	@echo "  make help        - Show this help"
	@echo ""
	@echo "Examples:"
	@echo "  make test                    # Full test suite"
	@echo "  make quick                   # Quick validation"
	@echo "  make session SESSION=session7 # Test Session 7 only"
	@echo ""
	@echo "For more options with the shell script:"
	@echo "  ./test_all_sessions.sh --help"
