ARG RELEASE_VERSION=20

FROM alpine:latest AS builder
ARG RELEASE_VERSION

RUN apk update && apk add curl

RUN curl -O -L https://github.com/clementreiffers/rust-fake-cf-api/releases/download/${RELEASE_VERSION}/fake-cf-api-release-alpine-v${RELEASE_VERSION}

RUN mv fake-cf-api-release-alpine-v${RELEASE_VERSION} fake-cf-api

FROM alpine AS runner

COPY --from=builder ./fake-cf-api ./

RUN chmod +x ./fake-cf-api

