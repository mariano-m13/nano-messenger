#!/bin/bash

echo "=== Checking for Remaining CryptoMode Issues ==="
cd /Users/mariano/Desktop/Code/nano-messenger

echo "1. Searching for incomplete match statements on CryptoMode..."
echo

# Search for match statements that might be incomplete
echo "Files containing 'match' and 'CryptoMode':"
grep -r "match.*crypto" src/ 2>/dev/null | grep -v "target/" | head -20 || true

echo
echo "2. Searching for match statements with QuantumSafe but not Quantum..."
grep -r "QuantumSafe =>" src/ 2>/dev/null | grep -v "Quantum =>" | head -20 || true

echo
echo "3. Files importing CryptoMode:"
grep -r "use.*CryptoMode" src/ 2>/dev/null | head -20 || true

echo
echo "4. Running cargo check with JSON output for structured errors..."
cargo check --message-format=json 2>&1 | grep '"level":"error"' | head -10 || true

echo
echo "5. Quick test of the most common error types..."
cargo check 2>&1 | grep -E "(E0004|E0063|E0255|E0277|E0425)" | sort | uniq -c || true

echo
echo "=== End of Analysis ==="
