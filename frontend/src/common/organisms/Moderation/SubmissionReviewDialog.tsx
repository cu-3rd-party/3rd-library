import { SubmissionStatusBadge } from "@/common/atoms";
import { SubmissionFileSection } from "@/common/molecules";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { MaterialSubmission, MaterialSubmissionFile } from "@/models";

type SubmissionReviewDialogProps = {
  submission: MaterialSubmission | null;
  files: MaterialSubmissionFile[];
  rejectComment: string;
  onRejectCommentChange: (value: string) => void;
  onClose: () => void;
  onApprove: () => void;
  onReject: () => void;
  onOpenFile: (file: MaterialSubmissionFile) => void;
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
}: SubmissionReviewDialogProps) => (
  <Dialog
    open={Boolean(submission)}
    onOpenChange={(open) => !open && onClose()}
  >
    {submission && (
      <DialogContent className="max-h-[calc(100vh-2rem)] max-w-2xl gap-0 overflow-hidden p-0">
        <DialogHeader className="border-b border-border px-6 pt-6 pb-4">
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

        <div className="max-h-[calc(100vh-14rem)] overflow-y-auto px-6 py-6">
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

        <DialogFooter className="shrink-0 border-t border-border bg-background px-6 py-4">
          <Button
            variant="destructive"
            onClick={onReject}
            disabled={!rejectComment.trim()}
          >
            Отклонить
          </Button>
          <Button onClick={onApprove}>Одобрить</Button>
        </DialogFooter>
      </DialogContent>
    )}
  </Dialog>
);
