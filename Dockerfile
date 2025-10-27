FROM rust:1-slim-bullseye AS builder

WORKDIR /usr/src/app

COPY Cargo.* ./
COPY src ./src
RUN cargo build --release
