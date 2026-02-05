import { Search } from "lucide-react";
import { useState } from "react";

import { Input } from "@/components/ui/input";
import { COURSES, DIFFICULTIES, MATERIAL_TYPES, SUBJECTS } from "@/constants";
import { FilterState, SortState } from "@/models";

import { DesktopFilters } from "./DesktopFilters";
import { DesktopSort } from "./DesktopSort";
import { FilterSortSheet } from "./FilterSortSheet";
import { MaterialActionBar } from "./MaterialActionBar";

type MaterialTopPanelProps = {
  searchQuery: string;
  onSearchChange: (value: string) => void;
  sortState: SortState;
  onSortChange: <K extends keyof SortState>(key: K, value: SortState[K]) => void;
  filterState: FilterState;
  onFilterChange: <K extends keyof FilterState>(key: K, value: FilterState[K]) => void;
  resetAll: () => void;
};

export const MaterialTopPanel = ({
  searchQuery,
  onSearchChange,
  sortState,
  onSortChange,
  filterState,
  onFilterChange,
  resetAll,
}: MaterialTopPanelProps) => {
  const [showFilters, setShowFilters] = useState(false);
  const [sheetOpen, setSheetOpen] = useState(false);
  const hasActiveFilters = 
    filterState.courses.length > 0 || 
    filterState.difficulties.length > 0 ||
    filterState.subjects.length > 0 ||
    filterState.types.length > 0
  
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
    <div className="w-full space-y-2">
      <div className="flex w-full gap-2">
        <div className="relative flex-1">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
          <Input
            placeholder="Поиск..."
            value={searchQuery}
            onChange={(e) => onSearchChange(e.target.value)}
            className="pl-9 border-border min-h-12"
          />
        </div>
        <MaterialActionBar 
          showFilters={showFilters}
          hasActiveFilters={hasActiveFilters}
          openMobileSheet={() => setSheetOpen(true)}
          setShowFilters={setShowFilters}
          resetAll={resetAll}
        />
      </div>
      
      <DesktopFilters 
        showFilters={showFilters}
        filterState={filterState}
        handleFilterChange={handleFilterChange}
        onFilterChange={onFilterChange}
      />

      <DesktopSort 
        showFilters={showFilters}
        sortState={sortState}
        onSortChange={onSortChange}
      />

      <FilterSortSheet 
        open={sheetOpen}
        onOpenChange={setSheetOpen}
        resetAll={resetAll}
        hasActiveFilters={hasActiveFilters}
        filterState={filterState}
        handleFilterChange={handleFilterChange}
        onFilterChange={onFilterChange}
        sortState={sortState}
        onSortChange={onSortChange}
      />
    </div>
    

)};