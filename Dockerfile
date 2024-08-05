FROM rust:1.67-alpine AS builder
WORKDIR /usr/src/myapp
COPY . .
RUN apk add --no-cache musl-dev openssl-dev
RUN cargo build --release

FROM alpine:latest
RUN apk add --no-cache openssl ca-certificates
COPY --from=builder /usr/src/myapp/target/release/* /
CMD ["/maze-generator"]
