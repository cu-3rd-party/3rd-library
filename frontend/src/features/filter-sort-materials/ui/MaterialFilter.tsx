import { useState } from "react";

import { DIFFICULTY_CONFIG } from "@/entities/material/lib";
import {
  Course,
  Difficulty,
  MaterialType,
  Subject,
} from "@/entities/material/model";
import { MaterialBadge } from "@/entities/material/ui/MaterialBadge";
import { cn } from "@/shared/lib";
import { Badge, MultiSelect } from "@/shared/ui";

import { getFilterAttributes } from "../lib";
import { FilterType } from "../model";

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
  const {
    placeholder,
    emptyText,
    searchPlaceholder,
    allItems,
    getLabel,
    getAdditionalLabel,
  } = getFilterAttributes(filterType);

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
        renderBadge={(val: T) => {
          return filterType !== "difficulty" ? (
            <Badge variant="secondary" className="text-xs">
              {getLabel(val)}
            </Badge>
          ) : (
            <MaterialBadge
              label={getLabel(val)}
              className={DIFFICULTY_CONFIG[val as Difficulty].className}
            />
          );
        }}
        searchPlaceholder={searchPlaceholder}
        onReset={onReset}
        renderItem={(val) => getAdditionalLabel?.(val) ?? getLabel(val)}
        getItemKey={(val) => val}
        getItemValue={(val) => val}
        isSelected={(val) => items.includes(val)}
      />
    </div>
  );
};
