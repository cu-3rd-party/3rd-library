FROM rust:1.92 AS builder
WORKDIR /app
COPY Cargo.toml ./
RUN mkdir -p src
RUN printf "fn main() {}\n" > src/main.rs
RUN cargo build --release
COPY src src
COPY .sqlx .sqlx
COPY migrations migrations
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/target/release/backend /backend
EXPOSE 8080
ENV RUST_LOG=debug
CMD ["/backend"]
