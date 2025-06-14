#!/bin/bash

echo "🧪 TESTING AFTER DELIMITER FIX"
echo "============================="
echo ""

cd /Users/mariano/Desktop/Code/nano-messenger

echo "Testing compilation..."
if cargo check --lib 2>&1 | tee delimiter_test.log | grep -q "error"; then
    ERROR_COUNT=$(grep -c "error" delimiter_test.log || echo "0")
    echo "❌ Still have $ERROR_COUNT errors"
    
    # Show errors
    echo -e "\nErrors found:"
    grep -A2 "error" delimiter_test.log | head -20
else
    echo "✅ SUCCESS! Compilation successful!"
    echo ""
    echo "The unclosed delimiter error has been fixed!"
    echo ""
    echo "You can now run:"
    echo "  cargo build --release"
    echo "  cargo test"
    echo "  cargo run --bin nano-client"
fi

echo -e "\n🏁 Test complete!"
