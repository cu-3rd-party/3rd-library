import { useState } from "react";

import { SingleSelect } from "@/common/molecules/ui/SingleSelect";
import { Label } from "@/components/ui/label";
import { TYPE_BADGES } from "@/constants";
import { MaterialType } from "@/models";


type TypeSelectorProps = {
  type: MaterialType;
  onSelect: (type: MaterialType) => void;
  className?: string;
};

export const TypeSelector = ({ type, onSelect, className }: TypeSelectorProps) => {
  const [open, setOpen] = useState(false);
  const getTypeData = (value: MaterialType) =>
    TYPE_BADGES.find((l) => l.value === value);

  return (
    <div className={`space-y-3 ${className || ''}`}>
      <Label className="text-lg font-semibold">Тип материала</Label>
      <SingleSelect
        open={open}
        onOpenChange={setOpen}
        selectedItem={type || null}
        onSelect={onSelect}
        items={TYPE_BADGES.map((d) => d.value)}
        placeholder="Выберите тип материала..."
        renderItem={(d) => {
          const type = getTypeData(d);
          return <span>{type?.label || d}</span>;
        }}
        getItemKey={(d) => d}
        getItemValue={(d) => d}
      />
    </div>
  );
};