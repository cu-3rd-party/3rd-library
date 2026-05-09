import { Course, COURSES } from "@/entities/material/model";
import { Button } from "@/shared/ui";

type CourseSelectorProps = {
  selectedCourse: string;
  onSelect: (course: Course) => void;
};

export const CourseSelector = ({
  selectedCourse,
  onSelect,
}: CourseSelectorProps) => {
  return (
    <div className="flex gap-2">
      {COURSES.map((course) => (
        <Button
          key={course}
          onClick={() => onSelect(course)}
          variant={
            selectedCourse === course ? "secondaryActive" : "secondaryInactive"
          }
          className={`font-semibold text-sm px-6`}
        >
          {course} курс
        </Button>
      ))}
    </div>
  );
};
