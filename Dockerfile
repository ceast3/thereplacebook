# 1️⃣ Use the latest stable Rust version
FROM rust:latest AS builder

# 2️⃣ Set the working directory
WORKDIR /app

# 3️⃣ Copy all files from your project to the container
COPY . .

# 4️⃣ Install required dependencies
RUN apt update && apt install -y pkg-config libssl-dev musl-tools

# 5️⃣ Set the target architecture explicitly
RUN rustup target add x86_64-unknown-linux-musl

# 6️⃣ Build the binary statically
RUN cargo build --release --target=x86_64-unknown-linux-musl

# 7️⃣ Use a lightweight base image for production
FROM debian:buster-slim

# 8️⃣ Set the working directory inside the new container
WORKDIR /app

# 9️⃣ Copy the statically compiled Rust binary
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/thereplacebook /app/thereplacebook

# 🔟 Ensure the binary has execution permissions
RUN chmod +x /app/thereplacebook

# 1️⃣1️⃣ Expose the application's port
EXPOSE 3000

# 1️⃣2️⃣ Set the startup command
CMD ["/app/thereplacebook"]