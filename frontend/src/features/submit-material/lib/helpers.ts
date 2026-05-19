import { MOCK_USER } from "@/app/mocks/data/auth";
import { RealWorldArticle } from "@/entities/material/api";
import { mapArticleToMaterial } from "@/entities/material/lib";
import { getSubmissionFiles } from "@/entities/submission/lib";
import {
  MaterialSubmission,
  MaterialSubmissionFile,
  MaterialUploadValue,
} from "@/entities/submission/model";
import { ApiRequestError } from "@/shared/api";

import { UploadMaterialFormValues } from "../ui";

export const emptyFormValues: UploadMaterialFormValues = {
  title: "",
  description: "",
  files: [],
  courses: [],
  subjects: [],
  difficulty: "none",
  type: "longread",
};

export const resolveCurrentUserId = (): string => {
  const payload = globalThis.localStorage?.getItem("authUser");
  if (!payload) return MOCK_USER.id;
  try {
    const parsed = JSON.parse(payload) as { username?: string };
    return parsed.username?.trim() || MOCK_USER.id;
  } catch {
    return MOCK_USER.id;
  }
};

export const isBrowserFile = (file: MaterialUploadValue): file is File =>
  file instanceof File;

export const isStoredSubmissionFile = (
  file: MaterialUploadValue,
): file is MaterialSubmissionFile => !isBrowserFile(file);

export const createApiSubmission = (
  article: RealWorldArticle,
  files: MaterialSubmissionFile[] = [],
): MaterialSubmission => ({
  id: article.slug,
  material: mapArticleToMaterial(article),
  files,
  file: files[0] || null,
  status: "approved",
  moderatorComment: "",
  createdAt: article.createdAt,
  updatedAt: article.updatedAt,
  submittedAt: article.createdAt,
  reviewedAt: article.updatedAt,
  publishedAt: article.updatedAt,
});

export const toArticleTags = (values: UploadMaterialFormValues): string[] => [
  ...values.courses.map((course) => `course:${course}`),
  ...values.subjects.map((subject) => `subject:${subject}`),
  `type:${values.type}`,
  `difficulty:${values.difficulty}`,
];

export const createFormValues = (
  submission: MaterialSubmission | null | undefined,
): UploadMaterialFormValues =>
  submission
    ? {
        title: submission.material.title,
        description: submission.material.description,
        files: [...getSubmissionFiles(submission)],
        courses: [...submission.material.courses],
        subjects: [...submission.material.subjects],
        difficulty: submission.material.difficulty,
        type: submission.material.type,
      }
    : emptyFormValues;

export const formatSubmissionDate = (value: string): string => {
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) return "Дата не указана";
  return new Intl.DateTimeFormat("ru-RU", {
    day: "2-digit",
    month: "2-digit",
    year: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  }).format(date);
};

export const getSubmitErrorMessage = (error: unknown): string => {
  if (error instanceof ApiRequestError) {
    if (error.status === 413)
      return "Размер одного из файлов превышает лимит сервера. Уменьшите файл и попробуйте снова.";
    if (error.status === 422) {
      const details = typeof error.payload === "string" ? error.payload.trim() : "";
      return details
        ? `Не удалось отправить материал: ${details}.`
        : "Сервер не принял данные заявки. Проверьте название и описание, затем повторите попытку.";
    }
  }
  return "Не удалось отправить материал. Попробуйте еще раз.";
};

export const getUpdateErrorMessage = (error: unknown): string => {
  if (error instanceof ApiRequestError) {
    if (error.status === 403) return "Эту заявку нельзя обновить в текущем статусе.";
    if (error.status === 400 || error.status === 422) {
      const details = typeof error.payload === "string" ? error.payload.trim() : "";
      return details
        ? `Не удалось обновить заявку: ${details}.`
        : "Сервер не принял изменения. Проверьте данные и повторите попытку.";
    }
  }
  return "Не удалось обновить заявку. Попробуйте еще раз.";
};
