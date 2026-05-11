import { MaterialSubmission, MaterialSubmissionFile } from "@/entities/submission/model";

export type SubmissionReviewDialogProps = {
  submission: MaterialSubmission | null;
  files: MaterialSubmissionFile[];
  rejectComment: string;
  actionErrorMessage?: string;
  onRejectCommentChange: (value: string) => void;
  onClose: () => void;
  onApprove: () => void;
  onReject: () => void;
  onOpenFile: (file: MaterialSubmissionFile) => void;
  isActionLoading?: boolean;
};
