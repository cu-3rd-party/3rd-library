from dataclasses import dataclass, field
from enum import Enum
from typing import Any


class Channel(str, Enum):
    POPUP = "popup"
    EMAIL = "email"


class DeliveryStatus(str, Enum):
    PENDING = "pending"
    SENT = "sent"
    FAILED = "failed"


@dataclass(frozen=True)
class Notification:
    notification_id: str
    recipient_id: str
    channels: tuple[Channel, ...]
    template: str
    data: dict[str, Any]
    created_at: int
    deliveries: dict[Channel, DeliveryStatus] = field(default_factory=dict)
