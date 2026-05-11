import { fetchJsonWithAuth } from "@/entities/session/api";
import { LibrarySubmission } from "@/entities/submission/api";
import { resolveApiUrl } from "@/shared/api";

export const fetchModerationSubmissionDetails = (submissionId: string) =>
  fetchJsonWithAuth<LibrarySubmission>(
    resolveApiUrl(
      `/api/materials/submissions/${encodeURIComponent(submissionId)}`,
    ),
  );
