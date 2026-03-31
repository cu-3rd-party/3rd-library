import { useState } from "react";

import { MultiSelect } from "@/common/molecules";
import { Badge } from "@/components/ui/badge";
import { Label } from "@/components/ui/label";
import { COURSES } from "@/constants";
import { cn } from "@/lib/utils";
import { Course } from "@/models";

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
