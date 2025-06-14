#!/bin/bash

echo "=== Testing Compilation Status After Fixes ==="
echo "Date: $(date)"
echo

cd /Users/mariano/Desktop/Code/nano-messenger

echo "Running cargo check --lib..."
echo "========================="

# Check library compilation
cargo check --lib 2>&1 | head -40

echo ""
echo "=== Checking specific modules ==="

echo "Checking media/security/encryption.rs..."
cargo check --lib --message-format short 2>&1 | grep "encryption.rs" | head -5

echo ""
echo "Checking media/compliance/hipaa.rs..."
cargo check --lib --message-format short 2>&1 | grep "hipaa.rs" | head -5

echo ""
echo "Checking media/compliance/gdpr.rs..."
cargo check --lib --message-format short 2>&1 | grep "gdpr.rs" | head -5

echo ""
echo "Checking media/compliance/auditing.rs..."
cargo check --lib --message-format short 2>&1 | grep "auditing.rs" | head -5

echo ""
echo "=== Summary ==="
echo "Checking if compilation succeeds..."
if cargo check --lib --quiet 2>/dev/null; then
    echo "✅ SUCCESS: Library compilation successful!"
else
    echo "❌ FAILED: Still has compilation errors"
    echo "Remaining error count:"
    cargo check --lib 2>&1 | grep "error\[" | wc -l
fi

echo ""
echo "=== Mac Import Status ==="
echo "The Mac trait import in encryption.rs:"
grep -n "use hmac::{Hmac, Mac}" src/media/security/encryption.rs || echo "Import not found"

echo ""
echo "Verification complete!"
