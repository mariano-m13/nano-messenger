#!/bin/bash

echo "🔧 QUICK COMPILATION TEST AFTER FIXING E0753"
echo "============================================"
echo ""

cd /Users/mariano/Desktop/Code/nano-messenger

echo "🧪 Testing library compilation..."
if cargo check --lib --message-format=short 2>&1 | tee quick_test_results.log; then
    echo ""
    echo "✅ SUCCESS! Library compiles!"
    
    # Count warnings
    WARNING_COUNT=$(grep -c "warning:" quick_test_results.log || echo "0")
    echo "📊 Compilation completed with $WARNING_COUNT warnings"
    
else
    echo ""
    echo "❌ Still has compilation errors. Analyzing top issues..."
    echo ""
    
    # Count errors by type
    E0753_COUNT=$(grep -c "error\[E0753\]" quick_test_results.log || echo "0")
    E0596_COUNT=$(grep -c "error\[E0596\]" quick_test_results.log || echo "0")
    E0283_COUNT=$(grep -c "error\[E0283\]" quick_test_results.log || echo "0")
    E0277_COUNT=$(grep -c "error\[E0277\]" quick_test_results.log || echo "0")
    E0599_COUNT=$(grep -c "error\[E0599\]" quick_test_results.log || echo "0")
    
    echo "📊 Error Breakdown:"
    echo "E0753 (doc comments): $E0753_COUNT"
    echo "E0596 (borrowing): $E0596_COUNT"
    echo "E0283 (type annotations): $E0283_COUNT"
    echo "E0277 (trait bounds): $E0277_COUNT" 
    echo "E0599 (method not found): $E0599_COUNT"
    echo ""
    
    echo "🔍 Next critical errors to fix:"
    grep -A1 "error\[" quick_test_results.log | head -10
fi

echo ""
echo "🏁 Quick test complete!"
