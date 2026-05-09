import { useState } from "react";

import { Course, COURSES } from "@/entities/material/model";
import { cn } from "@/shared/lib";
import { Badge, Label, MultiSelect } from "@/shared/ui";

type CourseSelectorProps = {
  courses: Course[];
  onToggle: (course: Course) => void;
  className?: string;
};

export const CourseSelector = ({
  courses,
  onToggle,
  className,
}: CourseSelectorProps) => {
  const [open, setOpen] = useState(false);

  return (
    <div className={cn("space-y-3", className)}>
      <Label className="text-lg font-semibold">Курсы</Label>
      <MultiSelect
        open={open}
        onOpenChange={setOpen}
        selectedItems={courses}
        onToggle={onToggle}
        items={COURSES}
        placeholder="Выберите курсы..."
        emptyText="Курс не найден."
        renderBadge={(course) => (
          <Badge variant="secondary" className="mr-1">
            {course} курс
          </Badge>
        )}
        renderItem={(course) => `${course} курс`}
        getItemKey={(course) => course.toString()}
        getItemValue={(course) => `${course} курс`}
        isSelected={(course) => courses.includes(course)}
      />
    </div>
  );
};
