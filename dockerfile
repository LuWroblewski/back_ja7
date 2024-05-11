FROM rust:latest

WORKDIR /app

COPY Cargo.toml ./

COPY . .

RUN cargo build --release

CMD ["./target/release/back_ja7"]