package application

import (
	"context"
	"strings"
	"time"

	"github.com/cu-3rd-party/3rd-library/auth/internal/domain"
	"golang.org/x/crypto/bcrypt"
)

type TokenManager interface {
	Issue(user domain.User) (string, error)
	Parse(token string) (string, error)
}

type AuthService struct {
	repo  domain.UserRepository
	token TokenManager
	now   func() time.Time
	idGen func() string
}

func NewAuthService(repo domain.UserRepository, token TokenManager, now func() time.Time, idGen func() string) *AuthService {
	return &AuthService{
		repo:  repo,
		token: token,
		now:   now,
		idGen: idGen,
	}
}

func (svc *AuthService) Register(ctx context.Context, name, email, password string) (domain.User, string, error) {
	name = strings.TrimSpace(name)
	email = strings.TrimSpace(strings.ToLower(email))
	if name == "" || email == "" || password == "" || !strings.Contains(email, "@") {
		return domain.User{}, "", domain.ErrInvalidInput
	}

	if _, err := svc.repo.GetByEmail(ctx, email); err == nil {
		return domain.User{}, "", domain.ErrEmailTaken
	}

	hash, err := bcrypt.GenerateFromPassword([]byte(password), bcrypt.DefaultCost)
	if err != nil {
		return domain.User{}, "", err
	}

	user := domain.User{
		ID:           svc.idGen(),
		Name:         name,
		Email:        email,
		PasswordHash: string(hash),
		CreatedAt:    svc.now(),
	}

	if err := svc.repo.Create(ctx, user); err != nil {
		return domain.User{}, "", err
	}

	token, err := svc.token.Issue(user)
	if err != nil {
		return domain.User{}, "", err
	}

	return user, token, nil
}

func (svc *AuthService) Login(ctx context.Context, email, password string) (domain.User, string, error) {
	email = strings.TrimSpace(strings.ToLower(email))
	if email == "" || password == "" {
		return domain.User{}, "", domain.ErrInvalidCredentials
	}

	user, err := svc.repo.GetByEmail(ctx, email)
	if err != nil {
		return domain.User{}, "", domain.ErrInvalidCredentials
	}

	if err := bcrypt.CompareHashAndPassword([]byte(user.PasswordHash), []byte(password)); err != nil {
		return domain.User{}, "", domain.ErrInvalidCredentials
	}

	token, err := svc.token.Issue(user)
	if err != nil {
		return domain.User{}, "", err
	}

	return user, token, nil
}

func (svc *AuthService) Validate(ctx context.Context, token string) (domain.User, error) {
	token = strings.TrimSpace(token)
	if token == "" {
		return domain.User{}, domain.ErrTokenInvalidOrEmpty
	}

	userID, err := svc.token.Parse(token)
	if err != nil {
		return domain.User{}, domain.ErrTokenInvalidOrEmpty
	}

	return svc.repo.GetByID(ctx, userID)
}

func (svc *AuthService) GetUser(ctx context.Context, userID string) (domain.User, error) {
	userID = strings.TrimSpace(userID)
	if userID == "" {
		return domain.User{}, domain.ErrInvalidInput
	}
	return svc.repo.GetByID(ctx, userID)
}
