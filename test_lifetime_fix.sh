#!/bin/bash

echo "🔧 Testing the lifetime fix for transfer.rs"
echo "============================================"

cd /Users/mariano/Desktop/Code/nano-messenger

echo "✅ Verifying fix is in place..."
if grep -q "let permit = match semaphore_clone.acquire().await" src/media/chunking/transfer.rs; then
    echo "   ✓ Semaphore acquisition moved inside task"
else
    echo "   ❌ Fix not properly applied"
    exit 1
fi

echo ""
echo "🏗️  Testing library compilation..."
cargo check --lib --quiet

if [ $? -eq 0 ]; then
    echo "   ✅ Library compiles successfully!"
    echo ""
    echo "🧪 Testing with specific target..."
    cargo check --lib --target-dir /tmp/nano-test-build 2>&1 | head -10
    
    if [ $? -eq 0 ]; then
        echo ""
        echo "🎉 COMPILATION SUCCESS! Lifetime issue resolved."
        echo ""
        echo "📋 Summary of fix:"
        echo "   • Moved semaphore.acquire() inside tokio::spawn task"
        echo "   • Eliminated borrowing conflict with 'static lifetime requirement"
        echo "   • Maintained proper error handling for permit acquisition"
        echo "   • All Arc clones now have appropriate lifetimes"
    else
        echo "❌ Some compilation issues remain"
    fi
else
    echo "   ❌ Library compilation failed"
    echo ""
    echo "🔍 Showing compilation errors:"
    cargo check --lib 2>&1 | tail -20
fi
