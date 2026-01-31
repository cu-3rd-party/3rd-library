import { CircleAlert, SendHorizonal } from "lucide-react";
import { useState } from "react";

import { FileUploadArea } from "@/common/atoms/";
import {MaterialAttributesSelector} from "@/common/molecules ";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip";


export const UploadMaterialForm = () => {
  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");
  const [file, setFile] = useState<File | null>(null);

  const [courses, setCourses] = useState<string[]>([]);
  const [subjects, setSubjects] = useState<string[]>([]);
  const [difficulties, setDifficulties] = useState<string[]>(["none"]);

  const toggleSelection = (item: string, currentList: string[], setList: (l: string[]) => void) => {
    if (currentList.includes(item)) {
      setList(currentList.filter((i) => i !== item));
    } else {
      setList([...currentList, item]);
    }
  };

  const toggleDifficulty = (value: string) => {
    setDifficulties((prev) => {
      if (value === "none") return ["none"];
      let newSelection = [...prev];
      if (newSelection.includes("none")) newSelection = newSelection.filter((i) => i !== "none");
      if (newSelection.includes(value)) {
        newSelection = newSelection.filter((i) => i !== value);
      } else {
        newSelection.push(value);
      }
      if (newSelection.length === 0) return ["none"];
      return newSelection;
    });
  };

  const handleSubmit = () => {
    console.log({ title, description, file, courses, subjects, difficulties });
    alert("Отправлено!");
  };

  return (
    <div className="space-y-4 md:space-y-8">
      {/* Название */}
      <div className="space-y-3">
        <Label htmlFor="title" className="text-lg font-semibold">Название</Label>
        <Input
          id="title"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          placeholder="Например: Полный курс лекций по матанализу"
          className="bg-background text-base md:text-lg py-6"
        />
      </div>

      <MaterialAttributesSelector
        courses={courses}
        subjects={subjects}
        difficulties={difficulties}
        onToggleCourse={(c) => toggleSelection(c, courses, setCourses)}
        onToggleSubject={(s) => toggleSelection(s, subjects, setSubjects)}
        onToggleDifficulty={toggleDifficulty}
      />

      <div className="space-y-3">
        <Label htmlFor="description" className="text-lg font-semibold">Описание</Label>
        <Textarea
          id="description"
          value={description}
          onChange={(e) => setDescription(e.target.value)}
          placeholder="Краткое содержание..."
          className="bg-background min-h-[150px] text-base resize-y"
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
            <TooltipContent side="top" className="max-w-[200px] text-center md:text-left">
              <p>Все материалы проходят предварительную проверку модератором перед публикацией.</p>
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