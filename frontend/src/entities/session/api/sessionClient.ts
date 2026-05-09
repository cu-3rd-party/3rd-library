import { fetchJsonBase } from "@/shared/api";

import { getAccessToken } from "../lib";

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

export const fetchWithAuth = (url: string, init?: RequestInit) =>
  fetch(url, {
    ...init,
    headers: withAuthHeaders(init?.headers),
  });

export const fetchJsonWithAuth = <T>(url: string, init?: RequestInit) =>
  fetchJsonBase<T>(url, {
    ...init,
    headers: withAuthHeaders(init?.headers),
  });
