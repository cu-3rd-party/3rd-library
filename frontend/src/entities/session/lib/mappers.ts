import { AuthUserResponse, NewAuthResponse } from "../api";

export const mapNewAuthResponse = (
  response: NewAuthResponse,
): AuthUserResponse => ({
  user: {
    email: response.user.email,
    token: response.tokens.access_token,
    username: response.user.name,
    bio: response.user.bio || "",
    image: response.user.image || null,
    roles: response.user.roles || ["user"],
  },
});

export const withDefaultRoles = (
  response: AuthUserResponse,
): AuthUserResponse => ({
  user: {
    ...response.user,
    roles:
      Array.isArray(response.user.roles) && response.user.roles.length > 0
        ? response.user.roles
        : ["user"],
  },
});
