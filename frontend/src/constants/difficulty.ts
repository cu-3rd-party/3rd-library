import { Difficulty, DifficultyBadge } from "@/models/material";

export const DIFFICULTY_CONFIG: Record<
  Difficulty,
  { label: string; className: string }
> = {
  none: {
    label: "Без уровня",
    className:
      "bg-secondary text-secondary-foreground hover:bg-secondary/80 border-transparent",
  },
  blue: {
    label: "Синий",
    className:
      "bg-blue-500 dark:bg-blue-500 dark:text-white hover:bg-blue-600 border-transparent shadow-sm shadow-blue-500/20",
  },
  red: {
    label: "Красный",
    className:
      "bg-red-500 dark:text-white, dark:bg-red-500  hover:bg-red-600 border-transparent shadow-sm shadow-red-500/20",
  },
  black: {
    label: "Черный",
    className:
      "bg-black dark:bg-black-500 dark:text-white hover:bg-neutral-900 border-transparent dark:border-white/30 dark:border shadow-sm shadow-black/20",
  },
};

export const DIFFICULTY_BADGES: DifficultyBadge[] = [
  {
    value: "none",
    label: "⚪ Без уровня",
    badgeClass:
      "bg-secondary text-secondary-foreground border-transparent",
    indicatorClass: "border border-foreground/30 bg-transparent",
  },
  {
    value: "blue",
    label: "🔵 Синий",
    badgeClass: "bg-blue-500 text-white border-transparent",
    indicatorClass: "bg-blue-500 border-transparent",
  },
  {
    value: "red",
    label: "🔴 Красный",
    badgeClass: "bg-red-500 text-white border-transparent",
    indicatorClass: "bg-red-500 border-transparent",
  },
  {
    value: "black",
    label: "⚫️ Черный",
    badgeClass:
      "bg-black text-white border-transparent dark:border-white/30 dark:border",
    indicatorClass:
      "bg-black border-transparent dark:bg-neutral-950 dark:border-white/30 dark:border",
  },
];
