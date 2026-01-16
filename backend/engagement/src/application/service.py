from __future__ import annotations

import time
import uuid

from domain.models import EngagementSummary, Interaction, InteractionType
from domain.repository import EngagementRepository


class EngagementService:
    def __init__(self, repository: EngagementRepository) -> None:
        self._repository = repository

    def record(self, content_id: str, user_id: str, interaction_type: InteractionType, occurred_at: int) -> str:
        content_id = content_id.strip()
        user_id = user_id.strip()
        if not content_id or not user_id:
            raise ValueError("content_id and user_id required")

        if occurred_at <= 0:
            occurred_at = int(time.time())

        interaction = Interaction(
            id=uuid.uuid4().hex,
            content_id=content_id,
            user_id=user_id,
            interaction_type=interaction_type,
            occurred_at=occurred_at,
        )
        self._repository.record(interaction)
        return interaction.id

    def summary(self, content_id: str) -> EngagementSummary:
        content_id = content_id.strip()
        if not content_id:
            raise ValueError("content_id required")
        return self._repository.summary(content_id)
