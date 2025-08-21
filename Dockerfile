FROM rust:latest
ARG SQLX_OFFLINE=true

WORKDIR /app
COPY . .

RUN cargo build --release

CMD ["./target/release/replicant"]