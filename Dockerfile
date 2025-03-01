# Use a Rust image based on x86_64 Linux to match the target
FROM messense/rust-musl-cross:x86_64-musl as builder

# Set the working directory
WORKDIR /app

# Copy Cargo files first to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Fetch dependencies separately for caching
RUN cargo fetch

# Copy the source code
COPY . .

# Build the Rust project targeting musl (statically linked)
RUN cargo build --release --target=x86_64-unknown-linux-musl

# Use a minimal base image for production
FROM alpine:latest

# Set working directory
WORKDIR /app

# Copy the built binary
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/thereplacebook /app/thereplacebook

# Ensure execution permissions
RUN chmod +x /app/thereplacebook

# Expose the application port
EXPOSE 3000

# Run the application
CMD ["/app/thereplacebook"]