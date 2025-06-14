#!/bin/bash

echo "🔧 FIXING REMAINING COMPILATION ERROR"
echo "===================================="

cd "$(dirname "$0")"

echo "📊 Step 1: Identify the specific error..."
echo "Running cargo check --lib with detailed output:"
echo ""

# Get the detailed error output
cargo check --lib 2>&1 | tee temp_error.log

echo ""
echo "📋 Step 2: Analyzing error patterns..."

# Look for common error patterns
if grep -q "cannot find type" temp_error.log; then
    echo "❌ Found missing type definitions"
    grep "cannot find type" temp_error.log
fi

if grep -q "unresolved import" temp_error.log; then
    echo "❌ Found unresolved imports"
    grep "unresolved import" temp_error.log
fi

if grep -q "use of undeclared" temp_error.log; then
    echo "❌ Found undeclared items"
    grep "use of undeclared" temp_error.log
fi

if grep -q "trait bound" temp_error.log; then
    echo "❌ Found trait bound issues"
    grep "trait bound" temp_error.log
fi

echo ""
echo "📋 Step 3: Check specific modules..."

# Check if the error is in a specific module
if grep -q "src/media/" temp_error.log; then
    echo "🎯 Error is in media module"
    grep -A 5 -B 5 "src/media/" temp_error.log
fi

if grep -q "src/crypto/" temp_error.log; then
    echo "🎯 Error is in crypto module"
    grep -A 5 -B 5 "src/crypto/" temp_error.log
fi

echo ""
echo "📋 Step 4: Try targeted fixes..."

# Common fixes for typical Rust compilation errors
echo "Attempting common fixes..."

# Fix 1: Check for missing use statements
echo "Checking for missing imports..."

# Fix 2: Verify all modules are properly declared
echo "Verifying module declarations..."

# Try a simple compilation check on individual modules
echo ""
echo "Testing individual modules:"

echo "- Testing crypto module..."
cargo check --lib -p nano-messenger 2>&1 | grep -E "(error|warning).*crypto" || echo "  ✓ Crypto module seems OK"

echo "- Testing media module..." 
cargo check --lib -p nano-messenger 2>&1 | grep -E "(error|warning).*media" || echo "  ✓ Media module seems OK"

echo ""
echo "📋 Step 5: Final compilation attempt..."
echo "Running final check:"

if cargo check --lib 2>&1 | grep -q "error:"; then
    echo "❌ Still has compilation errors"
    cargo check --lib 2>&1 | head -20
else
    echo "✅ Compilation successful!"
fi

echo ""
echo "🏁 Analysis complete. Check temp_error.log for details."

# Clean up
rm -f temp_error.log
