import { COURSES, SUBJECTS } from "@/constants/course";

import { UUID } from "./types";

export type Subject = typeof SUBJECTS[number];
export type Course = typeof COURSES[number];
export type MaterialType = "cheatlist" | "longread" | "shortread" | "demo" | "solution" | "other"
export type Difficulty = "none" | "blue" | "red" | "black"
export interface Material {
  id: UUID;
  authorId: UUID;
  title: string;
  pubDate: string;
  description: string;
  courses: Course[];
  subjects: Subject[];
  type: MaterialType;
  difficulty: Difficulty;
  authorName?: string
}

export interface DifficultyBadge {
  value: Difficulty;
  label: string;
  badgeClass: string;
  indicatorClass: string
}

export interface TypeBadge {
  value: MaterialType;
  label: string;
  badgeClass: string;
}