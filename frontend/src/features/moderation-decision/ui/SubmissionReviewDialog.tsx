import { Dialog, DialogContent } from "@/shared/ui";

import { SubmissionReviewDialogProps } from "./SubmissionReviewDialog.types";
import { SubmissionReviewDialogBody } from "./SubmissionReviewDialogBody";
import { SubmissionReviewDialogFooter } from "./SubmissionReviewDialogFooter";

export const SubmissionReviewDialog = ({
  submission,
  files,
  rejectComment,
  actionErrorMessage,
  onRejectCommentChange,
  onClose,
  onApprove,
  onReject,
  onOpenFile,
  isActionLoading = false,
}: SubmissionReviewDialogProps) => (
  <Dialog open={Boolean(submission)} onOpenChange={(open) => !open && onClose()}>
    {submission ? (
      <DialogContent className="flex h-[calc(100dvh-1rem)] max-h-[calc(100dvh-1rem)] w-[calc(100%-1rem)] max-w-2xl flex-col gap-0 overflow-hidden p-0 sm:h-auto sm:max-h-[calc(100dvh-2rem)] sm:w-full">
        <SubmissionReviewDialogBody
          submission={submission}
          files={files}
          rejectComment={rejectComment}
          actionErrorMessage={actionErrorMessage}
          onRejectCommentChange={onRejectCommentChange}
          onOpenFile={onOpenFile}
        />
        <SubmissionReviewDialogFooter
          rejectComment={rejectComment}
          isActionLoading={isActionLoading}
          onReject={onReject}
          onApprove={onApprove}
        />
      </DialogContent>
    ) : null}
  </Dialog>
);
