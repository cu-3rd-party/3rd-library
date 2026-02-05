import { Label } from "@radix-ui/react-label"

import { cn } from "@/lib/utils"
import { Course, Difficulty, FilterState, MaterialType, Subject } from "@/models"

import { MaterialFilter } from "./MaterialFilter"

type DesktopFiltersProps = {
  showFilters: boolean;
  filterState: FilterState;
  handleFilterChange: (val: string, arr: string[], filterKey: keyof FilterState) => void;
  onFilterChange: <K extends keyof FilterState>(key: K, value: FilterState[K]) => void;
}

export const DesktopFilters = ({
  showFilters,
  filterState,
  handleFilterChange,
  onFilterChange
}: DesktopFiltersProps) => {
  return (
    <div className={cn("hidden lg:grid grid-cols-4 lg:grid-cols-9 gap-2 overflow-hidden transition-all duration-(--std-duration) ease-in-out items-center", 
      showFilters ? "grid-rows-[1fr] opacity-100" : "grid-rows-[0fr] opacity-0"
    )}>
      <Label className="text-base xl:text-lg font-semibold col-span-4 lg:col-span-1 min-h-0">
        Фильтры
      </Label>
      <MaterialFilter<Course> 
        items={filterState.courses}
        filterType="course"
        onToggle={(val: string) => handleFilterChange(val, filterState.courses, "courses")}
        className="min-h-0 col-span-2"
      />
      <MaterialFilter<Difficulty>
        items={filterState.difficulties}
        filterType="difficulty"
        onToggle={(val: string) => handleFilterChange(val, filterState.difficulties, "difficulties")}
        onReset={() => onFilterChange("difficulties", [])}
        className="min-h-0 col-span-2"
      />
      <MaterialFilter<MaterialType>
        items={filterState.types}
        filterType="type"
        onToggle={(val: string) => handleFilterChange(val, filterState.types, "types")}
        onReset={() => onFilterChange("types", [])}
        className="min-h-0 col-span-2"
      />
      <MaterialFilter<Subject>
        items={filterState.subjects}
        filterType="subject"
        onToggle={(val: string) => handleFilterChange(val, filterState.subjects, "subjects")}
        onReset={() => onFilterChange("subjects", [])}
        className="min-h-0 col-span-2"
      />
    </div>
  )
}