import {
  Course,
  Difficulty,
  MaterialType,
  Subject,
} from "@/entities/material/model";

import { CourseSelector } from "./CourseSelector";
import { DifficultySelector } from "./DifficultySelector";
import { SubjectSelector } from "./SubjectSelector";
import { TypeSelector } from "./TypeSelector";

type AttributesSelectorProps = {
  courses: Course[];
  subjects: Subject[];
  type: MaterialType;
  difficulty: Difficulty;
  onToggleCourse: (val: Course) => void;
  onToggleSubject: (val: Subject) => void;
  onSelectType: (val: MaterialType) => void;
  onSelectDifficulty: (val: Difficulty) => void;
};

export const MaterialAttributesSelector = ({
  courses,
  subjects,
  type,
  difficulty,
  onToggleCourse,
  onToggleSubject,
  onSelectType,
  onSelectDifficulty,
}: AttributesSelectorProps) => {
  return (
    <div className="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-4 gap-4 lg:gap-6">
      <CourseSelector courses={courses} onToggle={onToggleCourse} />
      <SubjectSelector subjects={subjects} onToggle={onToggleSubject} />
      <TypeSelector type={type} onSelect={onSelectType} />
      <DifficultySelector
        difficulty={difficulty}
        onSelect={onSelectDifficulty}
      />
    </div>
  );
};
