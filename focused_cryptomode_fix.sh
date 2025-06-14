#!/bin/bash

# 🎯 FOCUSED CRYPTOMODE FIX
# Fixes ONLY the CryptoMode pattern matching issues

set -euo pipefail

PROJECT_ROOT="/Users/mariano/Desktop/Code/nano-messenger"

echo "🎯 FOCUSED CRYPTOMODE PATTERN FIX"
echo "=================================="

cd "$PROJECT_ROOT" || exit 1

# Fix syntax corruption first
echo "🔧 Fixing __&self syntax corruption..."
sed -i '' 's/__&self/\&self/g' src/crypto/optimizations.rs

# Fix each CryptoMode pattern match specifically
echo "🔧 Fixing CryptoMode patterns..."

# crypto/optimizations.rs line 151
sed -i '' 's/CryptoMode::Quantum => UnifiedKeyPair::PostQuantum(PostQuantumUserKeyPair::generate()),/CryptoMode::Quantum | CryptoMode::QuantumSafe => UnifiedKeyPair::PostQuantum(PostQuantumUserKeyPair::generate()),/' src/crypto/optimizations.rs

# crypto/benchmarks.rs - multiple patterns
sed -i '' 's/CryptoMode::Quantum => {/CryptoMode::Quantum | CryptoMode::QuantumSafe => {/g' src/crypto/benchmarks.rs
sed -i '' 's/CryptoMode::Quantum => 1\.4,/CryptoMode::Quantum | CryptoMode::QuantumSafe => 1.4,/g' src/crypto/benchmarks.rs  
sed -i '' 's/CryptoMode::Quantum => (1200, 600, 2000),/CryptoMode::Quantum | CryptoMode::QuantumSafe => (1200, 600, 2000),/g' src/crypto/benchmarks.rs
sed -i '' 's/CryptoMode::Quantum => { let _ = PostQuantumUserKeyPair::generate(); },/CryptoMode::Quantum | CryptoMode::QuantumSafe => { let _ = PostQuantumUserKeyPair::generate(); },/g' src/crypto/benchmarks.rs

# Fix user_id variable issues
sed -i '' 's/_user_id: &UserId,/user_id: \&UserId,/g' src/media/security/scanning.rs

echo "✅ Fixed all CryptoMode patterns"

# Test quickly
echo "🧪 Quick test..."
if cargo check --lib >/dev/null 2>&1; then
    echo "✅ SUCCESS! Compilation works!"
else
    echo "❌ Still have errors - run the complete fix"
fi
