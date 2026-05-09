import { Label } from "@/shared/ui";

import { SortOrderType, SortType, useFilterSortStore } from "../model";

import { MaterialSort } from "./MaterialSort";

export const MobileSort = () => {
  const { sortState, setSortState } = useFilterSortStore();
  return (
    <div className="flex flex-col gap-3">
      <Label className="text-lg font-semibold">Сортировка</Label>
      <MaterialSort<SortType>
        item={sortState.sortBy}
        sortSettingsType="type"
        onSelect={(val: SortType) => setSortState("sortBy", val)}
      />
      <MaterialSort<SortOrderType>
        item={sortState.order}
        sortSettingsType="order"
        onSelect={(val: SortOrderType) => setSortState("order", val)}
      />
    </div>
  );
};
