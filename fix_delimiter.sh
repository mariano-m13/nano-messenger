#!/bin/bash

echo "🔧 FIXING UNCLOSED DELIMITER ERROR"
echo "================================="
echo ""

cd /Users/mariano/Desktop/Code/nano-messenger

# Check the current state of the file
echo "Checking src/production/mod.rs for unclosed delimiters..."

# Count opening and closing braces
OPEN_BRACES=$(grep -c '{' src/production/mod.rs)
CLOSE_BRACES=$(grep -c '}' src/production/mod.rs)

echo "Opening braces: $OPEN_BRACES"
echo "Closing braces: $CLOSE_BRACES"
echo "Difference: $((OPEN_BRACES - CLOSE_BRACES))"

# The file is missing a closing brace at the end
if [ $OPEN_BRACES -gt $CLOSE_BRACES ]; then
    echo -e "\n❌ File is missing $((OPEN_BRACES - CLOSE_BRACES)) closing brace(s)"
    echo "Adding missing closing brace at the end of the file..."
    
    # Add a closing brace at the end
    echo "}" >> src/production/mod.rs
    
    echo "✅ Added missing closing brace"
else
    echo "✅ Braces are balanced"
fi

# Test compilation
echo -e "\n🧪 Testing compilation..."
if cargo check --lib 2>&1 | tee delimiter_fix.log | grep -q "error"; then
    ERROR_COUNT=$(grep -c "error" delimiter_fix.log || echo "0")
    echo "❌ Still have $ERROR_COUNT errors"
    
    # Show any remaining errors
    grep -A2 "error" delimiter_fix.log | head -10
else
    echo "✅ SUCCESS! Compilation successful!"
fi

echo -e "\n🏁 Delimiter fix complete!"
