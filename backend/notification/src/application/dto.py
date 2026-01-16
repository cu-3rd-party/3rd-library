import time
from datetime import datetime, timezone
from dataclasses import dataclass
from typing import Any

from domain.models import Channel


@dataclass(frozen=True)
class NotificationIntent:
    notification_id: str
    recipient_id: str
    channels: tuple[Channel, ...]
    template: str
    data: dict[str, Any]
    created_at: int

    @classmethod
    def from_payload(cls, payload: dict[str, Any]) -> "NotificationIntent":
        notification_id = str(payload.get("notification_id", "")).strip()
        recipient_id = str(payload.get("recipient_id", "")).strip()
        template = str(payload.get("template", "")).strip()
        raw_channels = payload.get("channels", [])
        channels: list[Channel] = []
        for channel in raw_channels or []:
            try:
                channels.append(Channel(str(channel).lower()))
            except ValueError as exc:
                raise ValueError(f"unsupported channel: {channel}") from exc
        data = payload.get("data") or {}
        created_at = _parse_created_at(payload.get("created_at"))
        if not notification_id or not recipient_id:
            raise ValueError("notification_id and recipient_id are required")
        if not channels:
            raise ValueError("at least one channel is required")
        return cls(
            notification_id=notification_id,
            recipient_id=recipient_id,
            channels=tuple(channels),
            template=template,
            data=data,
            created_at=created_at,
        )


def _parse_created_at(value: Any) -> int:
    if value is None or value == "":
        return int(time.time())
    if isinstance(value, (int, float)):
        return int(value)
    if isinstance(value, str):
        cleaned = value.replace("Z", "+00:00")
        try:
            parsed = datetime.fromisoformat(cleaned)
            if parsed.tzinfo is None:
                parsed = parsed.replace(tzinfo=timezone.utc)
            return int(parsed.timestamp())
        except ValueError:
            pass
    return int(time.time())
