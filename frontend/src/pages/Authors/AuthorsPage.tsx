// TODO: resolve import
import { Search } from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import { useNavigate } from "react-router-dom";

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
import { Input } from "@/shared/ui";
import { AuthorsGrid } from "@/widgets/authors-grid/ui";

const AuthorsPage = () => {
  const navigate = useNavigate();
  const [searchQuery, setSearchQuery] = useState("");
  const [authors, setAuthors] = useState<User[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [isError, setIsError] = useState(false);

  useEffect(() => {
    const abortController = new AbortController();

    const fetchUsers = async () => {
      setIsLoading(true);
      setIsError(false);

      try {
        try {
          const usersPayload = await fetchJsonWithAuth<LibraryUsersResponse>(
            resolveApiUrl("/api/users?limit=100"),
            {
              signal: abortController.signal,
            },
          );

          if (abortController.signal.aborted) return;

          const mappedAuthors = usersPayload.items.map((user) =>
            mapLibraryUserToUser(user),
          );
          const authorsWithoutImage = mappedAuthors.filter(
            (author) => !author.image,
          );

          if (authorsWithoutImage.length === 0) {
            setAuthors(
              mappedAuthors.sort((first, second) =>
                first.name.localeCompare(second.name),
              ),
            );
            return;
          }

          const detailedProfiles = await Promise.allSettled(
            authorsWithoutImage.map((author) =>
              fetchJsonWithAuth<LibraryUserWithMaterialsResponse>(
                resolveApiUrl(
                  `/api/users/${encodeURIComponent(author.id)}?limit=1`,
                ),
                {
                  signal: abortController.signal,
                },
              ),
            ),
          );

          if (abortController.signal.aborted) return;

          const imageByAuthorId = new Map<string, string | null>();
          authorsWithoutImage.forEach((author, index) => {
            const profileResult = detailedProfiles[index];
            if (profileResult?.status !== "fulfilled") return;

            imageByAuthorId.set(
              author.id,
              profileResult.value.user.image || null,
            );
          });

          const enrichedAuthors = mappedAuthors.map((author) => ({
            ...author,
            image: imageByAuthorId.get(author.id) ?? author.image ?? null,
          }));

          setAuthors(
            enrichedAuthors.sort((first, second) =>
              first.name.localeCompare(second.name),
            ),
          );
          return;
        } catch (error) {
          if (!(error instanceof ApiRequestError) || error.status !== 404) {
            throw error;
          }
        }

        const payload = await fetchJsonWithAuth<RealWorldArticlesResponse>(
          resolveApiUrl("/api/articles?limit=100"),
          {
            signal: abortController.signal,
          },
        );

        const usersById = new Map<string, User>();
        payload.articles.forEach((article) => {
          const userId = article.author.username;
          const existingUser = usersById.get(userId);

          if (existingUser) {
            usersById.set(userId, {
              ...existingUser,
              materialsCount: (existingUser.materialsCount || 0) + 1,
            });
            return;
          }

          usersById.set(userId, {
            ...mapProfileToUser(article.author),
            materialsCount: 1,
          });
        });

        const authorEntries = Array.from(usersById.entries());
        const profileResponses = await Promise.allSettled(
          authorEntries.map(([userId]) =>
            fetchJsonWithAuth<RealWorldProfileResponse>(
              resolveApiUrl(`/api/profiles/${encodeURIComponent(userId)}`),
              {
                signal: abortController.signal,
              },
            ),
          ),
        );

        if (abortController.signal.aborted) return;

        const nextAuthors = authorEntries.map(([, fallbackUser], index) => {
          const result = profileResponses[index];
          if (result?.status !== "fulfilled") {
            return fallbackUser;
          }

          return {
            ...mapProfileToUser(result.value.profile),
            materialsCount: fallbackUser.materialsCount,
          } satisfies User;
        });

        setAuthors(
          nextAuthors.sort((first, second) =>
            first.name.localeCompare(second.name),
          ),
        );
      } catch (error) {
        if (abortController.signal.aborted) return;

        if (import.meta.env.VITE_API === "mock") {
          console.warn("[Users] Falling back to local mock list.");
          setAuthors(MOCK_AUTHORS);
          setIsError(false);
          return;
        }

        console.error(error);
        setIsError(true);
      } finally {
        if (!abortController.signal.aborted) {
          setIsLoading(false);
        }
      }
    };

    fetchUsers();

    return () => abortController.abort();
  }, []);

  const filteredAuthors = useMemo(
    () =>
      authors.filter((author) =>
        author.name.toLowerCase().includes(searchQuery.toLowerCase()),
      ),
    [authors, searchQuery],
  );

  const handleAuthorClick = (id: string) => {
    navigate(`/authors/${encodeURIComponent(id)}`);
  };

  return (
    <div className="w-full px-4 py-6 xl:w-11/12 mx-auto max-w-screen-2xl space-y-6 lg:space-y-8">
      <div className="flex justify-end">
        <div className="relative w-full lg:w-72">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
          <Input
            placeholder="Поиск..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="pl-9 border-border rounded-lg h-10 lg:h-10"
          />
        </div>
      </div>

      {isLoading ? (
        <div className="col-span-full text-center py-20 text-muted-foreground bg-secondary/10 rounded-xl border border-dashed border-border/50">
          Загружаем пользователей...
        </div>
      ) : isError ? (
        <div className="col-span-full text-center py-20 text-destructive bg-secondary/10 rounded-xl border border-dashed border-border/50">
          Не удалось загрузить пользователей
        </div>
      ) : (
        <AuthorsGrid
          authors={filteredAuthors}
          onAuthorClick={handleAuthorClick}
        />
      )}
    </div>
  );
};

export default AuthorsPage;
