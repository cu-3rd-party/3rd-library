import { ChevronDown, Filter, X } from "lucide-react";

import { cn } from "@/shared/lib";
import {
  Button,
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/shared/ui";

import { useFilterSortStore } from "../model";

type MaterialActionBarProps = {
  showFilters: boolean;
  hasActiveFilters: boolean;
  openMobileSheet: () => void;
  setShowFilters: (val: boolean) => void;
};

export const MaterialActionBar = ({
  showFilters,
  hasActiveFilters,
  openMobileSheet,
  setShowFilters,
}: MaterialActionBarProps) => {
  const { resetFilters } = useFilterSortStore();
  return (
    <TooltipProvider delayDuration={300}>
      {/* makes filter & sort inputs appear (desktop) */}
      <Tooltip>
        <TooltipTrigger asChild>
          <Button
            variant="outline"
            size="icon-lg"
            onClick={() => setShowFilters(!showFilters)}
            className="hidden lg:flex min-w-12 min-h-12"
          >
            <ChevronDown
              className={cn(
                "h-4 w-4 transition-transform duration-(--std-duration)",
                showFilters ? "rotate-180" : "rotate-0",
              )}
            />
          </Button>
        </TooltipTrigger>
        <TooltipContent>
          {showFilters ? "Скрыть фильтры" : "Показать фильтры"}
        </TooltipContent>
      </Tooltip>

      {/* makes filter & sort sheet appear (mobile) */}
      <Button
        variant="outline"
        size="icon-lg"
        onClick={openMobileSheet}
        className="flex lg:hidden min-w-12 min-h-12"
      >
        <Filter
          className={cn(
            "h-4 w-4 transition-transform duration-(--std-duration)",
          )}
        />
      </Button>

      <Tooltip>
        <TooltipTrigger asChild>
          <Button
            variant="outline"
            size="icon"
            onClick={resetFilters}
            disabled={!hasActiveFilters}
            className={cn(
              "min-w-12 min-h-12 transition-all duration-(--std-duration)",
              hasActiveFilters ? "opacity-100" : "opacity-0",
            )}
          >
            <X className="h-4 w-4" />
          </Button>
        </TooltipTrigger>
        <TooltipContent>Сбросить все фильтры</TooltipContent>
      </Tooltip>
    </TooltipProvider>
  );
};
