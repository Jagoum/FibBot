FROM rust:latest AS builder

COPY . .
 
WORKDIR /app

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /target/release/fibbot /usr/local/bin/fibbot

ENTRYPOINT [ "/usr/local/bin/fibbot" ]
