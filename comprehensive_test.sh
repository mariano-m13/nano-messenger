#!/bin/bash

echo "🔧 Fixing nano-messenger compilation issues..."
echo

# Clean previous build artifacts completely
echo "🧹 Cleaning previous build artifacts..."
cargo clean
rm -f Cargo.lock

echo
echo "📦 Updating dependencies with new versions..."
cargo update

echo
echo "🔍 Checking code compilation..."
cargo check 2>&1 | tee build-check.log

echo
echo "🏗️ Attempting full build..."
cargo build 2>&1 | tee build-full.log

echo
echo "📋 Build Summary:"
if cargo check --quiet 2>/dev/null; then
    echo "✅ Code check passed!"
    if cargo build --quiet 2>/dev/null; then
        echo "✅ Full build successful!"
        echo "🎉 nano-messenger is ready to use!"
    else
        echo "⚠️  Code check passed but full build failed"
        echo "📄 Check build-full.log for details"
    fi
else
    echo "❌ Code check failed"
    echo "📄 Check build-check.log for details"
fi

echo
echo "🔍 Testing individual binaries..."
echo "Testing nano-client..."
if cargo build --bin nano-client --quiet 2>/dev/null; then
    echo "✅ nano-client built successfully"
else
    echo "❌ nano-client build failed"
fi

echo "Testing nano-relay..."
if cargo build --bin nano-relay --quiet 2>/dev/null; then
    echo "✅ nano-relay built successfully"
else
    echo "❌ nano-relay build failed"
fi

echo
echo "🏁 Build test complete!"
