# ----------- Builder Stage -----------
FROM rust:1.95-slim AS builder

WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy actual source
COPY src ./src

RUN cargo build --release

# ----------- Runtime Stage -----------
FROM debian:bookworm-slim

WORKDIR /app

# Install CA certificates
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/prexcore-payment-processor ./app

# Directory for .DAT files
RUN mkdir -p /app/balances

EXPOSE 8080

ENV RUST_LOG=info

CMD ["./app"]