import { MaterialSubmission } from "../model";

export const getPublishedMaterials = (submissions: MaterialSubmission[]) =>
  submissions
    .filter((submission) => submission.status === "approved")
    .map((submission) => submission.material);

export const getAuthorEditableSubmission = (
  submissions: MaterialSubmission[],
  authorId: string,
) =>
  submissions
    .filter(
      (submission) =>
        submission.material.authorId === authorId &&
        (submission.status === "draft" || submission.status === "rejected"),
    )
    .sort(
      (first, second) =>
        new Date(second.updatedAt).getTime() -
        new Date(first.updatedAt).getTime(),
    )[0];

export const getAuthorLatestSubmission = (
  submissions: MaterialSubmission[],
  authorId: string,
) =>
  submissions
    .filter((submission) => submission.material.authorId === authorId)
    .sort(
      (first, second) =>
        new Date(second.updatedAt).getTime() -
        new Date(first.updatedAt).getTime(),
    )[0];
