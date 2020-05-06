FROM rust:latest as builder

# 1a: Prepare for static linking
RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get install -y musl-tools && \
    rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/rurune
COPY ./ ./

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -rf src/

RUN rm -f /usr/src/rurune/target/x86_64-unknown-linux-musl/release/rurune*

RUN rm -f /usr/src/rurune/target/x86_64-unknown-linux-musl/release/deps/rurune*

RUN rm -f /usr/src/rurune/target/x86_64-unknown-linux-musl/release/rurune.d

COPY src/* src/

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:latest

RUN apk add --no-cache libpq

WORKDIR /root/

COPY ./static ./static

COPY --from=0 /usr/src/rurune/target/x86_64-unknown-linux-musl/release/rurune .

CMD ["./rurune"]
