from typing import Protocol

from domain.models import Channel, DeliveryStatus, Notification


class NotificationRepository(Protocol):
    def save(self, notification: Notification) -> None:
        raise NotImplementedError

    def record_delivery(
        self,
        notification_id: str,
        channel: Channel,
        status: DeliveryStatus,
        detail: str | None = None,
    ) -> None:
        raise NotImplementedError

    def get(self, notification_id: str) -> Notification | None:
        raise NotImplementedError
