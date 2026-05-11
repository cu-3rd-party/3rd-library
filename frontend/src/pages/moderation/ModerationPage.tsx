import { useMemo } from "react";

import { getSubmissionFiles, openMaterialFile } from "@/entities/submission/lib";
import {
  SubmissionReviewDialog,
  useModerationDecision,
} from "@/features/moderation-decision";
import {
  ModerationAccessDenied,
  ModerationSubmissionsState,
  ModerationTabs,
  useModerationAccess,
  useModerationSubmissions,
} from "@/features/moderation-submissions";
import { ModerationSubmissionGrid } from "@/widgets/moderation-grid/ui";

const ModerationPage = () => {
  const { hasModeratorAccess } = useModerationAccess();
  const {
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
  } = useModerationSubmissions({ hasModeratorAccess });
  const {
    selectedSubmissionId,
    rejectComment,
    setRejectComment,
    isActionLoading,
    actionErrorMessage,
    closeDialog,
    selectSubmission,
    handleApprove,
    handleReject,
  } = useModerationDecision({
    hasModeratorAccess,
    onSubmissionUpdated: applySubmissionUpdate,
    onReloadSubmissions: loadSubmissions,
  });

  const selectedSubmission = useMemo(
    () => submissions.find((submission) => submission.id === selectedSubmissionId) || null,
    [selectedSubmissionId, submissions],
  );

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

      {!hasModeratorAccess ? <ModerationAccessDenied /> : null}

      {hasModeratorAccess ? (
        <>
          <ModerationTabs
            value={statusFilter}
            counts={submissionCounts}
            onValueChange={setStatusFilter}
          />

          <ModerationSubmissionsState
            isLoading={isLoading}
            isError={isError}
            errorMessage={errorMessage}
          >
            <ModerationSubmissionGrid
              submissions={submissions}
              onSubmissionClick={(submission) => {
                selectSubmission(submission);
                void openSubmissionDetails(submission);
              }}
            />
          </ModerationSubmissionsState>

          <SubmissionReviewDialog
            submission={selectedSubmission}
            rejectComment={rejectComment}
            actionErrorMessage={actionErrorMessage}
            onRejectCommentChange={setRejectComment}
            onClose={() => {
              closeDialog();
              resetSubmissionDetailsLoading();
            }}
            onApprove={() => void handleApprove(selectedSubmission)}
            onReject={() => void handleReject(selectedSubmission)}
            onOpenFile={openMaterialFile}
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
