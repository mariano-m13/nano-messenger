#!/bin/bash

echo "🔧 COMPREHENSIVE COMPILATION ANALYSIS"
echo "===================================="
echo ""

cd /Users/mariano/Desktop/Code/nano-messenger

echo "🔍 Running detailed compilation analysis..."
echo ""

# Clean build first
cargo clean --quiet

# Get detailed error output
echo "📋 Detailed compilation errors:"
cargo check --lib --message-format=human 2>&1 | tee detailed_errors.log

echo ""
echo "🎯 Error Summary:"
ERROR_COUNT=$(grep -c "error\[" detailed_errors.log 2>/dev/null || echo "0")
WARNING_COUNT=$(grep -c "warning:" detailed_errors.log 2>/dev/null || echo "0")

echo "Total errors: $ERROR_COUNT"
echo "Total warnings: $WARNING_COUNT"

if [[ $ERROR_COUNT -gt 0 ]]; then
    echo ""
    echo "🚨 Most common error types:"
    grep "error\[E" detailed_errors.log | cut -d']' -f1 | sort | uniq -c | sort -nr | head -5
    
    echo ""
    echo "📍 Specific E0004 (non-exhaustive pattern) errors:"
    grep -A5 "error\[E0004\]" detailed_errors.log | head -20
    
    echo ""
    echo "📍 Other critical errors:"
    grep -A2 "error\[E" detailed_errors.log | grep -v "E0004" | head -15
fi

echo ""
echo "🏁 Analysis complete! Check detailed_errors.log for full output."
