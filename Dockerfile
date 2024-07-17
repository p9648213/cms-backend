# Use the official Rust image as the build image
FROM rust:1.79 as builder

# Create a new directory for the app
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Copy the .env file
COPY .env ./

# Build the application
RUN cargo build --release

# Use an Ubuntu base image
FROM ubuntu:22.04

# Install the libpq library
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

# Create a new directory for the app
WORKDIR /usr/src/app

# Copy the built binary from the build image
COPY --from=builder /usr/src/app/target/release/backend .

# Set the startup command to run the binary
CMD ["./backend"]