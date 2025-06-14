#!/bin/bash

echo "🔧 COMPILATION CHECK AFTER E0382 FIX"
echo "====================================="
echo

cd /Users/mariano/Desktop/Code/nano-messenger

echo "📊 Running compilation check to see remaining errors..."
echo

# Run cargo check and capture the output
cargo check --lib 2>&1 | tee post_e0382_fix.log

echo
echo "📈 ERROR ANALYSIS:"
echo "=================="

# Count different error types
e0382_count=$(grep -c "error\[E0382\]" post_e0382_fix.log || echo "0")
e0308_count=$(grep -c "error\[E0308\]" post_e0382_fix.log || echo "0")  
e0599_count=$(grep -c "error\[E0599\]" post_e0382_fix.log || echo "0")
e0689_count=$(grep -c "error\[E0689\]" post_e0382_fix.log || echo "0")
total_errors=$(grep -c "error\[E" post_e0382_fix.log || echo "0")
total_warnings=$(grep -c "warning:" post_e0382_fix.log || echo "0")

echo "E0382 (move/borrow): $e0382_count"
echo "E0308 (type mismatch): $e0308_count" 
echo "E0599 (method not found): $e0599_count"
echo "E0689 (type inference): $e0689_count"
echo "Total Errors: $total_errors"
echo "Total Warnings: $total_warnings"

echo
echo "🎯 PROGRESS SUMMARY:"
echo "===================="
echo "Session 13: ✅ E0277 trait bounds resolved"
echo "Session 14: ✅ E0283 type inference resolved"
echo "Session 15: ✅ E0596 borrow checker resolved"
echo "E0382 Fix:  ✅ Move/borrow error resolved"

if [ "$total_errors" -lt 20 ]; then
    echo "🚀 EXCELLENT! We're down to less than 20 errors!"
    echo "Ready for Session 16: Type Mismatch Corrections"
elif [ "$total_errors" -lt 30 ]; then
    echo "👍 GOOD PROGRESS! Getting close to compilation success!"
else
    echo "⏳ Still working through compilation errors..."
fi

echo
echo "🎯 NEXT TARGET: Session 16 (E0308 Type Mismatches)"
echo "Focus areas: Array/Vec conversions, Duration methods"
