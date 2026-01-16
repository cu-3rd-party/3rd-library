package main

import (
    "context"
    "log"
    "net"
    "os"

    "google.golang.org/grpc"
)

const defaultAddr = ":50051"

func main() {
    addr := envOrDefault("AUTH_GRPC_ADDR", defaultAddr)

    lis, err := net.Listen("tcp", addr)
    if err != nil {
        log.Fatalf("failed to listen: %v", err)
    }

    server := grpc.NewServer()
    // TODO: register generated gRPC service implementations.

    log.Printf("auth service listening on %s", addr)
    if err := server.Serve(lis); err != nil {
        log.Fatalf("server failed: %v", err)
    }
}

func envOrDefault(key, fallback string) string {
    if value, ok := os.LookupEnv(key); ok {
        return value
    }
    return fallback
}

// placeholder to avoid unused import once service scaffolding expands
var _ = context.Background
