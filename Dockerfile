FROM rust:1.93-bookworm AS builder
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends libpq-dev pkg-config && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations

RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates curl libpq5 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/hporterly-backend /usr/local/bin/hporterly-backend
COPY .env.example /app/.env.example

EXPOSE 8080

CMD ["hporterly-backend"]
