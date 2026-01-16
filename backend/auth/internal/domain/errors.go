package domain

import "errors"

var (
	ErrEmailTaken          = errors.New("email already registered")
	ErrInvalidCredentials  = errors.New("invalid credentials")
	ErrUserNotFound        = errors.New("user not found")
	ErrInvalidInput        = errors.New("invalid input")
	ErrTokenInvalidOrEmpty = errors.New("token invalid or empty")
)
