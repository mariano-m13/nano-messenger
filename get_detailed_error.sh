#!/bin/bash

echo "🔍 Getting detailed compilation error..."
echo ""

echo "📋 Running detailed compilation check..."
cargo check --features="image-processing" 2>&1
