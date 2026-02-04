import { useState } from "react";

import { MultiSelect } from "@/common/molecules/ui/MultiSelect";
import { Badge } from "@/components/ui/badge";
import { Label } from "@/components/ui/label";
import { SUBJECTS } from "@/constants";
import { Subject } from "@/models";

type SubjectSelectorProps = {
  subjects: Subject[];
  onToggle: (subject: Subject) => void;
  className?: string;
};

export const SubjectSelector = ({ subjects, onToggle, className }: SubjectSelectorProps) => {
  const [open, setOpen] = useState(false);

  return (
    <div className={`space-y-3 ${className || ''}`}>
      <Label className="text-lg font-semibold">Предметы</Label>
      <MultiSelect
        open={open}
        onOpenChange={setOpen}
        selectedItems={subjects}
        onToggle={onToggle}
        items={SUBJECTS}
        placeholder="Выберите предметы..."
        emptyText="Предмет не найден."
        searchPlaceholder="Найти предмет..."
        renderBadge={(s) => (
          <Badge variant="secondary" className="mr-1">
            {s}
          </Badge>
        )}
        renderItem={(s) => s}
        getItemKey={(s) => s}
        getItemValue={(s) => s}
        isSelected={(s) => subjects.includes(s)}
      />
    </div>
  );
}