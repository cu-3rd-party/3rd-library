import { Search } from "lucide-react";
import { useState } from "react";

import { Input } from "@/components/ui/input";
import { useFilterSortStore } from "@/store";

import { DesktopFilters } from "./DesktopFilters";
import { DesktopSort } from "./DesktopSort";
import { FilterSortSheet } from "./FilterSortSheet";
import { MaterialActionBar } from "./MaterialActionBar";

type MaterialTopPanelProps = {
  searchQuery: string;
  onSearchChange: (value: string) => void;
};

export const MaterialTopPanel = ({
  searchQuery,
  onSearchChange,
}: MaterialTopPanelProps) => {
  const { filterState } = useFilterSortStore(); 
  const [showFilters, setShowFilters] = useState(false);
  const [sheetOpen, setSheetOpen] = useState(false);
  const hasActiveFilters = 
    filterState.courses.length > 0 || 
    filterState.difficulties.length > 0 ||
    filterState.subjects.length > 0 ||
    filterState.types.length > 0

  return (
    <div className="w-full space-y-2">
      <div className="flex w-full gap-2">
        <div className="relative flex-1 align-center">
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
        />
      </div>
      
      <DesktopFilters 
        showFilters={showFilters}
      />

      <DesktopSort 
        showFilters={showFilters}
      />

      <FilterSortSheet 
        open={sheetOpen}
        onOpenChange={setSheetOpen}
        hasActiveFilters={hasActiveFilters}
      />
    </div>
    

)};