#!/bin/bash

echo "🔧 FIXING PERMISSIONS AND RUNNING COMPILATION TEST"
echo "=================================================="
echo ""

# Fix permissions for all shell scripts
echo "📋 Making all shell scripts executable..."
find /Users/mariano/Desktop/Code/nano-messenger -name "*.sh" -type f -exec chmod +x {} \;
echo "✅ Permissions fixed"
echo ""

# Navigate to project directory
cd /Users/mariano/Desktop/Code/nano-messenger

echo "🏗️  Running cargo check to identify compilation errors..."
echo ""

# Run cargo check and capture output
if cargo check --lib 2>&1 | tee compilation_errors.log; then
    echo ""
    echo "✅ SUCCESS! Library compiles without errors!"
    echo ""
    
    # Test full project build
    echo "🔧 Testing full project build..."
    if cargo build 2>&1 | tee build_errors.log; then
        echo "✅ EXCELLENT! Full project builds successfully!"
        echo ""
        echo "🎯 Next steps:"
        echo "  cargo test               # Run all tests"
        echo "  cargo build --release    # Build optimized version"
        echo "  cargo run --bin nano-client        # Run client"
        echo "  cargo run --bin nano-relay         # Run relay server"
    else
        echo "⚠️  Library compiles but full build has issues. See build_errors.log"
    fi
else
    echo ""
    echo "❌ COMPILATION ERRORS FOUND"
    echo "=========================="
    echo ""
    echo "📊 Error Analysis:"
    echo ""
    
    # Count and categorize errors
    ERROR_COUNT=$(grep -c "error\[" compilation_errors.log || echo "0")
    WARNING_COUNT=$(grep -c "warning:" compilation_errors.log || echo "0")
    
    echo "Total errors: $ERROR_COUNT"
    echo "Total warnings: $WARNING_COUNT"
    echo ""
    
    # Show specific error types
    echo "📋 Error breakdown:"
    echo ""
    
    # Show E0004 errors (non-exhaustive patterns)
    E0004_COUNT=$(grep -c "error\[E0004\]" compilation_errors.log || echo "0")
    if [ "$E0004_COUNT" -gt 0 ]; then
        echo "🔴 E0004 (non-exhaustive patterns): $E0004_COUNT"
        echo "   Example locations:"
        grep -n "error\[E0004\]" compilation_errors.log | head -3
        echo ""
    fi
    
    # Show other common errors
    E0425_COUNT=$(grep -c "error\[E0425\]" compilation_errors.log || echo "0")
    if [ "$E0425_COUNT" -gt 0 ]; then
        echo "🔴 E0425 (cannot find): $E0425_COUNT"
        echo "   Example locations:"
        grep -n "error\[E0425\]" compilation_errors.log | head -3
        echo ""
    fi
    
    E0412_COUNT=$(grep -c "error\[E0412\]" compilation_errors.log || echo "0")
    if [ "$E0412_COUNT" -gt 0 ]; then
        echo "🔴 E0412 (cannot find type): $E0412_COUNT" 
        echo "   Example locations:"
        grep -n "error\[E0412\]" compilation_errors.log | head -3
        echo ""
    fi
    
    # Show first few errors for immediate analysis
    echo "🔍 First 10 errors:"
    echo ""
    grep -A2 "error\[" compilation_errors.log | head -20
    echo ""
    
    echo "📁 Full error log saved to: compilation_errors.log"
    echo ""
    echo "🛠️  To fix these errors, examine the specific files and line numbers above."
fi

echo ""
echo "🏁 Fix and test complete!"
