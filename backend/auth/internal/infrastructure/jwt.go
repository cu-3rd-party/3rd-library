package infrastructure

import (
	"errors"
	"time"

	"github.com/golang-jwt/jwt/v5"
	"github.com/igor/3rd-library/auth/internal/application"
	"github.com/igor/3rd-library/auth/internal/domain"
)

type JWTManager struct {
	secret []byte
	ttl    time.Duration
}

type claims struct {
	Email string `json:"email"`
	jwt.RegisteredClaims
}

func NewJWTManager(secret string, ttl time.Duration) *JWTManager {
	return &JWTManager{
		secret: []byte(secret),
		ttl:    ttl,
	}
}

func (mgr *JWTManager) Issue(user domain.User) (string, error) {
	now := time.Now()
	claims := claims{
		Email: user.Email,
		RegisteredClaims: jwt.RegisteredClaims{
			Subject:   user.ID,
			IssuedAt:  jwt.NewNumericDate(now),
			ExpiresAt: jwt.NewNumericDate(now.Add(mgr.ttl)),
		},
	}

	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	return token.SignedString(mgr.secret)
}

func (mgr *JWTManager) Parse(token string) (string, error) {
	parsed, err := jwt.ParseWithClaims(token, &claims{}, func(_ *jwt.Token) (interface{}, error) {
		return mgr.secret, nil
	})
	if err != nil {
		return "", err
	}

	parsedClaims, ok := parsed.Claims.(*claims)
	if !ok || !parsed.Valid {
		return "", errors.New("token invalid")
	}
	if parsedClaims.Subject == "" {
		return "", errors.New("token missing subject")
	}
	return parsedClaims.Subject, nil
}

var _ application.TokenManager = (*JWTManager)(nil)
