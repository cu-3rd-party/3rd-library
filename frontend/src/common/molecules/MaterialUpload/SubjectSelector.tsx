import { useState } from "react";

import { MultiSelect } from "@/common/molecules";
import { Badge } from "@/components/ui/badge";
import { Label } from "@/components/ui/label";
import { SUBJECTS } from "@/constants";
import { cn } from "@/lib/utils";
import { Subject } from "@/models";

type SubjectSelectorProps = {
  subjects: Subject[];
  onToggle: (subject: Subject) => void;
  className?: string;
};

export const SubjectSelector = ({ subjects, onToggle, className }: SubjectSelectorProps) => {
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
}