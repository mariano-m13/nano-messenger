# Quick Start Guide

This guide will walk you through setting up and using nano-messenger for the first time.

## Prerequisites

- Rust (install from https://rustup.rs/)
- A terminal/command line

## 1. Build the Project

```bash
cd nano-messenger
cargo build --release
```

This creates two binaries:
- `./target/release/nano-relay` - The relay server
- `./target/release/nano-client` - The CLI client

## 2. Start a Relay Server

Open a terminal and start the relay:

```bash
./target/release/nano-relay --port 7733
```

You should see:
```
🚀 Nano-relay server listening on 127.0.0.1:7733
📬 Ready to relay encrypted messages
```

Leave this running in the background.

## 3. Initialize Your Client

In a new terminal, initialize your client:

```bash
./target/release/nano-client init
```

This generates your cryptographic keys and stores them in `~/.nano-messenger/keys.json`.

**⚠️ Important**: Keep this file safe! It contains your private keys.

## 4. Claim a Username

Choose a unique username and claim it:

```bash
./target/release/nano-client claim-username alice2024
```

Others can now send you messages using this username.

## 5. Send Your First Message

To send a message to someone else (who has also claimed a username):

```bash
./target/release/nano-client send bob2024 "Hey Bob, how are you?"
```

If `bob2024` doesn't exist yet, you'll get an error that the username wasn't found.

## 6. Check for Messages

To check for new messages:

```bash
./target/release/nano-client receive
```

This will:
- Check your first-contact inbox for messages from new people
- Check your conversation inboxes for messages from known contacts
- Show you any new messages with permission prompts

## 7. Manage Contacts

### List all contacts
```bash
./target/release/nano-client contacts list
```

### Allow a contact to continue messaging you
```bash
./target/release/nano-client contacts allow pubkey:abc123...
```

### Block a contact
```bash
./target/release/nano-client contacts block pubkey:abc123...
```

### Add nickname and memo for a contact
```bash
./target/release/nano-client contacts edit pubkey:abc123... \
  --nickname "Alice K." \
  --memo "Coffee shop friend"
```

### Search contacts
```bash
./target/release/nano-client contacts search coffee
```

## 8. View Message History

### See all recent messages
```bash
./target/release/nano-client messages
```

### See messages from a specific person
```bash
./target/release/nano-client messages --from alice2024
```

### Limit number of messages shown
```bash
./target/release/nano-client messages --limit 10
```

## 9. View Your Info

To see your public key and stats:

```bash
./target/release/nano-client info
```

## Example Workflow

Here's a complete example of two users communicating:

### Alice's setup:
```bash
# Terminal 1: Start relay
./target/release/nano-relay

# Terminal 2: Alice's commands
./target/release/nano-client init
./target/release/nano-client claim-username alice2024
```

### Bob's setup:
```bash
# Terminal 3: Bob's commands  
./target/release/nano-client --config-dir ~/.nano-messenger-bob init
./target/release/nano-client --config-dir ~/.nano-messenger-bob claim-username bob2024
```

### Conversation:
```bash
# Bob sends first message
./target/release/nano-client --config-dir ~/.nano-messenger-bob send alice2024 "Hi Alice!"

# Alice checks messages and sees permission prompt
./target/release/nano-client receive

# Alice allows Bob
./target/release/nano-client contacts allow pubkey:bob_public_key_here

# Now they can chat freely
./target/release/nano-client send bob2024 "Hey Bob! Nice to hear from you."
./target/release/nano-client --config-dir ~/.nano-messenger-bob receive
```

## Configuration

### Default Config Directory
By default, nano-client stores data in `~/.nano-messenger/`. You can change this:

```bash
./target/release/nano-client --config-dir /path/to/custom/dir init
```

### Default Relay Server
By default, the client connects to `127.0.0.1:7733`. You can specify a different relay:

```bash
./target/release/nano-client --relay 192.168.1.100:7733 send alice "Hello"
```

## Privacy Features

### What the Relay Sees
- Random inbox IDs that change for each message
- Encrypted message content (cannot be decrypted)
- Message timing and size (but not who is talking to whom)

### What the Relay Cannot See
- Your identity or username (unless you claim one publicly)
- Message content
- Who you're talking to
- Your conversation history

### Contact Permissions
- First messages from unknown senders always get through
- You must explicitly allow contacts to continue messaging
- Blocked contacts' messages are silently dropped
- Contact lists are stored locally and optionally synced (encrypted)

## Troubleshooting

### "Username not found"
The person hasn't claimed that username yet, or you're connecting to a different relay than they used.

### "Connection refused"
Make sure the relay server is running and you're using the correct address/port.

### "Permission denied"
You've been blocked by the recipient, or there's an issue with your message format.

### "Failed to decrypt"
This usually means the message wasn't intended for you, or there's a crypto mismatch.

## Next Steps

- Try the demo script: `chmod +x demo.sh && ./demo.sh`
- Run tests: `python3 test.py`
- Set up your own relay server for friends/family
- Explore the source code to understand the crypto
