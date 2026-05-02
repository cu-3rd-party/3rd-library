import { COURSES, DIFFICULTIES, MATERIAL_TYPES, SUBJECTS } from "@/constants";
import { resolveApiUrl } from "@/lib/api";
import { Material, MaterialSubmissionFile, User } from "@/models";

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
