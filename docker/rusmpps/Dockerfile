FROM rust:1.90-alpine AS builder
RUN apk add --no-cache musl-dev
WORKDIR /usr/src/rusmpp
COPY . .
RUN cargo build -p rusmpps --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /usr/src/rusmpp/target/x86_64-unknown-linux-musl/release/rusmpps /rusmpps
ENTRYPOINT ["/rusmpps"]

# docker build -t jadkhaddad/rusmpps:latest -f docker/rusmpps/Dockerfile .