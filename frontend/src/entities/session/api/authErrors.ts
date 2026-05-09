import { AUTH_ERROR_MESSAGES } from "../model";

import { AuthApiError } from "./types";

const matchesAny = (value: string, patterns: RegExp[]) =>
  patterns.some((pattern) => pattern.test(value));

export const parseAuthError = (payload: unknown) => {
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

export const resolveHumanReadableAuthErrorMessage = (
  status: number,
  rawMessage?: string | null,
) => {
  const normalized = (rawMessage || "").trim().toLowerCase();

  if (status === 401) return AUTH_ERROR_MESSAGES.INVALID_CREDENTIALS;

  if (
    status === 429 ||
    matchesAny(normalized, [
      /too many requests/i,
      /rate limit/i,
      /слишком много/i,
    ])
  ) {
    return AUTH_ERROR_MESSAGES.TOO_MANY_REQUESTS;
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
    return AUTH_ERROR_MESSAGES.SERVER_UNAVAILABLE;
  }

  if (
    matchesAny(normalized, [/verification/i, /code/i]) &&
    matchesAny(normalized, [/invalid/i, /incorrect/i, /wrong/i, /неверн/i])
  ) {
    return AUTH_ERROR_MESSAGES.INVALID_VERIFICATION_CODE;
  }

  if (
    matchesAny(normalized, [/verification/i, /code/i]) &&
    matchesAny(normalized, [/expired/i, /истек/i])
  ) {
    return AUTH_ERROR_MESSAGES.EXPIRED_VERIFICATION_CODE;
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
    return AUTH_ERROR_MESSAGES.EMAIL_ALREADY_REGISTERED;
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
    return AUTH_ERROR_MESSAGES.USERNAME_ALREADY_REGISTERED;
  }

  if (
    matchesAny(normalized, [/can't be blank/i, /required/i, /обязат/i]) ||
    status === 422
  ) {
    return AUTH_ERROR_MESSAGES.FILL_REQUIRED_FIELDS;
  }

  if (
    matchesAny(normalized, [/does not exist/i, /invalid credentials/i]) ||
    status === 403
  ) {
    return AUTH_ERROR_MESSAGES.INVALID_CREDENTIALS;
  }

  return AUTH_ERROR_MESSAGES.SERVER_UNAVAILABLE;
};
