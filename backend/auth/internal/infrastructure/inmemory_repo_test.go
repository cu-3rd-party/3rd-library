package infrastructure

import (
	"context"
	"testing"
	"time"

	"github.com/cu-3rd-party/3rd-library/auth/internal/domain"
)

func TestInMemoryUserRepository(t *testing.T) {
	repo := NewInMemoryUserRepository()
	user := domain.User{ID: "user-1", Name: "Jane", Email: "jane@example.com", PasswordHash: "hash", CreatedAt: time.Now()}

	if err := repo.Create(context.Background(), user); err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if err := repo.Create(context.Background(), user); err != domain.ErrEmailTaken {
		t.Fatalf("expected email taken, got %v", err)
	}

	got, err := repo.GetByEmail(context.Background(), "jane@example.com")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if got.ID != user.ID {
		t.Fatalf("expected user %q, got %q", user.ID, got.ID)
	}

	if _, err := repo.GetByID(context.Background(), "missing"); err != domain.ErrUserNotFound {
		t.Fatalf("expected not found, got %v", err)
	}
}
