


 # Use an official Rust runtime as a parent image
FROM rust:latest

# Set the working directory in the container
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . .

#Running test to ensure the code works as expected
RUN cargo test --release

# Build the Rust project
RUN cargo build --release

# Debugging step: List the contents of the /app directory
RUN ls -la /app

# Debugging step: List the contents of the /target/release directory
RUN ls -la /app/target/release

# Run the executable
#ENTRYPOINT ["/app/target/release/fibbot-test"]
ENTRYPOINT ["/app/target/release/fibbot"]
 






# FROM rust:latest AS builder


# WORKDIR /app
# COPY . .

# RUN cargo build --release

# FROM debian:bookworm-slim

# WORKDIR /app

# COPY --from=builder /target/release/fibbot /app

# ENTRYPOINT [ "/app/target/release/fibbot" ]


