import {
  extractApiErrorMessage,
  fetchWithAuth,
  resolveApiUrl,
} from "@/lib/api";

type AuthUserPayload = {
  email: string;
  password: string;
};

type RegisterUserPayload = AuthUserPayload & {
  username: string;
};

type AuthUser = {
  email: string;
  token: string;
  username: string;
  bio: string;
  image: string | null;
  roles: string[];
};

type AuthUserResponse = {
  user: AuthUser;
};

export type RegisterUserResult =
  | {
      status: "authorized";
      response: AuthUserResponse;
    }
  | {
      status: "verification_required";
      email: string;
      channel?: string | null;
    };

type LegacyAuthUserResponse = AuthUserResponse;

type NewAuthResponse = {
  user: {
    id: string;
    name: string;
    email: string;
    bio: string;
    image?: string | null;
    is_email_verified: boolean;
    can_submit_materials: boolean;
    roles: string[];
  };
  tokens: {
    access_token: string;
    refresh_token: string;
    expires_in: number;
  };
};

type NewRegisterResponse = {
  user: {
    id: string;
    name: string;
    email: string;
    bio: string;
    image?: string | null;
    is_email_verified: boolean;
    can_submit_materials: boolean;
    roles: string[];
  };
  verification_required?: boolean;
  verification_channel?: string | null;
};

type NewCurrentUserResponse = {
  id: string;
  name: string;
  email: string;
  bio: string;
  image?: string | null;
  is_email_verified: boolean;
  can_submit_materials: boolean;
  roles: string[];
};

type AuthApiError = {
  errors?: Record<string, string[]>;
};

const DEFAULT_AUTH_ERROR_MESSAGE = "Не удалось выполнить запрос";

class AuthRequestError extends Error {
  readonly status: number;

  constructor(status: number, message: string) {
    super(message);
    this.name = "AuthRequestError";
    this.status = status;
  }
}

const mapNewAuthResponse = (response: NewAuthResponse): AuthUserResponse => ({
  user: {
    email: response.user.email,
    token: response.tokens.access_token,
    username: response.user.name,
    bio: response.user.bio || "",
    image: response.user.image || null,
    roles: response.user.roles || ["user"],
  },
});

const withDefaultRoles = (response: AuthUserResponse): AuthUserResponse => ({
  user: {
    ...response.user,
    roles:
      Array.isArray(response.user.roles) && response.user.roles.length > 0
        ? response.user.roles
        : ["user"],
  },
});

const parseAuthError = (payload: unknown) => {
  if (!payload || typeof payload !== "object") {
    return null;
  }

  const errors = (payload as AuthApiError).errors;
  if (!errors || typeof errors !== "object") {
    return null;
  }

  const entries = Object.entries(errors).flatMap(([field, messages]) => {
    if (!Array.isArray(messages)) return [];
    return messages
      .filter((message): message is string => typeof message === "string")
      .map((message) => `${field} ${message}`);
  });

  if (entries.length === 0) return null;
  return entries.join(", ");
};

const requestAuth = async <T>(
  path: string,
  payload: Record<string, unknown>,
  options?: {
    method?: "POST" | "PUT" | "PATCH";
    withAuth?: boolean;
  },
): Promise<T> => {
  const method = options?.method || "POST";
  const request = options?.withAuth ? fetchWithAuth : fetch;
  const response = await request(resolveApiUrl(path), {
    method,
    credentials: "include",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(payload),
  });

  const contentType = response.headers.get("content-type") || "";
  const isJson = contentType.includes("application/json");
  const body = isJson ? ((await response.json()) as unknown) : null;

  if (!response.ok) {
    const parsedApiMessage = extractApiErrorMessage(body);
    const fallbackMessage =
      response.status === 401
        ? "Неверный email или пароль"
        : DEFAULT_AUTH_ERROR_MESSAGE;

    if (parsedApiMessage) {
      throw new AuthRequestError(response.status, parsedApiMessage);
    }

    const parsedMessage = parseAuthError(body);
    if (parsedMessage) {
      throw new AuthRequestError(response.status, parsedMessage);
    }

    throw new AuthRequestError(response.status, fallbackMessage);
  }

  if (!isJson) {
    throw new Error("Сервер вернул ответ в неожиданном формате");
  }

  return body as T;
};

export const loginUser = (payload: AuthUserPayload) =>
  requestAuth<NewAuthResponse>("/api/auth/login", payload)
    .then(mapNewAuthResponse)
    .catch(async (error: unknown) => {
      if (!(error instanceof AuthRequestError) || error.status !== 404) {
        throw error;
      }

      const response = await requestAuth<LegacyAuthUserResponse>("/api/users/login", {
        user: payload,
      });

      return withDefaultRoles(response);
    });

export const registerUser = (payload: RegisterUserPayload) =>
  requestAuth<NewRegisterResponse>("/api/auth/register", {
    name: payload.username,
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
        user: payload,
      });

      return {
        status: "authorized",
        response: withDefaultRoles(response),
      } as const;
    });

export const resendVerificationCode = async (payload: { email: string }) => {
  await requestAuth<null>("/api/auth/resend-verification-code", {
    email: payload.email,
  });
};

export const verifyEmailCode = async (payload: {
  email: string;
  code: string;
}) => {
  const response = await requestAuth<NewAuthResponse>("/api/auth/verify-email", {
    email: payload.email,
    code: payload.code,
  });

  return {
    user: mapNewAuthResponse(response).user,
    tokens: {
      accessToken: response.tokens.access_token,
      refreshToken: response.tokens.refresh_token,
      expiresIn: response.tokens.expires_in,
    },
  };
};

type UpdateCurrentUserPayload = {
  email: string;
  username: string;
  bio: string;
  image: string | null;
};

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
        globalThis.localStorage
          ?.getItem("accessToken")
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
            response.image !== undefined ? response.image || null : payload.image,
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
