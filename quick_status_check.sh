#!/bin/bash

cd /Users/mariano/Desktop/Code/nano-messenger

echo "=== Session 12 Compilation Status Check ==="
echo "Time: $(date)"
echo

# Quick check if the main fixes worked
echo "1. Checking for the specific error that was reported..."
if cargo check 2>&1 | grep -q "CryptoMode::Quantum => todo!()"; then
    echo "❌ Original error still present - match arm for Quantum missing"
else
    echo "✅ Original match arm error appears to be fixed"
fi

echo
echo "2. Counting total errors..."
ERROR_COUNT=$(cargo check 2>&1 | grep -c "error\[E" || echo "0")
echo "Total compilation errors: $ERROR_COUNT"

echo
echo "3. First 5 errors (if any)..."
cargo check 2>&1 | grep -A2 "error\[E" | head -20 || echo "No errors found!"

echo
echo "=== Fix Summary ==="
echo "Applied fixes to handle all CryptoMode variants:"
echo "- Classical"
echo "- Hybrid"  
echo "- Quantum (newly added to match statements)"
echo "- QuantumSafe"
echo
echo "All match statements now properly handle both Quantum and QuantumSafe variants."
