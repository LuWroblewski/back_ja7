FROM rust:latest

WORKDIR /app

COPY Cargo.toml ./

ENV DATABASE_URL=postgresql://postgres:123@localhost:5432/postgres
ENV DATABASE_SCHEMA=public
ENV JWT_SECRET=4r0P6khjE0nv8LBQJy4xoKkrumwEF01JcBxuZVdQ

COPY . .

RUN cargo build --release

CMD ["./target/release/backja"]