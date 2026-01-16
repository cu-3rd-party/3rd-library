from __future__ import annotations

from abc import ABC, abstractmethod

from .models import EngagementSummary, Interaction


class EngagementRepository(ABC):
    @abstractmethod
    def record(self, interaction: Interaction) -> None:
        raise NotImplementedError

    @abstractmethod
    def summary(self, content_id: str) -> EngagementSummary:
        raise NotImplementedError
