import { ExternalLink } from "lucide-react";

import { cn } from "@/shared/lib";
import { Button, getFileIcon, getFileIconStyles } from "@/shared/ui";

import { MaterialSubmissionFile } from "../model";

const formatBytes = (value: number) => `${(value / 1024 / 1024).toFixed(2)} MB`;
const formatDateTime = (value?: string) =>
  value ? new Date(value).toLocaleString("ru-RU") : "Еще не было";

type SubmissionFileSectionProps = {
  files: MaterialSubmissionFile[];
  submittedAt?: string;
  onOpenFile: (file: MaterialSubmissionFile) => void;
};

export const SubmissionFileSection = ({
  files,
  submittedAt,
  onOpenFile,
}: SubmissionFileSectionProps) => (
  <div className="rounded-xl border border-border bg-muted/30 p-4">
    <p className="text-sm font-medium text-foreground">
      Файлы ({files.length})
    </p>
    {files.length > 0 ? (
      <div className="mt-3 space-y-2">
        {files.map((file, index) => (
          <Button
            key={`${file.name}-${index}`}
            type="button"
            variant="outline"
            onClick={() => onOpenFile(file)}
            className="group h-auto w-full justify-between rounded-lg px-3 py-3"
          >
            <span className="flex min-w-0 items-center gap-3">
              <span
                className={cn(
                  "shrink-0 rounded-md p-1.5",
                  getFileIconStyles(file.name),
                )}
              >
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
                  {formatBytes(file.sizeBytes)}
                </span>
              </span>
            </span>
            <ExternalLink className="h-4 w-4 shrink-0 text-muted-foreground transition-colors group-hover:text-foreground" />
          </Button>
        ))}

        <div className="flex flex-wrap items-center gap-x-2 gap-y-1 text-xs text-muted-foreground">
          <span>Отправлена {formatDateTime(submittedAt)}</span>
        </div>
      </div>
    ) : (
      <p className="mt-2 text-sm text-muted-foreground">Файлы не прикреплены</p>
    )}
  </div>
);
