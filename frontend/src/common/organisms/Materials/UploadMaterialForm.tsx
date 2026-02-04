import { CircleAlert, SendHorizonal } from "lucide-react";
import { useState } from "react";

import { FileUploadArea } from "@/common/atoms/";
import { MaterialAttributesSelector } from "@/common/molecules";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
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
    <div className="space-y-4 md:space-y-8">
      <div className="space-y-3">
        <Label htmlFor="title" className="text-lg font-semibold">
          Название
        </Label>
        <Input
          id="title"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          placeholder="Например: Полный курс лекций по матанализу"
          className="text-base md:text-lg py-6"
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
        <Label htmlFor="description" className="text-lg font-semibold">
          Описание
        </Label>
        <Textarea
          id="description"
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

      <div className="pt-6 flex flex-col-reverse md:flex-row items-center justify-end gap-4 md:gap-6">
        <TooltipProvider>
          <Tooltip delayDuration={300}>
            <TooltipTrigger asChild>
              <div className="flex items-center gap-2 cursor-help text-muted-foreground hover:text-orange-500 transition-colors py-2 md:py-0">
                <span className="md:hidden text-sm">О модерации</span>
                <CircleAlert className="h-6 w-6" />
              </div>
            </TooltipTrigger>
            <TooltipContent
              side="top"
              className="max-w-50 text-center md:text-left"
            >
              <p>
                Все материалы проходят предварительную проверку модератором
                перед публикацией.
              </p>
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>

        <Button
          size="lg"
          className="w-full md:w-auto bg-orange-500 text-primary-foreground hover:bg-orange-600 font-bold px-4 text-md shadow-md transition-transform active:scale-95 flex items-center justify-center"
          onClick={handleSubmit}
        >
          Отправить
          <SendHorizonal className="h-5 w-5" />
        </Button>
      </div>
    </div>
  );
};
