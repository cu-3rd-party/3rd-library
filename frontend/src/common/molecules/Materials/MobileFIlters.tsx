import { Label } from "@radix-ui/react-label"

import { Course, Difficulty, MaterialType, Subject } from "@/models"
import { useFilterSortStore } from "@/store"

import { MaterialFilter } from "./MaterialFilter"

export const MobileFilters = () => {
  const { filterState, toggleFilter, resetFilter } = useFilterSortStore();
  return (
    <div className="flex flex-col gap-3">
      <Label className="text-lg font-semibold">Фильтры</Label>
      <MaterialFilter<Course> 
        items={filterState.courses}
        filterType="course"
        onToggle={(val: string) => toggleFilter("courses", val)}
      />
      <MaterialFilter<Difficulty>
        items={filterState.difficulties}
        filterType="difficulty"
        onToggle={(val: string) => toggleFilter("difficulties", val)}
        onReset={() => resetFilter("difficulties")}
      />
      <MaterialFilter<MaterialType>
        items={filterState.types}
        filterType="type"
        onToggle={(val: string) => toggleFilter("types", val)}
        onReset={() => resetFilter("types")}
      />
      <MaterialFilter<Subject>
        items={filterState.subjects}
        filterType="subject"
        onToggle={(val: string) => toggleFilter("subjects", val)}
        onReset={() => resetFilter("subjects")}
      />
    </div>
  )
}