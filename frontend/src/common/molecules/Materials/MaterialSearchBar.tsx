import { Search } from "lucide-react";

import { Input } from "@/components/ui/input";
import { COURSES, DIFFICULTIES, MATERIAL_TYPES, SUBJECTS } from "@/constants";
import { Course, Difficulty, FilterState, MaterialType, Subject } from "@/models";

import { MaterialFilter } from "./MaterialFilter"


type MaterialSearchBarProps = {
  searchQuery: string;
  onSearchChange: (value: string) => void;
  filters: FilterState;
  onFilterChange: <K extends keyof FilterState>(key: K, value: FilterState[K]) => void;
};

export const MaterialSearchBar = ({
  searchQuery,
  onSearchChange,
  filters,
  onFilterChange,
}: MaterialSearchBarProps) => {

const handleFilterChange = (val: string, arr: string[], filterKey: keyof FilterState) => {
  const newValues = arr.includes(val) ? arr.filter(item => item !== val) : [...arr, val];
  
  const orderMap: Record<keyof FilterState, readonly string[]> = {
    courses: COURSES,
    subjects: SUBJECTS,
    difficulties: DIFFICULTIES,
    types: MATERIAL_TYPES
  };
  
  const order = orderMap[filterKey];
  const sorted = newValues.sort((a, b) => order.indexOf(a) - order.indexOf(b));
  
  onFilterChange(filterKey, sorted as FilterState[keyof FilterState]);
}

  return (
    <div className="flex w-full lg:w-auto gap-2">
      <div className="relative flex-1 lg:flex-initial lg:w-80">
        <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
        <Input
          placeholder="Поиск..."
          value={searchQuery}
          onChange={(e) => onSearchChange(e.target.value)}
          className="pl-9 border-border min-h-12"
        />
      </div>

      <MaterialFilter<Course> 
        items={filters.courses}
        filterType="course"
        onToggle={(val: string) => handleFilterChange(val, filters.courses, "courses")}
        className="flex-1"
      />
      <MaterialFilter<Difficulty>
        items={filters.difficulties}
        filterType="difficulty"
        onToggle={(val: string) => handleFilterChange(val, filters.difficulties, "difficulties")}
        onReset={() => onFilterChange("difficulties", [])}
        className="flex-1"
      />
      <MaterialFilter<MaterialType>
        items={filters.types}
        filterType="type"
        onToggle={(val: string) => handleFilterChange(val, filters.types, "types")}
        onReset={() => onFilterChange("types", [])}
        className="flex-1"
      />
      <MaterialFilter<Subject>
        items={filters.subjects}
        filterType="subject"
        onToggle={(val: string) => handleFilterChange(val, filters.subjects, "subjects")}
        onReset={() => onFilterChange("subjects", [])}
        className="flex-1"
      />
    </div>

)};