import os
from concurrent import futures

import grpc

from application import EngagementService
from infrastructure import InMemoryEngagementRepository
from proto import engagement_pb2_grpc
from transport import EngagementGrpcService


def main() -> None:
    repository = InMemoryEngagementRepository()
    service = EngagementService(repository)

    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    engagement_pb2_grpc.add_EngagementServiceServicer_to_server(EngagementGrpcService(service), server)

    addr = os.getenv("ENGAGEMENT_GRPC_ADDR", "0.0.0.0:50054")
    server.add_insecure_port(addr)
    server.start()
    print(f"engagement service listening on {addr}")
    server.wait_for_termination()


if __name__ == "__main__":
    main()
