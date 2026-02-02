import { COURSES, SUBJECTS } from "@/constants/course";

import { UUID } from "./types";

export type Subject = typeof SUBJECTS[number];
export type Course = typeof COURSES[number];
export type MaterialType = "Лонгрид" | "Шортрид" | "Демо" | "Решение работы" | "Другое"
export type Difficulty = "none" | "blue" | "red" | "black"
export interface Material {
  id: UUID;
  authorId: UUID;
  title: string;
  pubDate: string;
  description: string;
  courses: Course[];
  subject: Subject;
  type: MaterialType;
  difficulty: Difficulty;
  authorName?: string
}