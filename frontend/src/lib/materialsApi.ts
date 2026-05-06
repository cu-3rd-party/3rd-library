import { COURSES, DIFFICULTIES, MATERIAL_TYPES, SUBJECTS } from "@/constants";
import { resolveApiUrl } from "@/lib/api";
import { Material, MaterialSubmission, MaterialSubmissionFile, User } from "@/models";

type RealWorldProfile = {
  username: string;
  bio: string;
  image: string | null;
  following: boolean;
};

export type RealWorldArticle = {
  slug: string;
  title: string;
  description: string;
  body?: string;
  tagList: string[];
  createdAt: string;
  updatedAt: string;
  favorited: boolean;
  favoritesCount: number;
  author: RealWorldProfile;
};

export type RealWorldArticlesResponse = {
  articles: RealWorldArticle[];
  articlesCount: number;
};

export type RealWorldArticleResponse = {
  article: RealWorldArticle;
};

export type RealWorldProfileResponse = {
  profile: RealWorldProfile;
};

export type LibraryUserPublicProfile = {
  id: string;
  name: string;
  bio: string;
  image?: string | null;
  is_email_verified: boolean;
  materials_count: number;
};

export type LibraryUsersResponse = {
  items: LibraryUserPublicProfile[];
  page: number;
  limit: number;
  total: number;
};

export type LibraryCurrentUserResponse = {
  id: string;
  name: string;
  email: string;
  bio: string;
  image?: string | null;
  is_email_verified: boolean;
  can_submit_materials: boolean;
  roles: string[];
};

export type LibraryMaterial = {
  id: string;
  author_id: string;
  author_name?: string | null;
  title: string;
  description: string;
  courses: string[];
  subjects: string[];
  type: string;
  difficulty: string;
  pub_date?: string | null;
};

export type LibraryMaterialFile = {
  id: string;
  name: string;
  size_bytes: number;
  extension: string;
  mime_type?: string | null;
  url?: string | null;
};

export type LibraryPaginatedMaterialsResponse = {
  items: LibraryMaterial[];
  page: number;
  limit: number;
  total: number;
};

export type LibraryMaterialDetailsResponse = {
  id: string;
  author_id: string;
  author_name?: string | null;
  author_image?: string | null;
  title: string;
  description: string;
  courses: string[];
  subjects: string[];
  type: string;
  difficulty: string;
  pub_date?: string | null;
  files: LibraryMaterialFile[];
  published_at?: string | null;
  submitted_at?: string | null;
};

export type LibraryUserWithMaterialsResponse = {
  user: LibraryUserPublicProfile;
  materials: LibraryPaginatedMaterialsResponse;
};

export type LibrarySubmission = {
  id: string;
  material: LibraryMaterial;
  files: LibraryMaterialFile[];
  status: string;
  moderator_comment: string;
  created_at: string;
  updated_at: string;
  submitted_at?: string | null;
  reviewed_at?: string | null;
  published_at?: string | null;
};

export type LibraryPaginatedSubmissionsResponse = {
  items: LibrarySubmission[];
  page: number;
  limit: number;
  total: number;
};

export type LibraryModerationCounters = {
  all: number;
  draft: number;
  pending_review: number;
  rejected: number;
  approved: number;
};

export type LibraryModerationResponse = {
  items: LibrarySubmission[];
  page: number;
  limit: number;
  total: number;
  counters: LibraryModerationCounters;
};

export type RealWorldAttachment = {
  attachmentId: string;
  fileName: string;
  url?: string | null;
  downloadUrl?: string | null;
  createdAt: string;
  updatedAt: string;
};

export type RealWorldAttachmentsResponse = {
  attachments: RealWorldAttachment[];
};

const DEFAULT_COURSE: (typeof COURSES)[number] = "1";
const DEFAULT_SUBJECT: (typeof SUBJECTS)[number] = "Алгоритмы";
const DEFAULT_TYPE: (typeof MATERIAL_TYPES)[number] = "longread";
const DEFAULT_DIFFICULTY: (typeof DIFFICULTIES)[number] = "none";

const formatDisplayDate = (value: string) => {
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) return "01.01.2026";

  return new Intl.DateTimeFormat("ru-RU", {
    day: "2-digit",
    month: "2-digit",
    year: "numeric",
  }).format(date);
};

const normalizeTag = (tag: string) => tag.trim().toLowerCase();

const findTagValues = (tags: string[], prefix: string) =>
  tags
    .map(normalizeTag)
    .filter((tag) => tag.startsWith(`${prefix}:`))
    .map((tag) => tag.slice(prefix.length + 1))
    .filter(Boolean);

const matchSubjectByTag = (tag: string) => {
  const normalized = normalizeTag(tag);
  if (!normalized) return null;

  const direct = SUBJECTS.find(
    (subject) => subject.toLowerCase() === normalized,
  );
  if (direct) return direct;

  if (normalized.includes("math") || normalized.includes("мат")) {
    return "Матан";
  }
  if (normalized.includes("lin") || normalized.includes("линал")) {
    return "Линал";
  }
  if (normalized.includes("diff") || normalized.includes("дифф")) {
    return "Диффуры";
  }
  if (normalized.includes("algo") || normalized.includes("алго")) {
    return "Алгоритмы";
  }
  if (normalized.includes("phys") || normalized.includes("физ")) {
    return "Физика";
  }
  if (normalized.includes("eng") || normalized.includes("англ")) {
    return "Английский";
  }
  if (normalized.includes("geom") || normalized.includes("ангем")) {
    return "Ангем";
  }

  return null;
};

const deriveSubjects = (tags: string[]) => {
  const subjectsFromTags = tags
    .map(matchSubjectByTag)
    .filter((subject): subject is (typeof SUBJECTS)[number] =>
      Boolean(subject),
    );

  const subjectsFromPrefix = findTagValues(tags, "subject")
    .map((value) => SUBJECTS.find((subject) => subject.toLowerCase() === value))
    .filter((subject): subject is (typeof SUBJECTS)[number] =>
      Boolean(subject),
    );

  const subjects = Array.from(
    new Set([...subjectsFromPrefix, ...subjectsFromTags]),
  );
  if (subjects.length === 0) return [DEFAULT_SUBJECT];
  return subjects;
};

const deriveCourse = (tags: string[]) => {
  const courseFromTag = findTagValues(tags, "course").find((value) =>
    COURSES.includes(value as (typeof COURSES)[number]),
  );
  if (courseFromTag) {
    return courseFromTag as (typeof COURSES)[number];
  }

  const hasSecondCourse = tags.some((tag) => {
    const normalized = normalizeTag(tag);
    return (
      normalized.includes("course2") ||
      normalized.includes("2курс") ||
      normalized.includes("курс2") ||
      normalized.includes("ii")
    );
  });

  return hasSecondCourse ? "2" : DEFAULT_COURSE;
};

const deriveType = (tags: string[]) => {
  const typeFromTag = findTagValues(tags, "type").find((value) =>
    MATERIAL_TYPES.includes(value as (typeof MATERIAL_TYPES)[number]),
  );
  if (typeFromTag) {
    return typeFromTag as (typeof MATERIAL_TYPES)[number];
  }

  const normalizedTags = tags.map(normalizeTag);

  if (normalizedTags.some((tag) => tag.includes("demo"))) return "demo";
  if (
    normalizedTags.some(
      (tag) => tag.includes("solution") || tag.includes("решение"),
    )
  ) {
    return "solution";
  }
  if (
    normalizedTags.some(
      (tag) => tag.includes("cheat") || tag.includes("шпаргал"),
    )
  ) {
    return "cheatlist";
  }
  if (
    normalizedTags.some(
      (tag) => tag.includes("short") || tag.includes("shortread"),
    )
  ) {
    return "shortread";
  }
  if (
    normalizedTags.some((tag) => tag.includes("long") || tag.includes("guide"))
  ) {
    return "longread";
  }

  return DEFAULT_TYPE;
};

const deriveDifficulty = (favoritesCount: number) => {
  if (favoritesCount >= 15) return "black";
  if (favoritesCount >= 8) return "red";
  if (favoritesCount >= 3) return "blue";
  return DEFAULT_DIFFICULTY;
};

const deriveDifficultyFromTags = (
  tags: string[],
  favoritesCount: number,
): (typeof DIFFICULTIES)[number] => {
  const difficultyFromTag = findTagValues(tags, "difficulty").find((value) =>
    DIFFICULTIES.includes(value as (typeof DIFFICULTIES)[number]),
  );
  if (difficultyFromTag) {
    return difficultyFromTag as (typeof DIFFICULTIES)[number];
  }

  return deriveDifficulty(favoritesCount);
};

const getFileExtension = (fileName: string) => {
  const fileNameParts = fileName.split(".");
  return fileNameParts.length > 1
    ? fileNameParts[fileNameParts.length - 1].toLowerCase()
    : "";
};

const normalizeCourse = (
  value: string,
): (typeof COURSES)[number] | null =>
  COURSES.includes(value as (typeof COURSES)[number])
    ? (value as (typeof COURSES)[number])
    : null;

const normalizeSubject = (
  value: string,
): (typeof SUBJECTS)[number] | null =>
  SUBJECTS.includes(value as (typeof SUBJECTS)[number])
    ? (value as (typeof SUBJECTS)[number])
    : null;

const normalizeMaterialType = (
  value: string,
): (typeof MATERIAL_TYPES)[number] =>
  MATERIAL_TYPES.includes(value as (typeof MATERIAL_TYPES)[number])
    ? (value as (typeof MATERIAL_TYPES)[number])
    : "other";

const normalizeDifficulty = (
  value: string,
): (typeof DIFFICULTIES)[number] =>
  DIFFICULTIES.includes(value as (typeof DIFFICULTIES)[number])
    ? (value as (typeof DIFFICULTIES)[number])
    : DEFAULT_DIFFICULTY;

export const mapArticleToMaterial = (article: RealWorldArticle): Material => {
  const course = deriveCourse(article.tagList);
  const subjects = deriveSubjects(article.tagList);

  return {
    id: article.slug,
    authorId: article.author.username,
    authorName: article.author.username,
    title: article.title,
    description: article.description || article.body || "",
    courses: [course],
    subjects,
    type: deriveType(article.tagList),
    difficulty: deriveDifficultyFromTags(
      article.tagList,
      article.favoritesCount,
    ),
    pubDate: formatDisplayDate(article.updatedAt || article.createdAt),
  };
};

export const mapLibraryUserToUser = (user: LibraryUserPublicProfile): User => ({
  id: user.id,
  name: user.name,
  bio: user.bio || "",
  image: user.image || null,
  verified: user.is_email_verified,
  isEmailVerified: user.is_email_verified,
  materialsCount: user.materials_count,
});

export const mapLibraryMaterialToMaterial = (
  material: LibraryMaterial,
): Material => {
  const courses = material.courses
    .map((course) => normalizeCourse(course))
    .filter((course): course is (typeof COURSES)[number] => Boolean(course));
  const subjects = material.subjects
    .map((subject) => normalizeSubject(subject))
    .filter((subject): subject is (typeof SUBJECTS)[number] =>
      Boolean(subject),
    );

  return {
    id: material.id,
    authorId: material.author_id,
    authorName: material.author_name || undefined,
    title: material.title,
    description: material.description || "",
    pubDate: formatDisplayDate(material.pub_date || ""),
    courses: courses.length > 0 ? courses : [DEFAULT_COURSE],
    subjects: subjects.length > 0 ? subjects : [DEFAULT_SUBJECT],
    type: normalizeMaterialType(material.type),
    difficulty: normalizeDifficulty(material.difficulty),
  };
};

export const mapLibraryMaterialFileToSubmissionFile = (
  file: LibraryMaterialFile,
): MaterialSubmissionFile => ({
  id: file.id,
  name: file.name,
  sizeBytes: file.size_bytes,
  extension: file.extension,
  url: file.url || undefined,
});

const normalizeSubmissionStatus = (status: string): MaterialSubmission["status"] => {
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

export const mapArticleToMaterialDetails = (
  article: RealWorldArticle,
): Material & {
  files: MaterialSubmissionFile[];
  submittedAt?: string;
  publishedAt?: string;
} => ({
  ...mapArticleToMaterial(article),
  description: article.body || article.description || "",
  files: [],
  submittedAt: article.createdAt,
  publishedAt: article.updatedAt,
});

export const mapAttachmentToMaterialFile = (
  articleSlug: string,
  attachment: RealWorldAttachment,
): MaterialSubmissionFile => ({
  id: attachment.attachmentId,
  name: attachment.fileName,
  sizeBytes: 0,
  extension: getFileExtension(attachment.fileName),
  url:
    attachment.downloadUrl ||
    attachment.url ||
    resolveApiUrl(
      `/api/articles/${encodeURIComponent(articleSlug)}/attachments/${encodeURIComponent(attachment.attachmentId)}`,
    ),
});

export const mapProfileToUser = (profile: RealWorldProfile): User => ({
  id: profile.username,
  name: profile.username,
  bio: profile.bio || "",
  image: profile.image,
  verified: true,
  isEmailVerified: true,
});
