import threading
import uuid
from datetime import datetime
from urllib.parse import parse_qs, urlparse

from clickhouse_driver import Client

from domain.models import ContentStats, InteractionType
from domain.repository import StatisticsRepository


class InMemoryStatisticsRepository(StatisticsRepository):
    def __init__(self) -> None:
        self._lock = threading.Lock()
        self._views: dict[str, int] = {}
        self._downloads: dict[str, int] = {}
        self._users: dict[str, set[str]] = {}
        self._last_seen: dict[str, int] = {}

    def record(self, content_id: str, user_id: str, interaction_type: InteractionType, occurred_at: int) -> str:
        with self._lock:
            if interaction_type == InteractionType.VIEW:
                self._views[content_id] = self._views.get(content_id, 0) + 1
            elif interaction_type == InteractionType.DOWNLOAD:
                self._downloads[content_id] = self._downloads.get(content_id, 0) + 1
            self._users.setdefault(content_id, set()).add(user_id)
            self._last_seen[content_id] = max(self._last_seen.get(content_id, 0), occurred_at)
        return uuid.uuid4().hex

    def stats_for(self, content_id: str) -> ContentStats:
        with self._lock:
            views = self._views.get(content_id, 0)
            downloads = self._downloads.get(content_id, 0)
            unique_users = len(self._users.get(content_id, set()))
            last_interaction_at = self._last_seen.get(content_id, 0)
        return ContentStats(
            views=views,
            downloads=downloads,
            unique_users=unique_users,
            last_interaction_at=last_interaction_at,
        )


class ClickHouseStatisticsRepository(StatisticsRepository):
    def __init__(self, dsn: str, table: str = "content_interactions") -> None:
        self._client = _client_from_dsn(dsn)
        self._table = table
        self._ensure_schema()

    def record(self, content_id: str, user_id: str, interaction_type: InteractionType, occurred_at: int) -> str:
        recorded_id = uuid.uuid4()
        timestamp = datetime.utcfromtimestamp(occurred_at)
        self._client.execute(
            f"""
            INSERT INTO {self._table}
                (content_id, user_id, interaction_type, occurred_at, recorded_id)
            VALUES
            """,
            [(content_id, user_id, interaction_type.value, timestamp, recorded_id)],
        )
        return recorded_id.hex

    def stats_for(self, content_id: str) -> ContentStats:
        rows = self._client.execute(
            f"""
            SELECT
                countIf(interaction_type = 'view') AS views,
                countIf(interaction_type = 'download') AS downloads,
                uniqExact(user_id) AS unique_users,
                toUnixTimestamp(max(occurred_at)) AS last_interaction_at
            FROM {self._table}
            WHERE content_id = %(content_id)s
            """,
            {"content_id": content_id},
        )
        if rows:
            views, downloads, unique_users, last_interaction_at = rows[0]
        else:
            views = downloads = unique_users = last_interaction_at = 0
        return ContentStats(
            views=int(views),
            downloads=int(downloads),
            unique_users=int(unique_users),
            last_interaction_at=int(last_interaction_at or 0),
        )

    def _ensure_schema(self) -> None:
        self._client.execute(
            f"""
            CREATE TABLE IF NOT EXISTS {self._table}
            (
                content_id String,
                user_id String,
                interaction_type LowCardinality(String),
                occurred_at DateTime,
                recorded_id UUID
            )
            ENGINE = MergeTree
            ORDER BY (content_id, occurred_at)
            """
        )


def _client_from_dsn(dsn: str) -> Client:
    parsed = urlparse(dsn)
    host = parsed.hostname or "localhost"
    port = parsed.port or 9000
    user = parsed.username or "default"
    password = parsed.password or ""
    database = parsed.path.lstrip("/") or "default"
    query = parse_qs(parsed.query)
    secure = query.get("secure", ["false"])[0].lower() in {"1", "true", "yes"}
    settings: dict[str, object] = {}
    for key, value in query.items():
        if key in {"secure"}:
            continue
        if len(value) == 1:
            settings[key] = value[0]
        else:
            settings[key] = value
    return Client(
        host=host,
        port=port,
        user=user,
        password=password,
        database=database,
        secure=secure,
        settings=settings or None,
    )
