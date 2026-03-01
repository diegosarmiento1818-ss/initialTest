# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the current directory contents into the container at /usr/src/app
COPY . .

# Set the working directory to the hello-rust folder
WORKDIR /usr/src/app/hello-rust

# Build the Rust application
RUN cargo build --release

# By starting of the container, run the Rust application
CMD cargo run