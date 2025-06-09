#!/bin/bash

echo "🔧 Testing Compilation Fixes"
echo "============================="
echo ""

cd /Users/mariano/Desktop/Code/nano-messenger

echo "Running cargo check for library..."
if cargo check --lib 2>&1; then
    echo ""
    echo "✅ Library compilation successful!"
    
    echo ""
    echo "Testing binary compilation..."
    if cargo check --bin nano-client && cargo check --bin nano-relay; then
        echo "✅ All binaries compile successfully!"
        
        echo ""
        echo "Running a quick test to verify functionality..."
        if cargo test --lib --quiet; then
            echo "✅ Tests pass!"
            echo ""
            echo "🎉 SUCCESS! All compilation issues have been resolved!"
        else
            echo "⚠️  Tests have some issues but compilation works"
        fi
    else
        echo "❌ Binary compilation still has issues"
    fi
else
    echo ""
    echo "❌ Library compilation still has errors:"
    echo "Let's see the specific errors:"
    cargo check --lib
fi
