import { useState } from "react";

import { Subject, SUBJECTS } from "@/entities/material/model";
import { cn } from "@/shared/lib";
import { Badge, Label, MultiSelect } from "@/shared/ui";

type SubjectSelectorProps = {
  subjects: Subject[];
  onToggle: (subject: Subject) => void;
  className?: string;
};

export const SubjectSelector = ({
  subjects,
  onToggle,
  className,
}: SubjectSelectorProps) => {
  const [open, setOpen] = useState(false);

  return (
    <div className={cn("space-y-3", className)}>
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
        renderBadge={(subject) => (
          <Badge variant="secondary" className="mr-1">
            {subject}
          </Badge>
        )}
        renderItem={(subject) => subject}
        getItemKey={(subject) => subject}
        getItemValue={(subject) => subject}
        isSelected={(subject) => subjects.includes(subject)}
      />
    </div>
  );
};
