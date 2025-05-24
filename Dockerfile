# Use the official Rust image as the base
FROM rust:1.71 as builder

# Create a new empty shell project
WORKDIR /app
RUN cargo init

# Copy over your manifests
COPY ./Cargo.toml ./Cargo.lock ./

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy your source code
COPY ./src ./src
COPY ./migrations ./migrations
COPY ./config.toml ./config.toml

# Build for release
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the build artifact from the builder stage
COPY --from=builder /app/target/release/rin_kokonoe /usr/local/bin/

# Create directories
RUN mkdir -p /data /rss

# Set the working directory
WORKDIR /app

# Copy the migrations and config
COPY --from=builder /app/migrations /app/migrations
COPY --from=builder /app/config.toml /app/config.toml

# Expose the API port
EXPOSE 8080

# Set environment variables
ENV RUST_LOG=info
ENV DATABASE_URL=sqlite:/data/rin_kokonoe.db
ENV RSS_OUTPUT_DIR=/rss
ENV CONFIG_FILE=/app/config.toml

# Run the binary
CMD ["rin_kokonoe"]
