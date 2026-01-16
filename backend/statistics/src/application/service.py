from __future__ import annotations

import time

from domain.models import ContentStats, InteractionType
from domain.repository import StatisticsRepository


class StatisticsService:
    def __init__(self, repository: StatisticsRepository) -> None:
        self._repository = repository

    def record(self, content_id: str, user_id: str, interaction_type: InteractionType, occurred_at: int) -> str:
        content_id = content_id.strip()
        user_id = user_id.strip()
        if not content_id or not user_id:
            raise ValueError("content_id and user_id required")
        if occurred_at <= 0:
            occurred_at = int(time.time())
        return self._repository.record(content_id, user_id, interaction_type, occurred_at)

    def stats_for(self, content_id: str) -> ContentStats:
        content_id = content_id.strip()
        if not content_id:
            raise ValueError("content_id required")
        return self._repository.stats_for(content_id)
