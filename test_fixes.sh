#!/bin/bash
cd /Users/mariano/Desktop/Code/nano-messenger

echo "Testing specific module compilation..."
echo "=========================================="

echo "1. Testing media/compatibility/mobile.rs compilation..."
cargo check --lib --quiet 2>&1 | grep -A 5 -B 5 "mobile.rs" | head -20

echo ""
echo "2. Testing if our MediaType::Unknown fix worked..."
cargo check --lib --quiet 2>&1 | grep -A 3 -B 3 "non-exhaustive" | head -10

echo ""
echo "3. Testing if MobileQualityLevel fix worked..."
cargo check --lib --quiet 2>&1 | grep -A 3 -B 3 "MobileQualityLevel" | head -10

echo ""
echo "4. Quick compilation status check..."
if cargo check --lib --quiet >/dev/null 2>&1; then
    echo "✅ Library compiles successfully!"
else
    echo "❌ Still have compilation errors"
    echo "First 10 compilation errors:"
    cargo check --lib 2>&1 | head -20
fi
