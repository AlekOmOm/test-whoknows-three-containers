FROM rust:1.81 as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /usr/src/app/target/release/backend /usr/local/bin/backend

EXPOSE 8081
CMD ["backend"]