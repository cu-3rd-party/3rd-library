import { FileText, FileCode, X } from "lucide-react";
import { useRef } from "react";

import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import { cn } from "@/lib/utils";

type FileUploadAreaProps = {
  file: File | null;
  onFileSelect: (file: File) => void;
  onFileRemove: () => void;
};

export const FileUploadArea = ({ file, onFileSelect, onFileRemove }: FileUploadAreaProps) => {
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files[0]) {
      const selectedFile = e.target.files[0];
      const fileName = selectedFile.name.toLowerCase();

      const allowedExtensions = [".pdf", ".doc", ".docx", ".ppt", ".pptx", ".ipynb"];
      const isValid = allowedExtensions.some((ext) => fileName.endsWith(ext));

      if (isValid) {
        onFileSelect(selectedFile);
      } else {
        alert("Пожалуйста, выберите файл: PDF, Word, PowerPoint или .ipynb");
      }
    }
  };

  const handleTriggerClick = () => {
    fileInputRef.current?.click();
  };

  const getFileIconStyles = (fileName: string) => {
    const name = fileName.toLowerCase();
    if (name.endsWith(".pdf")) return "bg-red-500/10 text-red-500";
    if (name.endsWith(".doc") || name.endsWith(".docx")) return "bg-blue-500/10 text-blue-500";
    if (name.endsWith(".ppt") || name.endsWith(".pptx")) return "bg-orange-500/10 text-orange-500";
    if (name.endsWith(".ipynb")) return "bg-yellow-500/10 text-yellow-600";
    return "bg-muted text-muted-foreground";
  };

  const getFileIcon = (fileName: string) => {
    if (fileName.toLowerCase().endsWith(".ipynb")) {
      return <FileCode className="h-6 w-6" />;
    }
    return <FileText className="h-6 w-6" />;
  };

  return (
    <div className="space-y-3">
      <Label className="text-lg font-semibold">Файл материала</Label>
      <input
        type="file"
        accept=".pdf,.doc,.docx,.ppt,.pptx,.ipynb"
        ref={fileInputRef}
        onChange={handleFileChange}
        className="hidden"
      />

      {!file ? (
        <div
          onClick={handleTriggerClick}
          className="border-2 border-dashed border-border/60 hover:border-primary/50 hover:bg-muted/30 rounded-lg p-6 md:p-8 flex flex-col items-center justify-center cursor-pointer transition-all gap-3 text-center touch-manipulation"
        >
          <div className="p-3 bg-muted rounded-full">
            <FileText className="h-6 w-6 text-muted-foreground" />
          </div>
          <div>
            <span className="font-semibold text-primary block mb-1">Загрузить файл</span>
            <span className="text-xs text-muted-foreground">PDF, DOCX, PPTX, IPYNB</span>
          </div>
        </div>
      ) : (
        <div className="flex items-center gap-3 p-4 border border-border rounded-lg bg-background overflow-hidden">
          <div className={cn("p-2 rounded shrink-0", getFileIconStyles(file.name))}>
            {getFileIcon(file.name)}
          </div>
          <div className="flex flex-col flex-1 min-w-0">
            <span className="font-medium text-sm truncate">{file.name}</span>
            <span className="text-xs text-muted-foreground">
              {(file.size / 1024 / 1024).toFixed(2)} MB
            </span>
          </div>
          <Button variant="ghost" size="icon" onClick={onFileRemove} className="shrink-0">
            <X className="h-5 w-5" />
          </Button>
        </div>
      )}
    </div>
  );
};