import { Button } from "@/components/ui/button";
import { COURSES } from "@/constants";

type CourseSelectorProps = {
  selectedCourse: number;
  onSelect: (course: number) => void;
};

export const CourseSelector = ({
  selectedCourse,
  onSelect,
}: CourseSelectorProps) => {
  return (
    <div className="flex gap-2">
      {COURSES.map((c) => (
        <Button
          key={c}
          onClick={() => onSelect(c)}
          variant={
            selectedCourse === c ? "secondaryActive" : "secondaryInactive"
          }
          className={`font-semibold text-sm px-6`}
        >
          {c} курс
        </Button>
      ))}
    </div>
  );
};
