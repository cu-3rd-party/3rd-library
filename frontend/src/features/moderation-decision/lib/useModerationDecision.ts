import { useCallback, useState } from "react";

import { mapLibrarySubmissionToMaterialSubmission } from "@/entities/submission/lib";
import { MaterialSubmission } from "@/entities/submission/model";

import { sendModerationDecision } from "../api";

type UseModerationDecisionParams = {
  hasModeratorAccess: boolean;
  onSubmissionUpdated: (submission: MaterialSubmission) => void;
  onReloadSubmissions: () => Promise<void>;
};

export const useModerationDecision = ({
  hasModeratorAccess,
  onSubmissionUpdated,
  onReloadSubmissions,
}: UseModerationDecisionParams) => {
  const [selectedSubmissionId, setSelectedSubmissionId] = useState<string | null>(
    null,
  );
  const [rejectComment, setRejectCommentState] = useState("");
  const [isActionLoading, setIsActionLoading] = useState(false);
  const [actionErrorMessage, setActionErrorMessage] = useState("");

  const closeDialog = useCallback(() => {
    setSelectedSubmissionId(null);
    setRejectCommentState("");
    setActionErrorMessage("");
  }, []);

  const selectSubmission = useCallback((submission: MaterialSubmission) => {
    setSelectedSubmissionId(submission.id);
    setRejectCommentState(submission.moderatorComment);
    setActionErrorMessage("");
  }, []);

  const setRejectComment = useCallback((value: string) => {
    setRejectCommentState(value);
    setActionErrorMessage("");
  }, []);

  const handleApprove = useCallback(
    async (submission: MaterialSubmission | null) => {
      if (!hasModeratorAccess || !submission) return;
      setIsActionLoading(true);
      setActionErrorMessage("");

      try {
        const response = await sendModerationDecision({
          submissionId: submission.id,
          action: "approve",
        });
        onSubmissionUpdated(mapLibrarySubmissionToMaterialSubmission(response));
        await onReloadSubmissions();
        closeDialog();
      } catch (error) {
        console.error(error);
        setActionErrorMessage("Не удалось одобрить заявку.");
      } finally {
        setIsActionLoading(false);
      }
    },
    [closeDialog, hasModeratorAccess, onReloadSubmissions, onSubmissionUpdated],
  );

  const handleReject = useCallback(
    async (submission: MaterialSubmission | null) => {
      if (!hasModeratorAccess || !submission || !rejectComment.trim()) return;
      setIsActionLoading(true);
      setActionErrorMessage("");

      try {
        const response = await sendModerationDecision({
          submissionId: submission.id,
          action: "reject",
          moderatorComment: rejectComment,
        });
        onSubmissionUpdated(mapLibrarySubmissionToMaterialSubmission(response));
        await onReloadSubmissions();
        closeDialog();
      } catch (error) {
        console.error(error);
        setActionErrorMessage("Не удалось отклонить заявку.");
      } finally {
        setIsActionLoading(false);
      }
    },
    [
      closeDialog,
      hasModeratorAccess,
      onReloadSubmissions,
      onSubmissionUpdated,
      rejectComment,
    ],
  );

  return {
    selectedSubmissionId,
    rejectComment,
    setRejectComment,
    isActionLoading,
    actionErrorMessage,
    closeDialog,
    selectSubmission,
    handleApprove,
    handleReject,
  };
};
