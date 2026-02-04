import { FileText, X } from "lucide-react";
import { useRef } from "react";

import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import { ALLOWED_MATERIAL_EXTENSIONS } from "@/constants";
import { cn } from "@/lib/utils";
import { getFileIcon, getFileIconStyles } from "@/utils";

type FileUploadAreaProps = {
  file: File | null;
  onFileSelect: (file: File) => void;
  onFileRemove: () => void;
};

export const FileUploadArea = ({
  file,
  onFileSelect,
  onFileRemove,
}: FileUploadAreaProps) => {
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files[0]) {
      const selectedFile = e.target.files[0];
      const fileName = selectedFile.name.toLowerCase();
      const isValid = ALLOWED_MATERIAL_EXTENSIONS.some((ext) => fileName.endsWith(ext));

      if (isValid) {
        onFileSelect(selectedFile);
      } else {
        alert("Пожалуйста, выберите файл: PDF, Word, PowerPoint или .ipynb");  // TODO
      }
    }
  };

  const handleTriggerClick = () => {
    fileInputRef.current?.click();
  };

  return (
    <div className="space-y-3">
      <Label className="text-lg font-semibold">Файл материала</Label>
      <input
        type="file"
        accept={ALLOWED_MATERIAL_EXTENSIONS.join(", ")}
        ref={fileInputRef}
        onChange={handleFileChange}
        className="hidden"
      />

      {!file ? (
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
              Загрузить файл
            </span>
            <span className="text-xs text-muted-foreground">
              { ALLOWED_MATERIAL_EXTENSIONS.join(", ") }
            </span>
          </div>
        </div>
      ) : (
        <div className="flex items-center gap-3 p-4 border border-border rounded-lg bg-card overflow-hidden">
          <div
            className={cn("p-2 rounded", getFileIconStyles(file.name))}
          >
            {getFileIcon(file.name)}
          </div>
          <div className="flex flex-col flex-1 min-w-0">
            <span className="font-medium text-sm truncate">{file.name}</span>
            <span className="text-xs text-muted-foreground">
              {(file.size / 1024 / 1024).toFixed(2)} MB
            </span>
          </div>
          <Button
            variant="ghost"
            size="icon"
            onClick={onFileRemove}
          >
            <X className="h-5 w-5" />
          </Button>
        </div>
      )}
    </div>
  );
};
