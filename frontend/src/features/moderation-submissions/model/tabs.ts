import { ModerationTabValue } from "@/entities/submission/model";

export const MODERATION_TABS: Array<{
  value: ModerationTabValue;
  label: string;
}> = [
  { value: "pending_review", label: "На ревью" },
  { value: "rejected", label: "Отклоненные" },
  { value: "approved", label: "Опубликованные" },
  { value: "all", label: "Все" },
];
