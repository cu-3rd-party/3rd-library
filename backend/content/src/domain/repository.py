from __future__ import annotations

from abc import ABC, abstractmethod
from typing import Iterable, Tuple

from .models import ContentItem


class ContentRepository(ABC):
    @abstractmethod
    def save(self, item: ContentItem, file_bytes: bytes) -> ContentItem:
        raise NotImplementedError

    @abstractmethod
    def get(self, content_id: str) -> Tuple[ContentItem, bytes]:
        raise NotImplementedError

    @abstractmethod
    def list_items(self, offset: int, limit: int) -> Tuple[Iterable[ContentItem], int | None]:
        raise NotImplementedError
