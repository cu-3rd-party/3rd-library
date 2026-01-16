from __future__ import annotations

from abc import ABC, abstractmethod

from .models import ContentStats, InteractionType


class StatisticsRepository(ABC):
    @abstractmethod
    def record(self, content_id: str, user_id: str, interaction_type: InteractionType, occurred_at: int) -> str:
        raise NotImplementedError

    @abstractmethod
    def stats_for(self, content_id: str) -> ContentStats:
        raise NotImplementedError
