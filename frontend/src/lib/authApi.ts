import {
  extractApiErrorMessage,
  fetchWithAuth,
  resolveApiUrl,
} from "@/lib/api";
import { getAccessToken } from "@/lib/authToken";

type AuthUserPayload = {
  email: string;
  password: string;
};

type RegisterUserPayload = AuthUserPayload & {
  name: string;
  surname: string;
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

const SERVER_UNAVAILABLE_MESSAGE = "Сервер недоступен. Попробуйте позже.";
const INVALID_CREDENTIALS_MESSAGE = "Неверный email или пароль.";
const EMAIL_ALREADY_REGISTERED_MESSAGE =
  "Пользователь с таким email уже зарегистрирован.";
const USERNAME_ALREADY_REGISTERED_MESSAGE =
  "Пользователь с таким именем уже зарегистрирован.";
const INVALID_VERIFICATION_CODE_MESSAGE = "Неверный код подтверждения.";
const EXPIRED_VERIFICATION_CODE_MESSAGE =
  "Срок действия кода истек. Запросите новый код.";
const TOO_MANY_REQUESTS_MESSAGE =
  "Слишком много попыток. Попробуйте чуть позже.";
const FILL_REQUIRED_FIELDS_MESSAGE = "Заполните обязательные поля.";

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

const matchesAny = (value: string, patterns: RegExp[]) =>
  patterns.some((pattern) => pattern.test(value));

const resolveHumanReadableAuthErrorMessage = (
  status: number,
  rawMessage?: string | null,
) => {
  const normalized = (rawMessage || "").trim().toLowerCase();

  if (status === 401) return INVALID_CREDENTIALS_MESSAGE;

  if (
    status === 429 ||
    matchesAny(normalized, [
      /too many requests/i,
      /rate limit/i,
      /слишком много/i,
    ])
  ) {
    return TOO_MANY_REQUESTS_MESSAGE;
  }

  if (
    matchesAny(normalized, [
      /failed to fetch/i,
      /networkerror/i,
      /network request failed/i,
      /fetch resource/i,
      /load failed/i,
    ])
  ) {
    return SERVER_UNAVAILABLE_MESSAGE;
  }

  if (
    matchesAny(normalized, [/verification/i, /code/i]) &&
    matchesAny(normalized, [/invalid/i, /incorrect/i, /wrong/i, /неверн/i])
  ) {
    return INVALID_VERIFICATION_CODE_MESSAGE;
  }

  if (
    matchesAny(normalized, [/verification/i, /code/i]) &&
    matchesAny(normalized, [/expired/i, /истек/i])
  ) {
    return EXPIRED_VERIFICATION_CODE_MESSAGE;
  }

  if (
    matchesAny(normalized, [
      /email taken/i,
      /email.*taken/i,
      /email.*already/i,
      /already registered/i,
      /already exists/i,
    ])
  ) {
    return EMAIL_ALREADY_REGISTERED_MESSAGE;
  }

  if (
    matchesAny(normalized, [
      /username taken/i,
      /username.*taken/i,
      /name.*taken/i,
      /username.*already/i,
      /name.*already/i,
    ])
  ) {
    return USERNAME_ALREADY_REGISTERED_MESSAGE;
  }

  if (
    matchesAny(normalized, [/can't be blank/i, /required/i, /обязат/i]) ||
    status === 422
  ) {
    return FILL_REQUIRED_FIELDS_MESSAGE;
  }

  if (
    matchesAny(normalized, [/does not exist/i, /invalid credentials/i]) ||
    status === 403
  ) {
    return INVALID_CREDENTIALS_MESSAGE;
  }

  return SERVER_UNAVAILABLE_MESSAGE;
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
  let response: Response;

  try {
    response = await request(resolveApiUrl(path), {
      method,
      credentials: "include",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(payload),
    });
  } catch {
    throw new AuthRequestError(0, SERVER_UNAVAILABLE_MESSAGE);
  }

  const contentType = response.headers.get("content-type") || "";
  const isJson = contentType.includes("application/json");
  const body = isJson ? ((await response.json()) as unknown) : null;

  if (!response.ok) {
    const parsedApiMessage = extractApiErrorMessage(body);
    const parsedMessage = parseAuthError(body);
    const rawMessage = parsedApiMessage || parsedMessage;
    const userMessage = resolveHumanReadableAuthErrorMessage(
      response.status,
      rawMessage,
    );
    throw new AuthRequestError(response.status, userMessage);
  }

  if (!isJson) {
    throw new AuthRequestError(response.status, SERVER_UNAVAILABLE_MESSAGE);
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

      const response = await requestAuth<LegacyAuthUserResponse>(
        "/api/users/login",
        {
          user: payload,
        },
      );

      return withDefaultRoles(response);
    });

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

export const resendVerificationCode = async (payload: { email: string }) => {
  await requestAuth<null>("/api/auth/resend-verification-code", {
    email: payload.email,
  });
};

export const verifyEmailCode = async (payload: {
  email: string;
  code: string;
}) => {
  const response = await requestAuth<NewAuthResponse>(
    "/api/auth/verify-email",
    {
      email: payload.email,
      code: payload.code,
    },
  );

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
