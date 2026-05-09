import { SUBJECTS } from "../model";

export const normalizeTag = (tag: string) => tag.trim().toLowerCase();

export const findTagValues = (tags: string[], prefix: string) =>
  tags
    .map(normalizeTag)
    .filter((tag) => tag.startsWith(`${prefix}:`))
    .map((tag) => tag.slice(prefix.length + 1))
    .filter(Boolean);

export const matchSubjectByTag = (tag: string) => {
  const normalized = normalizeTag(tag);
  if (!normalized) return null;

  const direct = SUBJECTS.find(
    (subject) => subject.toLowerCase() === normalized,
  );
  if (direct) return direct;

  if (normalized.includes("math") || normalized.includes("мат")) {
    return "Матан";
  }
  if (normalized.includes("lin") || normalized.includes("линал")) {
    return "Линал";
  }
  if (normalized.includes("diff") || normalized.includes("дифф")) {
    return "Диффуры";
  }
  if (normalized.includes("algo") || normalized.includes("алго")) {
    return "Алгоритмы";
  }
  if (normalized.includes("phys") || normalized.includes("физ")) {
    return "Физика";
  }
  if (normalized.includes("eng") || normalized.includes("англ")) {
    return "Английский";
  }
  if (normalized.includes("geom") || normalized.includes("ангем")) {
    return "Ангем";
  }

  return null;
};
