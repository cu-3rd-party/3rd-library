import time
import uuid

from domain.models import Comment, EngagementSummary
from domain.repository import EngagementRepository


class EngagementService:
    def __init__(self, repository: EngagementRepository) -> None:
        self._repository = repository

    def add_comment(self, content_id: str, user_id: str, body: str, created_at: int) -> Comment:
        content_id = content_id.strip()
        user_id = user_id.strip()
        body = body.strip()
        if not content_id or not user_id or not body:
            raise ValueError("content_id, user_id, and body required")

        if created_at <= 0:
            created_at = int(time.time())

        comment = Comment(
            id=uuid.uuid4().hex,
            content_id=content_id,
            user_id=user_id,
            body=body,
            created_at=created_at,
        )
        self._repository.add_comment(comment)
        return comment

    def list_comments(self, content_id: str) -> list[Comment]:
        content_id = content_id.strip()
        if not content_id:
            raise ValueError("content_id required")
        return self._repository.list_comments(content_id)

    def set_like(self, content_id: str, user_id: str, liked: bool) -> int:
        content_id = content_id.strip()
        user_id = user_id.strip()
        if not content_id or not user_id:
            raise ValueError("content_id and user_id required")
        return self._repository.set_like(content_id, user_id, liked)

    def summary(self, content_id: str, user_id: str | None) -> EngagementSummary:
        content_id = content_id.strip()
        if not content_id:
            raise ValueError("content_id required")
        user_id_value = user_id.strip() if user_id else None
        if user_id_value == "":
            user_id_value = None
        return self._repository.summary(content_id, user_id_value)
