import { useState } from "react";

import { SingleSelect } from "@/common/molecules";
import { Label } from "@/components/ui/label";
import { MATERIAL_TYPES, TYPE_CONFIG } from "@/constants";
import { cn } from "@/lib/utils";
import { MaterialType } from "@/models";

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
