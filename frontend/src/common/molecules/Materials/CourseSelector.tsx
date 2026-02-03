import { useState } from "react";

import { MultiSelect } from  "@/common/molecules/ui/MultiSelect";
import { Badge } from "@/components/ui/badge";
import { Label } from "@/components/ui/label";
import { COURSES } from "@/constants";
import { Course } from "@/models";

type CourseSelectorProps = {
  courses: Course[];
  onToggle: (course: Course) => void;
};

export const CourseSelector = ({ courses, onToggle }: CourseSelectorProps) => {
  const [open, setOpen] = useState(false);

  return (
    <div className="space-y-3">
      <Label className="text-lg font-semibold">Курсы</Label>
      <MultiSelect
        open={open}
        onOpenChange={setOpen}
        selectedItems={courses}
        onToggle={onToggle}
        items={COURSES}
        placeholder="Выберите курсы..."
        emptyText="Курс не найден."
        renderBadge={(c) => (
          <Badge variant="secondary" className="mr-1">
            {c} курс
          </Badge>
        )}
        renderItem={(c) => `${c} курс`}
        getItemKey={(c) => c.toString()}
        getItemValue={(c) => `${c} курс`}
        isSelected={(c) => courses.includes(c)}
      />
    </div>
  );
}