import { ReactNode } from "react";

import { SortOrderType, SortSettingsType, SortType } from "../model";
import { SORT_ORDER_TYPES, SORT_TYPES } from "../model/sort";
import { SORT_ORDER_CONFIG, SORT_TYPE_CONFIG } from "../model/sortConfig";

type SortAttributes = {
  allItems: readonly string[] | readonly number[];
  getLabel: (val: string) => string;
  getIcon: (val: string) => ReactNode;
};

export const getSortAttributes = (
  sortSettingsType: SortSettingsType,
): SortAttributes => {
  switch (sortSettingsType) {
    case "type": {
      return {
        allItems: SORT_TYPES,
        getLabel: (val: string) => SORT_TYPE_CONFIG[val as SortType].label,
        getIcon: (val: string) => SORT_TYPE_CONFIG[val as SortType].icon,
      };
    }
    case "order": {
      return {
        allItems: SORT_ORDER_TYPES,
        getLabel: (val: string) =>
          SORT_ORDER_CONFIG[val as SortOrderType].label,
        getIcon: (val: string) => SORT_ORDER_CONFIG[val as SortOrderType].icon,
      };
    }
  }
};
