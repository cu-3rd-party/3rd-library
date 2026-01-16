from __future__ import annotations

import threading

from domain.models import EngagementSummary, Interaction, InteractionType
from domain.repository import EngagementRepository


class InMemoryEngagementRepository(EngagementRepository):
    def __init__(self) -> None:
        self._lock = threading.Lock()
        self._views: dict[str, int] = {}
        self._downloads: dict[str, int] = {}
        self._users: dict[str, set[str]] = {}

    def record(self, interaction: Interaction) -> None:
        with self._lock:
            if interaction.interaction_type == InteractionType.VIEW:
                self._views[interaction.content_id] = self._views.get(interaction.content_id, 0) + 1
            elif interaction.interaction_type == InteractionType.DOWNLOAD:
                self._downloads[interaction.content_id] = self._downloads.get(interaction.content_id, 0) + 1
            self._users.setdefault(interaction.content_id, set()).add(interaction.user_id)

    def summary(self, content_id: str) -> EngagementSummary:
        with self._lock:
            views = self._views.get(content_id, 0)
            downloads = self._downloads.get(content_id, 0)
            unique_users = len(self._users.get(content_id, set()))
        return EngagementSummary(views=views, downloads=downloads, unique_users=unique_users)
