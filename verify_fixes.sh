#!/bin/bash

# Quick verification script for nano-messenger compilation fixes

echo "🔍 Verifying nano-messenger compilation fixes..."
echo "=============================================="
echo ""

cd /Users/mariano/Desktop/Code/nano-messenger

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Not in nano-messenger directory"
    exit 1
fi

echo "📋 Checking modified files..."
echo ""

# List the files we modified
files_modified=(
    "src/media/chunking/transfer.rs"
    "src/media/collaboration/galleries.rs"
    "src/media/processing/detection.rs"
    "src/media/chunking/integration_tests.rs"
    "src/media/chunking/mod.rs"
    "src/test_compilation.rs"
    "src/media/compatibility/web.rs"
)

for file in "${files_modified[@]}"; do
    if [ -f "$file" ]; then
        echo "✓ $file"
    else
        echo "✗ $file (missing)"
    fi
done

echo ""
echo "🏗️  Running quick compilation check..."
echo ""

# Try to compile just the library
cargo check --lib 2>&1 | grep -E "(error|warning|Checking|Finished)" | tail -20

if [ ${PIPESTATUS[0]} -eq 0 ]; then
    echo ""
    echo "✅ Library compilation check passed!"
    echo ""
    echo "🧪 Checking test compilation..."
    cargo check --tests 2>&1 | grep -E "(error|warning|Checking|Finished)" | tail -10
    
    if [ ${PIPESTATUS[0]} -eq 0 ]; then
        echo ""
        echo "🎉 All compilation checks passed! Your quantum-resistant messaging protocol is ready to build."
        echo ""
        echo "Run 'cargo build --release' for a full build."
    else
        echo ""
        echo "⚠️  Test compilation still has issues"
    fi
else
    echo ""
    echo "❌ Library compilation check failed"
fi