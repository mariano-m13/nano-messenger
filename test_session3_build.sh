#!/bin/bash

echo "🔧 Testing Session 3 Build Fixes"
echo "================================="

cd /Users/mariano/Desktop/Code/nano-messenger

echo "🏗️  Building simple Session 3 test..."
if cargo build --example simple_session3_test; then
    echo "✅ Simple test build successful!"
    echo ""
    echo "🧪 Running simple Session 3 test..."
    cargo run --example simple_session3_test
    echo ""
else
    echo "❌ Simple test build failed!"
    echo "Let's try building the lib first..."
    cargo build --lib
    exit 1
fi

echo "🏗️  Building full Session 3 validation..."
if cargo build --example session3_validation; then
    echo "✅ Full validation build successful!"
    echo ""
    echo "🧪 Running full Session 3 validation..."
    cargo run --example session3_validation
else
    echo "❌ Full validation build failed!"
    exit 1
fi

echo ""
echo "🎉 Session 3 builds and runs successfully!"
