import {
  Course,
  Difficulty,
  MaterialType,
  Subject,
} from "@/entities/material/model";

import { SORT_ORDER_TYPES, SORT_TYPES } from "./sort";

export type FilterType = "course" | "difficulty" | "subject" | "type";

export type FilterState = {
  courses: Course[];
  subjects: Subject[];
  difficulties: Difficulty[];
  types: MaterialType[];
};

export type SortSettingsType = "type" | "order";
export type SortType = (typeof SORT_TYPES)[number];
export type SortOrderType = (typeof SORT_ORDER_TYPES)[number];

export type SortState = {
  sortBy: SortType;
  order: SortOrderType;
};
