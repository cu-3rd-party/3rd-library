import { Button } from "@/components/ui/button";
import { Sheet, SheetContent } from "@/components/ui/sheet";
import { useFilterSortStore } from "@/store";

import { MobileFilters } from "./MobileFIlters";
import { MobileSort } from "./MobileSort";

type FilterSortSheetProps = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  hasActiveFilters: boolean;
};

export const FilterSortSheet = ({
  open,
  onOpenChange,
  hasActiveFilters,
}: FilterSortSheetProps) => {
  const { resetFilters } = useFilterSortStore();

  return (
    <Sheet open={open} onOpenChange={onOpenChange}>
      <SheetContent side="bottom" className="h-[60dvh]" showCloseButton={false}>
        <div className="h-full py-6 px-6 flex flex-col gap-3">
          <MobileFilters />
          <MobileSort />
          <div className="mt-auto flex flex-col gap-3">
            {hasActiveFilters && (
              <Button
                variant="destructive"
                className="min-h-12 text-base"
                onClick={() => {
                  resetFilters();
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
  );
};
