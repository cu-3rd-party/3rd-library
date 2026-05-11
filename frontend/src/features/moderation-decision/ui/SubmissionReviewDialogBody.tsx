import {
  SubmissionFileSection,
  SubmissionStatusBadge,
} from "@/entities/submission/ui";
import {
  DialogDescription,
  DialogHeader,
  DialogTitle,
  Label,
  Textarea,
} from "@/shared/ui";

import { SubmissionReviewDialogProps } from "./SubmissionReviewDialog.types";

type SubmissionReviewDialogBodyProps = Pick<
  SubmissionReviewDialogProps,
  | "submission"
  | "files"
  | "rejectComment"
  | "actionErrorMessage"
  | "onRejectCommentChange"
  | "onOpenFile"
>;

export const SubmissionReviewDialogBody = ({
  submission,
  files,
  rejectComment,
  actionErrorMessage,
  onRejectCommentChange,
  onOpenFile,
}: SubmissionReviewDialogBodyProps) => {
  if (!submission) return null;

  return (
    <>
      <DialogHeader className="border-b border-border px-4 pt-5 pb-4 sm:px-6 sm:pt-6">
        <div className="flex flex-wrap items-center gap-2 pr-8">
          <DialogTitle className="line-clamp-2">{submission.material.title}</DialogTitle>
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
            <p className="text-sm font-medium text-foreground">Описание материала</p>
            <p className="mt-2 whitespace-pre-wrap text-sm leading-6 text-muted-foreground">
              {submission.material.description}
            </p>
          </div>

          <SubmissionFileSection
            files={files}
            submittedAt={submission.submittedAt}
            onOpenFile={onOpenFile}
          />

          {submission.moderatorComment ? (
            <div className="rounded-xl border border-border bg-muted/30 p-4">
              <p className="text-sm font-medium text-foreground">
                Последний комментарий модератора
              </p>
              <p className="mt-2 whitespace-pre-wrap text-sm leading-6 text-muted-foreground">
                {submission.moderatorComment}
              </p>
            </div>
          ) : null}

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
              Комментарий обязателен только для отклонения и будет показан автору
              при повторной отправке.
            </p>
            {actionErrorMessage ? (
              <p className="text-sm text-destructive">{actionErrorMessage}</p>
            ) : null}
          </div>
        </div>
      </div>
    </>
  );
};
