import { useState } from "react";

import { MultiSelect } from "@/common/molecules";
import { Badge } from "@/components/ui/badge";
import { cn } from "@/lib/utils";
import {
  Course,
  Difficulty,
  FilterType,
  MaterialType,
  Subject,
} from "@/models";
import { getFilterAttributes } from "@/utils";

type MaterialFilterProps<T> = {
  items: T[];
  filterType: FilterType;
  onToggle: (item: T) => void;
  onReset?: () => void;
  className?: string;
};

export const MaterialFilter = <
  T extends Course | Difficulty | MaterialType | Subject,
>({
  items,
  filterType,
  onToggle,
  onReset,
  className,
}: MaterialFilterProps<T>) => {
  const [open, setOpen] = useState(false);
  const { placeholder, emptyText, searchPlaceholder, allItems, getLabel } =
    getFilterAttributes(filterType);

  return (
    <div className={cn("space-y-3", className)}>
      <MultiSelect<T>
        open={open}
        onOpenChange={setOpen}
        selectedItems={items}
        onToggle={onToggle}
        items={allItems as readonly T[]}
        placeholder={placeholder}
        emptyText={emptyText}
        renderBadge={(val: T) => (
          <Badge variant="secondary" className="text-xs">
            {getLabel(val)}
          </Badge>
        )}
        searchPlaceholder={searchPlaceholder}
        onReset={onReset}
        renderItem={(val) => getLabel(val)}
        getItemKey={(val) => val}
        getItemValue={(val) => val}
        isSelected={(val) => items.includes(val)}
      />
    </div>
  );
};
