import { useCallback, useEffect, useMemo, useState } from "react";

import { ModerationTabs } from "@/common/molecules/Moderation";
import {
  ModerationSubmissionGrid,
  SubmissionReviewDialog,
} from "@/common/organisms/Moderation";
import { ModerationTabValue } from "@/constants";
import { fetchJson, resolveApiUrl } from "@/lib/api";
import { MOCK_SUBMISSIONS } from "@/mocks/mockData";
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

type ModerationResponse = {
  items: MaterialSubmission[];
  page: number;
  limit: number;
  total: number;
  counters: Record<ModerationTabValue, number>;
};

const getFallbackModerationResponse = (
  statusFilter: ModerationTabValue,
): ModerationResponse => {
  const items = MOCK_SUBMISSIONS.filter((submission) =>
    statusFilter === "all" ? true : submission.status === statusFilter,
  );
  const counters = MOCK_SUBMISSIONS.reduce<Record<ModerationTabValue, number>>(
    (acc, submission) => {
      acc.all += 1;
      acc[submission.status] += 1;
      return acc;
    },
    { ...initialSubmissionCounts },
  );

  return {
    items,
    page: 1,
    limit: 20,
    total: items.length,
    counters,
  };
};

const ModerationPage = () => {
  const [statusFilter, setStatusFilter] =
    useState<ModerationTabValue>("pending_review");
  const [submissions, setSubmissions] = useState<MaterialSubmission[]>([]);
  const [submissionCounts, setSubmissionCounts] = useState<
    Record<ModerationTabValue, number>
  >({ ...initialSubmissionCounts });
  const [isLoading, setIsLoading] = useState(true);
  const [isError, setIsError] = useState(false);
  const [isActionLoading, setIsActionLoading] = useState(false);
  const [selectedSubmissionId, setSelectedSubmissionId] = useState<
    string | null
  >(null);
  const [rejectComment, setRejectComment] = useState("");

  const loadSubmissions = useCallback(async () => {
    setIsLoading(true);
    setIsError(false);

    try {
      const payload = await fetchJson<ModerationResponse>(
        resolveApiUrl(`/moderation/submissions?status=${statusFilter}`),
      );
      setSubmissions(payload.items);
      setSubmissionCounts(payload.counters);
    } catch (error) {
      if (import.meta.env.VITE_API === "mock") {
        console.warn(
          "[Moderation] Falling back to local mock moderation list.",
        );
        const fallback = getFallbackModerationResponse(statusFilter);
        setSubmissions(fallback.items);
        setSubmissionCounts(fallback.counters);
        setIsError(false);
        return;
      }

      console.error(error);
      setIsError(true);
    } finally {
      setIsLoading(false);
    }
  }, [statusFilter]);

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

  const openSubmission = (submission: MaterialSubmission) => {
    setSelectedSubmissionId(submission.id);
    setRejectComment(submission.moderatorComment);
  };

  const closeDialog = () => {
    setSelectedSubmissionId(null);
    setRejectComment("");
  };

  const handleApprove = async () => {
    if (!selectedSubmission) return;

    setIsActionLoading(true);

    try {
      await fetchJson<MaterialSubmission>(
        resolveApiUrl(
          `/moderation/submissions/${selectedSubmission.id}/decision`,
        ),
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({ action: "approve" }),
        },
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
    if (!selectedSubmission || !rejectComment.trim()) return;

    setIsActionLoading(true);

    try {
      await fetchJson<MaterialSubmission>(
        resolveApiUrl(
          `/moderation/submissions/${selectedSubmission.id}/decision`,
        ),
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            action: "reject",
            moderatorComment: rejectComment.trim(),
          }),
        },
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
          Здесь модераторы просматривают заявки авторов, открывают подробности,
          проверяют файл и решают, публиковать материал или возвращать его на
          доработку.
        </p>
      </div>

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
          Не удалось загрузить список заявок
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
        files={selectedSubmission ? getSubmissionFiles(selectedSubmission) : []}
        isActionLoading={isActionLoading}
      />
    </div>
  );
};

export default ModerationPage;
