import { DIFFICULTY_CONFIG, TYPE_CONFIG } from "@/entities/material/lib";
import {
  COURSES,
  DIFFICULTIES,
  Difficulty,
  MATERIAL_TYPES,
  MaterialType,
  SUBJECTS,
} from "@/entities/material/model";

import { FilterType } from "../model/types";

// if search placeholder exists, search bar will appear
type FilterAttributes = {
  placeholder: string;
  emptyText: string;
  searchPlaceholder?: string;
  allItems: readonly string[] | readonly number[];
  getLabel: (val: string) => string;
  getAdditionalLabel?: (val: string) => string;
};

export const getFilterAttributes = (
  filterType: FilterType,
): FilterAttributes => {
  switch (filterType) {
    case "course": {
      return {
        placeholder: "Выберите курсы...",
        emptyText: "Курс не найден",
        allItems: COURSES,
        getLabel: (val: string) => `${val} курс`,
      };
    }
    case "difficulty": {
      return {
        placeholder: "Выберите уровень...",
        emptyText: "Уровень не найден",
        allItems: DIFFICULTIES,
        getLabel: (val: string) => DIFFICULTY_CONFIG[val as Difficulty].label,
        getAdditionalLabel: (val: string) =>
          DIFFICULTY_CONFIG[val as Difficulty].label_add,
      };
    }
    case "subject": {
      return {
        placeholder: "Выберите предметы...",
        emptyText: "Предметы не найдены",
        searchPlaceholder: "Найти предмет...",
        allItems: SUBJECTS,
        getLabel: (val: string) => val,
      };
    }
    case "type": {
      return {
        placeholder: "Выберите тип...",
        emptyText: "Тип не найден",
        allItems: MATERIAL_TYPES,
        getLabel: (val: string) => TYPE_CONFIG[val as MaterialType].label,
      };
    }
  }
};
