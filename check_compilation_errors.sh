#!/bin/bash

echo "=== Checking Current Compilation Status ==="
echo "Date: $(date)"
echo

cd /Users/mariano/Desktop/Code/nano-messenger

echo "Running cargo check..."
cargo check 2>&1 | head -50

echo
echo "=== Running cargo check --lib ==="
cargo check --lib 2>&1 | head -20

echo
echo "=== Running cargo check --bin client ==="
cargo check --bin client 2>&1 | head -20

echo
echo "=== Checking for specific error patterns ==="
echo "Searching for 'Mac' related errors:"
cargo check 2>&1 | grep -i "mac" || echo "No Mac-related errors found"

echo
echo "Searching for missing import errors:"
cargo check 2>&1 | grep -i "not found in scope\|unresolved import\|missing" || echo "No obvious import errors found"
