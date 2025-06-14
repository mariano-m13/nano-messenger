#!/bin/bash

echo "🔧 Testing kamadak-exif dependency resolution..."
echo ""

# First test if the dependency can be resolved
echo "📋 Checking if kamadak-exif 0.3 can be resolved..."
if cargo check --no-default-features 2>&1 | grep -q "kamadak-exif"; then
    echo "❌ kamadak-exif dependency issue found"
    echo ""
    echo "🔍 Let's try different versions..."
    
    # Try version 0.2
    echo "Trying version 0.2..."
    sed -i '' 's/kamadak-exif = "0.3"/kamadak-exif = "0.2"/' Cargo.toml
    if cargo check --no-default-features --quiet; then
        echo "✅ Version 0.2 works!"
        exit 0
    fi
    
    # Try version 0.1
    echo "Trying version 0.1..."
    sed -i '' 's/kamadak-exif = "0.2"/kamadak-exif = "0.1"/' Cargo.toml
    if cargo check --no-default-features --quiet; then
        echo "✅ Version 0.1 works!"
        exit 0
    fi
    
    # Try just "0.5" without specifying exact version 
    echo "Trying latest compatible version..."
    sed -i '' 's/kamadak-exif = "0.1"/kamadak-exif = "0.5"/' Cargo.toml
    if cargo check --no-default-features --quiet; then
        echo "✅ Latest version works!"
        exit 0
    fi
    
    echo "❌ No working version found. Trying to make EXIF optional..."
    
else
    echo "✅ kamadak-exif 0.3 works!"
fi
