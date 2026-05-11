import { useCallback, useEffect, useState } from "react";

import { mapLibrarySubmissionToMaterialSubmission } from "@/entities/submission/lib";
import {
  initialModerationSubmissionCounts,
  MaterialSubmission,
  ModerationSubmissionCounts,
  ModerationTabValue,
} from "@/entities/submission/model";

import {
  fetchModerationSubmissionDetails,
  fetchModerationSubmissions,
  mapModerationCounters,
} from "../api";

import { getModerationLoadErrorMessage } from "./getModerationLoadErrorMessage";

type UseModerationSubmissionsParams = {
  hasModeratorAccess: boolean;
};

export const useModerationSubmissions = ({
  hasModeratorAccess,
}: UseModerationSubmissionsParams) => {
  const [statusFilter, setStatusFilter] = useState<ModerationTabValue>("all");
  const [submissions, setSubmissions] = useState<MaterialSubmission[]>([]);
  const [submissionCounts, setSubmissionCounts] = useState<ModerationSubmissionCounts>(
    { ...initialModerationSubmissionCounts },
  );
  const [isLoading, setIsLoading] = useState(true);
  const [isError, setIsError] = useState(false);
  const [errorMessage, setErrorMessage] = useState("");
  const [isSubmissionDetailsLoading, setIsSubmissionDetailsLoading] =
    useState(false);

  const loadSubmissions = useCallback(async () => {
    if (!hasModeratorAccess) {
      setSubmissions([]);
      setSubmissionCounts({ ...initialModerationSubmissionCounts });
      setIsLoading(false);
      setIsError(false);
      setErrorMessage("");
      return;
    }

    setIsLoading(true);
    setIsError(false);
    setErrorMessage("");

    try {
      const payload = await fetchModerationSubmissions({ status: statusFilter });
      setSubmissions(payload.items.map(mapLibrarySubmissionToMaterialSubmission));
      setSubmissionCounts(mapModerationCounters(payload));
    } catch (error) {
      console.error(error);
      setErrorMessage(getModerationLoadErrorMessage(error));
      setIsError(true);
    } finally {
      setIsLoading(false);
    }
  }, [hasModeratorAccess, statusFilter]);

  const applySubmissionUpdate = useCallback((nextSubmission: MaterialSubmission) => {
    setSubmissions((current) =>
      current.map((submission) =>
        submission.id === nextSubmission.id ? nextSubmission : submission,
      ),
    );
  }, []);

  const openSubmissionDetails = useCallback(
    async (submission: MaterialSubmission) => {
      if (!hasModeratorAccess) return;
      setIsSubmissionDetailsLoading(true);

      try {
        const payload = await fetchModerationSubmissionDetails(submission.id);
        applySubmissionUpdate(mapLibrarySubmissionToMaterialSubmission(payload));
      } catch (error) {
        console.warn(
          `[Moderation] Failed to load submission details for ${submission.id}.`,
          error,
        );
      } finally {
        setIsSubmissionDetailsLoading(false);
      }
    },
    [applySubmissionUpdate, hasModeratorAccess],
  );

  const resetSubmissionDetailsLoading = useCallback(() => {
    setIsSubmissionDetailsLoading(false);
  }, []);

  useEffect(() => {
    void loadSubmissions();
  }, [loadSubmissions]);

  return {
    statusFilter,
    setStatusFilter,
    submissions,
    submissionCounts,
    isLoading,
    isError,
    errorMessage,
    isSubmissionDetailsLoading,
    loadSubmissions,
    applySubmissionUpdate,
    openSubmissionDetails,
    resetSubmissionDetailsLoading,
  };
};
