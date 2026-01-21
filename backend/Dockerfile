FROM rust:1.92 AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml backend/Cargo.toml
RUN mkdir -p backend/src
RUN printf "fn main() {}\n" > backend/src/main.rs
RUN cargo build --manifest-path backend/Cargo.toml --release
COPY backend/src backend/src
COPY .sqlx .sqlx
COPY backend/migrations backend/migrations
RUN cargo build -p backend --release

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/target/release/backend /backend
RUN mkdir -p /var/app/upload
EXPOSE 8080
ENV RUST_LOG=debug
CMD ["/backend"]
