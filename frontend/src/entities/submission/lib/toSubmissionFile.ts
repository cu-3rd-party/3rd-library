import { getFileExtension } from "@/shared/lib";

import {
  MaterialSubmissionFile,
  MaterialSubmissionInput,
  MaterialUploadValue,
} from "../model";

const toSubmissionFile = (
  file: MaterialUploadValue,
): MaterialSubmissionFile => {
  if (file instanceof File) {
    return {
      name: file.name,
      sizeBytes: file.size,
      extension: getFileExtension(file.name),
      mimeType: file.type || undefined,
      url: URL.createObjectURL(file),
    };
  }

  return { ...file };
};

export const toSubmissionFiles = (input: MaterialSubmissionInput) => {
  const nextInputFiles =
    input.files.length > 0 ? input.files : input.file ? [input.file] : [];

  return nextInputFiles.map(toSubmissionFile);
};
