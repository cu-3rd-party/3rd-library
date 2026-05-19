import { RealWorldArticlesResponse } from "@/entities/material/api";
import { RealWorldProfileResponse } from "@/entities/user/api";
import { mapArticleToMaterial, mapLibraryMaterialToMaterial } from "@/entities/material/lib";
import { fetchJsonWithAuth } from "@/entities/session/api";
import { StoredAuthUser } from "@/entities/session/model";
import {
  LibraryCurrentUserResponse,
  LibraryUsersResponse,
  LibraryUserWithMaterialsResponse,
} from "@/entities/user/api";
import { mapLibraryUserToUser, mapProfileToUser } from "@/entities/user/lib";
import { resolveApiUrl } from "@/shared/api";

import { isNotFoundError, UUID_PATTERN } from "../lib/helpers";
import { UserProfileResponse } from "../model";

export type FetchProfileResult = {
  profile: UserProfileResponse;
  resolvedEmail: string;
};

const mapLibraryPayload = (payload: LibraryUserWithMaterialsResponse): UserProfileResponse => ({
  user: mapLibraryUserToUser(payload.user),
  materials: {
    items: payload.materials.items.map(mapLibraryMaterialToMaterial),
    page: payload.materials.page,
    limit: payload.materials.limit,
    total: payload.materials.total,
  },
});

export const fetchUserProfile = async (
  userId: string,
  signal: AbortSignal,
  currentAuthUser: StoredAuthUser | null,
): Promise<FetchProfileResult> => {
  const encodedUserId = encodeURIComponent(userId);
  const currentUsername = currentAuthUser?.username || "";
  const isOwnProfileByName = currentUsername.length > 0 && userId === currentUsername;
  let resolvedEmail = currentAuthUser?.email || "";
  let nextProfile: UserProfileResponse | null = null;

  if (UUID_PATTERN.test(userId)) {
    try {
      const payload = await fetchJsonWithAuth<LibraryUserWithMaterialsResponse>(
        resolveApiUrl(`/api/users/${encodedUserId}?limit=100`),
        { signal },
      );
      nextProfile = mapLibraryPayload(payload);
    } catch (error) {
      if (!isNotFoundError(error)) throw error;
    }
  }

  if (!nextProfile && isOwnProfileByName) {
    try {
      const mePayload = await fetchJsonWithAuth<LibraryCurrentUserResponse>(
        resolveApiUrl("/api/users/me"),
        { signal },
      );
      resolvedEmail = mePayload.email || resolvedEmail;

      try {
        const payload = await fetchJsonWithAuth<LibraryUserWithMaterialsResponse>(
          resolveApiUrl(`/api/users/${encodeURIComponent(mePayload.id)}?limit=100`),
          { signal },
        );
        nextProfile = mapLibraryPayload(payload);
      } catch (error) {
        if (!isNotFoundError(error)) throw error;
        nextProfile = {
          user: {
            id: mePayload.id,
            name: mePayload.name,
            bio: mePayload.bio || "",
            image: mePayload.image || currentAuthUser?.image || null,
            isEmailVerified: mePayload.is_email_verified,
            verified: mePayload.is_email_verified,
            materialsCount: 0,
          },
          materials: { items: [], page: 1, limit: 20, total: 0 },
        };
      }
    } catch (error) {
      if (!isNotFoundError(error)) throw error;
    }
  }

  if (!nextProfile && !UUID_PATTERN.test(userId)) {
    try {
      const usersPayload = await fetchJsonWithAuth<LibraryUsersResponse>(
        resolveApiUrl(`/api/users?search=${encodedUserId}&limit=100`),
        { signal },
      );
      const matchedUser = usersPayload.items.find(
        (item) => item.name.trim().toLowerCase() === userId.trim().toLowerCase(),
      );
      if (matchedUser) {
        const payload = await fetchJsonWithAuth<LibraryUserWithMaterialsResponse>(
          resolveApiUrl(`/api/users/${encodeURIComponent(matchedUser.id)}?limit=100`),
          { signal },
        );
        nextProfile = mapLibraryPayload(payload);
      }
    } catch (error) {
      if (!isNotFoundError(error)) throw error;
    }
  }

  if (!nextProfile) {
    const [profilePayload, articlesPayload] = await Promise.all([
      fetchJsonWithAuth<RealWorldProfileResponse>(
        resolveApiUrl(`/api/profiles/${encodedUserId}`),
        { signal },
      ),
      fetchJsonWithAuth<RealWorldArticlesResponse>(
        resolveApiUrl(`/api/articles?author=${encodedUserId}&limit=100`),
        { signal },
      ),
    ]);
    const userMaterials = articlesPayload.articles.map(mapArticleToMaterial);
    nextProfile = {
      user: mapProfileToUser(profilePayload.profile),
      materials: {
        items: userMaterials,
        page: 1,
        limit: userMaterials.length,
        total: articlesPayload.articlesCount,
      },
    };
  }

  return { profile: nextProfile, resolvedEmail };
};
