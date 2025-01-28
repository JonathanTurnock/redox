# Use the official Rust image as the build stage
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /usr/app

# Copy the Cargo.toml and Cargo.lock files to the working directory
COPY Cargo.toml Cargo.lock ./

# Create a dummy main file to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Pre-build dependencies to cache them
RUN cargo build --release --all-features && rm -rf src

# Copy the source code into the container
COPY src ./src

# Build the Rust project
RUN cargo build --release --all-features

# Use a minimal base image to reduce size
FROM ubuntu:latest

WORKDIR /usr/app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/app/target/release/redox /usr/local/bin/redox

COPY examples/lua/app.lua /usr/app/app.lua
COPY examples/lua/logger.yml /usr/app/logger.yml

# Command to run the application
CMD ["redox", "serve", "app.lua"]
