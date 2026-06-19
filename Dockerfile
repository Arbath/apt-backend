FROM rust:1.92 AS builder
# Builder
FROM rust:1.77-bookworm as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/apt-backend /usr/local/bin/app-backend
EXPOSE 8000
CMD ["app-backend"]