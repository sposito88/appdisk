FROM rust:1.75 as builder
WORKDIR /usr/src/appdisk
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /usr/src/appdisk/target/release/appdisk /usr/local/bin/
ENTRYPOINT ["appdisk"] 