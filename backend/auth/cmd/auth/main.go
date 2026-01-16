package main

import (
	"context"
	"crypto/rand"
	"encoding/hex"
	"log"
	"net"
	"os"
	"time"

	"github.com/cu-3rd-party/3rd-library/auth/internal/application"
	authpb "github.com/cu-3rd-party/3rd-library/auth/internal/gen"
	"github.com/cu-3rd-party/3rd-library/auth/internal/infrastructure"
	transportgrpc "github.com/cu-3rd-party/3rd-library/auth/internal/transport/grpc"
	"google.golang.org/grpc"
)

const defaultAddr = ":50051"

func main() {
	addr := envOrDefault("AUTH_GRPC_ADDR", defaultAddr)
	jwtSecret := envOrDefault("JWT_SECRET", "dev-secret")
	jwtTTL := envOrDefault("JWT_TTL", "24h")

	lis, err := net.Listen("tcp", addr)
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	ttl, err := time.ParseDuration(jwtTTL)
	if err != nil {
		log.Fatalf("invalid JWT_TTL: %v", err)
	}

	repo := infrastructure.NewInMemoryUserRepository()
	tokenManager := infrastructure.NewJWTManager(jwtSecret, ttl)
	svc := application.NewAuthService(repo, tokenManager, time.Now, newID)

	server := grpc.NewServer()
	authpb.RegisterAuthServiceServer(server, transportgrpc.NewAuthHandler(svc))

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

func newID() string {
	buf := make([]byte, 16)
	if _, err := rand.Read(buf); err != nil {
		return time.Now().UTC().Format("20060102150405.000000000")
	}
	return hex.EncodeToString(buf)
}
