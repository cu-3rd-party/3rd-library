import { Button } from "@/components/ui/button";
import { Sheet, SheetContent, SheetHeader, SheetTitle } from "@/components/ui/sheet";

import { MobileFilters, MobileFiltersProps } from "./MobileFIlters";
import { MobileSort, MobileSortProps } from "./MobileSort";



type SheetControlProps = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  resetAll: () => void;
  hasActiveFilters: boolean;
}

type FilterSortSheetProps = SheetControlProps & MobileFiltersProps & MobileSortProps;

export const FilterSortSheet = ({
  open,
  onOpenChange,
  resetAll,
  hasActiveFilters,
  filterState,
  sortState,
  handleFilterChange,
  onFilterChange,
  onSortChange
}: FilterSortSheetProps) => (
  <Sheet 
    open={open}
    onOpenChange={onOpenChange}
  >
    <SheetContent 
      side="bottom" 
      className="h-[80vh]"
      showCloseButton={false}
    >
      <div className="h-full py-6 px-6 flex flex-col gap-3">
        <MobileFilters 
          filterState={filterState}
          handleFilterChange={handleFilterChange}
          onFilterChange={onFilterChange}
        />
        <MobileSort 
          sortState={sortState}
          onSortChange={onSortChange}
        />
        <div className="mt-auto flex flex-col gap-3">
          {hasActiveFilters && (
            <Button
              variant="destructive"
              className="min-h-12 text-base"
              onClick={() => {
                resetAll();
              }}
            >
              Сбросить все фильтры
            </Button>
          )}
          <Button
            variant="primaryActive"
            className="min-h-12 text-lg"
            onClick={() => {
              onOpenChange(false);
            }}
          >
            Закрыть
          </Button>
        </div>

      </div>
    </SheetContent>
  </Sheet>
)