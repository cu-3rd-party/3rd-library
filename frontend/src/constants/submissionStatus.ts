import { SubmissionStatus } from "@/models";

export type ModerationTabValue = SubmissionStatus | "all";

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

export const MODERATION_TABS: Array<{
  value: ModerationTabValue;
  label: string;
}> = [
  { value: "pending_review", label: "На ревью" },
  { value: "rejected", label: "Отклоненные" },
  { value: "approved", label: "Опубликованные" },
  { value: "all", label: "Все" },
];
