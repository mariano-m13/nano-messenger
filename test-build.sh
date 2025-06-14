#!/bin/bash
# Test script to verify the config-validator binary builds successfully

cd /Users/mariano/Desktop/Code/nano-messenger

echo "🔧 Testing config-validator binary build..."
echo "============================================"

# Try to build just the config-validator binary
echo "Building config-validator binary..."
cargo build --bin config-validator 2>&1

if [ $? -eq 0 ]; then
    echo "✅ config-validator binary built successfully!"
    
    # Test the binary with help
    echo ""
    echo "Testing config-validator --help:"
    echo "================================"
    ./target/debug/config-validator --help
    
    # Test config validation with our staging config
    echo ""
    echo "Testing configuration validation:"
    echo "================================"
    ./target/debug/config-validator --config config/staging.toml --environment staging
    
else
    echo "❌ Failed to build config-validator binary"
    exit 1
fi
