# backend/Dockerfile
# Build stage
FROM rust:1.81 AS builder
WORKDIR /app/backend


COPY Cargo.toml ./
RUN mkdir -p src && echo 'fn main() {}' > src/main.rs
RUN cargo fetch

RUN rm -rf src
COPY src ./src
RUN cargo build --release

# Run stage 
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/backend/target/release/backend /usr/local/bin/backend
EXPOSE 8081
CMD ["backend"]