FROM rust:slim AS builder

WORKDIR /app

COPY . . 

RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app

COPY --from=builder /app/target/release/eth_service .

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
EXPOSE 3000
CMD ["./eth_service"]