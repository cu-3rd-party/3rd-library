import { Difficulty, DifficultyBadge } from "@/models/material";

export const DIFFICULTY_BADGES: DifficultyBadge[] = [
  {
    value: "none",
    label: "⚪ Без уровня",
  },
  {
    value: "blue",
    label: "🔵 Синий",
  },
  {
    value: "red",
    label: "🔴 Красный",
  },
  {
    value: "black",
    label: "⚫️ Черный",
  },
];

export const DIFFICULTY_CONFIG: Record<
  Difficulty,
  { label: string; className: string }
> = {
  none: {
    label: "Без уровня",
    className:
      "",
  },
  blue: {
    label: "Синий",
    className:
      "bg-blue-badge text-blue-badge-foreground",
  },
  red: {
    label: "Красный",
    className:
      "bg-red-badge text-red-badge-foreground",
  },
  black: {
    label: "Черный",
    className:
      "bg-background text-foreground",
  },
};
