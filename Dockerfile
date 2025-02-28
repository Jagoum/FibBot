FROM rust:latest

WORKDIR /app

# Install and set Rust version to 1.81.0

COPY . .

RUN cargo build --release

ENTRYPOINT ["./target/release/fibbot"]