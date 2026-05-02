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
};

type AuthUserResponse = {
  user: AuthUser;
};

type AuthApiError = {
  errors?: Record<string, string[]>;
};

const DEFAULT_AUTH_ERROR_MESSAGE = "Не удалось выполнить запрос";

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
    method?: "POST" | "PUT";
    withAuth?: boolean;
  },
): Promise<T> => {
  const method = options?.method || "POST";
  const request = options?.withAuth ? fetchWithAuth : fetch;
  const response = await request(resolveApiUrl(path), {
    method,
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
    if (parsedApiMessage) {
      throw new Error(parsedApiMessage);
    }

    const parsedMessage = parseAuthError(body);
    if (parsedMessage) {
      throw new Error(parsedMessage);
    }

    if (response.status === 401) {
      throw new Error("Неверный email или пароль");
    }

    throw new Error(DEFAULT_AUTH_ERROR_MESSAGE);
  }

  if (!isJson || !body) {
    throw new Error("Сервер вернул ответ в неожиданном формате");
  }

  return body as T;
};

export const loginUser = (payload: AuthUserPayload) =>
  requestAuth<AuthUserResponse>("/api/users/login", {
    user: payload,
  });

export const registerUser = (payload: RegisterUserPayload) =>
  requestAuth<AuthUserResponse>("/api/users", {
    user: payload,
  });

type UpdateCurrentUserPayload = {
  email: string;
  username: string;
  bio: string;
  image: string | null;
};

export const updateCurrentUser = (payload: UpdateCurrentUserPayload) =>
  requestAuth<AuthUserResponse>(
    "/api/user",
    {
      user: payload,
    },
    {
      method: "PUT",
      withAuth: true,
    },
  );
