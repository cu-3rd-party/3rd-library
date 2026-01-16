from __future__ import annotations

import threading
import uuid

from domain.models import ContentStats, InteractionType
from domain.repository import StatisticsRepository


class InMemoryStatisticsRepository(StatisticsRepository):
    def __init__(self) -> None:
        self._lock = threading.Lock()
        self._views: dict[str, int] = {}
        self._downloads: dict[str, int] = {}
        self._users: dict[str, set[str]] = {}
        self._last_seen: dict[str, int] = {}

    def record(self, content_id: str, user_id: str, interaction_type: InteractionType, occurred_at: int) -> str:
        with self._lock:
            if interaction_type == InteractionType.VIEW:
                self._views[content_id] = self._views.get(content_id, 0) + 1
            elif interaction_type == InteractionType.DOWNLOAD:
                self._downloads[content_id] = self._downloads.get(content_id, 0) + 1
            self._users.setdefault(content_id, set()).add(user_id)
            self._last_seen[content_id] = max(self._last_seen.get(content_id, 0), occurred_at)
        return uuid.uuid4().hex

    def stats_for(self, content_id: str) -> ContentStats:
        with self._lock:
            views = self._views.get(content_id, 0)
            downloads = self._downloads.get(content_id, 0)
            unique_users = len(self._users.get(content_id, set()))
            last_interaction_at = self._last_seen.get(content_id, 0)
        return ContentStats(
            views=views,
            downloads=downloads,
            unique_users=unique_users,
            last_interaction_at=last_interaction_at,
        )
