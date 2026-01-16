from __future__ import annotations

import time
import uuid

from domain.models import ContentItem
from domain.repository import ContentRepository


class ContentService:
    def __init__(self, repository: ContentRepository) -> None:
        self._repository = repository

    def upload(self, owner_id: str, title: str, description: str, filename: str, file_bytes: bytes) -> ContentItem:
        owner_id = owner_id.strip()
        title = title.strip()
        description = description.strip()
        filename = filename.strip()

        if not owner_id or not title or not filename or not file_bytes:
            raise ValueError("missing required fields")

        item = ContentItem(
            id=uuid.uuid4().hex,
            owner_id=owner_id,
            title=title,
            description=description,
            filename=filename,
            size_bytes=len(file_bytes),
            created_at=int(time.time()),
        )
        return self._repository.save(item, file_bytes)

    def get(self, content_id: str) -> tuple[ContentItem, bytes]:
        content_id = content_id.strip()
        if not content_id:
            raise ValueError("content_id required")
        return self._repository.get(content_id)

    def list_items(self, page_size: int, page_token: str) -> tuple[list[ContentItem], str]:
        if page_size <= 0:
            page_size = 20
        if page_size > 100:
            page_size = 100

        offset = 0
        if page_token:
            try:
                offset = int(page_token)
            except ValueError as exc:
                raise ValueError("invalid page_token") from exc

        items, next_offset = self._repository.list_items(offset, page_size)
        next_token = str(next_offset) if next_offset is not None else ""
        return list(items), next_token
