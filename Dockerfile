FROM rust:1.84-alpine AS builder
WORKDIR /usr/src/app
COPY . .
RUN apk --no-cache add openssl-dev openssl-libs-static musl-dev cmake build-base perl && \
    cargo build --release && \
    strip --strip-all target/release/prom2mqtt-export target/release/prom2mqtt-fetch

FROM alpine:3
COPY --from=builder --chmod=0755 /usr/src/app/target/release/prom2mqtt-export /usr/src/app/target/release/prom2mqtt-fetch /usr/sbin/
CMD ["prom2mqtt-export"]
