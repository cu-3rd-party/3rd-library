import os
from concurrent import futures

import grpc


def main() -> None:
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    # TODO: register generated gRPC servicers.

    addr = os.getenv("STATISTICS_GRPC_ADDR", "0.0.0.0:50052")
    server.add_insecure_port(addr)
    server.start()
    print(f"statistics service listening on {addr}")
    server.wait_for_termination()


if __name__ == "__main__":
    main()
