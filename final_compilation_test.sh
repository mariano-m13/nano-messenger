#!/bin/bash
cd /Users/mariano/Desktop/Code/nano-messenger

echo "Testing compilation after fixes..."
echo "=================================="

echo "Running cargo check..."
if cargo check --lib --quiet 2>&1 | grep -q "error"; then
    echo "❌ Still have compilation errors:"
    cargo check --lib 2>&1 | head -30
else
    echo "✅ Library compiles successfully!"
    echo ""
    echo "Checking for warnings:"
    cargo check --lib 2>&1 | grep "warning" | head -10
fi

echo ""
echo "Summary of fixes applied:"
echo "- Fixed MediaType::Unknown pattern match"
echo "- Added MobileQualityLevel struct"
echo "- Fixed FileReference.file_size access issue"
echo "- Added Default implementation for EncryptionMetadata"
echo "- Fixed StorageLocation constructor call"
echo "- Fixed ChaCha20Poly1305 KeyInit trait import"
echo "- Fixed file ID generation in chunked transfer"
echo "- Cleaned up unused imports"
