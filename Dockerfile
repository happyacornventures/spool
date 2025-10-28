FROM rust:1-slim-bullseye AS builder

WORKDIR /usr/src/app

COPY Cargo.* ./
COPY src ./src
RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y  --no-install-recommends coreutils bash docker.io docker-compose && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/spool /usr/local/bin/spool

ENTRYPOINT ["spool"]
