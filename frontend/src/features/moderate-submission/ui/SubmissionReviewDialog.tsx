import {
  MaterialSubmission,
  MaterialSubmissionFile,
} from "@/entities/submission/model";
import {
  SubmissionFileSection,
  SubmissionStatusBadge,
} from "@/entities/submission/ui";
import {
  Button,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  Label,
  Textarea,
} from "@/shared/ui";

type SubmissionReviewDialogProps = {
  submission: MaterialSubmission | null;
  files: MaterialSubmissionFile[];
  rejectComment: string;
  onRejectCommentChange: (value: string) => void;
  onClose: () => void;
  onApprove: () => void;
  onReject: () => void;
  onOpenFile: (file: MaterialSubmissionFile) => void;
  isActionLoading?: boolean;
};

export const SubmissionReviewDialog = ({
  submission,
  files,
  rejectComment,
  onRejectCommentChange,
  onClose,
  onApprove,
  onReject,
  onOpenFile,
  isActionLoading = false,
}: SubmissionReviewDialogProps) => (
  <Dialog
    open={Boolean(submission)}
    onOpenChange={(open) => !open && onClose()}
  >
    {submission && (
      <DialogContent className="flex h-[calc(100dvh-1rem)] max-h-[calc(100dvh-1rem)] w-[calc(100%-1rem)] max-w-2xl flex-col gap-0 overflow-hidden p-0 sm:h-auto sm:max-h-[calc(100dvh-2rem)] sm:w-full">
        <DialogHeader className="border-b border-border px-4 pt-5 pb-4 sm:px-6 sm:pt-6">
          <div className="flex flex-wrap items-center gap-2 pr-8">
            <DialogTitle className="line-clamp-2">
              {submission.material.title}
            </DialogTitle>
            <SubmissionStatusBadge status={submission.status} />
          </div>
          <DialogDescription>
            Проверьте описание и файл заявки, затем опубликуйте материал или
            верните его автору с комментарием.
          </DialogDescription>
        </DialogHeader>

        <div className="min-h-0 flex-1 overflow-y-auto px-4 py-4 sm:px-6 sm:py-6">
          <div className="space-y-5">
            <p className="text-sm text-muted-foreground">
              Автор:{" "}
              <span className="font-medium text-foreground">
                {submission.material.authorName || "Не указан"}
              </span>
            </p>

            <div className="rounded-xl border border-border bg-muted/30 p-4">
              <p className="text-sm font-medium text-foreground">
                Описание материала
              </p>
              <p className="mt-2 whitespace-pre-wrap text-sm leading-6 text-muted-foreground">
                {submission.material.description}
              </p>
            </div>

            <SubmissionFileSection
              files={files}
              submittedAt={submission.submittedAt}
              onOpenFile={onOpenFile}
            />

            {submission.moderatorComment && (
              <div className="rounded-xl border border-border bg-muted/30 p-4">
                <p className="text-sm font-medium text-foreground">
                  Последний комментарий модератора
                </p>
                <p className="mt-2 whitespace-pre-wrap text-sm leading-6 text-muted-foreground">
                  {submission.moderatorComment}
                </p>
              </div>
            )}

            <div className="space-y-2">
              <Label htmlFor="reject-comment">Комментарий при отклонении</Label>
              <Textarea
                id="reject-comment"
                value={rejectComment}
                onChange={(event) => onRejectCommentChange(event.target.value)}
                placeholder="Напишите, что автору нужно исправить перед повторной отправкой."
                className="min-h-28"
              />
              <p className="text-xs text-muted-foreground">
                Комментарий обязателен только для отклонения и будет показан
                автору при повторной отправке.
              </p>
            </div>
          </div>
        </div>

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
      </DialogContent>
    )}
  </Dialog>
);
