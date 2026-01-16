from dataclasses import dataclass
from enum import Enum


class InteractionType(str, Enum):
    VIEW = "view"
    DOWNLOAD = "download"


@dataclass(frozen=True)
class Interaction:
    id: str
    content_id: str
    user_id: str
    interaction_type: InteractionType
    occurred_at: int


@dataclass(frozen=True)
class EngagementSummary:
    views: int
    downloads: int
    unique_users: int
