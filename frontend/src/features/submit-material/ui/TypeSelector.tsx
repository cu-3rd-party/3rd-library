import { useState } from "react";

import { TYPE_CONFIG } from "@/entities/material/lib";
import { MATERIAL_TYPES, MaterialType } from "@/entities/material/model";
import { cn } from "@/shared/lib";
import { Label, SingleSelect } from "@/shared/ui";

type TypeSelectorProps = {
  type: MaterialType;
  onSelect: (type: MaterialType) => void;
  className?: string;
};

export const TypeSelector = ({
  type,
  onSelect,
  className,
}: TypeSelectorProps) => {
  const [open, setOpen] = useState(false);

  return (
    <div className={cn("space-y-3", className)}>
      <Label className="text-lg font-semibold">Тип материала</Label>
      <SingleSelect
        open={open}
        onOpenChange={setOpen}
        selectedItem={type || null}
        onSelect={onSelect}
        items={MATERIAL_TYPES}
        renderItem={(type) => <span>{TYPE_CONFIG[type]?.label || type}</span>}
        getItemKey={(type) => type}
        getItemValue={(type) => type}
      />
    </div>
  );
};
