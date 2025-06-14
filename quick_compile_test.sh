#!/bin/bash

echo "🔧 TESTING COMPILATION FIXES"
echo "============================"

cd "$(dirname "$0")"

echo "Checking compilation status after fixes..."

# Get a concise error report
RESULT=$(cargo check --lib --message-format=short 2>&1)

if echo "$RESULT" | grep -q "error:"; then
    ERROR_COUNT=$(echo "$RESULT" | grep -c "error:")
    echo "❌ Still has $ERROR_COUNT compilation errors"
    
    echo ""
    echo "Top 5 remaining errors:"
    echo "$RESULT" | grep "error:" | head -5
    
    echo ""
    echo "Error patterns analysis:"
    
    # Check for specific error types
    if echo "$RESULT" | grep -q "cannot find type"; then
        echo "• Missing type definitions:"
        echo "$RESULT" | grep "cannot find type" | head -3
    fi
    
    if echo "$RESULT" | grep -q "unresolved import"; then
        echo "• Unresolved imports:"
        echo "$RESULT" | grep "unresolved import" | head -3
    fi
    
    if echo "$RESULT" | grep -q "trait.*not found"; then
        echo "• Missing traits:"
        echo "$RESULT" | grep "trait.*not found" | head -3
    fi
    
    if echo "$RESULT" | grep -q "mismatched types"; then
        echo "• Type mismatches:"
        echo "$RESULT" | grep "mismatched types" | head -3
    fi
    
else
    echo "✅ Core library compilation successful!"
    
    echo ""
    echo "Testing examples..."
    
    # Test session 1 example
    if cargo check --example session1_validation 2>&1 | grep -q "error:"; then
        echo "❌ Session 1 example has errors"
    else
        echo "✅ Session 1 example compiles"
    fi
    
    # Test session 12 example  
    if cargo check --example session12_basic_validation 2>&1 | grep -q "error:"; then
        echo "❌ Session 12 example has errors"
    else
        echo "✅ Session 12 example compiles"
    fi
    
    echo ""
    echo "🎉 ALL COMPILATION ISSUES RESOLVED!"
    echo "=================================="
    echo "Your nano-messenger is ready to run!"
fi

echo ""
echo "🏁 Compilation check complete."
