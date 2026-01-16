import threading

from domain.models import Comment, EngagementSummary
from domain.repository import EngagementRepository


class InMemoryEngagementRepository(EngagementRepository):
    def __init__(self) -> None:
        self._lock = threading.Lock()
        self._comments: dict[str, list[Comment]] = {}
        self._likes: dict[str, set[str]] = {}

    def add_comment(self, comment: Comment) -> None:
        with self._lock:
            self._comments.setdefault(comment.content_id, []).append(comment)

    def list_comments(self, content_id: str) -> list[Comment]:
        with self._lock:
            return list(self._comments.get(content_id, []))

    def set_like(self, content_id: str, user_id: str, liked: bool) -> int:
        with self._lock:
            likes = self._likes.setdefault(content_id, set())
            if liked:
                likes.add(user_id)
            else:
                likes.discard(user_id)
            return len(likes)

    def summary(self, content_id: str, user_id: str | None) -> EngagementSummary:
        with self._lock:
            likes = self._likes.get(content_id, set())
            comments = len(self._comments.get(content_id, []))
            liked_by_user = user_id in likes if user_id else False
            likes_count = len(likes)
        return EngagementSummary(likes=likes_count, comments=comments, liked_by_user=liked_by_user)
