import { fetchJsonWithAuth } from "@/entities/session/api";
import { LibraryModerationResponse } from "@/entities/submission/api";
import { ModerationTabValue, toModerationQueryStatus } from "@/entities/submission/model";
import { resolveApiUrl } from "@/shared/api";

type FetchModerationSubmissionsParams = {
  status: ModerationTabValue;
  limit?: number;
};

const createSearchParams = ({ status, limit = 100 }: FetchModerationSubmissionsParams) => {
  const searchParams = new URLSearchParams({
    limit: String(limit),
  });
  const queryStatus = toModerationQueryStatus(status);
  if (queryStatus) {
    searchParams.set("status", queryStatus);
  }

  return searchParams;
};

export const fetchModerationSubmissions = ({
  status,
  limit,
}: FetchModerationSubmissionsParams) =>
  fetchJsonWithAuth<LibraryModerationResponse>(
    resolveApiUrl(
      `/api/moderation/submissions?${createSearchParams({ status, limit }).toString()}`,
    ),
  );

export const mapModerationCounters = (response: LibraryModerationResponse) => ({
  all: response.counters.all,
  draft: response.counters.draft,
  pending_review: response.counters.pending_review,
  rejected: response.counters.rejected,
  approved: response.counters.approved,
});
