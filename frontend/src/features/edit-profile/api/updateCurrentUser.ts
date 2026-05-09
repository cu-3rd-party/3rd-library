import {
  AuthRequestError,
  AuthUserResponse,
  NewCurrentUserResponse,
  requestAuth,
} from "@/entities/session/api";
import { getAccessToken, withDefaultRoles } from "@/entities/session/lib";

export const updateCurrentUser = (payload: UpdateCurrentUserPayload) =>
  requestAuth<NewCurrentUserResponse>(
    "/api/users/me",
    {
      name: payload.username,
      bio: payload.bio,
      image: payload.image,
    },
    {
      method: "PATCH",
      withAuth: true,
    },
  )
    .then((response) => {
      const storedToken =
        getAccessToken()
          ?.replace(/^Token\s+/i, "")
          .replace(/^Bearer\s+/i, "")
          .trim() || "";

      return {
        user: {
          email: response.email,
          token: storedToken,
          username: response.name,
          bio: response.bio || "",
          image:
            response.image !== undefined
              ? response.image || null
              : payload.image,
          roles: response.roles || ["user"],
        },
      } satisfies AuthUserResponse;
    })
    .catch(async (error: unknown) => {
      if (!(error instanceof AuthRequestError) || error.status !== 404) {
        throw error;
      }

      return requestAuth<AuthUserResponse>(
        "/api/user",
        {
          user: payload,
        },
        {
          method: "PUT",
          withAuth: true,
        },
      ).then(withDefaultRoles);
    });
