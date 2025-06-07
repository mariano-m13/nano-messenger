# Nano Messenger

A zero-knowledge, privacy-first messaging system built on a custom TCP protocol.

## Overview

Nano Messenger is designed to provide secure, private messaging without requiring personal information, centralized identity, or storing user data. The system uses end-to-end encryption and a novel inbox-based routing system to prevent traffic analysis.

## Core Features

### 🔒 Privacy & Security
- **Zero-knowledge**: Relays cannot decrypt or analyze message content
- **No registration**: No email, phone, or personal information required
- **End-to-end encryption**: All messages encrypted with ChaCha20Poly1305
- **Perfect forward secrecy**: ECDH key exchange with message counters
- **Traffic analysis resistance**: Messages use different random inbox IDs

### 📡 Decentralized Architecture
- **Custom TCP protocol**: No dependency on HTTP/WebSockets
- **Self-hostable relays**: Run your own relay servers
- **Federation ready**: Multiple relays can work together (planned)
- **Offline-first**: Messages cached until recipients come online

### 🎯 User Experience
- **Username claiming**: Users can claim human-readable usernames
- **First message delivery**: Anyone can send one message to start a conversation
- **Permission system**: Recipients allow/block contacts after first message
- **Contact management**: Local nicknames, memos, and search
- **Simple CLI**: Traditional command-line interface

## Architecture

### Message Flow

1. **First Contact**: Alice claims username `alice2024`, Bob sends first message
2. **Permission Grant**: Alice receives message, can allow or block Bob
3. **Ongoing Chat**: If allowed, Bob and Alice derive shared secret via ECDH
4. **Inbox Rotation**: Each message uses different inbox ID based on shared secret + counter

### Inbox System

- **First contact inbox**: `SHA256("first_contact:" + recipient_public_key)`
- **Conversation inboxes**: `SHA256(shared_secret + message_counter)`
- **Zero-knowledge**: Relay sees random inbox IDs, cannot link conversations

### Cryptography

- **Key Exchange**: X25519 ECDH for shared secrets
- **Signatures**: Ed25519 for message authentication
- **Encryption**: ChaCha20Poly1305 for message content
- **Hashing**: SHA256 for inbox derivation

## Quick Start

### 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Build Nano Messenger

```bash
cd nano-messenger
cargo build --release
```

### 3. Start a Relay Server

```bash
./target/release/nano-relay --port 7733
```

### 4. Initialize Your Client

```bash
./target/release/nano-client init
```

### 5. Claim a Username

```bash
./target/release/nano-client claim-username alice2024
```

### 6. Send Messages

```bash
# Send first message to someone
./target/release/nano-client send bob2024 "Hey, this is Alice!"

# Check for new messages
./target/release/nano-client receive
```

## CLI Commands

### User Management
```bash
nano-client init                    # Generate keypair
nano-client claim-username alice    # Claim username
nano-client info                    # Show user info
```

### Messaging
```bash
nano-client send bob "Hello!"       # Send message
nano-client receive                 # Check for new messages
nano-client messages                # Show message history
nano-client messages --from bob     # Filter by sender
```

### Contact Management
```bash
nano-client contacts list           # List all contacts
nano-client contacts allow <pubkey> # Allow contact
nano-client contacts block <pubkey> # Block contact
nano-client contacts edit <pubkey> --nickname "Bob K." --memo "Coffee shop friend"
nano-client contacts search coffee  # Search by nickname/memo
```

## Protocol Messages

### Message Envelope (TCP)
```json
{
  "version": "1.1",
  "inbox_id": "a7b3f2e8d9c1...",
  "payload": "base64_encrypted_blob",
  "nonce": "random_nonce",
  "expiry": 1720400000
}
```

### Encrypted Payload
```json
{
  "from_pubkey": "pubkey:abc123...",
  "timestamp": 1717405312,
  "body": "Hello world!",
  "counter": 1,
  "sig": "signature_of_above"
}
```

### Username Claim
```json
{
  "claim_type": "username_claim",
  "username": "alice2024",
  "public_keys": { ... },
  "timestamp": 1717405312,
  "sig": "signature"
}
```

## Configuration

### Client Config
- **Keys**: Stored in `~/.nano-messenger/keys.json`
- **Contacts**: Stored in `~/.nano-messenger/contacts.json`
- **Messages**: Stored in `~/.nano-messenger/messages.json` (planned)

### Relay Config
```bash
nano-relay \
  --address 0.0.0.0 \
  --port 7733 \
  --max-cache-size 1000 \
  --message-ttl 86400
```

## Security Model

### Threats Addressed
- **Traffic analysis**: Random inbox IDs prevent conversation linking
- **Message interception**: End-to-end encryption protects content
- **Spam/harassment**: Permission system with allow/block controls
- **Identity tracking**: No persistent identifiers required

### Trust Assumptions
- **Relay honesty**: Relays may log metadata but cannot decrypt content
- **Clock sync**: Message counters require reasonable time synchronization
- **Key security**: Users must protect their private keys

## Development

### Project Structure
```
nano-messenger/
├── src/
│   ├── lib.rs           # Main library
│   ├── crypto.rs        # Cryptographic operations
│   ├── protocol.rs      # Message formats
│   ├── inbox.rs         # Inbox derivation logic
│   ├── username.rs      # Username management
│   ├── contacts.rs      # Contact management
│   └── bin/
│       ├── client.rs    # CLI client
│       └── relay.rs     # Relay server
├── Cargo.toml
└── README.md
```

### Testing
```bash
cargo test                # Run all tests
cargo test crypto        # Test crypto module
cargo test --bin client  # Test client binary
```

### Minimal Dependencies
- `chacha20poly1305`: Symmetric encryption
- `x25519-dalek`: ECDH key exchange
- `ed25519-dalek`: Digital signatures
- `sha2`: Cryptographic hashing
- `serde`: JSON serialization
- `tokio`: Async networking
- `clap`: CLI parsing

## Roadmap

### Phase 1: Core Protocol ✅
- [x] Cryptographic primitives
- [x] Message envelope format
- [x] Inbox derivation system
- [x] Basic CLI client
- [x] Simple relay server

### Phase 2: Enhanced Features
- [ ] TCP client/relay communication
- [ ] Message persistence
- [ ] Multi-device sync
- [ ] File attachments
- [ ] Group messaging

### Phase 3: Advanced Privacy
- [ ] Relay federation
- [ ] Mixnet integration
- [ ] Tor/I2P support
- [ ] Traffic obfuscation
- [ ] Anonymous payments

## Contributing

1. Fork the repository
2. Create a feature branch
3. Implement changes with tests
4. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Disclaimer

This is experimental software. Do not use for sensitive communications without a thorough security review.
