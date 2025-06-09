#!/bin/bash
cd /Users/mariano/Desktop/Code/nano-messenger

echo "🔧 Testing Final Compilation Fixes..."
echo "====================================="

echo "1. Checking library compilation..."
if cargo check --lib 2>/dev/null; then
    echo "✅ Library compiles successfully!"
else
    echo "❌ Library compilation failed:"
    cargo check --lib 2>&1 | head -10
fi

echo ""
echo "2. Running crypto tests..."
if cargo test --no-run crypto::tests 2>/dev/null; then
    echo "✅ Crypto tests compile successfully!"
else
    echo "❌ Crypto tests compilation failed:"
    cargo test --no-run crypto::tests 2>&1 | head -10
fi

echo ""
echo "3. Testing Session 1 validation example..."
if cargo check --example session1_validation 2>/dev/null; then
    echo "✅ Session 1 validation example compiles successfully!"
else
    echo "❌ Session 1 validation example compilation failed:"
    cargo check --example session1_validation 2>&1 | head -10
fi

echo ""
echo "4. Running actual crypto tests..."
if cargo test crypto::tests 2>/dev/null; then
    echo "✅ Crypto tests pass successfully!"
else
    echo "❌ Crypto tests failed:"
    cargo test crypto::tests 2>&1 | head -15
fi

echo ""
echo "🎉 Final Session 1 Status:"
echo "=========================="

# Summary
success=true

if ! cargo check --lib >/dev/null 2>&1; then
    echo "❌ Library compilation: FAILED"
    success=false
else
    echo "✅ Library compilation: SUCCESS"
fi

if ! cargo test --no-run crypto::tests >/dev/null 2>&1; then
    echo "❌ Crypto test compilation: FAILED"
    success=false
else
    echo "✅ Crypto test compilation: SUCCESS"
fi

if ! cargo check --example session1_validation >/dev/null 2>&1; then
    echo "❌ Session 1 example compilation: FAILED"
    success=false
else
    echo "✅ Session 1 example compilation: SUCCESS"
fi

if ! cargo test crypto::tests >/dev/null 2>&1; then
    echo "❌ Crypto test execution: FAILED"
    success=false
else
    echo "✅ Crypto test execution: SUCCESS"
fi

if [ "$success" = true ]; then
    echo ""
    echo "🎉 SESSION 1: COMPLETELY SUCCESSFUL! 🎉"
    echo "======================================="
    echo "✅ All compilation issues resolved"
    echo "✅ All tests passing"
    echo "✅ Ready for Session 2: Post-Quantum Dependencies"
else
    echo ""
    echo "❌ Session 1: Still has issues"
    echo "Need to fix remaining problems"
fi
