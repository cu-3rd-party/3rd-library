import { create } from "zustand";
import { persist } from "zustand/middleware";

import { MOCK_SUBMISSIONS } from "@/mocks";
import type {
  Material,
  MaterialSubmission,
  MaterialSubmissionFile,
  MaterialSubmissionInput,
  MaterialUploadValue,
} from "@/models";

const formatDisplayDate = (date: Date) =>
  new Intl.DateTimeFormat("ru-RU", {
    day: "2-digit",
    month: "2-digit",
    year: "numeric",
  }).format(date);

const getFileExtension = (fileName: string) => {
  const fileParts = fileName.split(".");
  return fileParts.length > 1
    ? fileParts[fileParts.length - 1].toLowerCase()
    : "";
};

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

export const getSubmissionFiles = (
  submission: Pick<MaterialSubmission, "files" | "file">,
) => {
  if (submission.files?.length) return submission.files;
  return submission.file ? [submission.file] : [];
};

const toSubmissionFiles = (input: MaterialSubmissionInput) => {
  const nextInputFiles =
    input.files.length > 0 ? input.files : input.file ? [input.file] : [];

  return nextInputFiles.map(toSubmissionFile);
};

const normalizeSubmission = (submission: MaterialSubmission) => {
  const files = getSubmissionFiles(submission);

  return {
    ...submission,
    files,
    file: files[0] || null,
  };
};

const revokeRemovedBlobUrls = (
  previousFiles: MaterialSubmissionFile[],
  nextFiles: MaterialSubmissionFile[],
) => {
  const nextUrls = new Set(
    nextFiles
      .map((file) => file.url)
      .filter((url): url is string => Boolean(url)),
  );

  previousFiles.forEach((file) => {
    if (file.url?.startsWith("blob:") && !nextUrls.has(file.url)) {
      URL.revokeObjectURL(file.url);
    }
  });
};

const buildMaterial = (
  existingMaterial: Material | undefined,
  input: MaterialSubmissionInput,
  submittedAt: Date,
): Material => ({
  id: existingMaterial?.id || crypto.randomUUID(),
  authorId: input.authorId,
  authorName: input.authorName,
  title: input.title.trim(),
  description: input.description.trim(),
  courses: [...input.courses].sort(),
  subjects: [...input.subjects].sort(),
  type: input.type,
  difficulty: input.difficulty,
  pubDate: existingMaterial?.pubDate || formatDisplayDate(submittedAt),
});

type MaterialSubmissionStore = {
  submissions: MaterialSubmission[];
  submitSubmission: (input: MaterialSubmissionInput) => string;
  approveSubmission: (id: string) => void;
  rejectSubmission: (id: string, moderatorComment: string) => void;
};

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

export const useMaterialSubmissionStore = create<MaterialSubmissionStore>()(
  persist(
    (set) => ({
      submissions: MOCK_SUBMISSIONS.map(normalizeSubmission),

      submitSubmission: (input) => {
        const submittedAt = new Date();
        const submittedAtIso = submittedAt.toISOString();
        const nextFiles = toSubmissionFiles(input);
        const submissionId = input.id || crypto.randomUUID();

        set((state) => {
          const existingSubmission = state.submissions
            .map(normalizeSubmission)
            .find((submission) => submission.id === submissionId);
          const previousFiles = existingSubmission
            ? getSubmissionFiles(existingSubmission)
            : [];

          const nextSubmission: MaterialSubmission = {
            id: submissionId,
            material: buildMaterial(
              existingSubmission?.material,
              input,
              submittedAt,
            ),
            files: nextFiles,
            file: nextFiles[0] || null,
            status: "pending_review",
            moderatorComment: "",
            createdAt: existingSubmission?.createdAt || submittedAtIso,
            updatedAt: submittedAtIso,
            submittedAt: submittedAtIso,
            reviewedAt: undefined,
            publishedAt: existingSubmission?.publishedAt,
          };

          revokeRemovedBlobUrls(previousFiles, nextFiles);

          const submissions = existingSubmission
            ? state.submissions.map((submission) =>
                submission.id === submissionId ? nextSubmission : submission,
              )
            : [nextSubmission, ...state.submissions];

          return { submissions };
        });

        return submissionId;
      },

      approveSubmission: (id) =>
        set((state) => {
          const approvedAt = new Date();
          const approvedAtIso = approvedAt.toISOString();

          return {
            submissions: state.submissions.map((submission) =>
              submission.id === id
                ? {
                    ...submission,
                    status: "approved",
                    updatedAt: approvedAtIso,
                    reviewedAt: approvedAtIso,
                    publishedAt: approvedAtIso,
                    moderatorComment: "",
                    material: {
                      ...submission.material,
                      pubDate: formatDisplayDate(approvedAt),
                    },
                  }
                : submission,
            ),
          };
        }),

      rejectSubmission: (id, moderatorComment) =>
        set((state) => {
          const reviewedAt = new Date().toISOString();

          return {
            submissions: state.submissions.map((submission) =>
              submission.id === id
                ? {
                    ...submission,
                    status: "rejected",
                    moderatorComment: moderatorComment.trim(),
                    updatedAt: reviewedAt,
                    reviewedAt,
                  }
                : submission,
            ),
          };
        }),
    }),
    {
      name: "material-submission-store",
      version: 2,
      migrate: (persistedState) => {
        if (!persistedState || typeof persistedState !== "object") {
          return persistedState;
        }

        const state = persistedState as { submissions?: MaterialSubmission[] };

        if (!Array.isArray(state.submissions)) {
          return state;
        }

        return {
          ...state,
          submissions: state.submissions.map(normalizeSubmission),
        };
      },
    },
  ),
);
