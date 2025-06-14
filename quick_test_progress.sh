#!/bin/bash

echo "🧪 QUICK COMPILATION TEST AFTER FIXES"
echo "====================================="

cd "$(dirname "$0")"

echo "Running compilation test..."

# Get error count before and after
ERROR_OUTPUT=$(cargo check --lib 2>&1)
ERROR_COUNT=$(echo "$ERROR_OUTPUT" | grep -c "error\[" || echo "0")

echo "Current error count: $ERROR_COUNT"

if [ "$ERROR_COUNT" -eq "0" ]; then
    echo "🎉 SUCCESS! All compilation errors fixed!"
    
    echo ""
    echo "Testing examples..."
    
    # Test session 1
    if cargo check --example session1_validation 2>&1 | grep -q "error:"; then
        echo "❌ Session 1 example has issues"
        cargo check --example session1_validation 2>&1 | head -3
    else
        echo "✅ Session 1 example: WORKS!"
    fi
    
    # Test session 12
    if cargo check --example session12_basic_validation 2>&1 | grep -q "error:"; then
        echo "❌ Session 12 example has issues" 
        cargo check --example session12_basic_validation 2>&1 | head -3
    else
        echo "✅ Session 12 example: WORKS!"
    fi
    
elif [ "$ERROR_COUNT" -lt "20" ]; then
    echo "🚀 Great progress! Down to $ERROR_COUNT errors"
    echo ""
    echo "Remaining errors:"
    echo "$ERROR_OUTPUT" | grep "error\[" | head -5
else
    echo "📊 Still working on it... $ERROR_COUNT errors remaining"
    echo ""
    echo "Top 3 errors:"
    echo "$ERROR_OUTPUT" | grep "error\[" | head -3
fi

echo ""
echo "🏁 Test complete!"
