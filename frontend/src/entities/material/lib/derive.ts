import { COURSES, DIFFICULTIES, MATERIAL_TYPES, SUBJECTS } from "../model";

import {
  DEFAULT_COURSE,
  DEFAULT_DIFFICULTY,
  DEFAULT_SUBJECT,
  DEFAULT_TYPE,
} from "./defaults";
import { findTagValues, matchSubjectByTag, normalizeTag } from "./tags";

export const deriveSubjects = (tags: string[]) => {
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

export const deriveCourse = (tags: string[]) => {
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

export const deriveType = (tags: string[]) => {
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

export const deriveDifficulty = (favoritesCount: number) => {
  if (favoritesCount >= 15) return "black";
  if (favoritesCount >= 8) return "red";
  if (favoritesCount >= 3) return "blue";
  return DEFAULT_DIFFICULTY;
};

export const deriveDifficultyFromTags = (
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
