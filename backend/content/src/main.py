import os
from concurrent import futures

import grpc

from application import ContentService
from infrastructure import InMemoryContentRepository
from proto import content_pb2_grpc
from transport import ContentGrpcService


def main() -> None:
    storage_dir = os.getenv("CONTENT_STORAGE_DIR", "/tmp/3rd-library/content")
    repository = InMemoryContentRepository(storage_dir)
    service = ContentService(repository)

    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    content_pb2_grpc.add_ContentServiceServicer_to_server(ContentGrpcService(service), server)

    addr = os.getenv("CONTENT_GRPC_ADDR", "0.0.0.0:50055")
    server.add_insecure_port(addr)
    server.start()
    print(f"content service listening on {addr}")
    server.wait_for_termination()


if __name__ == "__main__":
    main()
