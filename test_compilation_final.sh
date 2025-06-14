#!/bin/bash

echo "🔧 Testing compilation without EXIF dependency..."
echo ""

echo "📋 Step 1: Testing minimal compilation (no default features)..."
if cargo check --no-default-features; then
    echo "✅ Minimal compilation successful!"
else
    echo "❌ Minimal compilation failed!"
    exit 1
fi

echo ""
echo "📋 Step 2: Testing with default features..."
if cargo check; then
    echo "✅ Default features compilation successful!"
else
    echo "❌ Default features compilation failed!"
    exit 1
fi

echo ""
echo "📋 Step 3: Testing image processing feature specifically..."
if cargo check --features="image-processing"; then
    echo "✅ Image processing feature compilation successful!"
else
    echo "❌ Image processing feature compilation failed!"
    exit 1
fi

echo ""
echo "🎉 All compilations successful!"
echo ""
echo "Note: EXIF functionality is temporarily disabled."
echo "To re-enable EXIF support later:"
echo "1. Add 'kamadak-exif = \"0.5\"' to Cargo.toml"
echo "2. Uncomment the EXIF code in images.rs"
echo "3. Test compilation again"
