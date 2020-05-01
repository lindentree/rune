FROM rust:latest

RUN apt-get update

RUN apt-get install musl-tools -y

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/rurune

COPY ./ ./

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -rf src/

RUN rm -f /usr/src/rust-web-demo/target/x86_64-unknown-linux-musl/release/rust-web-demo*

RUN rm -f /usr/src/rust-web-demo/target/x86_64-unknown-linux-musl/release/deps/rurune*

RUN rm -f /usr/src/rust-web-demo/target/x86_64-unknown-linux-musl/release/rurune.d

COPY src/* src/

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:latest

RUN apk add --no-cache libpq

WORKDIR /root/

COPY --from=0 /usr/src/rurune/target/x86_64-unknown-linux-musl/release/rurune .

CMD ["./rurune"]