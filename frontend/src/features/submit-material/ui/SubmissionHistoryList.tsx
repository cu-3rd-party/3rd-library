import { MaterialSubmission } from "@/entities/submission/model";
import { SUBMISSION_STATUS_UI } from "@/entities/submission/lib";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
  Badge,
  Button,
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/shared/ui";

import { formatSubmissionDate } from "../lib/helpers";

interface Props {
  submissions: MaterialSubmission[];
  editingSubmissionId: string | null;
  isSubmitting: boolean;
  onEdit: (submission: MaterialSubmission) => void;
}

export const SubmissionHistoryList = ({
  submissions,
  editingSubmissionId,
  isSubmitting,
  onEdit,
}: Props) => (
  <Card className="border-border">
    <Accordion type="single" collapsible>
      <AccordionItem value="history" className="border-b-0">
        <CardHeader className="pb-2">
          <AccordionTrigger className="py-0 hover:no-underline">
            <div className="space-y-1 text-left">
              <CardTitle className="text-lg">
                История заявок ({submissions.length})
              </CardTitle>
              <p className="text-sm text-muted-foreground">
                Здесь отображаются все отправленные материалы и их текущий статус.
              </p>
            </div>
          </AccordionTrigger>
        </CardHeader>

        <AccordionContent>
          <CardContent className="space-y-3 pt-0">
            {submissions.map((submission) => {
              const statusUi = SUBMISSION_STATUS_UI[submission.status];
              const updatedAt = formatSubmissionDate(
                submission.updatedAt || submission.createdAt,
              );
              const isEditingCurrent = editingSubmissionId === submission.id;

              return (
                <div
                  key={`${submission.id}-${submission.updatedAt}`}
                  className="rounded-lg border border-border bg-card/70 p-4 space-y-2"
                >
                  <div className="flex flex-col gap-2 sm:flex-row sm:items-start sm:justify-between">
                    <div className="min-w-0 space-y-1">
                      <p className="break-words font-medium text-foreground">
                        {submission.material.title}
                      </p>
                      <p className="text-xs text-muted-foreground">
                        Обновлено: {updatedAt}
                      </p>
                    </div>
                    <Badge variant="secondary" className={statusUi.badgeClassName}>
                      {statusUi.label}
                    </Badge>
                  </div>

                  {submission.status === "rejected" ? (
                    <div className="rounded-lg border border-red-500/20 bg-destructive/5 p-3 space-y-3">
                      <div>
                        <p className="text-sm font-medium text-foreground">
                          Комментарий модератора
                        </p>
                        <p className="mt-1 text-sm text-muted-foreground break-words">
                          {submission.moderatorComment || "Комментарий отсутствует"}
                        </p>
                      </div>
                      <Button
                        type="button"
                        variant={isEditingCurrent ? "secondary" : "outline"}
                        size="sm"
                        onClick={() => onEdit(submission)}
                        disabled={isSubmitting}
                      >
                        {isEditingCurrent ? "Редактирование открыто" : "Обновить заявку"}
                      </Button>
                    </div>
                  ) : (
                    <p className="text-sm text-muted-foreground">
                      {submission.status === "pending_review"
                        ? "Заявка находится на модерации."
                        : submission.status === "approved"
                          ? "Материал опубликован."
                          : "Черновик доступен для доработки."}
                    </p>
                  )}
                </div>
              );
            })}
          </CardContent>
        </AccordionContent>
      </AccordionItem>
    </Accordion>
  </Card>
);
