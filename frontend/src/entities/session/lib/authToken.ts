const ACCESS_TOKEN_COOKIE_KEY = "accessToken";
const LEGACY_ACCESS_TOKEN_STORAGE_KEY = "accessToken";
const DEFAULT_TOKEN_MAX_AGE_SECONDS = 60 * 60 * 24 * 30;

const getCookie = (name: string) => {
  const cookiePayload = globalThis.document?.cookie;
  if (!cookiePayload) return null;

  const encodedName = encodeURIComponent(name);
  const searchPrefix = `${encodedName}=`;
  const cookie = cookiePayload
    .split(";")
    .map((chunk) => chunk.trim())
    .find((chunk) => chunk.startsWith(searchPrefix));

  if (!cookie) return null;

  const rawValue = cookie.slice(searchPrefix.length);
  if (!rawValue) return null;

  try {
    return decodeURIComponent(rawValue);
  } catch {
    return rawValue;
  }
};

const shouldUseSecureCookie = () => globalThis.location?.protocol === "https:";

const removeLegacyAccessToken = () => {
  globalThis.localStorage?.removeItem(LEGACY_ACCESS_TOKEN_STORAGE_KEY);
};

export const clearAccessToken = () => {
  const cookieParts = [
    `${encodeURIComponent(ACCESS_TOKEN_COOKIE_KEY)}=`,
    "Path=/",
    "Max-Age=0",
    "SameSite=Lax",
  ];

  if (shouldUseSecureCookie()) {
    cookieParts.push("Secure");
  }

  globalThis.document.cookie = cookieParts.join("; ");
  removeLegacyAccessToken();
};

export const setAccessToken = (
  token: string,
  options?: {
    maxAgeSeconds?: number;
  },
) => {
  const normalizedToken = token.trim();
  if (!normalizedToken) {
    clearAccessToken();
    return;
  }

  const maxAgeSeconds = options?.maxAgeSeconds ?? DEFAULT_TOKEN_MAX_AGE_SECONDS;

  const cookieParts = [
    `${encodeURIComponent(ACCESS_TOKEN_COOKIE_KEY)}=${encodeURIComponent(normalizedToken)}`,
    "Path=/",
    `Max-Age=${maxAgeSeconds}`,
    "SameSite=Lax",
  ];

  if (shouldUseSecureCookie()) {
    cookieParts.push("Secure");
  }

  globalThis.document.cookie = cookieParts.join("; ");
  removeLegacyAccessToken();
};

export const getAccessToken = () => {
  const cookieToken = getCookie(ACCESS_TOKEN_COOKIE_KEY)?.trim();
  if (cookieToken) return cookieToken;

  const legacyToken =
    globalThis.localStorage?.getItem(LEGACY_ACCESS_TOKEN_STORAGE_KEY)?.trim() ||
    "";

  if (!legacyToken) return null;

  setAccessToken(legacyToken);
  return legacyToken;
};
