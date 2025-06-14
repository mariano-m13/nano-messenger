#!/bin/bash

echo "🔧 FIXING MISSING CLOSING BRACE"
echo "=============================="
echo ""

cd /Users/mariano/Desktop/Code/nano-messenger

# Check the last few lines of the file
echo "Last 10 lines of src/production/mod.rs:"
tail -10 src/production/mod.rs

# Count braces
echo -e "\n📊 Brace count:"
OPEN=$(grep -c '{' src/production/mod.rs)
CLOSE=$(grep -c '}' src/production/mod.rs)
echo "Opening braces: $OPEN"
echo "Closing braces: $CLOSE"
echo "Missing: $((OPEN - CLOSE))"

# If missing a closing brace, add it
if [ $OPEN -gt $CLOSE ]; then
    echo -e "\n✅ Adding missing closing brace..."
    echo "}" >> src/production/mod.rs
    echo "Done!"
else
    echo -e "\n✅ Braces are already balanced"
fi

# Test compilation
echo -e "\n🧪 Testing compilation..."
if cargo check --lib 2>&1 | grep -q "error"; then
    echo "❌ Still has errors"
    cargo check --lib 2>&1 | grep -A2 "error" | head -10
else
    echo "✅ SUCCESS! Compilation successful!"
fi

echo -e "\n🏁 Fix complete!"
