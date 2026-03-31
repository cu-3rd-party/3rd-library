import { useMemo, useState } from "react";

import { ModerationTabs } from "@/common/molecules/Moderation";
import {
  ModerationSubmissionGrid,
  SubmissionReviewDialog,
} from "@/common/organisms/Moderation";
import { ModerationTabValue } from "@/constants";
import { MaterialSubmission, MaterialSubmissionFile } from "@/models";
import { getSubmissionFiles, useMaterialSubmissionStore } from "@/store";
import { openMaterialFile } from "@/utils";

const initialSubmissionCounts: Record<ModerationTabValue, number> = {
  all: 0,
  draft: 0,
  pending_review: 0,
  rejected: 0,
  approved: 0,
};

const ModerationPage = () => {
  const submissions = useMaterialSubmissionStore((state) => state.submissions);
  const approveSubmission = useMaterialSubmissionStore(
    (state) => state.approveSubmission,
  );
  const rejectSubmission = useMaterialSubmissionStore(
    (state) => state.rejectSubmission,
  );

  const [statusFilter, setStatusFilter] =
    useState<ModerationTabValue>("pending_review");
  const [selectedSubmissionId, setSelectedSubmissionId] = useState<
    string | null
  >(null);
  const [rejectComment, setRejectComment] = useState("");

  const filteredSubmissions = useMemo(
    () =>
      submissions.filter((submission) =>
        statusFilter === "all" ? true : submission.status === statusFilter,
      ),
    [statusFilter, submissions],
  );

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

  const submissionCounts = useMemo(
    () =>
      submissions.reduce<Record<ModerationTabValue, number>>(
        (acc, submission) => {
          acc.all += 1;
          acc[submission.status] += 1;
          return acc;
        },
        { ...initialSubmissionCounts },
      ),
    [submissions],
  );

  const closeDialog = () => {
    setSelectedSubmissionId(null);
    setRejectComment("");
  };

  const handleApprove = () => {
    if (!selectedSubmission) return;

    approveSubmission(selectedSubmission.id);
    closeDialog();
  };

  const handleReject = () => {
    if (!selectedSubmission || !rejectComment.trim()) return;

    rejectSubmission(selectedSubmission.id, rejectComment);
    closeDialog();
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

      <ModerationSubmissionGrid
        submissions={filteredSubmissions}
        onSubmissionClick={openSubmission}
      />

      <SubmissionReviewDialog
        submission={selectedSubmission}
        rejectComment={rejectComment}
        onRejectCommentChange={setRejectComment}
        onClose={closeDialog}
        onApprove={handleApprove}
        onReject={handleReject}
        onOpenFile={handleOpenFile}
        files={selectedSubmission ? getSubmissionFiles(selectedSubmission) : []}
      />
    </div>
  );
};

export default ModerationPage;
