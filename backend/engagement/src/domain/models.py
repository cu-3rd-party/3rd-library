from dataclasses import dataclass


@dataclass(frozen=True)
class EngagementSummary:
    likes: int
    comments: int
    liked_by_user: bool


@dataclass(frozen=True)
class Comment:
    id: str
    content_id: str
    user_id: str
    body: str
    created_at: int
