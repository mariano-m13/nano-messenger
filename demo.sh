#!/bin/bash

# Demo script for nano-messenger
# This demonstrates the basic workflow of the messaging system

echo "🚀 Nano Messenger Demo"
echo "====================="

# Build the project
echo "Building project..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed"
    exit 1
fi

echo "✅ Build successful!"
echo ""

# Start relay in background
echo "🔄 Starting relay server..."
./target/release/nano-relay --port 7733 &
RELAY_PID=$!

# Wait for relay to start
sleep 2

echo "✅ Relay server started (PID: $RELAY_PID)"
echo ""

# Setup Alice
echo "👩 Setting up Alice..."
export ALICE_DIR="/tmp/nano-alice"
rm -rf "$ALICE_DIR"
mkdir -p "$ALICE_DIR"

./target/release/nano-client --config-dir "$ALICE_DIR" init
./target/release/nano-client --config-dir "$ALICE_DIR" claim-username alice2024

echo ""

# Setup Bob  
echo "👨 Setting up Bob..."
export BOB_DIR="/tmp/nano-bob"
rm -rf "$BOB_DIR"
mkdir -p "$BOB_DIR"

./target/release/nano-client --config-dir "$BOB_DIR" init
./target/release/nano-client --config-dir "$BOB_DIR" claim-username bob2024

echo ""

# Bob sends first message to Alice
echo "📨 Bob sends first message to Alice..."
./target/release/nano-client --config-dir "$BOB_DIR" send alice2024 "Hey Alice! This is Bob from the demo."

echo ""

# Alice checks for messages
echo "📬 Alice checks for messages..."
./target/release/nano-client --config-dir "$ALICE_DIR" receive

echo ""

# Show Alice's contacts
echo "👥 Alice's contacts:"
./target/release/nano-client --config-dir "$ALICE_DIR" contacts list

echo ""

# Cleanup
echo "🧹 Cleaning up..."
kill $RELAY_PID 2>/dev/null
rm -rf "$ALICE_DIR" "$BOB_DIR"

echo "✅ Demo complete!"
echo ""
echo "To run manually:"
echo "1. cargo build --release"
echo "2. ./target/release/nano-relay"
echo "3. ./target/release/nano-client init"
echo "4. ./target/release/nano-client claim-username <username>"
echo "5. ./target/release/nano-client send <recipient> <message>"
echo "6. ./target/release/nano-client receive"
