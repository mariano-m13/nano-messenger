#!/bin/bash

echo "🔍 Checking all Rust files for unclosed delimiters..."
echo "===================================================="

cd /Users/mariano/Desktop/Code/nano-messenger

# Find all .rs files and check brace balance
find src -name "*.rs" -type f | while read -r file; do
    # Count opening and closing braces
    open_count=$(grep -o '{' "$file" 2>/dev/null | wc -l | tr -d ' ')
    close_count=$(grep -o '}' "$file" 2>/dev/null | wc -l | tr -d ' ')
    
    if [ "$open_count" -ne "$close_count" ]; then
        diff=$((open_count - close_count))
        echo ""
        echo "❌ $file"
        echo "   Missing $diff closing brace(s)"
        echo "   Open: $open_count, Close: $close_count"
        
        # Show last 5 lines of the file
        echo "   Last 5 lines:"
        tail -5 "$file" | nl -v $(($(wc -l < "$file") - 4))
    fi
done

echo ""
echo "✅ Check complete!"
