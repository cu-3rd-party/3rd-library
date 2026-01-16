from pathlib import Path
import sys

import pytest

ROOT = Path(__file__).resolve().parents[1]
sys.path.append(str(ROOT / "src"))

from application.service import EngagementService
from infrastructure.repository import InMemoryEngagementRepository


def test_add_comment_and_list():
    repo = InMemoryEngagementRepository()
    service = EngagementService(repo)

    comment = service.add_comment("content", "user", "hello", created_at=0)
    assert comment.id
    assert comment.created_at > 0

    comments = service.list_comments("content")
    assert comments == [comment]


def test_set_like_and_summary():
    repo = InMemoryEngagementRepository()
    service = EngagementService(repo)

    count = service.set_like("content", "user", True)
    assert count == 1

    summary = service.summary("content", "user")
    assert summary.likes == 1
    assert summary.comments == 0
    assert summary.liked_by_user is True

    count = service.set_like("content", "user", False)
    assert count == 0
    summary = service.summary("content", "user")
    assert summary.liked_by_user is False


def test_validation_errors():
    repo = InMemoryEngagementRepository()
    service = EngagementService(repo)

    with pytest.raises(ValueError):
        service.add_comment("", "user", "body", created_at=1)
    with pytest.raises(ValueError):
        service.list_comments(" ")
    with pytest.raises(ValueError):
        service.set_like("", "user", True)
    with pytest.raises(ValueError):
        service.summary(" ", None)
