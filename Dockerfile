FROM rust:latest AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/fibbot /usr/local/bin/fibbot

ENTRYPOINT [ "/usr/local/bin/fibbot" ]
