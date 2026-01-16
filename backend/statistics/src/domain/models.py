from dataclasses import dataclass
from enum import Enum


class InteractionType(str, Enum):
    VIEW = "view"
    DOWNLOAD = "download"


@dataclass(frozen=True)
class ContentStats:
    views: int
    downloads: int
    unique_users: int
    last_interaction_at: int
