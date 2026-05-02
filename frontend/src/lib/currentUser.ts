import { AUTHORS_PREFIX } from "@/constants/routePrefixes";

type RawStoredAuthUser = {
  email?: unknown;
  username?: unknown;
  bio?: unknown;
  image?: unknown;
};

export type StoredAuthUser = {
  email: string;
  username: string;
  bio: string;
  image: string | null;
};

const AUTH_USER_STORAGE_KEY = "authUser";
const ACCESS_TOKEN_STORAGE_KEY = "accessToken";
const AUTH_USER_UPDATED_EVENT = "auth-user-updated";

const notifyAuthUserUpdated = () => {
  globalThis.window?.dispatchEvent(new Event(AUTH_USER_UPDATED_EVENT));
};

const resolveStoredAuthUser = () => {
  const payload = globalThis.localStorage?.getItem(AUTH_USER_STORAGE_KEY);
  if (!payload) return null;

  try {
    const parsed = JSON.parse(payload) as RawStoredAuthUser;
    if (typeof parsed.username !== "string") return null;
    const username = parsed.username.trim();
    if (!username) return null;

    return {
      email: typeof parsed.email === "string" ? parsed.email : "",
      username,
      bio: typeof parsed.bio === "string" ? parsed.bio : "",
      image:
        typeof parsed.image === "string"
          ? parsed.image
          : parsed.image === null
            ? null
            : null,
    } satisfies StoredAuthUser;
  } catch {
    return null;
  }
};

export const getCurrentAuthUser = () => resolveStoredAuthUser();

export const persistCurrentAuthUser = (user: StoredAuthUser) => {
  globalThis.localStorage?.setItem(AUTH_USER_STORAGE_KEY, JSON.stringify(user));
  notifyAuthUserUpdated();
};

export const clearCurrentAuthUser = () => {
  globalThis.localStorage?.removeItem(AUTH_USER_STORAGE_KEY);
  notifyAuthUserUpdated();
};

export const clearCurrentSession = () => {
  globalThis.localStorage?.removeItem(ACCESS_TOKEN_STORAGE_KEY);
  clearCurrentAuthUser();
};

export const subscribeToCurrentAuthUser = (
  callback: (user: StoredAuthUser | null) => void,
) => {
  const handleStorage = (event: StorageEvent) => {
    if (event.key && event.key !== AUTH_USER_STORAGE_KEY) return;
    callback(resolveStoredAuthUser());
  };
  const handleCustomEvent = () => {
    callback(resolveStoredAuthUser());
  };

  globalThis.window?.addEventListener("storage", handleStorage);
  globalThis.window?.addEventListener(
    AUTH_USER_UPDATED_EVENT,
    handleCustomEvent,
  );

  return () => {
    globalThis.window?.removeEventListener("storage", handleStorage);
    globalThis.window?.removeEventListener(
      AUTH_USER_UPDATED_EVENT,
      handleCustomEvent,
    );
  };
};

export const resolveCurrentProfilePath = () => {
  const user = resolveStoredAuthUser();
  if (!user) return AUTHORS_PREFIX;

  return `${AUTHORS_PREFIX}/${encodeURIComponent(user.username)}`;
};
