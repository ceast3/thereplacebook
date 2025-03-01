# Use the latest Rust image with Cargo
FROM rust:latest AS builder

# Set working directory
WORKDIR /app

# Copy source files
COPY . .

# Install dependencies required for musl
RUN apt update && apt install -y \
    musl-tools \
    musl-dev \
    build-essential \
    cmake \
    clang \
    pkg-config \
    libssl-dev \
    gcc-multilib \
    g++-multilib \
    curl

# Add the musl target for Rust
RUN rustup target add x86_64-unknown-linux-musl

#force cargo to use the correct musl compiler
ENV CC=musl-gcc
ENV CXX=musl-g++

# Build the Rust application
RUN cargo build --release --target=x86_64-unknown-linux-musl

# Use a minimal base image
FROM debian:buster-slim

# Set working directory
WORKDIR /app

# Install OpenSSL runtime
RUN apt update && apt install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy compiled binary from the builder
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/thereplacebook /app/thereplacebook

# Ensure the binary has execution permissions
RUN chmod +x /app/thereplacebook

# Expose the application port
EXPOSE 3000

# Run the application
CMD ["/app/thereplacebook"]