#!/bin/bash

echo "🔧 Applying final compilation fixes..."

# Fix the Ed25519PrivateKey deserialization issue in crypto.rs line 113
sed -i.bak 's/verifying_key = Some(Ed25519PublicKey::from_bytes(&key_bytes));/verifying_key = Some(Ed25519PublicKey::from_bytes(\&key_bytes).map_err(de::Error::custom)?);/' src/crypto.rs

# Build and check if there are still issues
echo "🔨 Building after fixes..."
cargo build 2>&1

echo ""
echo "✅ If the build succeeds, all issues are resolved!"
echo "⚠️  If there are still issues, they will be shown above."
