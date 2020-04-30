FROM rust:latest

WORKDIR /usr/src/rurune

COPY . .

RUN cargo build --release

RUN cargo install --path .

CMD ["/usr/local/cargo/bin/rurune"]