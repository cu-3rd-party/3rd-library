import { SubmissionStatus } from "./types";

export type ModerationTabValue = SubmissionStatus | "all";

export type ModerationSubmissionCounts = Record<ModerationTabValue, number>;

export const initialModerationSubmissionCounts: ModerationSubmissionCounts = {
  all: 0,
  draft: 0,
  pending_review: 0,
  rejected: 0,
  approved: 0,
};

export const toModerationQueryStatus = (
  status: ModerationTabValue,
): SubmissionStatus | null => (status === "all" ? null : status);
