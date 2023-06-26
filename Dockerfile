FROM alpine:edge as builder

RUN apk add git curl pkgconfig openssl-dev gcc musl-dev rustup cargo
#RUN rustup-init -t x86_64-unknown-linux-musl --default-toolchain nightly --profile minimal -y

COPY ./ ./

RUN cargo build --release

FROM alpine as runner

COPY ./.env ./
COPY --from=builder ./target/release/ ./

CMD ["./rust-fake-cf-api"]

