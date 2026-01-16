from abc import ABC, abstractmethod

from .models import Comment, EngagementSummary


class EngagementRepository(ABC):
    @abstractmethod
    def add_comment(self, comment: Comment) -> None:
        raise NotImplementedError

    @abstractmethod
    def list_comments(self, content_id: str) -> list[Comment]:
        raise NotImplementedError

    @abstractmethod
    def set_like(self, content_id: str, user_id: str, liked: bool) -> int:
        raise NotImplementedError

    @abstractmethod
    def summary(self, content_id: str, user_id: str | None) -> EngagementSummary:
        raise NotImplementedError
