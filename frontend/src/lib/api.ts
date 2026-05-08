import { getAccessToken } from "@/lib/authToken";

export const resolveApiUrl = (path: string) => {
  const apiBaseUrl =
    import.meta.env.VITE_API === "mock"
      ? import.meta.env.MOCK_API_URL || globalThis.location.origin
      : import.meta.env.VITE_API_URL || globalThis.location.origin;

  return new URL(path, apiBaseUrl).toString();
};

type ApiErrorPayload = {
  message?: unknown;
  errors?: unknown;
};

export class ApiRequestError extends Error {
  readonly status: number;
  readonly url: string;
  readonly payload: unknown;

  constructor(status: number, url: string, payload: unknown) {
    super(`Request failed: ${status}`);
    this.name = "ApiRequestError";
    this.status = status;
    this.url = url;
    this.payload = payload;
  }
}

const withAuthHeaders = (headersInit?: HeadersInit) => {
  const headers = new Headers(headersInit);
  const accessToken = getAccessToken();

  if (accessToken && !headers.has("Authorization")) {
    const hasScheme =
      accessToken.startsWith("Token ") || accessToken.startsWith("Bearer ");
    headers.set(
      "Authorization",
      hasScheme ? accessToken : `Token ${accessToken}`,
    );
  }

  return headers;
};

const readErrorPayload = async (response: Response) => {
  const contentType = response.headers.get("content-type") || "";
  const isJson = contentType.includes("application/json");

  if (isJson) {
    try {
      return (await response.json()) as unknown;
    } catch {
      return null;
    }
  }

  try {
    return await response.text();
  } catch {
    return null;
  }
};

export const extractApiErrorMessage = (payload: unknown) => {
  if (!payload || typeof payload !== "object") return null;

  const { message, errors } = payload as ApiErrorPayload;
  if (typeof message === "string" && message.trim()) {
    return message.trim();
  }

  if (!errors || typeof errors !== "object") return null;

  const entries = Object.entries(errors).flatMap(([field, value]) => {
    if (Array.isArray(value)) {
      return value
        .filter((item): item is string => typeof item === "string")
        .map((item) => `${field} ${item}`.trim());
    }

    if (typeof value === "string") {
      return [`${field} ${value}`.trim()];
    }

    return [];
  });

  if (entries.length === 0) return null;
  return entries.join(", ");
};

export const fetchWithAuth = (url: string, init?: RequestInit) =>
  fetch(url, {
    ...init,
    headers: withAuthHeaders(init?.headers),
  });

export const fetchJson = async <T>(
  url: string,
  init?: RequestInit,
): Promise<T> => {
  const response = await fetchWithAuth(url, init);

  if (!response.ok) {
    const payload = await readErrorPayload(response);
    const parsedMessage = extractApiErrorMessage(payload);

    if (parsedMessage) {
      throw new ApiRequestError(
        response.status,
        response.url || url,
        parsedMessage,
      );
    }

    throw new ApiRequestError(response.status, response.url || url, payload);
  }

  const contentType = response.headers.get("content-type") || "";
  if (!contentType.includes("application/json")) {
    throw new Error(`Expected JSON response, received "${contentType}"`);
  }

  return (await response.json()) as T;
};
