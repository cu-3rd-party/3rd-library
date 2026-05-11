import { Button, DialogFooter } from "@/shared/ui";

type SubmissionReviewDialogFooterProps = {
  rejectComment: string;
  isActionLoading: boolean;
  onReject: () => void;
  onApprove: () => void;
};

export const SubmissionReviewDialogFooter = ({
  rejectComment,
  isActionLoading,
  onReject,
  onApprove,
}: SubmissionReviewDialogFooterProps) => (
  <DialogFooter className="shrink-0 border-t border-border bg-background px-4 py-3 pb-[max(0.75rem,env(safe-area-inset-bottom))] sm:px-6 sm:py-4">
    <Button
      variant="destructive"
      onClick={onReject}
      disabled={!rejectComment.trim() || isActionLoading}
      className="w-full sm:w-auto"
    >
      Отклонить
    </Button>
    <Button
      onClick={onApprove}
      className="w-full sm:w-auto"
      disabled={isActionLoading}
    >
      Одобрить
    </Button>
  </DialogFooter>
);
