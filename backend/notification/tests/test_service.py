from pathlib import Path
import sys

import pytest

ROOT = Path(__file__).resolve().parents[1]
sys.path.append(str(ROOT / "src"))

from application.dto import NotificationIntent
from application.service import NotificationService
from domain.models import Channel, DeliveryStatus
from infrastructure.repository import InMemoryNotificationRepository


class RecordingSender:
    def __init__(self, fail: bool = False) -> None:
        self._fail = fail
        self.sent = []

    def send(self, notification):
        self.sent.append(notification)
        if self._fail:
            raise RuntimeError("boom")


def test_handle_intent_records_delivery():
    repo = InMemoryNotificationRepository()
    popup_sender = RecordingSender(fail=True)
    email_sender = RecordingSender()
    service = NotificationService(repo, popup_sender, email_sender)

    intent = NotificationIntent(
        notification_id="notif-1",
        recipient_id="user-1",
        channels=(Channel.POPUP, Channel.EMAIL),
        template="welcome",
        data={"name": "Sam"},
        created_at=123,
    )

    service.handle_intent(intent)

    stored = repo.get("notif-1")
    assert stored is not None
    assert stored.deliveries[Channel.POPUP] == DeliveryStatus.FAILED
    assert stored.deliveries[Channel.EMAIL] == DeliveryStatus.SENT
    assert len(popup_sender.sent) == 1
    assert len(email_sender.sent) == 1


def test_intent_validation_errors():
    with pytest.raises(ValueError):
        NotificationIntent.from_payload({"notification_id": "", "recipient_id": ""})
    with pytest.raises(ValueError):
        NotificationIntent.from_payload({"notification_id": "1", "recipient_id": "2", "channels": []})
    with pytest.raises(ValueError):
        NotificationIntent.from_payload({"notification_id": "1", "recipient_id": "2", "channels": ["sms"]})
