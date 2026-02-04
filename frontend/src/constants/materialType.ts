import { TypeBadge } from "@/models/material";

export const TYPE_BADGES: TypeBadge[] = [
  {
    value: "demo",
    label: "Демо",
    badgeClass: "bg-blue-badge text-blue-badge-foreground border-transparent",
  },
  {
    value: "longread",
    label: "Лонгрид",
    badgeClass: "bg-secondary text-secondary-foreground border-transparent",
  },
  {
    value: "solution",
    label: "Решение работы",
    badgeClass: "bg-black text-white hover:bg-neutral-900 border-transparent dark:border-white/30 dark:border",
  },
  {
    value: "cheatlist",
    label: "Читлист",
    badgeClass: "bg-secondary text-secondary-foreground hover:bg-secondary/80 border-transparent",
  },
  {
    value: "shortread",
    label: "Шортрид",
    badgeClass: "bg-blue-500 text-white hover:bg-blue-600 border-transparent",
  },
  {
    value: "other",
    label: "Другое",
    badgeClass: "bg-black text-white hover:bg-neutral-900 border-transparent dark:border-white/30 dark:border",
  },
];
