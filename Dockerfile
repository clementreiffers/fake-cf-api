FROM rust as builder

RUN cargo install
RUN cargo build --release

FROM alpine as runner
