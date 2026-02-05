import { Label } from "@radix-ui/react-label"

import { Course, Difficulty, FilterState, MaterialType, Subject } from "@/models"

import { MaterialFilter } from "./MaterialFilter"

export type MobileFiltersProps = {
  filterState: FilterState;
  handleFilterChange: (val: string, arr: string[], filterKey: keyof FilterState) => void;
  onFilterChange: <K extends keyof FilterState>(key: K, value: FilterState[K]) => void;
}

export const MobileFilters = ({
  filterState,
  handleFilterChange,
  onFilterChange
}: MobileFiltersProps) => {
  return (
    <div className="flex flex-col gap-3">
      <Label className="text-lg font-semibold">Фильтры</Label>
      <MaterialFilter<Course> 
        items={filterState.courses}
        filterType="course"
        onToggle={(val: string) => handleFilterChange(val, filterState.courses, "courses")}
      />
      <MaterialFilter<Difficulty>
        items={filterState.difficulties}
        filterType="difficulty"
        onToggle={(val: string) => handleFilterChange(val, filterState.difficulties, "difficulties")}
        onReset={() => onFilterChange("difficulties", [])}
      />
      <MaterialFilter<MaterialType>
        items={filterState.types}
        filterType="type"
        onToggle={(val: string) => handleFilterChange(val, filterState.types, "types")}
        onReset={() => onFilterChange("types", [])}
      />
      <MaterialFilter<Subject>
        items={filterState.subjects}
        filterType="subject"
        onToggle={(val: string) => handleFilterChange(val, filterState.subjects, "subjects")}
        onReset={() => onFilterChange("subjects", [])}
      />
    </div>
  )
}