FROM rust:1.24.0

WORKDIR /usr/src/app
COPY . .

RUN cargo build

CMD ["./target/debug/app"]