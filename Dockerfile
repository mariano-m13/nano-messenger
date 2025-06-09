# Production Dockerfile for Quantum-Resistant Nano-Messenger
# Session 8: Production Hardening

# Multi-stage build for optimized production image
FROM rust:1.75-slim AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app user for security
RUN groupadd -r nano-messenger && useradd -r -g nano-messenger nano-messenger

# Set working directory
WORKDIR /app

# Copy dependency files first for better caching
COPY Cargo.toml Cargo.lock ./

# Create dummy source files to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    mkdir -p src/bin && \
    echo "fn main() {}" > src/bin/client.rs && \
    echo "fn main() {}" > src/bin/relay.rs

# Build dependencies
RUN cargo build --release --bins
RUN rm -rf src

# Copy source code
COPY src ./src
COPY tests ./tests

# Build application with optimizations
RUN cargo build --release --bins

# Runtime stage - minimal image
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    libssl1.1 \
    && rm -rf /var/lib/apt/lists/*

# Create app user and directories
RUN groupadd -r nano-messenger && useradd -r -g nano-messenger nano-messenger
RUN mkdir -p /etc/nano-messenger /var/lib/nano-messenger /var/log/nano-messenger
RUN chown -R nano-messenger:nano-messenger /var/lib/nano-messenger /var/log/nano-messenger

# Copy binaries from builder stage
COPY --from=builder /app/target/release/nano-relay /usr/local/bin/
COPY --from=builder /app/target/release/nano-client /usr/local/bin/

# Set permissions
RUN chmod +x /usr/local/bin/nano-relay /usr/local/bin/nano-client

# Copy default configuration (can be overridden)
COPY config/production.toml /etc/nano-messenger/config.toml.example

# Health check script
COPY --chmod=755 << 'EOF' /usr/local/bin/healthcheck.sh
#!/bin/bash
set -e
curl -f http://localhost:8080/health || exit 1
EOF

# Switch to non-root user
USER nano-messenger

# Expose ports
EXPOSE 8080 8443 9090

# Set environment variables
ENV RUST_LOG=info
ENV ENVIRONMENT=production

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
    CMD /usr/local/bin/healthcheck.sh

# Default command
CMD ["nano-relay", "--config", "/etc/nano-messenger/config.toml"]

# Metadata
LABEL maintainer="Nano-Messenger Team" \
      version="2.0.0" \
      description="Quantum-Resistant Nano-Messenger - Production Ready" \
      org.opencontainers.image.title="Quantum-Resistant Nano-Messenger" \
      org.opencontainers.image.description="Secure messaging with post-quantum cryptography" \
      org.opencontainers.image.version="2.0.0" \
      org.opencontainers.image.vendor="Nano-Messenger Project" \
      org.opencontainers.image.url="https://github.com/nano-messenger/quantum-resistant" \
      org.opencontainers.image.documentation="https://github.com/nano-messenger/quantum-resistant/docs" \
      org.opencontainers.image.source="https://github.com/nano-messenger/quantum-resistant" \
      org.opencontainers.image.licenses="MIT"