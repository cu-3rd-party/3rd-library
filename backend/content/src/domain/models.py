from dataclasses import dataclass


@dataclass(frozen=True)
class ContentItem:
    id: str
    owner_id: str
    title: str
    description: str
    filename: str
    size_bytes: int
    created_at: int
