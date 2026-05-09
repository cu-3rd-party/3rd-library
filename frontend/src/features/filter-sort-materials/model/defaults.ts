import { FilterState, SortState } from "./types";

export const defaultFilterState: FilterState = {
  courses: [],
  subjects: [],
  difficulties: [],
  types: [],
};

export const defaultSortState: SortState = {
  sortBy: "date",
  order: "decreasing",
};
