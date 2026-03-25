import { ExternalLink, User } from "lucide-react";
import { useMemo } from "react";
import { Link, useNavigate, useParams } from "react-router-dom";

import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
  AUTHORS_PREFIX,
  DIFFICULTY_CONFIG,
  MATERIALS_PREFIX,
  TYPE_CONFIG,
} from "@/constants";
import { cn } from "@/lib/utils";
import { getSubmissionFiles, useMaterialSubmissionStore } from "@/store";
import {
  getCourseName,
  getFileIcon,
  getFileIconStyles,
  openMaterialFile,
} from "@/utils";

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

const MaterialDetailsPage = () => {
  const { id } = useParams();
  const navigate = useNavigate();
  const submissions = useMaterialSubmissionStore((state) => state.submissions);

  const submission = useMemo(
    () =>
      submissions.find(
        (item) => item.material.id === id && item.status === "approved",
      ) || null,
    [id, submissions],
  );

  const handleOpenFile = openMaterialFile;

  const typeData = submission ? TYPE_CONFIG[submission.material.type] : null;
  const difficultyData = submission
    ? DIFFICULTY_CONFIG[submission.material.difficulty]
    : null;
  const subjectPreview = submission?.material.subjects.slice(0, 2) || [];
  const subjectText =
    subjectPreview.length > 0 ? subjectPreview.join(" · ") : "Не указан";
  const extraSubjectsCount = Math.max(
    0,
    (submission?.material.subjects.length || 0) - 2,
  );
  const uploadDate = submission
    ? formatUploadDate(submission.submittedAt, submission.material.pubDate)
    : "Не указана";
  const files = submission ? getSubmissionFiles(submission) : [];

  if (!submission) {
    return (
      <div className="w-full px-4 py-6 xl:w-11/12 mx-auto max-w-screen-2xl">
        <div className="rounded-xl border border-border bg-card px-6 py-14 text-center text-muted-foreground">
          <p className="text-base">Материал не найден</p>
          <Button
            variant="outline"
            className="mt-4"
            onClick={() => navigate(MATERIALS_PREFIX)}
          >
            К материалам
          </Button>
        </div>
      </div>
    );
  }

  return (
    <div className="w-full px-4 py-6 xl:w-11/12 mx-auto max-w-screen-2xl space-y-6 lg:space-y-8">
      <div className="space-y-3">
        <div className="flex flex-wrap items-start gap-3">
          <div className="min-w-0 flex-1">
            <div className="flex flex-wrap items-center gap-2">
              <h1 className="min-w-0 break-words text-2xl font-bold leading-tight tracking-tight lg:text-3xl">
                {submission.material.title}
              </h1>
            </div>
          </div>
          <p className="ml-auto max-w-full text-right text-base font-semibold text-foreground sm:max-w-xs lg:text-lg">
            {subjectText}
            {extraSubjectsCount > 0 && (
              <span className="ml-1 text-sm text-muted-foreground">
                +{extraSubjectsCount}
              </span>
            )}
          </p>
          {difficultyData && (
            <span
              className={cn(
                "inline-flex shrink-0 items-center rounded-full border border-border/60 px-2.5 py-1 text-xs font-semibold lg:text-sm",
                difficultyData.className || "bg-muted text-foreground",
              )}
            >
              {difficultyData.label}
            </span>
          )}
        </div>
        <div className="flex items-center gap-3 text-sm text-muted-foreground">
          <span className="flex h-8 w-8 items-center justify-center rounded-full border border-border bg-muted/40">
            <User className="h-4 w-4 text-foreground/80" />
          </span>
          <p>
            
            {submission.material.authorName ? (
              submission.material.authorId ? (
                <Link
                  to={`${AUTHORS_PREFIX}/${submission.material.authorId}`}
                  className="font-medium text-foreground underline-offset-4 hover:underline"
                >
                  {submission.material.authorName}
                </Link>
              ) : (
                <span className="font-medium text-foreground">
                  {submission.material.authorName}
                </span>
              )
            ) : (
              <span className="font-medium text-foreground">Не указан</span>
            )}
          </p>
        </div>
        <div className="flex flex-wrap items-center gap-x-4 gap-y-1 text-sm lg:text-base font-medium text-foreground">
          <p>{getCourseName(submission.material.courses)}</p>
          {typeData && <p>{typeData.label}</p>}
        </div>
        <p className="max-w-4xl whitespace-pre-wrap text-sm leading-relaxed text-muted-foreground">
          {submission.material.description}
        </p>
      </div>

      <Card className="border-border bg-card">
        <CardHeader className="pb-2">
          <CardTitle className="text-base">Файлы материала</CardTitle>
        </CardHeader>
        <CardContent className="space-y-2">
          {files.length > 0 ? (
            <>
              {files.map((file, index) => (
                <Button
                  key={`${file.name}-${index}`}
                  type="button"
                  variant="outline"
                  onClick={() => handleOpenFile(file)}
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
                        {(file.sizeBytes / 1024 / 1024).toFixed(2)} MB
                      </span>
                    </span>
                  </span>
                  <ExternalLink className="h-4 w-4 shrink-0 text-muted-foreground transition-colors group-hover:text-foreground" />
                </Button>
              ))}
            </>
          ) : (
            <p className="text-sm text-muted-foreground">
              Файлы не прикреплены
            </p>
          )}
        </CardContent>
      </Card>
      <p className="text-right text-xs text-muted-foreground">
        Загружен: {uploadDate}
      </p>
    </div>
  );
};

export default MaterialDetailsPage;
