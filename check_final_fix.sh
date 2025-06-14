#!/bin/bash

cd /Users/mariano/Desktop/Code/nano-messenger

echo "🔍 Checking the lifetime fix in transfer.rs..."
echo "=========================================="
echo ""

# Check if the Arc<Semaphore> fix is applied
echo "✓ Checking transfer.rs modifications..."
grep -n "upload_semaphore: Arc<Semaphore>" src/media/chunking/transfer.rs || echo "❌ Arc<Semaphore> not found"
echo ""

echo "🏗️  Running cargo check..."
cargo check --lib 2>&1 | tail -20

if [ ${PIPESTATUS[0]} -eq 0 ]; then
    echo ""
    echo "✅ Library compilation successful!"
    echo ""
    echo "🧪 Checking test compilation..."
    cargo check --tests 2>&1 | tail -10
    
    if [ ${PIPESTATUS[0]} -eq 0 ]; then
        echo ""
        echo "🎉 All compilation issues resolved!"
    fi
else
    echo ""
    echo "❌ Still has compilation errors"
fi