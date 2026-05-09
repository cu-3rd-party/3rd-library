import { extractApiErrorMessage, resolveApiUrl } from "@/shared/api";

import { AUTH_ERROR_MESSAGES } from "../model";

import {
  parseAuthError,
  resolveHumanReadableAuthErrorMessage,
} from "./authErrors";
import { fetchWithAuth } from "./sessionClient";

export class AuthRequestError extends Error {
  readonly status: number;

  constructor(status: number, message: string) {
    super(message);
    this.name = "AuthRequestError";
    this.status = status;
  }
}

export const requestAuth = async <T>(
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
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(payload),
    });
  } catch {
    throw new AuthRequestError(0, AUTH_ERROR_MESSAGES.SERVER_UNAVAILABLE);
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
    throw new AuthRequestError(
      response.status,
      AUTH_ERROR_MESSAGES.SERVER_UNAVAILABLE,
    );
  }

  return body as T;
};
