import { MOCK_AUTHORS } from "@/app/mocks/data/user";
import { RealWorldArticlesResponse } from "@/entities/material/api";
import { fetchJsonWithAuth } from "@/entities/session/api";
import {
  LibraryUsersResponse,
  LibraryUserWithMaterialsResponse,
  RealWorldProfileResponse,
} from "@/entities/user/api";
import { mapLibraryUserToUser, mapProfileToUser } from "@/entities/user/lib";
import { User } from "@/entities/user/model";
import { ApiRequestError, resolveApiUrl } from "@/shared/api";

const sortByName = (users: User[]): User[] =>
  [...users].sort((a, b) => a.name.localeCompare(b.name));

export const fetchAuthors = async (signal: AbortSignal): Promise<User[]> => {
  try {
    try {
      const usersPayload = await fetchJsonWithAuth<LibraryUsersResponse>(
        resolveApiUrl("/api/users?limit=100"),
        { signal },
      );

      if (signal.aborted) return [];

      const mappedAuthors = usersPayload.items.map(mapLibraryUserToUser);
      const authorsWithoutImage = mappedAuthors.filter((a) => !a.image);

      if (authorsWithoutImage.length === 0) return sortByName(mappedAuthors);

      const detailedProfiles = await Promise.allSettled(
        authorsWithoutImage.map((author) =>
          fetchJsonWithAuth<LibraryUserWithMaterialsResponse>(
            resolveApiUrl(`/api/users/${encodeURIComponent(author.id)}?limit=1`),
            { signal },
          ),
        ),
      );

      if (signal.aborted) return [];

      const imageByAuthorId = new Map<string, string | null>();
      authorsWithoutImage.forEach((author, index) => {
        const result = detailedProfiles[index];
        if (result?.status !== "fulfilled") return;
        imageByAuthorId.set(author.id, result.value.user.image || null);
      });

      return sortByName(
        mappedAuthors.map((author) => ({
          ...author,
          image: imageByAuthorId.get(author.id) ?? author.image ?? null,
        })),
      );
    } catch (error) {
      if (!(error instanceof ApiRequestError) || error.status !== 404) throw error;
    }

    const articlesPayload = await fetchJsonWithAuth<RealWorldArticlesResponse>(
      resolveApiUrl("/api/articles?limit=100"),
      { signal },
    );

    const usersById = new Map<string, User>();
    articlesPayload.articles.forEach((article) => {
      const userId = article.author.username;
      const existing = usersById.get(userId);
      if (existing) {
        usersById.set(userId, { ...existing, materialsCount: (existing.materialsCount || 0) + 1 });
        return;
      }
      usersById.set(userId, { ...mapProfileToUser(article.author), materialsCount: 1 });
    });

    const authorEntries = Array.from(usersById.entries());
    const profileResponses = await Promise.allSettled(
      authorEntries.map(([userId]) =>
        fetchJsonWithAuth<RealWorldProfileResponse>(
          resolveApiUrl(`/api/profiles/${encodeURIComponent(userId)}`),
          { signal },
        ),
      ),
    );

    if (signal.aborted) return [];

    return sortByName(
      authorEntries.map(([, fallbackUser], index) => {
        const result = profileResponses[index];
        if (result?.status !== "fulfilled") return fallbackUser;
        return { ...mapProfileToUser(result.value.profile), materialsCount: fallbackUser.materialsCount } satisfies User;
      }),
    );
  } catch (error) {
    if (signal.aborted) throw error;
    if (import.meta.env.VITE_API === "mock") {
      console.warn("[Users] Falling back to local mock list.");
      return MOCK_AUTHORS;
    }
    throw error;
  }
};
