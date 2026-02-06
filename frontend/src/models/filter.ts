import { Course, Difficulty, MaterialType, Subject } from "./material";

export type FilterType = "course" | "difficulty" | "subject" | "type"

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