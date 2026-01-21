FROM rust:1.76-bookworm AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

COPY . .

RUN cargo build --release


FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 👇 CHANGE THIS to your actual binary name
COPY --from=builder /app/target/release/server_hex /usr/local/bin/app

EXPOSE 3000

CMD ["app"]
