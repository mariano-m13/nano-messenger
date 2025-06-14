#!/bin/bash

# Final Error Detective Script
# Find and fix the last remaining compilation error

cd /Users/mariano/Desktop/Code/nano-messenger

echo "🔍 FINAL ERROR DETECTIVE"
echo "========================"
echo "Investigating the last remaining compilation error..."
echo ""

echo "Getting detailed error information..."
echo "------------------------------------"

# Get the specific error details
echo "Full error details:"
cargo check --lib 2>&1 | grep -A 5 -B 5 "error\["

echo ""
echo "Looking for specific error patterns..."
echo "-------------------------------------"

# Check for common error patterns
echo "Checking for missing imports..."
cargo check --lib 2>&1 | grep -E "(cannot find|unresolved import)" | head -5

echo ""
echo "Checking for type mismatches..."
cargo check --lib 2>&1 | grep -E "(expected|found)" | head -5

echo ""
echo "Checking for trait issues..."
cargo check --lib 2>&1 | grep -E "(trait|implementation)" | head -5

echo ""
echo "Getting the most critical error..."
echo "---------------------------------"
cargo check --lib 2>&1 | grep -A 10 "error\[" | head -15
