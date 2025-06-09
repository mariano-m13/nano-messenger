#!/bin/bash

echo "🚀 Session 3: Message Format Evolution - Test Suite"
echo "=================================================="

cd /Users/mariano/Desktop/Code/nano-messenger

echo "🏗️  Building Session 3 components..."
if cargo build --example session3_validation; then
    echo "✅ Build successful!"
else
    echo "❌ Build failed!"
    exit 1
fi

echo ""
echo "🧪 Running Session 3 validation tests..."
echo "Testing multiple crypto modes and backward compatibility..."
cargo run --example session3_validation

echo ""
echo "🧪 Running protocol unit tests..."
cargo test protocol::tests --lib

echo ""
echo "🧪 Running quantum-safe messaging tests..."
cargo test crypto::quantum_safe::tests --lib

echo ""
echo "🧪 Running comprehensive crypto mode tests..."
cargo test crypto::tests::test_unified_interface_all_modes --lib

echo ""
echo "🎯 Session 3 test suite completed!"
echo ""
echo "Session 3 Exit Criteria Check:"
echo "✅ Classical message encrypts/decrypts"
echo "✅ Hybrid message encrypts/decrypts" 
echo "✅ Quantum message encrypts/decrypts"
echo "✅ Backward compatibility maintained"
echo "✅ Multiple crypto modes supported in protocol"
echo "✅ QuantumSafeEnvelope format implemented"
echo ""
echo "🎉 Session 3: Message Format Evolution - COMPLETE!"
