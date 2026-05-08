FROM rust:1.92-bookworm AS builder
WORKDIR /app
COPY Cargo.toml ./
RUN mkdir -p src
RUN printf "fn main() {}\n" > src/main.rs
RUN cargo build --release
COPY src src
COPY migrations migrations
RUN cargo build --release

FROM debian:13-slim
COPY --from=builder /app/target/release/backend /backend
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
EXPOSE 8080
ENV RUST_LOG=debug
CMD ["/backend"]