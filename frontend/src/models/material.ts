import { COURSES, DIFFICULTIES, MATERIAL_TYPES, SUBJECTS } from "@/constants/material";

import { UUID } from "./types";

export type Subject = typeof SUBJECTS[number];
export type Course = typeof COURSES[number];
export type MaterialType = typeof MATERIAL_TYPES[number];
export type Difficulty = typeof DIFFICULTIES[number];
export type FilterType = "course" | "difficulty" | "subject" | "type"
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

export type FilterState = {
  courses: Course[];
  subjects: Subject[];
  difficulties: Difficulty[];
  types: MaterialType[];
};

export const defaultFilterState: FilterState = {
  courses: [],
  subjects: [],
  difficulties: [],
  types: []
}