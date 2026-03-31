import { SORT_ORDER_TYPES, SORT_TYPES } from "@/constants";

export type SortSettingsType = "type" | "order";
export type SortType = (typeof SORT_TYPES)[number];
export type SortOrderType = (typeof SORT_ORDER_TYPES)[number];

export type SortState = {
  sortBy: SortType;
  order: SortOrderType;
};

export const defaultSortState: SortState = {
  sortBy: "date",
  order: "decreasing",
};
