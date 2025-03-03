# Use musl cross-compilation image for Rust
FROM messense/rust-musl-cross:x86_64-musl as builder

# Set working directory
WORKDIR /app

# Copy only Cargo files and `src/` to maximize caching
COPY Cargo.toml Cargo.lock src/ ./

# Fetch dependencies
RUN cargo fetch

# Copy the rest of the application
COPY . .

# Build the Rust application statically
RUN cargo build --release --target=x86_64-unknown-linux-musl

# Use a minimal Alpine base image
FROM alpine:latest

# Set working directory
WORKDIR /app

# Install required system packages for runtime
RUN apk add --no-cache ca-certificates aws-cli

# Install required system packages for runtime
RUN apk add --no-cache ca-certificates

# Copy compiled binary from builder stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/thereplacebook /app/thereplacebook

# Copy static files separately to ensure they exist in the final container
COPY static/ /app/static/

# Set execute permissions for the binary
RUN chmod +x /app/thereplacebook

# Pass AWS credentials to the container
ENV AWS_REGION=us-east-1
ENV AWS_DEFAULT_REGION=us-east-1

# Expose application port
EXPOSE 80

# Run the application
CMD ["/app/thereplacebook"]