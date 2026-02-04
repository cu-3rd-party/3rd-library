import { SendHorizonal } from "lucide-react";
import { useState } from "react";

import { FileUploadArea } from "@/common/atoms/";
import { MaterialAttributesSelector } from "@/common/molecules";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { Course, Difficulty, MaterialType, Subject } from "@/models";

export const UploadMaterialForm = () => {
  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");
  const [file, setFile] = useState<File | null>(null);

  const [courses, setCourses] = useState<Course[]>([]);
  const [subjects, setSubjects] = useState<Subject[]>([]);
  const [difficulty, setDifficulty] = useState<Difficulty>("none");
  const [type, setType] = useState<MaterialType>("longread");

  const toggleCourses = (
    item: Course,
  ) => {
    if (courses.includes(item)) {
      setCourses(courses.filter((i) => i !== item));
    } else {
      setCourses([...courses, item].sort());
    }
  };

  const toggleSubjects = (
    item: Subject,
  ) => {
    if (subjects.includes(item)) {
      setSubjects(subjects.filter((i) => i !== item));
    } else {
      setSubjects([...subjects, item].sort());
    }
  };

  const selectType = (value: MaterialType) => {
    setType(value);
  };

  const selectDifficulty = (value: Difficulty) => {
    setDifficulty(value);
  };


  const handleSubmit = () => {
    console.log({ title, description, file, courses, subjects, difficulty });
    alert("Отправлено!");
  };

  return (
    <div className="space-y-4 lg:space-y-8">
      <div className="space-y-3">
        <Label htmlFor="material-title" className="text-lg font-semibold">
          Название
        </Label>
        <Input
          id="material-title"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          placeholder="Например: Полный курс лекций по матанализу"
          className="py-6"
        />
      </div>

      <MaterialAttributesSelector
        courses={courses}
        subjects={subjects}
        difficulty={difficulty}
        type={type}
        onToggleCourse={toggleCourses}
        onToggleSubject={toggleSubjects}
        onSelectType={selectType}
        onSelectDifficulty={selectDifficulty}
      />

      <div className="space-y-3">
        <Label htmlFor="material-description" className="text-lg font-semibold">
          Описание
        </Label>
        <Textarea
          id="material-description"
          value={description}
          onChange={(e) => setDescription(e.target.value)}
          placeholder="Краткое содержание..."
          className="min-h-37.5 resize-y"
        />
      </div>

      <FileUploadArea
        file={file}
        onFileSelect={setFile}
        onFileRemove={() => setFile(null)}
      />

      <div className="pt-6 flex items-center justify-center lg:justify-end gap-4 lg:gap-6">
        <Button
          size="lg"
          className="font-semibold px-4 flex items-center justify-center"
          onClick={handleSubmit}
        >
          Отправить материал
          <SendHorizonal className="h-5 w-5" />
        </Button>
      </div>
    </div>
  );
};
