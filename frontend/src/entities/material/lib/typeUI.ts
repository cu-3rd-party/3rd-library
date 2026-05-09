import { MaterialType } from "../model";

export const TYPE_CONFIG: Record<
  MaterialType,
  { label: string; className: string }
> = {
  demo: {
    label: "Демо",
    className: "bg-green-badge text-green-badge-foreground",
  },
  longread: {
    label: "Лонгрид",
    className: "bg-blue-badge text-blue-badge-foreground",
  },
  solution: {
    label: "Решение",
    className: "bg-orange-badge text-orange-badge-foreground",
  },
  cheatlist: {
    label: "Читлист",
    className: "bg-violet-badge text-violet-badge-foreground",
  },
  shortread: {
    label: "Шортрид",
    className: "bg-lime-badge text-lime-badge-foreground",
  },
  other: {
    label: "Другое",
    className: "bg-muted text-muted-foreground",
  },
};
