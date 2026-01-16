import os
from concurrent import futures

import grpc

from application import StatisticsService
from infrastructure import ClickHouseStatisticsRepository, InMemoryStatisticsRepository
from proto import statistics_pb2_grpc
from transport import StatisticsGrpcService


def main() -> None:
    repository = _build_repository()
    service = StatisticsService(repository)

    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    statistics_pb2_grpc.add_StatisticsServiceServicer_to_server(StatisticsGrpcService(service), server)

    addr = os.getenv("STATISTICS_GRPC_ADDR", "0.0.0.0:50052")
    server.add_insecure_port(addr)
    server.start()
    print(f"statistics service listening on {addr}")
    server.wait_for_termination()


def _build_repository():
    repository_kind = os.getenv("STATISTICS_REPOSITORY", "").strip().lower()
    if repository_kind in {"memory", "in_memory", "inmemory"}:
        return InMemoryStatisticsRepository()
    dsn = os.getenv("CLICKHOUSE_DSN")
    if dsn:
        table = os.getenv("CLICKHOUSE_TABLE", "content_interactions")
        return ClickHouseStatisticsRepository(dsn=dsn, table=table)
    print("CLICKHOUSE_DSN not set, falling back to in-memory statistics repository.")
    return InMemoryStatisticsRepository()


if __name__ == "__main__":
    main()
