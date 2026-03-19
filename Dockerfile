# -------------- Build stage --------------
# Use the official Rust image as the base image
FROM rust:1.93.0 AS builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the current directory contents into the container at /usr/src/app
COPY . .

# Set the working directory to the hello-rust folder
WORKDIR /usr/src/app/hello-rust

# Build the Rust application
RUN cargo build --release


# -------------- Run stage --------------
# Use a smaller base image for the runtime environment
FROM debian:latest

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy only the compiled binary
COPY --from=builder /usr/src/app/hello-rust/target/release/hello-rust /usr/local/bin/hello-rust

# By starting of the container, run the Rust application
CMD ["hello-rust"]