FROM rust:latest as builder

WORKDIR /app

# Install SQLx CLI for migrations (совместимая версия с sqlx 0.8)
RUN cargo install sqlx-cli --features postgres --locked

# Copy all files needed for build
COPY Cargo.toml Cargo.lock* ./
COPY migrations ./migrations
COPY templates ./templates
COPY src ./src

# Update dependencies and build the application
RUN cargo update && cargo build --release

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

