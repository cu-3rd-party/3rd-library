import { User } from "lucide-react";
import { useEffect, useState } from "react";
import { Link } from "react-router-dom";

import { DIFFICULTY_CONFIG } from "@/entities/material/lib";
import { MaterialDetailsResponse } from "../model";
import { AUTHORS_PREFIX } from "@/shared/constants";
import { cn } from "@/shared/lib";

interface Props {
  materialDetails: MaterialDetailsResponse;
}

export const MaterialDetailsHeader = ({ materialDetails }: Props) => {
  const [imageError, setImageError] = useState(false);

  useEffect(() => {
    setImageError(false);
  }, [materialDetails.authorId, materialDetails.authorImage]);

  const difficultyData = DIFFICULTY_CONFIG[materialDetails.difficulty];
  const subjectPreview = materialDetails.subjects.slice(0, 2);
  const subjectText = subjectPreview.length > 0 ? subjectPreview.join(" · ") : "Не указан";
  const extraSubjectsCount = Math.max(0, materialDetails.subjects.length - 2);
  const authorAvatarSrc = materialDetails.authorImage || null;

  return (
    <div className="space-y-3">
      <div className="flex flex-col gap-3 sm:flex-row sm:flex-wrap sm:items-start">
        <div className="min-w-0 flex-1">
          <h1 className="min-w-0 break-words text-2xl font-bold leading-tight tracking-tight lg:text-3xl">
            {materialDetails.title}
          </h1>
        </div>
        <div className="flex w-full flex-wrap items-center gap-2 sm:ml-auto sm:w-auto sm:justify-end">
          <p className="max-w-full text-base font-semibold text-foreground sm:max-w-xs sm:text-right lg:text-lg">
            {subjectText}
            {extraSubjectsCount > 0 && (
              <span className="ml-1 text-sm text-muted-foreground">+{extraSubjectsCount}</span>
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
      </div>
      <div className="flex items-center gap-3 text-sm text-muted-foreground">
        <span className="flex h-8 w-8 items-center justify-center rounded-full border border-border bg-muted/40">
          {!imageError && authorAvatarSrc ? (
            <img
              src={authorAvatarSrc}
              alt={materialDetails.authorName || "Автор"}
              className="h-full w-full rounded-full object-cover"
              onError={() => setImageError(true)}
            />
          ) : (
            <User className="h-4 w-4 text-foreground/80" />
          )}
        </span>
        <p>
          {materialDetails.authorName ? (
            materialDetails.authorId ? (
              <Link
                to={`${AUTHORS_PREFIX}/${encodeURIComponent(materialDetails.authorId)}`}
                className="font-medium text-foreground underline-offset-4 hover:underline"
              >
                {materialDetails.authorName}
              </Link>
            ) : (
              <span className="font-medium text-foreground">{materialDetails.authorName}</span>
            )
          ) : (
            <span className="font-medium text-foreground">Не указан</span>
          )}
        </p>
      </div>
    </div>
  );
};
