import { SendHorizonal } from "lucide-react";
import type { FormEvent } from "react";

import { FileUploadArea } from "@/common/atoms/";
import { MaterialAttributesSelector } from "@/common/molecules";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import {
  Course,
  Difficulty,
  MaterialType,
  MaterialUploadValue,
  Subject,
} from "@/models";

export type UploadMaterialFormValues = {
  title: string;
  description: string;
  files: MaterialUploadValue[];
  courses: Course[];
  subjects: Subject[];
  difficulty: Difficulty;
  type: MaterialType;
};

type UploadMaterialFormProps = {
  values: UploadMaterialFormValues;
  onTitleChange: (value: string) => void;
  onDescriptionChange: (value: string) => void;
  onFilesSelect: (files: File[]) => void;
  onFileRemove: (index: number) => void;
  onToggleCourse: (item: Course) => void;
  onToggleSubject: (item: Subject) => void;
  onSelectType: (value: MaterialType) => void;
  onSelectDifficulty: (value: Difficulty) => void;
  onSubmit: () => void;
  submitLabel?: string;
  submitDisabled?: boolean;
};

export const UploadMaterialForm = ({
  values,
  onTitleChange,
  onDescriptionChange,
  onFilesSelect,
  onFileRemove,
  onToggleCourse,
  onToggleSubject,
  onSelectType,
  onSelectDifficulty,
  onSubmit,
  submitLabel = "Отправить материал",
  submitDisabled = false,
}: UploadMaterialFormProps) => {
  const handleSubmit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    onSubmit();
  };

  return (
    <form className="space-y-4 lg:space-y-8" onSubmit={handleSubmit}>
      <div className="space-y-3">
        <Label htmlFor="material-title" className="text-lg font-semibold">
          Название
        </Label>
        <Input
          id="material-title"
          value={values.title}
          onChange={(e) => onTitleChange(e.target.value)}
          placeholder="Например: Курс лекций по матану"
          className="py-6 placeholder:truncate"
        />
      </div>

      <MaterialAttributesSelector
        courses={values.courses}
        subjects={values.subjects}
        difficulty={values.difficulty}
        type={values.type}
        onToggleCourse={onToggleCourse}
        onToggleSubject={onToggleSubject}
        onSelectType={onSelectType}
        onSelectDifficulty={onSelectDifficulty}
      />

      <div className="space-y-3">
        <Label htmlFor="material-description" className="text-lg font-semibold">
          Описание
        </Label>
        <Textarea
          id="material-description"
          value={values.description}
          onChange={(e) => onDescriptionChange(e.target.value)}
          placeholder="Краткое содержание..."
          className="min-h-37.5 resize-y"
        />
      </div>

      <FileUploadArea
        files={values.files}
        onFilesSelect={onFilesSelect}
        onFileRemove={onFileRemove}
      />

      <div className="pt-6 flex items-center justify-center lg:justify-end gap-4 lg:gap-6">
        <Button
          type="submit"
          size="lg"
          className="font-semibold px-4 flex items-center justify-center"
          disabled={submitDisabled}
        >
          {submitLabel}
          <SendHorizonal className="h-5 w-5" />
        </Button>
      </div>
    </form>
  );
};
