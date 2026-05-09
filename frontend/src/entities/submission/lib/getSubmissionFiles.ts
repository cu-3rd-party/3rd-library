import { MaterialSubmission } from "../model";

export const getSubmissionFiles = (
  submission: Pick<MaterialSubmission, "files" | "file">,
) => {
  if (submission.files?.length) return submission.files;
  return submission.file ? [submission.file] : [];
};
