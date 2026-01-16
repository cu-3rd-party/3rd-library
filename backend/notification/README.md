# Notification Service

Python gRPC notification service that consumes Kafka events and delivers
notifications via popup and email channels. The implementation follows DDD:
domain models and policies live in the domain layer, orchestration in the
application layer, and IO (Kafka, gRPC, SMTP, persistence) in infrastructure.

## Responsibilities

- Consume Kafka events that represent notification intents.
- Fan-out to channel-specific senders (popup, email).
- Persist notification delivery state for observability and retries.
- Expose gRPC endpoints for querying notification status.

## DDD structure

Use the following package boundaries when implementing the service:

- `src/domain/`
  - `models.py`: Entities/value objects (Notification, Recipient, Channel).
  - `policies.py`: Domain rules (routing, opt-out, throttling).
  - `events.py`: Domain events (NotificationCreated, DeliveryFailed).
  - `repository.py`: Abstract repository interfaces.
- `src/application/`
  - `service.py`: Use cases (ingest event, dispatch notification, retry).
  - `dto.py`: Data transfer objects for inbound Kafka payloads and gRPC.
- `src/infrastructure/`
  - `kafka/consumer.py`: Kafka consumer and message decoding.
  - `email/sender.py`: SMTP/email provider adapter.
  - `popup/sender.py`: Popup delivery adapter.
  - `repository.py`: MongoDB implementation of repositories.
  - `grpc/servicer.py`: gRPC server wiring.
- `src/main.py`: Composition root (wire adapters and start servers).

## Kafka contract

Consume a topic that emits a notification intent. Suggested payload:

```json
{
  "notification_id": "uuid",
  "recipient_id": "uuid",
  "channels": ["popup", "email"],
  "template": "string",
  "data": {"key": "value"},
  "created_at": "RFC3339 timestamp"
}
```

## gRPC

Definitions live in `proto/notification.proto`. Implement gRPC handlers in
`src/infrastructure/grpc/servicer.py` and delegate to application services.

## Local development

- Configure env vars (Kafka broker, MongoDB URI, SMTP creds).
- Run `python -m src.main`.
