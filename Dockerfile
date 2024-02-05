# syntax=docker/dockerfile:1

# Use the official Rust image as the base image
FROM rust:1.75.0 AS builder

# Set the working directory inside the container
WORKDIR /app

# Copy the entire project to the container
COPY . .

# Build the application
RUN cargo build --release

# Create a new image without the build environment
FROM debian:bullseye-slim AS final

# Set the working directory inside the container
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/rustchat /app/rustchat

# Expose the port your Rocket application will run on
EXPOSE 3000

# Command to run your application
CMD ["/app/rustchat"]
