# Gateway

Rust API gateway using Axum for HTTP ingress and Tonic for gRPC to internal services.

## Notes

- Redis is used for caching and rate limiting.
- Kafka is used to invalidate cache entries.
- gRPC client stubs should be generated from `proto/*.proto`.
