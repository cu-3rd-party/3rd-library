import { ExternalLink } from "lucide-react";

import { openMaterialFile } from "@/entities/submission/lib";
import { MaterialSubmissionFile } from "@/entities/submission/model";
import { cn } from "@/shared/lib";
import {
  Button,
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  getFileIcon,
  getFileIconStyles,
} from "@/shared/ui";

const formatUploadDate = (submittedAt?: string, fallback?: string) => {
  if (!submittedAt) return fallback || "Не указана";
  const date = new Date(submittedAt);
  if (Number.isNaN(date.getTime())) return fallback || "Не указана";
  return new Intl.DateTimeFormat("ru-RU", {
    day: "2-digit",
    month: "2-digit",
    year: "numeric",
  }).format(date);
};

interface Props {
  files: MaterialSubmissionFile[];
  submittedAt?: string;
  pubDate?: string;
}

export const MaterialDetailsFiles = ({ files, submittedAt, pubDate }: Props) => {
  const uploadDate = formatUploadDate(submittedAt, pubDate);

  return (
    <>
      <Card className="border-border bg-card">
        <CardHeader className="pb-2">
          <CardTitle className="text-base">Файлы материала</CardTitle>
        </CardHeader>
        <CardContent className="space-y-2">
          {files.length > 0 ? (
            files.map((file, index) => (
              <Button
                key={`${file.name}-${index}`}
                type="button"
                variant="outline"
                onClick={() => openMaterialFile(file)}
                className="group h-auto w-full justify-between rounded-lg px-3 py-3"
              >
                <span className="flex min-w-0 items-center gap-3">
                  <span className={cn("shrink-0 rounded-md p-1.5", getFileIconStyles(file.name))}>
                    {getFileIcon(file.name)}
                  </span>
                  <span className="min-w-0 space-y-1 text-left">
                    <span
                      className="block truncate text-sm font-medium text-foreground"
                      title={file.name}
                    >
                      {file.name}
                    </span>
                    <span className="block text-xs text-muted-foreground">
                      {(file.extension || "unknown").toUpperCase()} •{" "}
                      {(file.sizeBytes / 1024 / 1024).toFixed(2)} MB
                    </span>
                  </span>
                </span>
                <ExternalLink className="h-4 w-4 shrink-0 text-muted-foreground transition-colors group-hover:text-foreground" />
              </Button>
            ))
          ) : (
            <p className="text-sm text-muted-foreground">Файлы не прикреплены</p>
          )}
        </CardContent>
      </Card>
      <p className="text-right text-xs text-muted-foreground">Загружен: {uploadDate}</p>
    </>
  );
};
