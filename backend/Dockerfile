FROM rust:1.85 as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml backend/Cargo.toml
COPY backend/src backend/src
RUN cargo build -p backend --release

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/target/release/backend /backend
EXPOSE 8080
ENV RUST_LOG=info
ENTRYPOINT ["/backend"]
