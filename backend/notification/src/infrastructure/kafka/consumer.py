from __future__ import annotations

import json
import logging
from typing import Any

from kafka import KafkaConsumer

from application.dto import NotificationIntent
from application.service import NotificationService

logger = logging.getLogger(__name__)


class NotificationKafkaConsumer:
    def __init__(
        self,
        service: NotificationService,
        brokers: str,
        topic: str,
        group_id: str,
        auto_offset_reset: str = "earliest",
    ) -> None:
        self._service = service
        self._brokers = brokers
        self._topic = topic
        self._group_id = group_id
        self._auto_offset_reset = auto_offset_reset

    def run(self) -> None:
        consumer = KafkaConsumer(
            self._topic,
            bootstrap_servers=self._brokers.split(","),
            group_id=self._group_id,
            enable_auto_commit=True,
            auto_offset_reset=self._auto_offset_reset,
            value_deserializer=lambda v: v.decode("utf-8"),
        )
        logger.info("kafka consumer started topic=%s group=%s", self._topic, self._group_id)
        for message in consumer:
            payload = self._decode_message(message.value)
            if payload is None:
                continue
            try:
                intent = NotificationIntent.from_payload(payload)
                self._service.handle_intent(intent)
            except Exception as exc:
                logger.exception("failed to process message: %s", exc)

    @staticmethod
    def _decode_message(raw_value: str) -> dict[str, Any] | None:
        try:
            return json.loads(raw_value)
        except json.JSONDecodeError:
            logger.exception("invalid json payload")
            return None
