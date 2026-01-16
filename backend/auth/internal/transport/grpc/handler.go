package grpc

import (
	"context"

	"github.com/igor/3rd-library/auth/internal/application"
	"github.com/igor/3rd-library/auth/internal/domain"
	authpb "github.com/igor/3rd-library/auth/internal/gen"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

type AuthHandler struct {
	authpb.UnimplementedAuthServiceServer
	svc *application.AuthService
}

func NewAuthHandler(svc *application.AuthService) *AuthHandler {
	return &AuthHandler{svc: svc}
}

func (h *AuthHandler) Register(ctx context.Context, req *authpb.RegisterRequest) (*authpb.AuthResponse, error) {
	user, token, err := h.svc.Register(ctx, req.GetName(), req.GetEmail(), req.GetPassword())
	if err != nil {
		return nil, mapDomainError(err)
	}
	return &authpb.AuthResponse{
		Token: token,
		User:  toProtoUser(user),
	}, nil
}

func (h *AuthHandler) Login(ctx context.Context, req *authpb.LoginRequest) (*authpb.AuthResponse, error) {
	user, token, err := h.svc.Login(ctx, req.GetEmail(), req.GetPassword())
	if err != nil {
		return nil, mapDomainError(err)
	}
	return &authpb.AuthResponse{
		Token: token,
		User:  toProtoUser(user),
	}, nil
}

func (h *AuthHandler) Validate(ctx context.Context, req *authpb.ValidateRequest) (*authpb.ValidateResponse, error) {
	user, err := h.svc.Validate(ctx, req.GetToken())
	if err != nil {
		return nil, mapDomainError(err)
	}
	return &authpb.ValidateResponse{User: toProtoUser(user)}, nil
}

func (h *AuthHandler) GetUser(ctx context.Context, req *authpb.GetUserRequest) (*authpb.GetUserResponse, error) {
	user, err := h.svc.GetUser(ctx, req.GetUserId())
	if err != nil {
		return nil, mapDomainError(err)
	}
	return &authpb.GetUserResponse{User: toProtoUser(user)}, nil
}

func toProtoUser(user domain.User) *authpb.User {
	return &authpb.User{
		Id:        user.ID,
		Name:      user.Name,
		Email:     user.Email,
		CreatedAt: user.CreatedAt.Unix(),
	}
}

func mapDomainError(err error) error {
	switch err {
	case domain.ErrInvalidInput:
		return status.Error(codes.InvalidArgument, err.Error())
	case domain.ErrInvalidCredentials:
		return status.Error(codes.Unauthenticated, err.Error())
	case domain.ErrEmailTaken:
		return status.Error(codes.AlreadyExists, err.Error())
	case domain.ErrUserNotFound:
		return status.Error(codes.NotFound, err.Error())
	case domain.ErrTokenInvalidOrEmpty:
		return status.Error(codes.Unauthenticated, err.Error())
	default:
		return status.Error(codes.Internal, err.Error())
	}
}
