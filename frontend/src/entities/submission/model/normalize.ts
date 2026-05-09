import { getSubmissionFiles } from "../lib";

import { MaterialSubmission } from "./types";

export const normalizeSubmission = (submission: MaterialSubmission) => {
  const files = getSubmissionFiles(submission);

  return {
    ...submission,
    files,
    file: files[0] || null,
  };
};
