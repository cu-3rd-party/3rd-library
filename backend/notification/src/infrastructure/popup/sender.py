from __future__ import annotations

import logging

from domain.models import Notification

logger = logging.getLogger(__name__)


class PopupSender:
    def send(self, notification: Notification) -> None:
        logger.info(
            "popup notification queued id=%s recipient=%s",
            notification.notification_id,
            notification.recipient_id,
        )
