import { MaterialSubmissionFile } from "@/entities/submission/model";
import { resolveApiUrl } from "@/shared/api";
import { formatDisplayDate, getFileExtension } from "@/shared/lib";

import { LibraryMaterial, RealWorldArticle, RealWorldAttachment } from "../api";
import { COURSES, Material, SUBJECTS } from "../model";

import { DEFAULT_COURSE, DEFAULT_SUBJECT } from "./defaults";
import {
  deriveCourse,
  deriveDifficultyFromTags,
  deriveSubjects,
  deriveType,
} from "./derive";
import {
  normalizeCourse,
  normalizeDifficulty,
  normalizeMaterialType,
  normalizeSubject,
} from "./normalize";

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
