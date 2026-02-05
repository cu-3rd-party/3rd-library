import { Label } from "@radix-ui/react-label"

import { SortOrderType, SortState, SortType} from "@/models"

import { MaterialSort } from "./MaterialSort"

export type MobileSortProps = {
  sortState: SortState;
  onSortChange: <K extends keyof SortState>(key: K, value: SortState[K]) => void;
}

export const MobileSort = ({
  sortState,
  onSortChange,
}: MobileSortProps) => {
  return (
    <div className="flex flex-col gap-3">
      <Label className="text-lg font-semibold">Сортировка</Label>
      <MaterialSort<SortType>
        item={sortState.sortBy}
        sortSettingsType="type"
        onSelect={(val: SortType) => onSortChange("sortBy", val)}
      />
      <MaterialSort<SortOrderType>
        item={sortState.order}
        sortSettingsType="order"
        onSelect={(val: SortOrderType) => onSortChange("order", val)}
      />
    </div>
  )
}