import { FileText, X } from "lucide-react";
import { useRef } from "react";

import { MaterialUploadValue } from "@/entities/submission/model";
import { ALLOWED_MATERIAL_EXTENSIONS } from "@/shared/constants";
import { cn } from "@/shared/lib";
import { Button, getFileIcon, getFileIconStyles, Label } from "@/shared/ui";

const MAX_FILES_COUNT = 5;

type FileUploadAreaProps = {
  files: MaterialUploadValue[];
  onFilesSelect: (files: File[]) => void;
  onFileRemove: (index: number) => void;
};

export const FileUploadArea = ({
  files,
  onFilesSelect,
  onFileRemove,
}: FileUploadAreaProps) => {
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const selectedFiles = Array.from(e.target.files || []);
    if (selectedFiles.length === 0) return;

    const validFiles = selectedFiles.filter((selectedFile) => {
      const fileName = selectedFile.name.toLowerCase();
      return ALLOWED_MATERIAL_EXTENSIONS.some((ext) => fileName.endsWith(ext));
    });

    if (validFiles.length !== selectedFiles.length) {
      alert("Пожалуйста, выберите файлы: PDF, Word, PowerPoint или .ipynb");
    }

    const availableSlots = Math.max(0, MAX_FILES_COUNT - files.length);
    if (availableSlots === 0) {
      alert(`Можно прикрепить не больше ${MAX_FILES_COUNT} файлов.`);
      e.target.value = "";
      return;
    }

    if (validFiles.length > availableSlots) {
      alert(`Можно прикрепить не больше ${MAX_FILES_COUNT} файлов.`);
    }

    const nextFiles = validFiles.slice(0, availableSlots);
    if (nextFiles.length > 0) {
      onFilesSelect(nextFiles);
    }

    e.target.value = "";
  };

  const handleTriggerClick = () => {
    fileInputRef.current?.click();
  };

  return (
    <div className="space-y-3">
      <Label className="text-lg font-semibold">Файлы материала</Label>
      <input
        type="file"
        multiple
        accept={ALLOWED_MATERIAL_EXTENSIONS.join(", ")}
        ref={fileInputRef}
        onChange={handleFileChange}
        className="hidden"
      />

      {files.length === 0 ? (
        <div
          onClick={handleTriggerClick}
          className="group bg-input border-2 border-dashed border-border hover:border-ring rounded-lg p-6 
          lg:p-8 flex flex-col items-center justify-center cursor-pointer transition-colors duration-(--std-duration) gap-3 text-center touch-manipulation"
        >
          <div className="p-3 bg-muted rounded-full">
            <FileText className="h-6 w-6 text-muted-foreground transition-colors duration-(--std-duration) group-hover:text-ring" />
          </div>
          <div>
            <span className="font-semibold text-foreground group-hover:text-primary-foreground transition-colors duration-(--std-duration) block mb-1">
              Загрузить файлы
            </span>
            <span className="text-xs text-muted-foreground">
              До {MAX_FILES_COUNT} файлов, разрешенные форматы:{" "}
              {ALLOWED_MATERIAL_EXTENSIONS.join(", ")}
            </span>
          </div>
        </div>
      ) : (
        <div className="space-y-3">
          <div className="flex items-center justify-between gap-2">
            <span className="text-xs text-muted-foreground">
              Выбрано файлов: {files.length}/{MAX_FILES_COUNT}
            </span>
            {files.length < MAX_FILES_COUNT && (
              <Button
                type="button"
                variant="outline"
                size="sm"
                onClick={handleTriggerClick}
              >
                Добавить файл
              </Button>
            )}
          </div>

          <div className="space-y-2">
            {files.map((file, index) => {
              const fileName = file.name;
              const fileSize =
                file instanceof File ? file.size : file.sizeBytes;

              return (
                <div
                  key={`${fileName}-${index}`}
                  className="flex items-center gap-3 p-4 border border-border rounded-lg bg-card overflow-hidden"
                >
                  <div
                    className={cn("p-2 rounded", getFileIconStyles(fileName))}
                  >
                    {getFileIcon(fileName)}
                  </div>
                  <div className="flex flex-col flex-1 min-w-0">
                    <span
                      className="font-medium text-sm truncate"
                      title={fileName}
                    >
                      {fileName}
                    </span>
                    <span className="text-xs text-muted-foreground">
                      {fileSize
                        ? `${(fileSize / 1024 / 1024).toFixed(2)} MB`
                        : "Размер не указан"}
                    </span>
                  </div>
                  <Button
                    type="button"
                    variant="ghost"
                    size="icon"
                    onClick={() => onFileRemove(index)}
                  >
                    <X className="h-5 w-5" />
                  </Button>
                </div>
              );
            })}
          </div>
        </div>
      )}
    </div>
  );
};
