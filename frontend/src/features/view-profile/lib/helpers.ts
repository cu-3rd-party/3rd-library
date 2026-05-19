import { MOCK_AUTHORS } from "@/app/mocks/data/user";
import { MOCK_SUBMISSIONS } from "@/app/mocks/data/submission";
import { MOCK_USER } from "@/app/mocks/data/auth";
import { StoredAuthUser } from "@/entities/session/model";
import { ApiRequestError, resolveApiUrl } from "@/shared/api";

import { UserProfileResponse } from "../model";

export const UUID_PATTERN =
  /^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;

export const MAX_BIO_LENGTH = 2_000;

export const isNotFoundError = (error: unknown): boolean =>
  error instanceof ApiRequestError && error.status === 404;

export const decodeRouteUserId = (value?: string): string => {
  const fallback = (value || "").trim();
  try {
    return decodeURIComponent(fallback).trim();
  } catch {
    return fallback;
  }
};

export const readFileAsDataUrl = (file: File): Promise<string> =>
  new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => {
      if (typeof reader.result !== "string") {
        reject(new Error("Failed to read image as data URL."));
        return;
      }
      resolve(reader.result);
    };
    reader.onerror = () => reject(new Error("Failed to read image file."));
    reader.readAsDataURL(file);
  });

export const isSameAuthUser = (
  first: StoredAuthUser | null,
  second: StoredAuthUser,
): boolean =>
  first
    ? first.email === second.email &&
      first.username === second.username &&
      first.bio === second.bio &&
      first.image === second.image &&
      first.roles.join(",") === second.roles.join(",")
    : false;

export const getFallbackProfile = (userId: string): UserProfileResponse | null => {
  const fallbackUser = [MOCK_USER, ...MOCK_AUTHORS].find((u) => u.id === userId);
  if (!fallbackUser) return null;

  const userMaterials = MOCK_SUBMISSIONS.filter(
    (s) => s.status === "approved" && s.material.authorId === userId,
  ).map((s) => s.material);

  return {
    user: fallbackUser,
    materials: { items: userMaterials, page: 1, limit: 20, total: userMaterials.length },
  };
};

export const resolvePfpUrl = (imageId: string | null | undefined): string | null => {
  if (!imageId) return null;
  if (UUID_PATTERN.test(imageId))
    return resolveApiUrl(`/api/pfps/${encodeURIComponent(imageId)}`);
  return imageId;
};
