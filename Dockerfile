# 1️⃣ Use the latest stable Rust version
FROM rust:latest as builder

# 2️⃣ Set the working directory
WORKDIR /app

# 3️⃣ Copy all files from your project to the container
COPY . .

# 4️⃣ Install required dependencies
RUN apt update && apt install -y pkg-config libssl-dev

# 5️⃣ Cache dependencies to speed up builds
RUN cargo build --release

# 6️⃣ Use a lightweight base image for production
FROM debian:buster-slim

# 7️⃣ Set the working directory inside the new container
WORKDIR /app

# 8️⃣ Copy the compiled Rust binary from the builder container
COPY --from=builder /app/target/release/thereplacebook /app/thereplacebook

# 9️⃣ Expose the application's port (Ensure this matches your app’s port)
EXPOSE 3000

# 🔟 Set executable permissions and define the startup command
CMD ["/app/thereplacebook"]