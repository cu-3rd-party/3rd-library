import { Label } from "@radix-ui/react-label"

import { cn } from "@/lib/utils"
import { SortOrderType, SortState, SortType} from "@/models"

import { MaterialSort } from "./MaterialSort"

type DesktopSortProps = {
  showFilters: boolean;
  sortState: SortState;
  onSortChange: <K extends keyof SortState>(key: K, value: SortState[K]) => void;
}

export const DesktopSort = ({
  showFilters,
  sortState,
  onSortChange,
}: DesktopSortProps) => {
  return (
    <div className={cn("hidden lg:grid grid-cols-4 lg:grid-cols-9 gap-2 overflow-hidden transition-all duration-(--std-duration) ease-in-out", 
      showFilters ? "grid-rows-[1fr]" : "grid-rows-[0fr]"
    )}>
      <Label className="text-base xl:text-lg font-semibold col-span-4 lg:col-span-1 min-h-0">
        Сортировка
      </Label>
      <MaterialSort<SortType>
        item={sortState.sortBy}
        sortSettingsType="type"
        onSelect={(val: SortType) => onSortChange("sortBy", val)}
        className="min-h-0 col-span-2"
      />
      <MaterialSort<SortOrderType>
        item={sortState.order}
        sortSettingsType="order"
        onSelect={(val: SortOrderType) => onSortChange("order", val)}
        className="min-h-0 col-span-2"
      />
    </div>
  )
}