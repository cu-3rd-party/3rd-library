import {
  Course,
  Difficulty,
  MaterialType,
  Subject,
} from "@/entities/material/model";
import { cn } from "@/shared/lib";
import { Label } from "@/shared/ui";

import { useFilterSortStore } from "../model";

import { MaterialFilter } from "./MaterialFilter";

type DesktopFiltersProps = {
  showFilters: boolean;
};

export const DesktopFilters = ({ showFilters }: DesktopFiltersProps) => {
  const { filterState, toggleFilter, resetFilter } = useFilterSortStore();
  return (
    <div
      className={cn(
        "hidden lg:grid grid-cols-4 lg:grid-cols-9 gap-2 overflow-hidden transition-all duration-(--std-duration) ease-in-out items-center",
        showFilters
          ? "grid-rows-[1fr] opacity-100"
          : "grid-rows-[0fr] opacity-0",
      )}
    >
      <Label className="text-base xl:text-lg font-semibold col-span-4 lg:col-span-1 min-h-0">
        Фильтры
      </Label>
      <MaterialFilter<Course>
        items={filterState.courses}
        filterType="course"
        onToggle={(val: string) => toggleFilter("courses", val)}
        className="min-h-0 col-span-2"
      />
      <MaterialFilter<Difficulty>
        items={filterState.difficulties}
        filterType="difficulty"
        onToggle={(val: string) => toggleFilter("difficulties", val)}
        onReset={() => resetFilter("difficulties")}
        className="min-h-0 col-span-2"
      />
      <MaterialFilter<MaterialType>
        items={filterState.types}
        filterType="type"
        onToggle={(val: string) => toggleFilter("types", val)}
        onReset={() => resetFilter("types")}
        className="min-h-0 col-span-2"
      />
      <MaterialFilter<Subject>
        items={filterState.subjects}
        filterType="subject"
        onToggle={(val: string) => toggleFilter("subjects", val)}
        onReset={() => resetFilter("subjects")}
        className="min-h-0 col-span-2"
      />
    </div>
  );
};
