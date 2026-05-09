import { SubmissionStatus } from "../model";

type SubmissionStatusUiConfig = {
  label: string;
  badgeClassName: string;
};

export const SUBMISSION_STATUS_UI: Record<
  SubmissionStatus,
  SubmissionStatusUiConfig
> = {
  draft: {
    label: "Черновик",
    badgeClassName: "bg-muted text-muted-foreground",
  },
  pending_review: {
    label: "На модерации",
    badgeClassName: "bg-orange-badge text-orange-badge-foreground",
  },
  rejected: {
    label: "Отклонено",
    badgeClassName: "bg-red-badge text-red-badge-foreground",
  },
  approved: {
    label: "Опубликовано",
    badgeClassName: "bg-green-badge text-green-badge-foreground",
  },
};
