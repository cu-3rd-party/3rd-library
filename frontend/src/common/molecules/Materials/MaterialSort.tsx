import { useState } from "react";

import { SingleSelect } from "@/common/molecules";
import { cn } from "@/lib/utils";
import { SortOrderType, SortSettingsType, SortType } from "@/models";
import { getSortAttributes } from "@/utils";

type MaterialSortProps<T> = {
  item: T;
  sortSettingsType: SortSettingsType;
  onSelect: (item: T) => void;
  className?: string;
};

export const MaterialSort = <T extends SortType | SortOrderType>({
  item,
  sortSettingsType,
  onSelect,
  className,
}: MaterialSortProps<T>) => {
  const [open, setOpen] = useState(false);
  const { allItems, getLabel, getIcon } = getSortAttributes(sortSettingsType);

  return (
    <div className={cn("space-y-3", className)}>
      <SingleSelect<T>
        open={open}
        onOpenChange={setOpen}
        selectedItem={item}
        onSelect={onSelect}
        items={allItems as readonly T[]}
        renderItem={(val) => getLabel(val)}
        getItemKey={(val) => val}
        getItemValue={(val) => val}
        getItemIcon={(val) => getIcon(val)}
      />
    </div>
  );
};
