# Library

![Architecture Illustration](docs/architecture-illustration.jpg)

## Structure

- `backend/gateway` - Rust API gateway (Axum + Tonic), Redis cache, Kafka invalidation hooks.
- `backend/auth` - Go gRPC auth service with PostgreSQL + Kafka.
- `backend/statistics` - Python gRPC stats service with ClickHouse + Kafka.
- `backend/notification` - Python gRPC notification service with MongoDB + Kafka.
- `backend/engagement` - Python gRPC engagement service (comments/likes) with MongoDB + Kafka.
- `backend/content` - Python gRPC content service with MongoDB + Kafka.
- `proto` - Shared gRPC service definitions.
- `frontend` - Vue.js frontend app.

Each service is a minimal starting point with bootstrapped config and placeholder handlers.
