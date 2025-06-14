#!/bin/bash
cd /Users/mariano/Desktop/Code/nano-messenger
echo "Capturing current compilation errors..."
echo "======================================="

echo "Running cargo check..."
cargo check 2>&1 | tee current_errors.log

echo ""
echo "Current error count summary:"
cargo check 2>&1 | grep -c "error:" || echo "0"
echo "Current warning count summary:" 
cargo check 2>&1 | grep -c "warning:" || echo "0"
