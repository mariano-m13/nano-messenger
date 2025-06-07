#!/bin/bash

echo "🧹 Cleaning the project..."
cargo clean

echo "📦 Updating dependencies..."
cargo update

echo "🔨 Building the project..."
cargo build

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "🧪 Running tests..."
    cargo test
else
    echo "❌ Build failed. Check the output above for errors."
fi
