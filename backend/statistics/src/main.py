import os
from concurrent import futures

import grpc

from application import StatisticsService
from infrastructure import InMemoryStatisticsRepository
from proto import statistics_pb2_grpc
from transport import StatisticsGrpcService


def main() -> None:
    repository = InMemoryStatisticsRepository()
    service = StatisticsService(repository)

    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    statistics_pb2_grpc.add_StatisticsServiceServicer_to_server(StatisticsGrpcService(service), server)

    addr = os.getenv("STATISTICS_GRPC_ADDR", "0.0.0.0:50052")
    server.add_insecure_port(addr)
    server.start()
    print(f"statistics service listening on {addr}")
    server.wait_for_termination()


if __name__ == "__main__":
    main()
