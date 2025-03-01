FROM rust:latest AS builder


WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /target/release/fibbot /app

ENTRYPOINT [ "/app/target/release/fibbot" ]
