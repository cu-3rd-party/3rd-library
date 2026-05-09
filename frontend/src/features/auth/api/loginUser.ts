import {
  AuthRequestError,
  LegacyAuthUserResponse,
  NewAuthResponse,
  requestAuth,
} from "@/entities/session/api";
import { mapNewAuthResponse, withDefaultRoles } from "@/entities/session/lib";

import { AuthUserPayload } from "./types";

export const loginUser = (payload: AuthUserPayload) =>
  requestAuth<NewAuthResponse>("/api/auth/login", payload)
    .then(mapNewAuthResponse)
    .catch(async (error: unknown) => {
      if (!(error instanceof AuthRequestError) || error.status !== 404) {
        throw error;
      }

      const response = await requestAuth<LegacyAuthUserResponse>(
        "/api/users/login",
        {
          user: payload,
        },
      );

      return withDefaultRoles(response);
    });
