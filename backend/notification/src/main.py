import logging
import os
from concurrent import futures
from threading import Thread

import grpc

from application import NotificationService
from infrastructure import (
    InMemoryNotificationRepository,
    MongoNotificationRepository,
    NotificationKafkaConsumer,
    PopupSender,
    SmtpEmailSender,
)


def main() -> None:
    logging.basicConfig(level=os.getenv("NOTIFICATION_LOG_LEVEL", "INFO"))

    repository = _build_repository()
    popup_sender = PopupSender()
    email_sender = _build_email_sender()
    service = NotificationService(repository, popup_sender, email_sender)

    _start_grpc_server()

    brokers = os.getenv("KAFKA_BROKERS", "localhost:9092")
    topic = os.getenv("NOTIFICATION_KAFKA_TOPIC", "notifications")
    group_id = os.getenv("NOTIFICATION_KAFKA_GROUP", "notification-service")
    consumer = NotificationKafkaConsumer(service, brokers=brokers, topic=topic, group_id=group_id)
    consumer.run()


def _start_grpc_server() -> None:
    addr = os.getenv("NOTIFICATION_GRPC_ADDR")
    if not addr:
        return
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    server.add_insecure_port(addr)
    server.start()
    logging.getLogger(__name__).info("notification gRPC listening on %s", addr)
    thread = Thread(target=server.wait_for_termination, daemon=True)
    thread.start()


def _build_repository():
    repository_kind = os.getenv("NOTIFICATION_REPOSITORY", "").strip().lower()
    if repository_kind in {"memory", "in_memory", "inmemory"}:
        return InMemoryNotificationRepository()
    dsn = os.getenv("MONGODB_URI")
    if dsn:
        database = os.getenv("MONGODB_DATABASE", "notification")
        collection = os.getenv("MONGODB_COLLECTION", "notifications")
        return MongoNotificationRepository(dsn=dsn, database=database, collection=collection)
    logging.getLogger(__name__).info("MONGODB_URI not set, using in-memory repository.")
    return InMemoryNotificationRepository()


def _build_email_sender() -> SmtpEmailSender:
    host = os.getenv("SMTP_HOST", "")
    port = int(os.getenv("SMTP_PORT", "587"))
    username = os.getenv("SMTP_USERNAME") or None
    password = os.getenv("SMTP_PASSWORD") or None
    sender_email = os.getenv("SMTP_FROM", "")
    use_tls = os.getenv("SMTP_USE_TLS", "true").lower() != "false"
    return SmtpEmailSender(
        host=host,
        port=port,
        username=username,
        password=password,
        sender_email=sender_email,
        use_tls=use_tls,
    )


if __name__ == "__main__":
    main()
