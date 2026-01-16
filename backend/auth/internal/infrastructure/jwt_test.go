package infrastructure

import (
	"testing"
	"time"

	"github.com/igor/3rd-library/auth/internal/domain"
)

func TestJWTManagerIssueAndParse(t *testing.T) {
	mgr := NewJWTManager("secret", time.Minute)
	user := domain.User{ID: "user-1", Email: "user@example.com"}

	token, err := mgr.Issue(user)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if token == "" {
		t.Fatalf("expected token")
	}

	userID, err := mgr.Parse(token)
	if err != nil {
		t.Fatalf("unexpected parse error: %v", err)
	}
	if userID != "user-1" {
		t.Fatalf("expected user id, got %q", userID)
	}
}

func TestJWTManagerParseInvalid(t *testing.T) {
	mgr := NewJWTManager("secret", time.Minute)

	if _, err := mgr.Parse("not-a-token"); err == nil {
		t.Fatalf("expected error")
	}
}
