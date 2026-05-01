import { MOCK_AUTHORS, MOCK_SUBMISSIONS, MOCK_USER } from "@/mocks/mockData";
import { Material, MaterialSubmission, SubmissionStatus, User } from "@/models";

export type SubmissionCounters = Record<SubmissionStatus | "all", number>;

const cloneSubmissionFile = (
  file: MaterialSubmission["files"][number] | null | undefined,
) => (file ? { ...file } : file);

const cloneSubmission = (
  submission: MaterialSubmission,
): MaterialSubmission => ({
  ...submission,
  material: {
    ...submission.material,
    courses: [...submission.material.courses],
    subjects: [...submission.material.subjects],
  },
  files: submission.files.map((file) => ({ ...file })),
  file: cloneSubmissionFile(submission.file) || null,
});

const dedupeUsers = (users: User[]) => {
  const seen = new Set<string>();
  return users.filter((user) => {
    if (seen.has(user.id)) return false;
    seen.add(user.id);
    return true;
  });
};

let usersDb: User[] = dedupeUsers([MOCK_USER, ...MOCK_AUTHORS]).map((user) => ({
  ...user,
}));
let submissionsDb: MaterialSubmission[] = MOCK_SUBMISSIONS.map(cloneSubmission);

export const getUsersDb = () => usersDb.map((user) => ({ ...user }));

export const getUserById = (id: string) =>
  usersDb.find((user) => user.id === id) || null;

export const getSubmissionsDb = () => submissionsDb.map(cloneSubmission);

export const setSubmissionsDb = (next: MaterialSubmission[]) => {
  submissionsDb = next.map(cloneSubmission);
};

export const upsertSubmission = (nextSubmission: MaterialSubmission) => {
  const next = cloneSubmission(nextSubmission);
  const index = submissionsDb.findIndex((item) => item.id === next.id);

  if (index === -1) {
    submissionsDb = [next, ...submissionsDb];
    return;
  }

  submissionsDb = submissionsDb.map((item, itemIndex) =>
    itemIndex === index ? next : item,
  );
};

export const getSubmissionCounters = (
  items: MaterialSubmission[],
): SubmissionCounters =>
  items.reduce<SubmissionCounters>(
    (acc, item) => {
      acc.all += 1;
      acc[item.status] += 1;
      return acc;
    },
    {
      all: 0,
      draft: 0,
      pending_review: 0,
      rejected: 0,
      approved: 0,
    },
  );

export const getPublishedMaterials = () =>
  getSubmissionsDb()
    .filter((submission) => submission.status === "approved")
    .map((submission) => submission.material);

export const findApprovedSubmissionByMaterialId = (materialId: string) =>
  getSubmissionsDb().find(
    (submission) =>
      submission.material.id === materialId && submission.status === "approved",
  ) || null;

export const getUserPublishedMaterials = (userId: string): Material[] =>
  getPublishedMaterials().filter((material) => material.authorId === userId);

export const ensureUserExists = (user: User) => {
  if (usersDb.some((item) => item.id === user.id)) return;
  usersDb = [{ ...user }, ...usersDb];
};
