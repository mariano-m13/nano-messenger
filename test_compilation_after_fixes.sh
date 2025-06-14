#!/bin/bash

cd /Users/mariano/Desktop/Code/nano-messenger

echo "🔧 Testing nano-messenger compilation after fixes..."
echo "================================================"
echo ""

echo "🏗️  Running cargo build..."
cargo build 2>&1 | tail -30

if [ ${PIPESTATUS[0]} -eq 0 ]; then
    echo ""
    echo "✅ Build successful!"
    echo ""
    echo "🧪 Running cargo test --no-run to check test compilation..."
    cargo test --no-run 2>&1 | tail -30
    
    if [ ${PIPESTATUS[0]} -eq 0 ]; then
        echo ""
        echo "✅ All tests compile successfully!"
        echo ""
        echo "🎉 All compilation issues have been resolved! 🎉"
    else
        echo ""
        echo "❌ Test compilation failed"
    fi
else
    echo ""
    echo "❌ Build failed"
fi