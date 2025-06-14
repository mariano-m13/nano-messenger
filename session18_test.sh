#!/bin/bash

# Session 18: Feature Gating & Configuration Test
# Focus on resolving cfg feature warnings and missing features

echo "🔧 Session 18: Testing Feature Gating & Configuration fixes"
echo "================================================================="

# Set up environment
export RUST_BACKTRACE=1

# Clean previous builds
echo "🧹 Cleaning previous builds..."
cargo clean > /dev/null 2>&1

# Test basic features
echo "📦 Testing basic feature compilation..."
if cargo check --lib 2> session18_basic_errors.log; then
    echo "✅ Basic compilation successful!"
else
    echo "❌ Basic compilation failed"
    echo "🔍 Feature-related errors:"
    grep -n -E "feature|cfg\(" session18_basic_errors.log | head -10
fi

# Test individual features
echo ""
echo "🎯 Testing individual features..."

features=("image-processing" "video-processing" "session11-basic" "s3-storage" "compliance-basic")

for feature in "${features[@]}"; do
    echo "  Testing feature: $feature"
    if cargo check --features="$feature" --lib 2> "session18_${feature}_errors.log"; then
        echo "    ✅ Feature '$feature' compiles successfully"
    else
        echo "    ❌ Feature '$feature' compilation failed"
        echo "    🔍 Issues:"
        grep -n -E "error\[E[0-9]+\]" "session18_${feature}_errors.log" | head -3 | sed 's/^/      /'
    fi
done

# Test all features together
echo ""
echo "🌟 Testing all features together..."
if cargo check --all-features 2> session18_all_features_errors.log; then
    echo "✅ All features compilation successful!"
else
    echo "❌ All features compilation failed"
    
    echo ""
    echo "🔍 Feature configuration errors:"
    grep -n -E "feature.*not.*found|cfg\(feature.*\)|unresolved import" session18_all_features_errors.log | head -10
    
    echo ""
    echo "🔍 Missing dependency errors:"
    grep -n -E "could not find|crate.*not found" session18_all_features_errors.log | head -5
fi

# Test media processing specifically
echo ""
echo "📷 Testing media processing features..."
if cargo check --features="media-full" --lib 2> session18_media_errors.log; then
    echo "✅ Media features compilation successful!"
else
    echo "❌ Media features compilation failed"
    echo "🔍 Media processing errors:"
    grep -n -A 2 "media/processing" session18_media_errors.log | head -10
fi

# Test compliance features
echo ""
echo "📋 Testing compliance features..."
if cargo check --features="compliance-full" --lib 2> session18_compliance_errors.log; then
    echo "✅ Compliance features compilation successful!"
else
    echo "❌ Compliance features compilation failed"
    echo "🔍 Compliance errors:"
    grep -n -A 2 "compliance" session18_compliance_errors.log | head -10
fi

# Check for cfg warnings
echo ""
echo "⚠️  Checking for cfg warnings..."
cargo check --lib 2>&1 | grep -E "warning.*cfg|warning.*feature" > session18_cfg_warnings.log
if [ -s session18_cfg_warnings.log ]; then
    echo "Found cfg warnings:"
    cat session18_cfg_warnings.log | head -15
else
    echo "✅ No cfg warnings found!"
fi

# Summary
echo ""
echo "📊 Session 18 Summary:"
echo "======================"

if [ -f session18_all_features_errors.log ]; then
    TOTAL_ERRORS=$(grep -c "error\[E" session18_all_features_errors.log)
    FEATURE_ERRORS=$(grep -c "feature.*not.*found\|cfg.*feature" session18_all_features_errors.log)
    
    echo "Total compilation errors: $TOTAL_ERRORS"
    echo "Feature-related errors: $FEATURE_ERRORS"
    
    if [ "$FEATURE_ERRORS" -eq 0 ]; then
        echo "🎉 Session 18 target (feature warnings) successfully resolved!"
    else
        echo "⚠️  Some feature errors remain"
    fi
else
    echo "🎉 No compilation errors detected!"
fi

# Check warning count
if [ -f session18_cfg_warnings.log ]; then
    WARNING_COUNT=$(wc -l < session18_cfg_warnings.log)
    echo "Cfg/feature warnings: $WARNING_COUNT"
    
    if [ "$WARNING_COUNT" -eq 0 ]; then
        echo "🎉 Feature configuration warnings resolved!"
    fi
fi

echo ""
echo "📄 Detailed error logs saved to:"
echo "  - session18_*_errors.log files"
echo "  - session18_cfg_warnings.log"

echo ""
echo "🎯 Next: Session 19 (Code Hygiene & Optimization) - Target: ~60 warnings resolved"
