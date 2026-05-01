export const resolveApiUrl = (path: string) => {
  const apiBaseUrl =
    import.meta.env.VITE_API === "mock"
      ? import.meta.env.MOCK_API_URL || globalThis.location.origin
      : import.meta.env.VITE_API_URL || globalThis.location.origin;

  return new URL(path, apiBaseUrl).toString();
};

export const fetchJson = async <T>(
  url: string,
  init?: RequestInit,
): Promise<T> => {
  const response = await fetch(url, init);

  if (!response.ok) {
    throw new Error(`Request failed: ${response.status}`);
  }

  const contentType = response.headers.get("content-type") || "";
  if (!contentType.includes("application/json")) {
    throw new Error(`Expected JSON response, received "${contentType}"`);
  }

  return (await response.json()) as T;
};
