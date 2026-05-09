import { create } from "zustand";
import { persist } from "zustand/middleware";

import { Material } from "@/entities/material/model";
import { formatDisplayDate } from "@/shared/lib";

import { getSubmissionFiles, toSubmissionFiles } from "../lib";

import { normalizeSubmission } from "./normalize";
import {
  MaterialSubmission,
  MaterialSubmissionFile,
  MaterialSubmissionInput,
} from "./types";

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

export const useMaterialSubmissionStore = create<MaterialSubmissionStore>()(
  persist(
    (set) => ({
      submissions: [],

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
