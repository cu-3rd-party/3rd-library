import { UUID } from "@/shared/model";

import { COURSES, DIFFICULTIES, MATERIAL_TYPES, SUBJECTS } from "./constants";

export type Subject = (typeof SUBJECTS)[number];
export type Course = (typeof COURSES)[number];
export type MaterialType = (typeof MATERIAL_TYPES)[number];
export type Difficulty = (typeof DIFFICULTIES)[number];

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
  authorName?: string;
}
