FROM rust:latest AS builder

WORKDIR /app

RUN rustup install 1.81.0 && rustup default 1.81.0

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /target/release/fibbot /usr/local/bin/fibbot

ENTRYPOINT [ "/usr/local/bin/fibbot" ]
