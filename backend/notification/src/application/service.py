import logging
from typing import Protocol

from application.dto import NotificationIntent
from domain.models import Channel, DeliveryStatus, Notification
from domain.repository import NotificationRepository

logger = logging.getLogger(__name__)


class NotificationSender(Protocol):
    def send(self, notification: Notification) -> None:
        raise NotImplementedError


class NotificationService:
    def __init__(
        self,
        repository: NotificationRepository,
        popup_sender: NotificationSender,
        email_sender: NotificationSender,
    ) -> None:
        self._repository = repository
        self._senders: dict[Channel, NotificationSender] = {
            Channel.POPUP: popup_sender,
            Channel.EMAIL: email_sender,
        }

    def handle_intent(self, intent: NotificationIntent) -> None:
        notification = Notification(
            notification_id=intent.notification_id,
            recipient_id=intent.recipient_id,
            channels=intent.channels,
            template=intent.template,
            data=intent.data,
            created_at=intent.created_at,
        )
        self._repository.save(notification)
        for channel in notification.channels:
            sender = self._senders.get(channel)
            if not sender:
                logger.warning("no sender for channel %s", channel.value)
                self._repository.record_delivery(
                    notification.notification_id,
                    channel,
                    DeliveryStatus.FAILED,
                    detail="missing sender",
                )
                continue
            try:
                sender.send(notification)
            except Exception as exc:
                logger.exception("delivery failed for %s via %s", notification.notification_id, channel.value)
                self._repository.record_delivery(
                    notification.notification_id,
                    channel,
                    DeliveryStatus.FAILED,
                    detail=str(exc),
                )
            else:
                self._repository.record_delivery(
                    notification.notification_id,
                    channel,
                    DeliveryStatus.SENT,
                )
