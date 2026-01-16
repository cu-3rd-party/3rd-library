from pathlib import Path
import sys

import pytest

ROOT = Path(__file__).resolve().parents[1]
sys.path.append(str(ROOT / "src"))

from application.service import StatisticsService
from domain.models import InteractionType
from infrastructure.repository import InMemoryStatisticsRepository


def test_record_and_stats_roundtrip():
    repo = InMemoryStatisticsRepository()
    service = StatisticsService(repo)

    service.record("content", "user-1", InteractionType.VIEW, occurred_at=100)
    service.record("content", "user-2", InteractionType.DOWNLOAD, occurred_at=200)
    service.record("content", "user-1", InteractionType.VIEW, occurred_at=150)

    stats = service.stats_for("content")
    assert stats.views == 2
    assert stats.downloads == 1
    assert stats.unique_users == 2
    assert stats.last_interaction_at == 200


def test_record_validation_errors():
    repo = InMemoryStatisticsRepository()
    service = StatisticsService(repo)

    with pytest.raises(ValueError):
        service.record("", "user", InteractionType.VIEW, occurred_at=1)
    with pytest.raises(ValueError):
        service.record("content", " ", InteractionType.DOWNLOAD, occurred_at=1)
    with pytest.raises(ValueError):
        service.stats_for(" ")
