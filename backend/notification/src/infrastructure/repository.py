from __future__ import annotations

import copy
import time
from typing import Any

from pymongo import MongoClient

from domain.models import Channel, DeliveryStatus, Notification
from domain.repository import NotificationRepository


class InMemoryNotificationRepository(NotificationRepository):
    def __init__(self) -> None:
        self._store: dict[str, Notification] = {}
        self._deliveries: dict[str, dict[Channel, DeliveryStatus]] = {}
        self._delivery_details: dict[str, dict[Channel, str | None]] = {}

    def save(self, notification: Notification) -> None:
        self._store[notification.notification_id] = notification
        self._deliveries[notification.notification_id] = {
            channel: DeliveryStatus.PENDING for channel in notification.channels
        }
        self._delivery_details[notification.notification_id] = {}

    def record_delivery(
        self,
        notification_id: str,
        channel: Channel,
        status: DeliveryStatus,
        detail: str | None = None,
    ) -> None:
        self._deliveries.setdefault(notification_id, {})[channel] = status
        if detail:
            self._delivery_details.setdefault(notification_id, {})[channel] = detail

    def get(self, notification_id: str) -> Notification | None:
        notification = self._store.get(notification_id)
        if not notification:
            return None
        deliveries = self._deliveries.get(notification_id, {})
        return Notification(
            notification_id=notification.notification_id,
            recipient_id=notification.recipient_id,
            channels=notification.channels,
            template=notification.template,
            data=copy.deepcopy(notification.data),
            created_at=notification.created_at,
            deliveries=deliveries.copy(),
        )


class MongoNotificationRepository(NotificationRepository):
    def __init__(
        self,
        dsn: str,
        database: str = "notification",
        collection: str = "notifications",
    ) -> None:
        self._client = MongoClient(dsn)
        self._collection = self._client[database][collection]

    def save(self, notification: Notification) -> None:
        document = {
            "_id": notification.notification_id,
            "recipient_id": notification.recipient_id,
            "channels": [channel.value for channel in notification.channels],
            "template": notification.template,
            "data": notification.data,
            "created_at": notification.created_at,
            "deliveries": {channel.value: DeliveryStatus.PENDING.value for channel in notification.channels},
            "delivery_details": {},
            "updated_at": int(time.time()),
        }
        self._collection.update_one({"_id": notification.notification_id}, {"$set": document}, upsert=True)

    def record_delivery(
        self,
        notification_id: str,
        channel: Channel,
        status: DeliveryStatus,
        detail: str | None = None,
    ) -> None:
        update: dict[str, Any] = {
            f"deliveries.{channel.value}": status.value,
            "updated_at": int(time.time()),
        }
        if detail:
            update[f"delivery_details.{channel.value}"] = detail
        self._collection.update_one({"_id": notification_id}, {"$set": update}, upsert=True)

    def get(self, notification_id: str) -> Notification | None:
        document = self._collection.find_one({"_id": notification_id})
        if not document:
            return None
        channels = tuple(Channel(channel) for channel in document.get("channels", []))
        deliveries_raw = document.get("deliveries", {})
        deliveries = {Channel(key): DeliveryStatus(value) for key, value in deliveries_raw.items()}
        return Notification(
            notification_id=document["_id"],
            recipient_id=document.get("recipient_id", ""),
            channels=channels,
            template=document.get("template", ""),
            data=document.get("data", {}) or {},
            created_at=int(document.get("created_at", 0)),
            deliveries=deliveries,
        )
