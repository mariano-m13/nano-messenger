#!/bin/bash

# Session 17: Method Resolution & Missing APIs Test
# Focus on E0599 and E0689 errors

echo "🔧 Session 17: Testing E0599 (method not found) and E0689 (numeric type ambiguity) fixes"
echo "=================================================================================="

# Set up environment
export RUST_BACKTRACE=1

# Clean previous builds
echo "🧹 Cleaning previous builds..."
cargo clean > /dev/null 2>&1

# Test basic library compilation
echo "📚 Testing library compilation..."
if cargo check --lib 2> session17_errors.log; then
    echo "✅ Library compilation successful!"
else
    echo "❌ Library compilation failed. Checking for E0599 and E0689 errors..."
    
    # Look for specific error types
    echo ""
    echo "🔍 E0599 errors (method not found):"
    grep -n "E0599" session17_errors.log || echo "  ✅ No E0599 errors found"
    
    echo ""
    echo "🔍 E0689 errors (numeric type ambiguity):"
    grep -n "E0689" session17_errors.log || echo "  ✅ No E0689 errors found"
    
    echo ""
    echo "🔍 Other compilation errors:"
    grep -E "error\[E[0-9]+\]" session17_errors.log | head -10
fi

# Test with session11-basic features
echo ""
echo "📦 Testing with session11-basic features..."
if cargo check --features="session11-basic" 2> session17_features_errors.log; then
    echo "✅ Features compilation successful!"
else
    echo "❌ Features compilation failed"
    
    # Look for method resolution issues in features
    echo ""
    echo "🔍 Method resolution errors in features:"
    grep -n "E0599\|E0689" session17_features_errors.log || echo "  ✅ No method resolution errors found"
fi

# Test specific compliance modules
echo ""
echo "📋 Testing compliance modules specifically..."
if cargo check --lib -p nano-messenger 2> session17_compliance_errors.log; then
    echo "✅ Compliance modules check successful!"
else
    echo "❌ Compliance modules check failed"
    
    echo ""
    echo "🔍 Issues in compliance modules:"
    grep -n -A 3 -B 1 "src/media/compliance" session17_compliance_errors.log || echo "  ✅ No compliance module errors found"
fi

# Summary
echo ""
echo "📊 Session 17 Summary:"
echo "======================"

if [ -f session17_errors.log ]; then
    TOTAL_ERRORS=$(grep -c "error\[E" session17_errors.log)
    E0599_COUNT=$(grep -c "E0599" session17_errors.log)
    E0689_COUNT=$(grep -c "E0689" session17_errors.log)
    
    echo "Total compilation errors: $TOTAL_ERRORS"
    echo "E0599 (method not found): $E0599_COUNT"
    echo "E0689 (numeric ambiguity): $E0689_COUNT"
    
    if [ "$E0599_COUNT" -eq 0 ] && [ "$E0689_COUNT" -eq 0 ]; then
        echo "🎉 Session 17 target errors (E0599, E0689) successfully resolved!"
    else
        echo "⚠️  Some target errors remain"
    fi
else
    echo "🎉 No compilation errors detected!"
fi

echo ""
echo "📄 Detailed error logs saved to:"
echo "  - session17_errors.log"
echo "  - session17_features_errors.log" 
echo "  - session17_compliance_errors.log"
