#!/bin/bash

cd /Users/mariano/Desktop/Code/nano-messenger

# Run the detailed error analysis
./detailed_error_analysis.sh

# Also check if there's already a full_compile_output.log
if [ -f "full_compile_output.log" ]; then
    echo -e "\n\n=== SHOWING FIRST E0753 ERRORS FROM LOG ==="
    grep -A 5 "error\[E0753\]" full_compile_output.log | head -30
fi
