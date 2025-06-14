#!/bin/bash

echo "🔧 HOLISTIC DELIMITER FIX FOR RUST PROJECT"
echo "=========================================="
echo ""

cd /Users/mariano/Desktop/Code/nano-messenger

echo "🔍 Checking for compilation after health_monitoring.rs fix..."

# Test if the specific health_monitoring.rs fix resolved the issue
echo "Testing library compilation..."
if cargo check --lib --quiet 2>delimiter_fix_test.log; then
    echo "✅ SUCCESS! Primary delimiter issue fixed!"
    echo ""
    echo "📋 Summary of Applied Fixes:"
    echo "- Fixed missing closing brace for impl HealthMonitor block in health_monitoring.rs"
    echo "- Removed extra closing brace at end of health_monitoring.rs"
    echo ""
    
    # Additional validation - check for any remaining delimiter issues in all Rust files
    echo "🧹 Performing comprehensive validation of all Rust files..."
    
    RUST_FILES=$(find src -name "*.rs" -type f)
    DELIMITER_ISSUES=0
    
    echo "Checking delimiter balance in all Rust files..."
    for file in $RUST_FILES; do
        # Count opening and closing braces
        OPEN_BRACES=$(grep -o '{' "$file" | wc -l)
        CLOSE_BRACES=$(grep -o '}' "$file" | wc -l)
        
        if [ "$OPEN_BRACES" -ne "$CLOSE_BRACES" ]; then
            echo "⚠️  POTENTIAL ISSUE: $file has $OPEN_BRACES opening braces and $CLOSE_BRACES closing braces"
            DELIMITER_ISSUES=$((DELIMITER_ISSUES + 1))
        fi
    done
    
    if [ "$DELIMITER_ISSUES" -eq 0 ]; then
        echo "✅ No brace imbalance detected in any Rust files"
    else
        echo "⚠️  Found $DELIMITER_ISSUES files with potential brace imbalances"
    fi
    
    echo ""
    echo "🏗️  Running full project compilation test..."
    if cargo build --quiet 2>full_build_test.log; then
        echo "✅ EXCELLENT! Full project builds successfully!"
        echo ""
        echo "🚀 Project Status: READY FOR DEVELOPMENT"
        echo ""
        echo "Available commands:"
        echo "  cargo build --release    # Build optimized version"
        echo "  cargo test               # Run all tests"
        echo "  cargo run --bin nano-client        # Run client"
        echo "  cargo run --bin nano-relay         # Run relay server"
        echo "  cargo run --bin config-validator   # Validate config"
        echo ""
        echo "All delimiter issues have been resolved holistically!"
        
    else
        echo "❌ Full build still has issues:"
        cat full_build_test.log | head -20
        echo ""
        echo "The delimiter fix was successful, but other compilation issues remain."
    fi
    
else
    echo "❌ Compilation still has delimiter errors:"
    cat delimiter_fix_test.log
    echo ""
    echo "🔧 Attempting additional fixes..."
    
    # Look for common delimiter patterns that might need fixing
    echo "Searching for other potential delimiter issues..."
    
    # Find files with }} patterns (double closing braces)
    echo "Checking for double closing braces..."
    grep -rn "}}" src/ && echo "Found potential double closing brace issues"
    
    # Find files with unmatched braces using a simple pattern
    echo "Scanning all Rust files for obvious syntax issues..."
    
    find src -name "*.rs" -exec sh -c '
        for file do
            # Check for obviously malformed patterns
            if grep -q "impl.*{.*impl" "$file"; then
                echo "Potential nested impl issue in: $file"
            fi
            if grep -q "}.*}.*}$" "$file"; then
                echo "Potential triple closing brace in: $file"
            fi
        done
    ' sh {} +
fi

echo ""
echo "🏁 Holistic delimiter fix analysis complete!"
echo ""
echo "Files processed: $(find src -name "*.rs" | wc -l) Rust files"
echo "Focus was on: src/production/health_monitoring.rs (primary fix applied)"
echo ""
echo "If issues persist, please check the error log above for specific guidance."
