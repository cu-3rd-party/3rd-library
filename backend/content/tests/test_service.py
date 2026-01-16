from pathlib import Path
import sys

import pytest

ROOT = Path(__file__).resolve().parents[1]
sys.path.append(str(ROOT / "src"))

from application.service import ContentService
from infrastructure.repository import InMemoryContentRepository


def test_upload_and_get_roundtrip(tmp_path):
    repo = InMemoryContentRepository(str(tmp_path))
    service = ContentService(repo)

    item = service.upload("owner", "Title", "Desc", "file.txt", b"hello")

    assert item.id
    assert item.size_bytes == 5
    stored_item, data = service.get(item.id)
    assert stored_item == item
    assert data == b"hello"


def test_upload_validations(tmp_path):
    repo = InMemoryContentRepository(str(tmp_path))
    service = ContentService(repo)

    with pytest.raises(ValueError):
        service.upload("", "Title", "Desc", "file.txt", b"data")
    with pytest.raises(ValueError):
        service.upload("owner", " ", "Desc", "file.txt", b"data")
    with pytest.raises(ValueError):
        service.upload("owner", "Title", "Desc", " ", b"data")
    with pytest.raises(ValueError):
        service.upload("owner", "Title", "Desc", "file.txt", b"")


def test_list_items_pagination(tmp_path):
    repo = InMemoryContentRepository(str(tmp_path))
    service = ContentService(repo)

    first = service.upload("owner", "One", "Desc", "a.txt", b"1")
    second = service.upload("owner", "Two", "Desc", "b.txt", b"2")

    items, next_token = service.list_items(1, "")
    assert items == [first]
    assert next_token == "1"

    items, next_token = service.list_items(1, next_token)
    assert items == [second]
    assert next_token == ""

    items, next_token = service.list_items(0, "")
    assert len(items) == 2
    assert next_token == ""
