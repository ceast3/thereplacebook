# 1️⃣ Use Rust official image for building
FROM rust:latest AS builder

# 2️⃣ Install dependencies for musl-based builds
RUN apt update && apt install -y \
    musl-tools \
    musl-dev \
    build-essential \
    pkg-config \
    libssl-dev \
    cmake \
    clang

# 3️⃣ Set working directory
WORKDIR /app

# 4️⃣ Copy Cargo files separately to optimize Docker caching
COPY Cargo.toml Cargo.lock ./

# 5️⃣ Fetch dependencies before copying the entire source code
RUN cargo fetch

# 6️⃣ Copy the project source code
COPY . .

# 7️⃣ Set the target for musl
RUN rustup target add x86_64-unknown-linux-musl

# 8️⃣ Explicitly set environment variables for OpenSSL
ENV OPENSSL_DIR=/usr \
    OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu \
    OPENSSL_INCLUDE_DIR=/usr/include \
    PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig

# 9️⃣ Build the Rust application with musl
RUN cargo build --release --target=x86_64-unknown-linux-musl

# 🔟 Use a lightweight production image
FROM alpine:latest

# 1️⃣1️⃣ Install required runtime dependencies
RUN apk --no-cache add ca-certificates

# 1️⃣2️⃣ Set working directory
WORKDIR /usr/local/bin

# 1️⃣3️⃣ Copy the compiled binary from the builder stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/thereplacebook /usr/local/bin/thereplacebook

# 1️⃣4️⃣ Ensure execution permissions
RUN chmod +x /usr/local/bin/thereplacebook

# 1️⃣5️⃣ Expose the application's port
EXPOSE 3000

# 1️⃣6️⃣ Define the startup command
CMD ["./thereplacebook"]