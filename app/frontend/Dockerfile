FROM rust:1.81 AS builder
WORKDIR /app/frontend
# Fix cargo lock handling
COPY Cargo.toml ./
RUN mkdir -p src && echo 'fn main() {}' > src/main.rs
RUN cargo fetch
# Remove the dummy file and copy actual source
RUN rm -rf src
COPY src ./src
COPY templates ./templates
COPY static ./static
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app/frontend
COPY --from=builder /app/frontend/static ./static
COPY --from=builder /app/frontend/templates ./templates
COPY --from=builder /app/frontend/target/release/frontend /usr/local/bin/frontend
EXPOSE 8080
CMD ["frontend"]