FROM rust:latest

WORKDIR /usr/src/app

COPY . .

WORKDIR /usr/src/app/hello-rust
RUN cargo build --release

CMD cargo run