import { LibraryMaterialFile } from "@/entities/material/api";
import { mapLibraryMaterialToMaterial } from "@/entities/material/lib";
import { formatDisplayDate } from "@/shared/lib";

import { LibrarySubmission } from "../api/types";
import { MaterialSubmission, MaterialSubmissionFile } from "../model";

export const mapLibraryMaterialFileToSubmissionFile = (
  file: LibraryMaterialFile,
): MaterialSubmissionFile => ({
  id: file.id,
  name: file.name,
  sizeBytes: file.size_bytes,
  extension: file.extension,
  url: file.url || undefined,
});

const normalizeSubmissionStatus = (
  status: string,
): MaterialSubmission["status"] => {
  if (status === "draft") return "draft";
  if (status === "pending_review") return "pending_review";
  if (status === "rejected") return "rejected";
  return "approved";
};

export const mapLibrarySubmissionToMaterialSubmission = (
  submission: LibrarySubmission,
): MaterialSubmission => {
  const mappedMaterial = mapLibraryMaterialToMaterial(submission.material);
  const submittedAt = submission.submitted_at || undefined;
  const createdAt = submission.created_at;
  const updatedAt = submission.updated_at;
  const publishedAt = submission.published_at || undefined;

  const displayDateSource =
    submittedAt || createdAt || updatedAt || publishedAt || "";

  return {
    id: submission.id,
    material: {
      ...mappedMaterial,
      pubDate: formatDisplayDate(displayDateSource),
    },
    files: submission.files.map(mapLibraryMaterialFileToSubmissionFile),
    file: submission.files[0]
      ? mapLibraryMaterialFileToSubmissionFile(submission.files[0])
      : null,
    status: normalizeSubmissionStatus(submission.status),
    moderatorComment: submission.moderator_comment || "",
    createdAt,
    updatedAt,
    submittedAt,
    reviewedAt: submission.reviewed_at || undefined,
    publishedAt,
  };
};
