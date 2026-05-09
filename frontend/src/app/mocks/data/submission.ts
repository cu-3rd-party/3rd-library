import { MaterialSubmission } from "@/entities/submission/model";
import { DEFAULT_USER_ID } from "@/shared/constants";

import { createApprovedSubmission, createMockFile } from "../factories";

import { MOCK_MATERIALS } from "./material";

export const MOCK_SUBMISSIONS: MaterialSubmission[] = [
  ...MOCK_MATERIALS.map((material, index) =>
    createApprovedSubmission(
      material,
      `2026-01-${String((index % 9) + 10)}T10:00:00.000Z`,
      createMockFile(
        `${material.title}.pdf`,
        (index + 2) * 1024 * 1024,
        "application/pdf",
      ),
    ),
  ),
  {
    id: "pending-submission-demo",
    material: {
      id: "pending-material-demo",
      authorId: DEFAULT_USER_ID,
      authorName: "Даниил Матанович",
      title: "Конспект по линейной алгебре для ревью",
      pubDate: "18.03.2026",
      description:
        "Компактный конспект с определениями, примерами и типовыми ошибками перед контрольной.",
      courses: ["1"],
      subjects: ["Линал"],
      type: "cheatlist",
      difficulty: "blue",
    },
    files: [
      createMockFile(
        "linear-algebra-cheatsheet.pdf",
        1572864,
        "application/pdf",
      ),
    ],
    status: "pending_review",
    moderatorComment: "",
    createdAt: "2026-03-18T08:30:00.000Z",
    updatedAt: "2026-03-18T08:30:00.000Z",
    submittedAt: "2026-03-18T08:30:00.000Z",
  },
  {
    id: "rejected-submission-demo",
    material: {
      id: "rejected-material-demo",
      authorId: DEFAULT_USER_ID,
      authorName: "Даниил Матанович",
      title: "Черновик шпаргалки по диффурам",
      pubDate: "17.03.2026",
      description:
        "Набор коротких формул и алгоритмов решения задач, который автор сможет доработать после замечаний.",
      courses: ["2"],
      subjects: ["Диффуры"],
      type: "cheatlist",
      difficulty: "red",
    },
    files: [
      createMockFile(
        "diffeq-draft.pptx",
        2621440,
        "application/vnd.openxmlformats-officedocument.presentationml.presentation",
      ),
    ],
    status: "rejected",
    moderatorComment:
      "Добавьте более понятное описание и проверьте, что в файле нет пустых слайдов перед повторной отправкой.",
    createdAt: "2026-03-17T12:00:00.000Z",
    updatedAt: "2026-03-19T09:15:00.000Z",
    submittedAt: "2026-03-17T12:00:00.000Z",
    reviewedAt: "2026-03-19T09:15:00.000Z",
  },
];
