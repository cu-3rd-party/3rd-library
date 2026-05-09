import { ExternalLink, User } from "lucide-react";
import { useEffect, useState } from "react";
import { Link, useNavigate, useParams } from "react-router-dom";

import { MOCK_SUBMISSIONS } from "@/app/mocks/data/submission";
import {
  LibraryMaterialDetailsResponse,
  RealWorldArticleResponse,
  RealWorldAttachmentsResponse,
} from "@/entities/material/api";
import {
  DIFFICULTY_CONFIG,
  getCourseName,
  mapArticleToMaterialDetails,
  mapAttachmentToMaterialFile,
  mapLibraryMaterialToMaterial,
  TYPE_CONFIG,
} from "@/entities/material/lib";
import { Material } from "@/entities/material/model";
import { fetchJsonWithAuth } from "@/entities/session/api";
import {
  mapLibraryMaterialFileToSubmissionFile,
  openMaterialFile,
} from "@/entities/submission/lib";
import { MaterialSubmissionFile } from "@/entities/submission/model";
import { RealWorldProfileResponse } from "@/entities/user/api";
import { ApiRequestError, resolveApiUrl } from "@/shared/api";
import { AUTHORS_PREFIX, MATERIALS_PREFIX } from "@/shared/constants";
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

type MaterialDetailsResponse = Material & {
  authorImage?: string | null;
  files: MaterialSubmissionFile[];
  submittedAt?: string;
  publishedAt?: string;
};

const getFallbackMaterial = (
  materialId: string,
): MaterialDetailsResponse | null => {
  const submission =
    MOCK_SUBMISSIONS.find(
      (item) => item.material.id === materialId && item.status === "approved",
    ) || null;

  if (!submission) return null;

  return {
    ...submission.material,
    files: submission.files,
    submittedAt: submission.submittedAt,
    publishedAt: submission.publishedAt,
  };
};

const MaterialDetailsPage = () => {
  const { id } = useParams();
  const navigate = useNavigate();
  const [materialDetails, setMaterialDetails] =
    useState<MaterialDetailsResponse | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [isError, setIsError] = useState(false);
  const [imageError, setImageError] = useState(false);

  useEffect(() => {
    const materialId = id || "";
    if (!materialId) {
      setIsLoading(false);
      setIsError(true);
      return;
    }

    const abortController = new AbortController();

    const fetchMaterial = async () => {
      setIsLoading(true);
      setIsError(false);

      try {
        try {
          const materialPayload =
            await fetchJsonWithAuth<LibraryMaterialDetailsResponse>(
              resolveApiUrl(`/api/materials/${encodeURIComponent(materialId)}`),
              { signal: abortController.signal },
            );

          const mappedMaterial = mapLibraryMaterialToMaterial(materialPayload);

          setMaterialDetails({
            ...mappedMaterial,
            authorImage: materialPayload.author_image || null,
            files: materialPayload.files.map(
              mapLibraryMaterialFileToSubmissionFile,
            ),
            submittedAt: materialPayload.submitted_at || undefined,
            publishedAt: materialPayload.published_at || undefined,
          });
          return;
        } catch (error) {
          if (!(error instanceof ApiRequestError) || error.status !== 404) {
            throw error;
          }
        }

        const articlePath = `/api/articles/${encodeURIComponent(materialId)}`;
        const [articlePayload, attachmentsPayload] = await Promise.all([
          fetchJsonWithAuth<RealWorldArticleResponse>(
            resolveApiUrl(articlePath),
            {
              signal: abortController.signal,
            },
          ),
          fetchJsonWithAuth<RealWorldAttachmentsResponse>(
            resolveApiUrl(`${articlePath}/attachments`),
            { signal: abortController.signal },
          ).catch((error: unknown) => {
            if (!abortController.signal.aborted) {
              console.warn(
                "[MaterialDetails] Failed to load attachments.",
                error,
              );
            }
            return { attachments: [] };
          }),
        ]);

        const mappedMaterial = mapArticleToMaterialDetails(
          articlePayload.article,
        );
        const encodedAuthorId = encodeURIComponent(mappedMaterial.authorId);
        const authorProfilePayload =
          await fetchJsonWithAuth<RealWorldProfileResponse>(
            resolveApiUrl(`/api/profiles/${encodedAuthorId}`),
            {
              signal: abortController.signal,
            },
          ).catch((error: unknown) => {
            if (!abortController.signal.aborted) {
              console.warn(
                "[MaterialDetails] Failed to load author profile.",
                error,
              );
            }
            return null;
          });

        const files = attachmentsPayload.attachments.map((attachment) =>
          mapAttachmentToMaterialFile(materialId, attachment),
        );
        setMaterialDetails({
          ...mappedMaterial,
          authorImage:
            authorProfilePayload?.profile.image ??
            articlePayload.article.author.image,
          files,
        });
      } catch (error) {
        if (abortController.signal.aborted) return;

        if (import.meta.env.VITE_API === "mock") {
          const fallbackMaterial = getFallbackMaterial(materialId);
          if (fallbackMaterial) {
            console.warn(
              "[MaterialDetails] Falling back to local mock details.",
            );
            setMaterialDetails(fallbackMaterial);
            setIsError(false);
            return;
          }
        }

        console.error(error);
        setIsError(true);
      } finally {
        if (!abortController.signal.aborted) {
          setIsLoading(false);
        }
      }
    };

    fetchMaterial();

    return () => abortController.abort();
  }, [id]);

  useEffect(() => {
    setImageError(false);
  }, [materialDetails?.authorId, materialDetails?.authorImage]);

  const handleOpenFile = openMaterialFile;

  const typeData = materialDetails ? TYPE_CONFIG[materialDetails.type] : null;
  const difficultyData = materialDetails
    ? DIFFICULTY_CONFIG[materialDetails.difficulty]
    : null;
  const subjectPreview = materialDetails?.subjects.slice(0, 2) || [];
  const subjectText =
    subjectPreview.length > 0 ? subjectPreview.join(" · ") : "Не указан";
  const extraSubjectsCount = Math.max(
    0,
    (materialDetails?.subjects.length || 0) - 2,
  );
  const uploadDate = materialDetails
    ? formatUploadDate(materialDetails.submittedAt, materialDetails.pubDate)
    : "Не указана";
  const files = materialDetails?.files || [];
  const authorAvatarSrc = materialDetails
    ? materialDetails.authorImage || null
    : null;

  if (isLoading) {
    return (
      <div className="w-full px-4 py-6 xl:w-11/12 mx-auto max-w-screen-2xl">
        <div className="rounded-xl border border-border bg-card px-6 py-14 text-center text-muted-foreground">
          <p className="text-base">Загружаем материал...</p>
        </div>
      </div>
    );
  }

  if (isError || !materialDetails) {
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
        <div className="flex flex-col gap-3 sm:flex-row sm:flex-wrap sm:items-start">
          <div className="min-w-0 flex-1">
            <div className="flex flex-wrap items-center gap-2">
              <h1 className="min-w-0 break-words text-2xl font-bold leading-tight tracking-tight lg:text-3xl">
                {materialDetails.title}
              </h1>
            </div>
          </div>
          <div className="flex w-full flex-wrap items-center gap-2 sm:ml-auto sm:w-auto sm:justify-end">
            <p className="max-w-full text-base font-semibold text-foreground sm:max-w-xs sm:text-right lg:text-lg">
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
                <span className="font-medium text-foreground">
                  {materialDetails.authorName}
                </span>
              )
            ) : (
              <span className="font-medium text-foreground">Не указан</span>
            )}
          </p>
        </div>
        <div className="flex flex-wrap items-center gap-x-4 gap-y-1 text-sm lg:text-base font-medium text-foreground">
          <p>{getCourseName(materialDetails.courses)}</p>
          {typeData && <p>{typeData.label}</p>}
        </div>
        <p className="max-w-4xl whitespace-pre-wrap text-sm leading-relaxed text-muted-foreground">
          {materialDetails.description}
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
