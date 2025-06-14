#!/bin/bash

echo "🔍 QUICK DIAGNOSTIC CHECK"
echo "========================"
echo ""

cd /Users/mariano/Desktop/Code/nano-messenger

# Check if the issue is with test compilation vs library compilation
echo "1. Checking library compilation..."
if cargo check --lib 2>&1 | tee lib_check.log; then
    echo "✅ Library compiles successfully"
else
    echo "❌ Library compilation failed"
    echo "Errors: $(grep -c "error\[E" lib_check.log || echo "0")"
fi

echo -e "\n2. Checking test compilation..."
if cargo check --tests 2>&1 | tee test_check.log; then
    echo "✅ Tests compile successfully"
else
    echo "❌ Test compilation failed"
    echo "Errors: $(grep -c "error\[E" test_check.log || echo "0")"
fi

echo -e "\n3. Checking examples compilation..."
if cargo check --examples 2>&1 | tee examples_check.log; then
    echo "✅ Examples compile successfully"
else
    echo "❌ Examples compilation failed"
    echo "Errors: $(grep -c "error\[E" examples_check.log || echo "0")"
fi

echo -e "\n4. Checking with all features..."
if cargo check --all-features 2>&1 | tee all_features_check.log; then
    echo "✅ All features compile successfully"
else
    echo "❌ All features compilation failed"
    echo "Errors: $(grep -c "error\[E" all_features_check.log || echo "0")"
fi

echo -e "\n📊 Summary:"
echo "Library errors: $(grep -c "error\[E" lib_check.log 2>/dev/null || echo "0")"
echo "Test errors: $(grep -c "error\[E" test_check.log 2>/dev/null || echo "0")"
echo "Example errors: $(grep -c "error\[E" examples_check.log 2>/dev/null || echo "0")"
echo "All features errors: $(grep -c "error\[E" all_features_check.log 2>/dev/null || echo "0")"

# If any have errors, show the first few
for log in lib_check.log test_check.log examples_check.log all_features_check.log; do
    if [ -f "$log" ] && grep -q "error\[E" "$log" 2>/dev/null; then
        echo -e "\n❌ Errors in $log:"
        grep -A2 "error\[E" "$log" | head -10
    fi
done

echo -e "\n🏁 Diagnostic complete!"
