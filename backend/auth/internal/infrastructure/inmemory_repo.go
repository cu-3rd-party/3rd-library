package infrastructure

import (
	"context"
	"sync"

	"github.com/igor/3rd-library/auth/internal/domain"
)

type InMemoryUserRepository struct {
	mu      sync.RWMutex
	byID    map[string]domain.User
	byEmail map[string]string
}

func NewInMemoryUserRepository() *InMemoryUserRepository {
	return &InMemoryUserRepository{
		byID:    make(map[string]domain.User),
		byEmail: make(map[string]string),
	}
}

func (repo *InMemoryUserRepository) Create(_ context.Context, user domain.User) error {
	repo.mu.Lock()
	defer repo.mu.Unlock()

	if _, exists := repo.byEmail[user.Email]; exists {
		return domain.ErrEmailTaken
	}

	repo.byID[user.ID] = user
	repo.byEmail[user.Email] = user.ID
	return nil
}

func (repo *InMemoryUserRepository) GetByEmail(_ context.Context, email string) (domain.User, error) {
	repo.mu.RLock()
	defer repo.mu.RUnlock()

	id, ok := repo.byEmail[email]
	if !ok {
		return domain.User{}, domain.ErrUserNotFound
	}
	user, ok := repo.byID[id]
	if !ok {
		return domain.User{}, domain.ErrUserNotFound
	}
	return user, nil
}

func (repo *InMemoryUserRepository) GetByID(_ context.Context, userID string) (domain.User, error) {
	repo.mu.RLock()
	defer repo.mu.RUnlock()

	user, ok := repo.byID[userID]
	if !ok {
		return domain.User{}, domain.ErrUserNotFound
	}
	return user, nil
}
