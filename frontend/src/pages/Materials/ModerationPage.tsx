import { useCallback, useEffect, useMemo, useState } from "react";

import { ModerationTabs } from "@/common/molecules/Moderation";
import {
  ModerationSubmissionGrid,
  SubmissionReviewDialog,
} from "@/common/organisms/Moderation";
import { ModerationTabValue } from "@/constants";
import { fetchJson, resolveApiUrl } from "@/lib/api";
import {
  mapArticleToMaterial,
  RealWorldArticle,
  RealWorldArticlesResponse,
} from "@/lib/materialsApi";
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

const getFallbackModerationResponse = (statusFilter: ModerationTabValue) => {
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
    counters,
  };
};

const mapArticleToSubmission = (
  article: RealWorldArticle,
): MaterialSubmission => ({
  id: article.slug,
  material: mapArticleToMaterial(article),
  files: [],
  file: null,
  status: "approved",
  moderatorComment: "",
  createdAt: article.createdAt,
  updatedAt: article.updatedAt,
  submittedAt: article.createdAt,
  reviewedAt: article.updatedAt,
  publishedAt: article.updatedAt,
});

const ModerationPage = () => {
  const [statusFilter, setStatusFilter] = useState<ModerationTabValue>("all");
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
      const payload = await fetchJson<RealWorldArticlesResponse>(
        resolveApiUrl("/api/articles?limit=100"),
      );
      const approvedSubmissions = payload.articles
        .map(mapArticleToSubmission)
        .sort(
          (first, second) =>
            new Date(second.updatedAt).getTime() -
            new Date(first.updatedAt).getTime(),
        );
      const items =
        statusFilter === "all" || statusFilter === "approved"
          ? approvedSubmissions
          : [];

      setSubmissions(items);
      setSubmissionCounts({
        ...initialSubmissionCounts,
        all: approvedSubmissions.length,
        approved: approvedSubmissions.length,
      });
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
      if (selectedSubmission.status !== "approved") {
        setSubmissions((current) =>
          current.map((submission) =>
            submission.id === selectedSubmission.id
              ? { ...submission, status: "approved", moderatorComment: "" }
              : submission,
          ),
        );
      }

      alert(
        "В текущем backend модерация недоступна: материалы отображаются как опубликованные.",
      );
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
      setSubmissions((current) =>
        current.map((submission) =>
          submission.id === selectedSubmission.id
            ? {
                ...submission,
                status: "rejected",
                moderatorComment: rejectComment.trim(),
              }
            : submission,
        ),
      );

      alert(
        "В текущем backend нет endpoint для отклонения: изменения сохранены только локально.",
      );
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
          Backend сейчас отдает опубликованные материалы через `/api/articles`,
          поэтому здесь показан список публикаций без серверной модерации.
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
