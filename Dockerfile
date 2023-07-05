FROM rust as builder

COPY ./ ./

RUN cargo build --release

FROM rust as runner

COPY --from=builder ./target/release/rust-fake-cf-api ./

CMD ["./rust-fake-cf-api"]
