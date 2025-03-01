# 1️⃣ Use an official Rust image for building
FROM rust:latest AS builder

# 2️⃣ Set the working directory inside the container
WORKDIR /app

# 3️⃣ Copy the project files
COPY . .

# 4️⃣ Cache dependencies to speed up builds
RUN cargo build --release

# 5️⃣ Use a lightweight base image for production
FROM debian:bullseye-slim

# 6️⃣ Set up necessary dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# 7️⃣ Set the working directory for the runtime
WORKDIR /app

# 8️⃣ Copy the compiled Rust binary from the builder stage
COPY --from=builder /app/target/release/thereplacebook .

# 9️⃣ Expose the app's port (same as in your Rust app)
EXPOSE 3000

# 🔟 Run the Rust application
CMD ["./thereplacebook"]