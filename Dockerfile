# Use musl cross-compilation image for Rust on Mac M1
FROM messense/rust-musl-cross:x86_64-musl as builder

# Set working directory
WORKDIR /app

# Install necessary dependencies
#RUN apt update && apt install -y \
#    pkg-config \
#    cmake \
#    clang \
#    curl \
#    && rm -rf /var/lib/apt/lists/*

# Copy only necessary files for caching dependencies
COPY Cargo.toml Cargo.lock ./

# Fetch dependencies before adding source code
RUN cargo fetch

# Copy application source code
COPY . .

# Build the Rust application statically
RUN cargo build --release --target=x86_64-unknown-linux-musl

# Use a minimal base image
FROM alpine:latest

# Set working directory
WORKDIR /app

# Install required system packages for runtime
RUN apk add --no-cache ca-certificates

# Copy compiled binary from builder stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/thereplacebook /app/thereplacebook

# Copy static files separately to ensure they exist in final container
COPY static/ /app/static/

# Set execute permissions
RUN chmod +x /app/thereplacebook

# Pass AWS credentials to the container
ENV AWS_REGION=us-east-1
ENV AWS_DEFAULT_REGION=us-east-1

# Expose application port
EXPOSE 80

# Run the application
CMD ["/app/thereplacebook"]