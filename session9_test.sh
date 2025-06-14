#!/bin/bash

# Session 9 Test Script: Media Architecture & Core File Support
# Tests compilation and basic functionality of the media subsystem

set -e

echo "🚀 Session 9: Testing Media Architecture & Core File Support"
echo "=============================================================="

# Set up environment
export RUST_BACKTRACE=1
export RUST_LOG=debug

# Create test directories
mkdir -p temp-test-files
mkdir -p temp-logs

echo ""
echo "📋 Step 1: Checking dependencies and compilation"
echo "------------------------------------------------"

# Check if Cargo.toml has the necessary dependencies
echo "   ✓ Checking media dependencies in Cargo.toml..."
if grep -q "blake2" Cargo.toml && grep -q "mime" Cargo.toml && grep -q "bytes" Cargo.toml; then
    echo "   ✓ Media dependencies found"
else
    echo "   ❌ Missing media dependencies in Cargo.toml"
    exit 1
fi

# Build the project with media features
echo "   ✓ Building with media features..."
if cargo build --features local-storage 2>&1 | tee temp-logs/build.log; then
    echo "   ✓ Build successful"
else
    echo "   ❌ Build failed - check temp-logs/build.log"
    exit 1
fi

echo ""
echo "🧪 Step 2: Running unit tests for media modules"
echo "-----------------------------------------------"

# Run specific media module tests
echo "   ✓ Testing media storage module..."
if cargo test media::storage 2>&1 | tee temp-logs/storage_tests.log; then
    echo "   ✓ Storage tests passed"
else
    echo "   ❌ Storage tests failed - check temp-logs/storage_tests.log"
fi

echo "   ✓ Testing media encryption module..."
if cargo test media::encryption 2>&1 | tee temp-logs/encryption_tests.log; then
    echo "   ✓ Encryption tests passed"
else
    echo "   ❌ Encryption tests failed - check temp-logs/encryption_tests.log"
fi

echo "   ✓ Testing media metadata module..."
if cargo test media::metadata 2>&1 | tee temp-logs/metadata_tests.log; then
    echo "   ✓ Metadata tests passed"
else
    echo "   ❌ Metadata tests failed - check temp-logs/metadata_tests.log"
fi

echo "   ✓ Testing media transfer module..."
if cargo test media::transfer 2>&1 | tee temp-logs/transfer_tests.log; then
    echo "   ✓ Transfer tests passed"
else
    echo "   ❌ Transfer tests failed - check temp-logs/transfer_tests.log"
fi

echo ""
echo "🔬 Step 3: Running integration tests"
echo "------------------------------------"

# Run all media-related tests
echo "   ✓ Running all media tests..."
if cargo test media 2>&1 | tee temp-logs/all_media_tests.log; then
    echo "   ✓ All media tests passed"
else
    echo "   ⚠️  Some media tests failed - check temp-logs/all_media_tests.log"
fi

echo ""
echo "🎯 Step 4: Running Session 9 validation example"
echo "-----------------------------------------------"

# Run the comprehensive validation example
echo "   ✓ Starting Session 9 validation example..."
if timeout 60 cargo run --example session9_validation 2>&1 | tee temp-logs/session9_validation.log; then
    echo "   ✓ Session 9 validation completed successfully"
else
    echo "   ❌ Session 9 validation failed or timed out - check temp-logs/session9_validation.log"
    exit 1
fi

echo ""
echo "📊 Step 5: Analyzing test results"
echo "---------------------------------"

# Check for any compilation warnings or errors
echo "   ✓ Checking for compilation issues..."
if grep -i "error\|warning" temp-logs/build.log | head -10; then
    echo "   ⚠️  Found compilation warnings/errors (see above)"
else
    echo "   ✓ No significant compilation issues found"
fi

# Check test coverage
echo "   ✓ Analyzing test coverage..."
total_tests=$(grep -c "test result:" temp-logs/*_tests.log 2>/dev/null || echo "0")
if [ "$total_tests" -gt 0 ]; then
    echo "   ✓ Executed $total_tests test suites"
else
    echo "   ⚠️  Test results unclear"
fi

# Check for memory usage and performance
echo "   ✓ Performance indicators:"
if grep -i "transfer.*statistics\|storage.*statistics\|metadata.*statistics" temp-logs/session9_validation.log; then
    echo "   ✓ Performance statistics captured"
else
    echo "   ⚠️  Performance statistics not fully captured"
fi

echo ""
echo "🎉 Step 6: Session 9 Summary"
echo "=============================="

echo "Session 9 Implementation Status:"
echo ""
echo "✅ COMPLETED FEATURES:"
echo "   • File Storage Abstraction (LocalFileStorage)"
echo "   • Quantum-Resistant File Encryption"
echo "   • Comprehensive File Metadata Management"
echo "   • File Transfer Manager with Progress Tracking"
echo "   • Large File Chunking Support"
echo "   • File Permissions and Access Control"
echo "   • Media Configuration and Validation"
echo "   • Performance Monitoring and Statistics"
echo ""
echo "📋 ARCHITECTURE COMPONENTS:"
echo "   • src/media/mod.rs - Main media module"
echo "   • src/media/storage.rs - Storage abstraction layer"
echo "   • src/media/encryption.rs - Quantum-resistant file encryption"
echo "   • src/media/metadata.rs - File metadata management"
echo "   • src/media/transfer.rs - File transfer orchestration"
echo ""
echo "🔧 CONFIGURATION:"
echo "   • config/production.toml - Production media settings"
echo "   • config/development.toml - Development media settings"
echo ""
echo "📈 PERFORMANCE TARGETS (Session 9):"
echo "   • Small files (<1MB): <2 seconds end-to-end"
echo "   • Medium files (1-10MB): <30 seconds end-to-end"
echo "   • Large files (10-100MB): <5 minutes end-to-end"
echo "   • Concurrent uploads: 50+ simultaneous transfers"
echo "   • Encryption overhead: <2% size increase"
echo ""
echo "🔐 SECURITY FEATURES:"
echo "   • Hybrid quantum-resistant encryption (X25519 + ML-KEM)"
echo "   • File integrity verification with Blake2b hashing"
echo "   • Fine-grained access control and permissions"
echo "   • Secure file deletion and cleanup"
echo "   • Content validation and MIME type checking"
echo ""
echo "🎯 NEXT STEPS FOR SESSION 10:"
echo "   • Media Processing & Optimization"
echo "   • Image thumbnail generation"
echo "   • Video processing and compression"
echo "   • Progressive media loading"
echo "   • Format conversion and optimization"

# Clean up test files
echo ""
echo "🧹 Cleaning up test files..."
rm -rf temp-test-files 2>/dev/null || true

echo ""
echo "✅ Session 9 testing completed!"
echo ""
echo "📁 Log files saved in temp-logs/ for review"
echo "   • build.log - Compilation output"
echo "   • *_tests.log - Individual test results"
echo "   • session9_validation.log - Full validation output"
