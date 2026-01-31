import { Button } from "@/components/ui/button";

type CourseSelectorProps = {
  selectedCourse: 1 | 2;
  onSelect: (course: 1 | 2) => void;
};

export const CourseSelector = ({
  selectedCourse,
  onSelect,
}: CourseSelectorProps) => {
  return (
    <div className="flex gap-2">
      <Button
        onClick={() => onSelect(1)}
        className={`font-semibold text-sm px-6 ${
          selectedCourse === 1
            ? "bg-orange-500 hover:bg-orange-600 text-white"
            : "bg-secondary text-secondary-foreground hover:bg-secondary/80"
        }`}
      >
        1 курс
      </Button>
      <Button
        onClick={() => onSelect(2)}
        className={`font-semibold text-md px-6 ${
          selectedCourse === 2
            ? "bg-orange-500 hover:bg-orange-600 text-white"
            : "bg-secondary text-secondary-foreground hover:bg-secondary/80"
        }`}
      >
        2 курс
      </Button>
    </div>
  );
};
