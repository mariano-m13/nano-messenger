#!/bin/bash

echo "🔧 Testing Build After Fixing CLI Issue"
echo "======================================="

cd /Users/mariano/Desktop/Code/nano-messenger

echo "📋 Changes made:"
echo "   • Removed #[cfg(test)] from test_session_11 module"
echo "   • Made test functions available for CLI binary"
echo ""

echo "🏗️  Testing library compilation..."
if cargo check --lib --quiet; then
    echo "✅ Library compiles successfully"
else
    echo "❌ Library compilation failed"
    exit 1
fi

echo ""
echo "🏗️  Testing binary compilation..."
if cargo check --bin session11_cli --quiet; then
    echo "✅ CLI binary compiles successfully"
else
    echo "❌ CLI binary compilation failed"
    echo "Showing specific binary errors:"
    cargo check --bin session11_cli 2>&1 | tail -10
    exit 1
fi

echo ""
echo "🏗️  Testing full build..."
if timeout 120 cargo build --quiet 2>/dev/null; then
    echo "✅ Full build successful!"
    echo ""
    echo "🎉 ALL ISSUES RESOLVED!"
    echo ""
    echo "✅ Lifetime issue: FIXED"
    echo "✅ CLI compilation: FIXED" 
    echo "✅ Full build: WORKING"
    echo ""
    echo "🚀 Ready to use:"
    echo "   • cargo run --bin session11_cli test quick"
    echo "   • cargo run --bin session11_cli upload <file>"
    echo "   • cargo test"
else
    echo "❌ Full build failed"
    echo "Showing errors:"
    cargo build 2>&1 | tail -15
fi
