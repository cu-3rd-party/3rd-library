import { Difficulty } from "@/models/material";

export const DIFFICULTY_CONFIG: Record<
  Difficulty,
  { label: string; label_add: string; className: string }
> = {
  none: {
    label: "Без уровня",
    label_add: "⚪ Без уровня",
    className: "",
  },
  blue: {
    label: "Синий",
    label_add: "🔵 Синий",
    className: "bg-blue-badge text-blue-badge-foreground",
  },
  red: {
    label: "Красный",
    label_add: "🔴 Красный",
    className: "bg-red-badge text-red-badge-foreground",
  },
  black: {
    label: "Черный",
    label_add: "⚫️ Черный",
    className: "bg-background text-foreground",
  },
};
