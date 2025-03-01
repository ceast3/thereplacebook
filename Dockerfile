# Use the latest stable Rust version
FROM rust:latest AS builder

# Set the working directory
WORKDIR /app

# Copy all files from your project to the container
COPY . .

# Install required dependencies
RUN apt update && apt install -y \
    musl-tools \
    musl-dev \
    build-essential \
    cmake \
    clang \
    pkg-config \
    libssl-dev \
    curl

# Install `cross` inside the container
RUN cargo install cross

# Ensure the `cargo` bin directory is in the `PATH`
ENV PATH="/root/.cargo/bin:$PATH"

# Add the required Rust target
RUN rustup target add x86_64-unknown-linux-musl

# Build the Rust application using `cross`
RUN cross build --release --target x86_64-unknown-linux-musl

# Use a minimal base image for the final stage
FROM debian:buster-slim

# Set the working directory
WORKDIR /app

# Install OpenSSL runtime
RUN apt update && apt install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/thereplacebook /app/thereplacebook

# Ensure the binary has execution permissions
RUN chmod +x /app/thereplacebook

# Expose the application port
EXPOSE 3000

# Set the startup command
CMD ["/app/thereplacebook"]