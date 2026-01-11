FROM rust:1.75 as builder

WORKDIR /app

# Install SQLx CLI for migrations
RUN cargo install sqlx-cli --features postgres

# Copy dependency files
COPY Cargo.toml Cargo.lock* ./

# Copy source code
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/target/release/blog /app/blog

# Copy migrations
COPY --from=builder /app/migrations ./migrations

# Copy static files
COPY --from=builder /app/src/static ./src/static

# Expose port
EXPOSE 3000

# Run the application
CMD ["./blog"]

