import { MOCK_SUBMISSIONS } from "@/app/mocks/data/submission";
import { MOCK_USER } from "@/app/mocks/data/auth";
import { RealWorldArticlesResponse } from "@/entities/material/api";
import { fetchJsonWithAuth } from "@/entities/session/api";
import { LibraryPaginatedSubmissionsResponse } from "@/entities/submission/api";
import {
  mapLibrarySubmissionToMaterialSubmission,
} from "@/entities/submission/lib";
import { MaterialSubmission } from "@/entities/submission/model";
import { LibraryCurrentUserResponse } from "@/entities/user/api";
import { ApiRequestError, resolveApiUrl } from "@/shared/api";

import { createApiSubmission } from "./helpers";

export type FetchSubmissionsResult = { userId: string; submissions: MaterialSubmission[] };

const sortByDate = (items: MaterialSubmission[]): MaterialSubmission[] =>
  [...items].sort(
    (a, b) => new Date(b.updatedAt).getTime() - new Date(a.updatedAt).getTime(),
  );

export const fetchUserSubmissions = async (
  signal: AbortSignal,
): Promise<FetchSubmissionsResult> => {
  try {
    try {
      const userPayload = await fetchJsonWithAuth<LibraryCurrentUserResponse>(
        resolveApiUrl("/api/users/me"),
        { signal },
      );

      try {
        const payload = await fetchJsonWithAuth<LibraryPaginatedSubmissionsResponse>(
          resolveApiUrl("/api/materials/submissions?limit=100"),
          { signal },
        );
        return {
          userId: userPayload.id,
          submissions: sortByDate(payload.items.map(mapLibrarySubmissionToMaterialSubmission)),
        };
      } catch (error) {
        if (error instanceof ApiRequestError && error.status === 500)
          return { userId: userPayload.id, submissions: [] };
        throw error;
      }
    } catch (error) {
      if (!(error instanceof ApiRequestError) || error.status !== 404) throw error;
    }

    const { user } = await fetchJsonWithAuth<{ user: { username: string } }>(
      resolveApiUrl("/api/user"),
      { signal },
    );

    const { articles } = await fetchJsonWithAuth<RealWorldArticlesResponse>(
      resolveApiUrl(`/api/articles?author=${encodeURIComponent(user.username)}&limit=100`),
      { signal },
    );

    return {
      userId: user.username,
      submissions: sortByDate(articles.map((article) => createApiSubmission(article))),
    };
  } catch (error) {
    if (signal.aborted) throw error;

    if (import.meta.env.VITE_API === "mock") {
      console.warn("[Upload] Falling back to local submissions list.");
      return {
        userId: MOCK_USER.id,
        submissions: MOCK_SUBMISSIONS.filter((s) => s.material.authorId === MOCK_USER.id),
      };
    }

    throw error;
  }
};
