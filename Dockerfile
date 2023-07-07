FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && apt-get install -y musl-tools

COPY ./ ./

RUN cargo build --target=x86_64-unknown-linux-musl --release

FROM alpine AS runner

COPY --from=builder /target/x86_64-unknown-linux-musl/release/rust-fake-cf-api ./

