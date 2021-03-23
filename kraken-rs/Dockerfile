FROM th0masb/rust-x-compile-aarch64-linux-musl:latest AS builder

WORKDIR /root/build

COPY Cargo.toml ./
COPY src/ ./src/
RUN cargo build --release --target aarch64-unknown-linux-musl

FROM --platform=linux/arm64 alpine:3.13

RUN apk update && apk add ca-certificates

COPY --from=builder /root/build/target/aarch64-unknown-linux-musl/release/kraken-rs /app/

CMD /app/kraken-rs
