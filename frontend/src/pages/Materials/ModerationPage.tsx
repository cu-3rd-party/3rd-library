import { useCallback, useEffect, useMemo, useState } from "react";

import { ModerationTabs } from "@/common/molecules/Moderation";
import {
  ModerationSubmissionGrid,
  SubmissionReviewDialog,
} from "@/common/organisms/Moderation";
import { ModerationTabValue } from "@/constants";
import { ApiRequestError, fetchJson, resolveApiUrl } from "@/lib/api";
import {
  getCurrentAuthUser,
  isModerator,
  subscribeToCurrentAuthUser,
} from "@/lib/currentUser";
import {
  LibraryModerationResponse,
  LibrarySubmission,
  mapLibrarySubmissionToMaterialSubmission,
} from "@/lib/materialsApi";
import { MaterialSubmission, MaterialSubmissionFile } from "@/models";
import { getSubmissionFiles } from "@/store";
import { openMaterialFile } from "@/utils";

const initialSubmissionCounts: Record<ModerationTabValue, number> = {
  all: 0,
  draft: 0,
  pending_review: 0,
  rejected: 0,
  approved: 0,
};

const moderationStatusToQueryValue = (status: ModerationTabValue) =>
  status === "all" ? null : status;

const ModerationPage = () => {
  const [currentAuthUser, setCurrentAuthUser] = useState(() =>
    getCurrentAuthUser(),
  );
  const [statusFilter, setStatusFilter] = useState<ModerationTabValue>("all");
  const [submissions, setSubmissions] = useState<MaterialSubmission[]>([]);
  const [submissionCounts, setSubmissionCounts] = useState<
    Record<ModerationTabValue, number>
  >({ ...initialSubmissionCounts });
  const [isLoading, setIsLoading] = useState(true);
  const [isError, setIsError] = useState(false);
  const [errorMessage, setErrorMessage] = useState("");
  const [isActionLoading, setIsActionLoading] = useState(false);
  const [selectedSubmissionId, setSelectedSubmissionId] = useState<
    string | null
  >(null);
  const [rejectComment, setRejectComment] = useState("");
  const [isSubmissionDetailsLoading, setIsSubmissionDetailsLoading] =
    useState(false);
  const hasModeratorAccess = isModerator(currentAuthUser);

  useEffect(() => subscribeToCurrentAuthUser(setCurrentAuthUser), []);

  const loadSubmissions = useCallback(async () => {
    if (!hasModeratorAccess) {
      setSubmissions([]);
      setSubmissionCounts({ ...initialSubmissionCounts });
      setIsLoading(false);
      setIsError(false);
      return;
    }

    setIsLoading(true);
    setIsError(false);
    setErrorMessage("");

    try {
      const queryStatus = moderationStatusToQueryValue(statusFilter);
      const searchParams = new URLSearchParams({
        limit: "100",
      });
      if (queryStatus) {
        searchParams.set("status", queryStatus);
      }

      const payload = await fetchJson<LibraryModerationResponse>(
        resolveApiUrl(`/api/moderation/submissions?${searchParams.toString()}`),
      );

      const items = payload.items.map(mapLibrarySubmissionToMaterialSubmission);
      setSubmissions(items);
      setSubmissionCounts({
        all: payload.counters.all,
        draft: payload.counters.draft,
        pending_review: payload.counters.pending_review,
        rejected: payload.counters.rejected,
        approved: payload.counters.approved,
      });
    } catch (error) {
      if (error instanceof ApiRequestError) {
        if (error.status === 403) {
          setErrorMessage("Недостаточно прав для доступа к модерации.");
        } else if (error.status === 500) {
          setErrorMessage(
            "Ошибка на сервере модерации. Проверьте схему БД и логи backend.",
          );
        } else {
          setErrorMessage(
            `Не удалось загрузить модерацию (HTTP ${error.status}).`,
          );
        }
      } else {
        setErrorMessage("Не удалось загрузить модерацию.");
      }

      console.error(error);
      setIsError(true);
    } finally {
      setIsLoading(false);
    }
  }, [hasModeratorAccess, statusFilter]);

  useEffect(() => {
    loadSubmissions();
  }, [loadSubmissions]);

  const selectedSubmission = useMemo(
    () =>
      submissions.find(
        (submission) => submission.id === selectedSubmissionId,
      ) || null,
    [selectedSubmissionId, submissions],
  );

  const openSubmission = async (submission: MaterialSubmission) => {
    if (!hasModeratorAccess) return;
    setSelectedSubmissionId(submission.id);
    setRejectComment(submission.moderatorComment);

    setIsSubmissionDetailsLoading(true);

    try {
      const payload = await fetchJson<LibrarySubmission>(
        resolveApiUrl(
          `/api/materials/submissions/${encodeURIComponent(submission.id)}`,
        ),
      );
      const detailedSubmission =
        mapLibrarySubmissionToMaterialSubmission(payload);
      setSubmissions((current) =>
        current.map((item) =>
          item.id === detailedSubmission.id ? detailedSubmission : item,
        ),
      );
    } catch (error) {
      console.warn(
        `[Moderation] Failed to load submission details for ${submission.id}.`,
        error,
      );
    } finally {
      setIsSubmissionDetailsLoading(false);
    }
  };

  const closeDialog = () => {
    setSelectedSubmissionId(null);
    setRejectComment("");
    setIsSubmissionDetailsLoading(false);
  };

  const handleApprove = async () => {
    if (!hasModeratorAccess) return;
    if (!selectedSubmission) return;

    setIsActionLoading(true);

    try {
      const response = await fetchJson<LibrarySubmission>(
        resolveApiUrl(
          `/api/moderation/submissions/${encodeURIComponent(selectedSubmission.id)}/decision`,
        ),
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            action: "approve",
          }),
        },
      );

      const nextSubmission = mapLibrarySubmissionToMaterialSubmission(response);
      setSubmissions((current) =>
        current.map((submission) =>
          submission.id === nextSubmission.id ? nextSubmission : submission,
        ),
      );
      await loadSubmissions();
      closeDialog();
    } catch (error) {
      console.error(error);
      alert("Не удалось одобрить заявку.");
    } finally {
      setIsActionLoading(false);
    }
  };

  const handleReject = async () => {
    if (!hasModeratorAccess) return;
    if (!selectedSubmission || !rejectComment.trim()) return;

    setIsActionLoading(true);

    try {
      const response = await fetchJson<LibrarySubmission>(
        resolveApiUrl(
          `/api/moderation/submissions/${encodeURIComponent(selectedSubmission.id)}/decision`,
        ),
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            action: "reject",
            moderator_comment: rejectComment.trim(),
          }),
        },
      );

      const nextSubmission = mapLibrarySubmissionToMaterialSubmission(response);
      setSubmissions((current) =>
        current.map((submission) =>
          submission.id === nextSubmission.id ? nextSubmission : submission,
        ),
      );
      await loadSubmissions();
      closeDialog();
    } catch (error) {
      console.error(error);
      alert("Не удалось отклонить заявку.");
    } finally {
      setIsActionLoading(false);
    }
  };

  const handleOpenFile = (file: MaterialSubmissionFile) => {
    openMaterialFile(file);
  };

  return (
    <div className="w-full px-4 py-6 xl:w-11/12 mx-auto max-w-screen-2xl space-y-6 lg:space-y-8">
      <div className="space-y-2">
        <h1 className="text-2xl font-bold tracking-tight lg:text-3xl">
          Модерация материалов
        </h1>
        <p className="max-w-3xl text-sm text-muted-foreground lg:text-base">
          Просматривайте заявки и принимайте решение по публикации материалов.
        </p>
      </div>

      {!hasModeratorAccess ? (
        <div className="rounded-xl border border-border bg-card px-6 py-14 text-center text-muted-foreground">
          У вас нет прав модератора для просмотра и обработки заявок.
        </div>
      ) : null}

      {hasModeratorAccess ? (
        <>
          <ModerationTabs
            value={statusFilter}
            counts={submissionCounts}
            onValueChange={setStatusFilter}
          />

          {isLoading ? (
            <div className="rounded-xl border border-border bg-card px-6 py-14 text-center text-muted-foreground">
              Загружаем заявки...
            </div>
          ) : isError ? (
            <div className="rounded-xl border border-border bg-card px-6 py-14 text-center text-destructive">
              {errorMessage || "Не удалось загрузить список заявок"}
            </div>
          ) : (
            <ModerationSubmissionGrid
              submissions={submissions}
              onSubmissionClick={openSubmission}
            />
          )}

          <SubmissionReviewDialog
            submission={selectedSubmission}
            rejectComment={rejectComment}
            onRejectCommentChange={setRejectComment}
            onClose={closeDialog}
            onApprove={handleApprove}
            onReject={handleReject}
            onOpenFile={handleOpenFile}
            files={
              selectedSubmission ? getSubmissionFiles(selectedSubmission) : []
            }
            isActionLoading={isActionLoading || isSubmissionDetailsLoading}
          />
        </>
      ) : null}
    </div>
  );
};

export default ModerationPage;
