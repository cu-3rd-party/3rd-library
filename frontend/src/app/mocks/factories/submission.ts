import { Material } from "@/entities/material/model";
import {
  MaterialSubmission,
  MaterialSubmissionFile,
} from "@/entities/submission/model";

export const createApprovedSubmission = (
  material: Material,
  createdAt: string,
  file: MaterialSubmissionFile,
): MaterialSubmission => ({
  id: `submission-${material.id}`,
  material: {
    ...material,
    courses: [...material.courses],
    subjects: [...material.subjects],
  },
  files: [file],
  file,
  status: "approved",
  moderatorComment: "",
  createdAt,
  updatedAt: createdAt,
  submittedAt: createdAt,
  reviewedAt: createdAt,
  publishedAt: createdAt,
});

export const createMockFile = (
  name: string,
  sizeBytes: number,
  mimeType?: string,
): MaterialSubmissionFile => ({
  url: `data:text/plain;charset=utf-8,${encodeURIComponent(
    `Демо-предпросмотр файла: ${name}`,
  )}`,
  name,
  sizeBytes,
  extension: name.split(".").pop()?.toLowerCase() || "",
  mimeType,
});
