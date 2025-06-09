#!/bin/bash
# Quick compilation test script

cd /Users/mariano/Desktop/Code/nano-messenger

echo "🔧 Testing compilation after fixes..."
echo "===================================="

# Try to build the config-validator binary specifically
echo "Building config-validator binary..."
if cargo build --bin config-validator 2>&1; then
    echo "✅ config-validator binary built successfully!"
    echo ""
    
    # Test if we can run the config validator
    echo "Testing config-validator help:"
    echo "=============================="
    ./target/debug/config-validator --help
    
    echo ""
    echo "Testing staging configuration validation:"
    echo "========================================"
    ./target/debug/config-validator --config config/staging.toml --environment staging
    
else
    echo "❌ Build failed"
    exit 1
fi
