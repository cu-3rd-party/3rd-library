import {
  AuthRequestError,
  AuthUserResponse,
  NewRegisterResponse,
  requestAuth,
} from "@/entities/session/api";
import { withDefaultRoles } from "@/entities/session/lib";

import { RegisterUserPayload } from "./types";

export const registerUser = (payload: RegisterUserPayload) => {
  const fullName = `${payload.name} ${payload.surname}`.trim();

  return requestAuth<NewRegisterResponse>("/api/auth/register", {
    name: fullName,
    email: payload.email,
    password: payload.password,
  })
    .then((response) => {
      if (response.verification_required) {
        return {
          status: "verification_required",
          email: response.user.email,
          channel: response.verification_channel,
        } as const;
      }

      throw new Error(
        "Регистрация выполнена, но сервер не вернул токен авторизации. Подтвердите email кодом из письма или выполните вход.",
      );
    })
    .catch(async (error: unknown) => {
      if (!(error instanceof AuthRequestError) || error.status !== 404) {
        throw error;
      }

      const response = await requestAuth<AuthUserResponse>("/api/users", {
        user: {
          username: fullName,
          email: payload.email,
          password: payload.password,
        },
      });

      return {
        status: "authorized",
        response: withDefaultRoles(response),
      } as const;
    });
};
