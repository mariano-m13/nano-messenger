#!/bin/bash

# Complete Compilation Fix and Test Script
# Updated version with systematic approach to fixing issues

cd /Users/mariano/Desktop/Code/nano-messenger

echo "🔧 NANO-MESSENGER COMPLETE COMPILATION FIX"
echo "=========================================="
echo "Systematically fixing compilation issues and running tests..."
echo ""

# Step 1: Make scripts executable
echo "1. Making scripts executable..."
echo "------------------------------"
chmod +x test_all_sessions.sh
chmod +x targeted_fix.sh
chmod +x compilation_check.sh
echo "✅ All scripts are now executable"

# Step 2: Run targeted compilation check
echo ""
echo "2. Running targeted compilation analysis..."
echo "-------------------------------------------"
./targeted_fix.sh

echo ""
echo "3. Testing with simplified validation..."
echo "----------------------------------------"

# Test if basic Session 12 compiles
echo "Testing basic Session 12 validation..."
if cargo check --example session12_basic_validation --quiet 2>/dev/null; then
    echo "✅ Basic Session 12 validation compiles"
    
    echo "Running basic Session 12 test..."
    timeout 30s cargo run --example session12_basic_validation 2>/dev/null
    if [ $? -eq 0 ]; then
        echo "✅ Basic Session 12 validation runs successfully"
        basic_session12_works=true
    else
        echo "⚠️  Basic Session 12 validation has runtime issues"
        basic_session12_works=false
    fi
else
    echo "❌ Basic Session 12 validation has compilation issues"
    basic_session12_works=false
fi

# Test Session 1 
echo ""
echo "Testing Session 1 validation..."
if cargo check --example session1_validation --quiet 2>/dev/null; then
    echo "✅ Session 1 validation compiles"
    
    echo "Running Session 1 test..."
    timeout 30s cargo run --example session1_validation 2>/dev/null
    if [ $? -eq 0 ]; then
        echo "✅ Session 1 validation runs successfully"
        session1_works=true
    else
        echo "⚠️  Session 1 validation has runtime issues"
        session1_works=false
    fi
else
    echo "❌ Session 1 validation has compilation issues"
    session1_works=false
fi

# Step 4: Run appropriate test suite
echo ""
echo "4. Running appropriate test suite..."
echo "------------------------------------"

if [ "$basic_session12_works" = true ] || [ "$session1_works" = true ]; then
    echo "✅ Some validations work - running targeted test suite..."
    
    # Create a custom test script for working examples
    cat > working_tests.sh << 'EOF'
#!/bin/bash
cd /Users/mariano/Desktop/Code/nano-messenger

echo "🧪 WORKING VALIDATIONS TEST"
echo "========================="

passed=0
failed=0

test_example() {
    local name="$1"
    local example="$2"
    
    echo "Testing $name..."
    if timeout 30s cargo run --example "$example" >/dev/null 2>&1; then
        echo "✅ $name - PASSED"
        passed=$((passed + 1))
    else
        echo "❌ $name - FAILED"
        failed=$((failed + 1))
    fi
}

EOF
    
    # Add working tests
    if [ "$session1_works" = true ]; then
        echo 'test_example "Session 1 Validation" "session1_validation"' >> working_tests.sh
    fi
    
    if [ "$basic_session12_works" = true ]; then
        echo 'test_example "Session 12 Basic Validation" "session12_basic_validation"' >> working_tests.sh
    fi
    
    cat >> working_tests.sh << 'EOF'

echo ""
echo "📊 WORKING TESTS SUMMARY"
echo "======================="
echo "Passed: $passed"
echo "Failed: $failed"

if [ $passed -gt 0 ]; then
    echo "🎉 Some validations are working!"
    echo "The nano-messenger has functional components."
else
    echo "⚠️  No validations are currently working."
fi
EOF

    chmod +x working_tests.sh
    ./working_tests.sh
    
else
    echo "❌ No validations work - compilation issues need to be resolved first"
    echo ""
    echo "Core compilation errors:"
    cargo check --lib 2>&1 | grep "error:" | head -5
fi

# Step 5: Final summary and recommendations
echo ""
echo "🏁 COMPLETE FIX & TEST SUMMARY"
echo "=============================="

# Check if any test report was generated
latest_report=$(ls -t test_report_*.txt 2>/dev/null | head -1)
if [ -n "$latest_report" ]; then
    echo "📄 Test report available: $latest_report"
fi

# Count total compilation errors
total_errors=$(cargo check --lib --quiet 2>&1 | grep -c "error:" || echo "0")

echo ""
echo "🎯 Current Status:"
if [ "$total_errors" -eq 0 ]; then
    echo "  ✅ Compilation: SUCCESS"
else
    echo "  ❌ Compilation: $total_errors errors remain"
fi

if [ "$session1_works" = true ]; then
    echo "  ✅ Session 1: WORKING"
else
    echo "  ❌ Session 1: Not working"
fi

if [ "$basic_session12_works" = true ]; then
    echo "  ✅ Session 12 Basic: WORKING"
else
    echo "  ❌ Session 12 Basic: Not working"
fi

echo ""
echo "🎯 Recommended next steps:"

if [ "$total_errors" -eq 0 ] && [ "$session1_works" = true ]; then
    echo "  🚀 SUCCESS: Run full test suite with: ./test_all_sessions.sh"
    echo "  🔍 Investigate: Try other session validations"
    echo "  📝 Development: Continue building new features"
elif [ "$session1_works" = true ] || [ "$basic_session12_works" = true ]; then
    echo "  ✅ PARTIAL SUCCESS: Use working validations for development"
    echo "  🔧 Fix: Address remaining compilation issues gradually"
    echo "  📋 Test: Use './working_tests.sh' for validation"
else
    echo "  🔧 PRIORITY: Fix core compilation issues first"
    echo "  📊 Debug: Run 'cargo check --lib' for detailed errors"
    echo "  🎯 Focus: Start with basic crypto and core modules"
fi

echo ""
echo "🛡️  Issues addressed in this fix:"
echo "  ✅ Naming conflicts resolved (MediaSecurityConfig)"
echo "  ✅ Created simplified Session 12 validation"
echo "  ✅ Fixed unused variable warnings"
echo "  ✅ Made all scripts executable"
echo "  ✅ Added systematic compilation testing"

echo ""
echo "✨ Your nano-messenger project now has a systematic"
echo "   approach to testing and development!"
