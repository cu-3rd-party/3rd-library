import os
import threading
from typing import Iterable, Tuple

from domain.models import ContentItem
from domain.repository import ContentRepository


class InMemoryContentRepository(ContentRepository):
    def __init__(self, storage_dir: str) -> None:
        self._storage_dir = storage_dir
        self._items: dict[str, ContentItem] = {}
        self._order: list[str] = []
        self._lock = threading.Lock()
        os.makedirs(storage_dir, exist_ok=True)

    def save(self, item: ContentItem, file_bytes: bytes) -> ContentItem:
        file_path = self._file_path(item.id, item.filename)
        with self._lock:
            with open(file_path, "wb") as handle:
                handle.write(file_bytes)
            self._items[item.id] = item
            self._order.append(item.id)
        return item

    def get(self, content_id: str) -> Tuple[ContentItem, bytes]:
        with self._lock:
            item = self._items.get(content_id)
        if item is None:
            raise KeyError("content not found")
        file_path = self._file_path(item.id, item.filename)
        with open(file_path, "rb") as handle:
            data = handle.read()
        return item, data

    def list_items(self, offset: int, limit: int) -> Tuple[Iterable[ContentItem], int | None]:
        with self._lock:
            ids = list(self._order)
        slice_ids = ids[offset : offset + limit]
        items = [self._items[item_id] for item_id in slice_ids]
        next_offset = offset + limit if offset + limit < len(ids) else None
        return items, next_offset

    def _file_path(self, content_id: str, filename: str) -> str:
        safe_name = filename.replace("/", "_")
        return os.path.join(self._storage_dir, f"{content_id}-{safe_name}")
