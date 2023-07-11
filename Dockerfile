ARG RELEASE_VERSION=20

FROM alpine:latest AS builder
ARG RELEASE_VERSION

RUN apk update && apk add curl unzip

RUN curl -O -L https://github.com/clementreiffers/rust-fake-cf-api/releases/download/${RELEASE_VERSION}/release-alpine-v${RELEASE_VERSION}.zip

RUN unzip release-alpine-v${RELEASE_VERSION}.zip

FROM alpine AS runner

COPY --from=builder ./rust-fake-cf-api ./

