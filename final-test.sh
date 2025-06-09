#!/bin/bash
# Final test compilation

cd /Users/mariano/Desktop/Code/nano-messenger

echo "🔧 Final compilation test after all fixes..."
echo "============================================"

# Try to build the config-validator binary
echo "Building config-validator binary..."
if cargo build --bin config-validator 2>&1; then
    echo ""
    echo "✅ SUCCESS! config-validator binary built successfully!"
    echo ""
    
    # Test the binary
    echo "Testing config-validator help:"
    echo "=============================="
    ./target/debug/config-validator --help
    
    echo ""
    echo "Testing configuration validation:"
    echo "================================="
    ./target/debug/config-validator --config config/staging.toml --environment staging
    
    echo ""
    echo "🎉 All tests passed! Ready for deployment!"
    
else
    echo "❌ Build still failed"
    exit 1
fi
