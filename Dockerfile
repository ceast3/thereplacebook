# 1️⃣ Use the latest stable Rust version
FROM rust:latest AS builder

# 2️⃣ Set the working directory
WORKDIR /app

# 3️⃣ Copy all files from your project to the container
COPY . .

# 4️⃣ Install required dependencies (including OpenSSL)
RUN apt update && apt install -y \
    pkg-config \
    libssl-dev \
    musl-tools \
    musl-dev \
    build-essential \
    cmake \
    clang

# 5️⃣ Set the target architecture explicitly
RUN rustup target add x86_64-unknown-linux-musl

# 6️⃣ Set environment variables for OpenSSL
ENV OPENSSL_DIR=/usr \
    OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu \
    OPENSSL_INCLUDE_DIR=/usr/include \
    PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig

# 7️⃣ Build the binary statically
RUN cargo build --release --target=x86_64-unknown-linux-musl

# 8️⃣ Use a lightweight base image for production
FROM debian:buster-slim

# 9️⃣ Set the working directory inside the new container
WORKDIR /app

# 🔟 Install OpenSSL runtime in the final container
RUN apt update && apt install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# 1️⃣1️⃣ Copy the statically compiled Rust binary
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/thereplacebook /app/thereplacebook

# 1️⃣2️⃣ Ensure the binary has execution permissions
RUN chmod +x /app/thereplacebook

# 1️⃣3️⃣ Expose the application's port
EXPOSE 3000

# 1️⃣4️⃣ Set the startup command
CMD ["/app/thereplacebook"]