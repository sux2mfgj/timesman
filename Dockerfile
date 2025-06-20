# Multi-stage Dockerfile for TimesMan server
# Builds a minimal container image with just the server binary

# Build stage
FROM rust:1.75-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd --create-home --shell /bin/bash app

# Set working directory
WORKDIR /app

# Copy workspace configuration
COPY Cargo.toml Cargo.lock ./
COPY .cargo .cargo

# Copy source code
COPY timesman-type/ timesman-type/
COPY timesman-grpc/ timesman-grpc/
COPY timesman-bstore/ timesman-bstore/
COPY timesman-server/ timesman-server/

# Build the server binary in release mode
RUN cargo build --release --bin timesman-server

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd --create-home --shell /bin/bash --uid 1000 timesman

# Create directories for the application
RUN mkdir -p /app /data /config /logs && \
    chown -R timesman:timesman /app /data /config /logs

# Copy the binary from builder stage
COPY --from=builder /app/target/release/timesman-server /app/timesman-server

# Copy configuration template
COPY timesman-server/config.toml /config/config.example.toml

# Copy documentation
COPY docs/ /app/docs/
COPY README.md /app/
COPY LICENSE /app/

# Set permissions
RUN chmod +x /app/timesman-server && \
    chown -R timesman:timesman /app

# Switch to app user
USER timesman

# Set working directory
WORKDIR /app

# Environment variables
ENV RUST_LOG=info
ENV TIMESMAN_CONFIG=/config/config.toml
ENV TIMESMAN_JWT_SECRET=change-this-secret-in-production
ENV TIMESMAN_LISTEN=0.0.0.0:50051

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD /app/timesman-server --version || exit 1

# Expose gRPC port
EXPOSE 50051

# Default command
CMD ["/app/timesman-server", "--config", "/config/config.toml"]

# Labels
LABEL org.opencontainers.image.title="TimesMan Server"
LABEL org.opencontainers.image.description="Time tracking server with JWT authentication"
LABEL org.opencontainers.image.vendor="TimesMan Team"
LABEL org.opencontainers.image.licenses="MIT"
LABEL org.opencontainers.image.source="https://github.com/your-repo/timesman"