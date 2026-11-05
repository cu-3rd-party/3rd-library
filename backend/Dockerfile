# syntax=docker/dockerfile:1.7
# Контекст сборки — корень репозитория (там workspace Cargo.toml): docker build -f backend/Dockerfile .

FROM lukemathwalker/cargo-chef:latest-rust-1-trixie AS chef
WORKDIR /src

# Layer 1: рецепт зависимостей (кэш переживает изменения исходников)
FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
COPY backend/crates backend/crates
COPY stress-test stress-test
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS build
# SMTP включён всегда: без SMTP_HOST коды подтверждения только пишутся в лог
ARG FEATURES=library-web/smtp
COPY --from=planner /src/recipe.json recipe.json
RUN --mount=type=cache,id=cargo-registry,target=/usr/local/cargo/registry \
    cargo chef cook --release --package library-web --features "$FEATURES" --recipe-path recipe.json

# Layer 2: исходники
COPY Cargo.toml Cargo.lock ./
COPY backend/crates backend/crates
COPY stress-test stress-test
RUN --mount=type=cache,id=cargo-registry,target=/usr/local/cargo/registry \
    cargo build --release --locked --package library-web --features "$FEATURES" \
    && cp target/release/library-web /app

FROM gcr.io/distroless/cc-debian13:nonroot AS runtime
WORKDIR /app

ENV LISTEN_ADDR=0.0.0.0:8080
ENV METRICS_ADDR=0.0.0.0:9091
ENV RUST_LOG=info,sqlx=warn

COPY --from=build /app /app/library-web

EXPOSE 8080 9091

# в distroless нет wget/curl — бинарь умеет проверять себя сам
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD ["/app/library-web", "healthcheck"]

ENTRYPOINT ["/app/library-web"]
