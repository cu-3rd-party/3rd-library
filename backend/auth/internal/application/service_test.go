package application

import (
	"context"
	"testing"
	"time"

	"github.com/igor/3rd-library/auth/internal/domain"
	"github.com/igor/3rd-library/auth/internal/infrastructure"
	"golang.org/x/crypto/bcrypt"
)

type stubTokenManager struct {
	issueToken string
	issueErr   error
	parseUser  string
	parseErr   error
}

func (s *stubTokenManager) Issue(_ domain.User) (string, error) {
	return s.issueToken, s.issueErr
}

func (s *stubTokenManager) Parse(_ string) (string, error) {
	return s.parseUser, s.parseErr
}

func TestAuthServiceRegisterSuccess(t *testing.T) {
	repo := infrastructure.NewInMemoryUserRepository()
	token := &stubTokenManager{issueToken: "token"}
	now := time.Date(2024, 1, 1, 0, 0, 0, 0, time.UTC)

	svc := NewAuthService(repo, token, func() time.Time { return now }, func() string { return "user-1" })

	user, issued, err := svc.Register(context.Background(), " Jane ", "JANE@EXAMPLE.COM", "password")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if issued != "token" {
		t.Fatalf("expected token, got %q", issued)
	}
	if user.ID != "user-1" {
		t.Fatalf("expected user id, got %q", user.ID)
	}
	if user.Email != "jane@example.com" {
		t.Fatalf("expected normalized email, got %q", user.Email)
	}
	if user.Name != "Jane" {
		t.Fatalf("expected trimmed name, got %q", user.Name)
	}
	if !user.CreatedAt.Equal(now) {
		t.Fatalf("expected created at %v, got %v", now, user.CreatedAt)
	}
	if err := bcrypt.CompareHashAndPassword([]byte(user.PasswordHash), []byte("password")); err != nil {
		t.Fatalf("expected hashed password to validate: %v", err)
	}
}

func TestAuthServiceRegisterValidation(t *testing.T) {
	repo := infrastructure.NewInMemoryUserRepository()
	svc := NewAuthService(repo, &stubTokenManager{}, time.Now, func() string { return "id" })

	_, _, err := svc.Register(context.Background(), "", "nope", "")
	if err != domain.ErrInvalidInput {
		t.Fatalf("expected invalid input error, got %v", err)
	}
}

func TestAuthServiceRegisterEmailTaken(t *testing.T) {
	repo := infrastructure.NewInMemoryUserRepository()
	existing := domain.User{ID: "user-1", Name: "Jane", Email: "jane@example.com", PasswordHash: "hash", CreatedAt: time.Now()}
	if err := repo.Create(context.Background(), existing); err != nil {
		t.Fatalf("create error: %v", err)
	}

	svc := NewAuthService(repo, &stubTokenManager{}, time.Now, func() string { return "id" })

	_, _, err := svc.Register(context.Background(), "Jane", "jane@example.com", "password")
	if err != domain.ErrEmailTaken {
		t.Fatalf("expected email taken error, got %v", err)
	}
}

func TestAuthServiceLoginSuccess(t *testing.T) {
	repo := infrastructure.NewInMemoryUserRepository()
	hash, err := bcrypt.GenerateFromPassword([]byte("secret"), bcrypt.DefaultCost)
	if err != nil {
		t.Fatalf("hash error: %v", err)
	}
	user := domain.User{ID: "user-1", Name: "Jane", Email: "jane@example.com", PasswordHash: string(hash), CreatedAt: time.Now()}
	if err := repo.Create(context.Background(), user); err != nil {
		t.Fatalf("create error: %v", err)
	}

	svc := NewAuthService(repo, &stubTokenManager{issueToken: "token"}, time.Now, func() string { return "id" })

	gotUser, issued, err := svc.Login(context.Background(), "jane@example.com", "secret")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if issued != "token" {
		t.Fatalf("expected token, got %q", issued)
	}
	if gotUser.ID != user.ID {
		t.Fatalf("expected user %q, got %q", user.ID, gotUser.ID)
	}
}

func TestAuthServiceLoginInvalidCredentials(t *testing.T) {
	repo := infrastructure.NewInMemoryUserRepository()
	hash, err := bcrypt.GenerateFromPassword([]byte("secret"), bcrypt.DefaultCost)
	if err != nil {
		t.Fatalf("hash error: %v", err)
	}
	user := domain.User{ID: "user-1", Name: "Jane", Email: "jane@example.com", PasswordHash: string(hash), CreatedAt: time.Now()}
	if err := repo.Create(context.Background(), user); err != nil {
		t.Fatalf("create error: %v", err)
	}

	svc := NewAuthService(repo, &stubTokenManager{issueToken: "token"}, time.Now, func() string { return "id" })

	_, _, err = svc.Login(context.Background(), "jane@example.com", "wrong")
	if err != domain.ErrInvalidCredentials {
		t.Fatalf("expected invalid credentials, got %v", err)
	}
}

func TestAuthServiceValidate(t *testing.T) {
	repo := infrastructure.NewInMemoryUserRepository()
	user := domain.User{ID: "user-1", Name: "Jane", Email: "jane@example.com", PasswordHash: "hash", CreatedAt: time.Now()}
	if err := repo.Create(context.Background(), user); err != nil {
		t.Fatalf("create error: %v", err)
	}

	svc := NewAuthService(repo, &stubTokenManager{parseUser: "user-1"}, time.Now, func() string { return "id" })

	got, err := svc.Validate(context.Background(), "token")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if got.ID != user.ID {
		t.Fatalf("expected user %q, got %q", user.ID, got.ID)
	}

	_, err = svc.Validate(context.Background(), "")
	if err != domain.ErrTokenInvalidOrEmpty {
		t.Fatalf("expected invalid token error, got %v", err)
	}
}

func TestAuthServiceGetUser(t *testing.T) {
	repo := infrastructure.NewInMemoryUserRepository()
	user := domain.User{ID: "user-1", Name: "Jane", Email: "jane@example.com", PasswordHash: "hash", CreatedAt: time.Now()}
	if err := repo.Create(context.Background(), user); err != nil {
		t.Fatalf("create error: %v", err)
	}

	svc := NewAuthService(repo, &stubTokenManager{}, time.Now, func() string { return "id" })

	got, err := svc.GetUser(context.Background(), "user-1")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if got.ID != user.ID {
		t.Fatalf("expected user %q, got %q", user.ID, got.ID)
	}

	_, err = svc.GetUser(context.Background(), " ")
	if err != domain.ErrInvalidInput {
		t.Fatalf("expected invalid input error, got %v", err)
	}
}
