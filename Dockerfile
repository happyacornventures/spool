FROM rust:1-slim-bullseye AS builder

WORKDIR /usr/src/app

COPY Cargo.* ./
COPY src ./src
RUN cargo build --release

FROM debian:bullseye-slim

COPY --from=builder /usr/src/app/target/release/spool /usr/local/bin/spool

ENTRYPOINT ["spool"]
