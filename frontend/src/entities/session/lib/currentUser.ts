import { AUTHORS_PREFIX } from "@/shared/constants";

import { RawStoredAuthUser, StoredAuthUser } from "../model";

import { clearAccessToken } from "./authToken";

const AUTH_USER_STORAGE_KEY = "authUser";
const AUTH_USER_UPDATED_EVENT = "auth-user-updated";
const DEFAULT_USER_ROLE = "user";
const MODERATOR_ROLE = "moderator";

const normalizeRoles = (value: unknown) => {
  if (!Array.isArray(value)) return [DEFAULT_USER_ROLE];

  const roles = value
    .filter((item): item is string => typeof item === "string")
    .map((item) => item.trim().toLowerCase())
    .filter(Boolean);

  if (roles.length === 0) return [DEFAULT_USER_ROLE];
  return Array.from(new Set(roles));
};

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
      roles: normalizeRoles(parsed.roles),
    } satisfies StoredAuthUser;
  } catch {
    return null;
  }
};

export const getCurrentAuthUser = () => resolveStoredAuthUser();

export const persistCurrentAuthUser = (user: StoredAuthUser) => {
  const normalizedUser: StoredAuthUser = {
    ...user,
    roles: normalizeRoles(user.roles),
  };
  globalThis.localStorage?.setItem(
    AUTH_USER_STORAGE_KEY,
    JSON.stringify(normalizedUser),
  );
  notifyAuthUserUpdated();
};

export const clearCurrentAuthUser = () => {
  globalThis.localStorage?.removeItem(AUTH_USER_STORAGE_KEY);
  notifyAuthUserUpdated();
};

export const clearCurrentSession = () => {
  clearAccessToken();
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

export const isModerator = (user: StoredAuthUser | null) =>
  Boolean(user?.roles?.includes(MODERATOR_ROLE));

export const canAccessModeration = () => isModerator(resolveStoredAuthUser());
