import { COURSES, DIFFICULTIES, MATERIAL_TYPES, SUBJECTS } from "../model";

import { DEFAULT_DIFFICULTY } from "./defaults";

export const normalizeCourse = (
  value: string,
): (typeof COURSES)[number] | null =>
  COURSES.includes(value as (typeof COURSES)[number])
    ? (value as (typeof COURSES)[number])
    : null;

export const normalizeSubject = (
  value: string,
): (typeof SUBJECTS)[number] | null =>
  SUBJECTS.includes(value as (typeof SUBJECTS)[number])
    ? (value as (typeof SUBJECTS)[number])
    : null;

export const normalizeMaterialType = (
  value: string,
): (typeof MATERIAL_TYPES)[number] =>
  MATERIAL_TYPES.includes(value as (typeof MATERIAL_TYPES)[number])
    ? (value as (typeof MATERIAL_TYPES)[number])
    : "other";

export const normalizeDifficulty = (
  value: string,
): (typeof DIFFICULTIES)[number] =>
  DIFFICULTIES.includes(value as (typeof DIFFICULTIES)[number])
    ? (value as (typeof DIFFICULTIES)[number])
    : DEFAULT_DIFFICULTY;
