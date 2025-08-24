# Multi-stage build for Rust backend
FROM cgr.dev/chainguard/rust:latest-dev as builder

# Create app directory
WORKDIR /app

# Copy Cargo files for dependency caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer will be cached unless Cargo.toml changes)
RUN cargo build --release

# Remove dummy source
RUN rm -rf src

# Copy actual source code
COPY src ./src

# Build the actual application
RUN cargo build --release

# Runtime stage - Chainguard distroless Rust runtime
FROM cgr.dev/chainguard/glibc-dynamic:latest

# Create app directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/triage /app/triage

# Switch to non-root user (Chainguard images run as non-root by default)
USER nonroot:nonroot

# Expose port
EXPOSE 3001

# Health check - Note: distroless images don't have curl, so we use the binary itself
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD ["/app/triage", "--health-check"] || exit 1

# Run the binary
ENTRYPOINT ["/app/triage"]