import { useEffect, useMemo, useState } from "react";

import {
  UploadMaterialForm,
  type UploadMaterialFormValues,
} from "@/common/organisms/Materials/UploadMaterialForm";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { SUBMISSION_STATUS_UI } from "@/constants";
import {
  ApiRequestError,
  fetchJson,
  fetchWithAuth,
  resolveApiUrl,
} from "@/lib/api";
import {
  mapArticleToMaterial,
  mapAttachmentToMaterialFile,
  RealWorldArticle,
  RealWorldArticleResponse,
  RealWorldArticlesResponse,
  RealWorldAttachment,
  RealWorldAttachmentsResponse,
} from "@/lib/materialsApi";
import { MOCK_SUBMISSIONS, MOCK_USER } from "@/mocks/mockData";
import {
  Course,
  MaterialSubmission,
  MaterialSubmissionFile,
  Subject,
} from "@/models";
import {
  getAuthorEditableSubmission,
  getAuthorLatestSubmission,
  getSubmissionFiles,
} from "@/store";

const emptyFormValues: UploadMaterialFormValues = {
  title: "",
  description: "",
  files: [],
  courses: [],
  subjects: [],
  difficulty: "none",
  type: "longread",
};

const createFormValues = (
  submission: ReturnType<typeof getAuthorEditableSubmission>,
): UploadMaterialFormValues =>
  submission
    ? {
        title: submission.material.title,
        description: submission.material.description,
        files: [...getSubmissionFiles(submission)],
        courses: [...submission.material.courses],
        subjects: [...submission.material.subjects],
        difficulty: submission.material.difficulty,
        type: submission.material.type,
      }
    : emptyFormValues;

type CurrentUserResponse = {
  user: {
    username: string;
  };
};

type AttachmentResponse = {
  attachment: RealWorldAttachment;
};

const resolveCurrentUserId = () => {
  const payload = globalThis.localStorage?.getItem("authUser");
  if (!payload) return MOCK_USER.id;

  try {
    const parsed = JSON.parse(payload) as { username?: string };
    return parsed.username?.trim() || MOCK_USER.id;
  } catch {
    return MOCK_USER.id;
  }
};

const isBrowserFile = (
  file: UploadMaterialFormValues["files"][number],
): file is File => file instanceof File;

const createApiSubmission = (
  article: RealWorldArticle,
  files: MaterialSubmissionFile[] = [],
): MaterialSubmission => ({
  id: article.slug,
  material: mapArticleToMaterial(article),
  files,
  file: files[0] || null,
  status: "approved",
  moderatorComment: "",
  createdAt: article.createdAt,
  updatedAt: article.updatedAt,
  submittedAt: article.createdAt,
  reviewedAt: article.updatedAt,
  publishedAt: article.updatedAt,
});

const toArticleTags = (values: UploadMaterialFormValues) => [
  ...values.courses.map((course) => `course:${course}`),
  ...values.subjects.map((subject) => `subject:${subject}`),
  `type:${values.type}`,
  `difficulty:${values.difficulty}`,
];

const uploadAttachment = (articleSlug: string, file: File) => {
  const formData = new FormData();
  formData.append("file", file);

  return fetchJson<AttachmentResponse>(
    resolveApiUrl(
      `/api/articles/${encodeURIComponent(articleSlug)}/attachments`,
    ),
    {
      method: "POST",
      body: formData,
    },
  );
};

const deleteArticle = async (articleSlug: string) => {
  const response = await fetchWithAuth(
    resolveApiUrl(`/api/articles/${encodeURIComponent(articleSlug)}`),
    {
      method: "DELETE",
    },
  );

  if (!response.ok && response.status !== 404) {
    throw new Error(`Failed to rollback article: ${response.status}`);
  }
};

const getSubmitErrorMessage = (error: unknown) => {
  if (error instanceof ApiRequestError) {
    if (error.status === 413) {
      return "Размер одного из файлов превышает лимит сервера. Уменьшите файл и попробуйте снова.";
    }

    if (error.status === 422) {
      const details =
        typeof error.payload === "string" ? error.payload.trim() : "";

      if (details) {
        return `Не удалось отправить материал: ${details}.`;
      }

      return "Сервер не принял данные заявки. Проверьте название и описание, затем повторите попытку.";
    }
  }

  return "Не удалось отправить материал. Попробуйте еще раз.";
};

const UploadMaterialPage = () => {
  const [currentUserId, setCurrentUserId] = useState(resolveCurrentUserId);
  const [submissions, setSubmissions] = useState<MaterialSubmission[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [isError, setIsError] = useState(false);

  const editableSubmission = useMemo(
    () => getAuthorEditableSubmission(submissions, currentUserId),
    [currentUserId, submissions],
  );
  const latestSubmission = useMemo(
    () => getAuthorLatestSubmission(submissions, currentUserId),
    [currentUserId, submissions],
  );
  const latestStatusUi = latestSubmission
    ? SUBMISSION_STATUS_UI[latestSubmission.status]
    : null;

  const [values, setValues] = useState<UploadMaterialFormValues>(() =>
    createFormValues(editableSubmission),
  );
  const [isSubmitting, setIsSubmitting] = useState(false);

  useEffect(() => {
    const abortController = new AbortController();

    const fetchSubmissions = async () => {
      setIsLoading(true);
      setIsError(false);

      try {
        const currentUserPayload = await fetchJson<CurrentUserResponse>(
          resolveApiUrl("/api/user"),
          { signal: abortController.signal },
        );
        const nextCurrentUserId = currentUserPayload.user.username;
        setCurrentUserId(nextCurrentUserId);

        const articlesPayload = await fetchJson<RealWorldArticlesResponse>(
          resolveApiUrl(
            `/api/articles?author=${encodeURIComponent(nextCurrentUserId)}&limit=100`,
          ),
          { signal: abortController.signal },
        );

        setSubmissions(
          articlesPayload.articles
            .map((article) => createApiSubmission(article))
            .sort(
              (first, second) =>
                new Date(second.updatedAt).getTime() -
                new Date(first.updatedAt).getTime(),
            ),
        );
      } catch (error) {
        if (abortController.signal.aborted) return;

        if (import.meta.env.VITE_API === "mock") {
          console.warn("[Upload] Falling back to local submissions list.");
          setCurrentUserId(MOCK_USER.id);
          setSubmissions(
            MOCK_SUBMISSIONS.filter(
              (submission) => submission.material.authorId === MOCK_USER.id,
            ),
          );
          setIsError(false);
          return;
        }

        console.error(error);
        setIsError(true);
      } finally {
        if (!abortController.signal.aborted) {
          setIsLoading(false);
        }
      }
    };

    fetchSubmissions();

    return () => abortController.abort();
  }, []);

  useEffect(() => {
    setValues(createFormValues(editableSubmission));
  }, [editableSubmission]);

  const updateValue = <K extends keyof UploadMaterialFormValues>(
    key: K,
    value: UploadMaterialFormValues[K],
  ) => {
    setValues((current) => ({
      ...current,
      [key]: value,
    }));
  };

  const toggleCourses = (item: Course) => {
    setValues((current) => ({
      ...current,
      courses: current.courses.includes(item)
        ? current.courses.filter((course) => course !== item)
        : [...current.courses, item].sort(),
    }));
  };

  const toggleSubjects = (item: Subject) => {
    setValues((current) => ({
      ...current,
      subjects: current.subjects.includes(item)
        ? current.subjects.filter((subject) => subject !== item)
        : [...current.subjects, item].sort(),
    }));
  };

  const handleFilesSelect = (selectedFiles: File[]) => {
    setValues((current) => ({
      ...current,
      files: [...current.files, ...selectedFiles],
    }));
  };

  const handleSubmit = async () => {
    const hasInvalidState =
      !values.title.trim() ||
      values.courses.length === 0 ||
      values.subjects.length === 0 ||
      values.files.length === 0;

    if (hasInvalidState) {
      alert(
        "Заполните название, курсы, предметы и прикрепите хотя бы один файл.",
      );
      return;
    }

    setIsSubmitting(true);
    let createdArticleSlug: string | null = null;

    try {
      const browserFiles = values.files.filter(isBrowserFile);
      const articlePayload = await fetchJson<RealWorldArticleResponse>(
        resolveApiUrl("/api/articles"),
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            article: {
              title: values.title.trim(),
              description: values.description.trim() || values.title.trim(),
              body: values.description.trim(),
              tagList: toArticleTags(values),
            },
          }),
        },
      );
      createdArticleSlug = articlePayload.article.slug;

      await Promise.all(
        browserFiles.map((file) =>
          uploadAttachment(articlePayload.article.slug, file),
        ),
      );

      const attachmentsPayload = await fetchJson<RealWorldAttachmentsResponse>(
        resolveApiUrl(
          `/api/articles/${encodeURIComponent(articlePayload.article.slug)}/attachments`,
        ),
      );

      const submitted = createApiSubmission(
        articlePayload.article,
        attachmentsPayload.attachments.map((attachment) =>
          mapAttachmentToMaterialFile(articlePayload.article.slug, attachment),
        ),
      );

      setSubmissions((current) => {
        const nextSubmissions = current.filter(
          (item) => item.id !== submitted.id,
        );
        return [submitted, ...nextSubmissions];
      });

      setValues(emptyFormValues);
      alert("Материал успешно отправлен на модерацию.");
    } catch (error) {
      if (createdArticleSlug) {
        try {
          await deleteArticle(createdArticleSlug);
        } catch (rollbackError) {
          console.warn(
            `[Upload] Failed to rollback article ${createdArticleSlug}.`,
            rollbackError,
          );
        }
      }

      if (import.meta.env.VITE_API === "mock") {
        console.warn("[Submissions] Mock API unavailable.");
        alert("Не удалось отправить материал в mock API.");
        return;
      }

      console.error(error);
      alert(getSubmitErrorMessage(error));
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div className="w-full px-4 py-6 xl:w-11/12 mx-auto max-w-screen-2xl space-y-6 lg:space-y-8">
      <div className="space-y-2">
        <h1 className="text-2xl lg:text-3xl font-bold tracking-tight">
          Отправка материала
        </h1>
        <p className="text-sm lg:text-base text-muted-foreground max-w-3xl">
          Заявка попадет к модераторам. Если ее вернут на доработку, форма
          сохранит заполненные данные и покажет комментарий модератора.
        </p>
      </div>

      {isLoading ? (
        <div className="rounded-xl border border-border bg-card px-6 py-10 text-center text-muted-foreground">
          Загружаем ваши заявки...
        </div>
      ) : isError ? (
        <div className="rounded-xl border border-border bg-card px-6 py-10 text-center text-destructive">
          Не удалось загрузить ваши заявки
        </div>
      ) : null}

      {latestSubmission && (
        <Card className="border-border">
          <CardHeader className="pb-3">
            <div className="flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between">
              <CardTitle className="text-lg">Последняя заявка</CardTitle>
              <Badge
                variant="secondary"
                className={latestStatusUi?.badgeClassName}
              >
                {latestStatusUi?.label}
              </Badge>
            </div>
          </CardHeader>
          <CardContent className="space-y-3">
            <p className="font-medium text-foreground">
              {latestSubmission.material.title}
            </p>
            {latestSubmission.status === "rejected" ? (
              <div className="rounded-lg border border-red-500/20 bg-destructive/5 p-4">
                <p className="text-sm font-medium text-foreground">
                  Комментарий модератора
                </p>
                <p className="mt-1 text-sm text-muted-foreground">
                  {latestSubmission.moderatorComment}
                </p>
              </div>
            ) : (
              <p className="text-sm text-muted-foreground">
                {latestSubmission.status === "pending_review"
                  ? "Заявка уже находится на проверке."
                  : "Этот материал уже опубликован в библиотеке."}
              </p>
            )}
          </CardContent>
        </Card>
      )}

      <div className="rounded-xl border border-border bg-card p-4 lg:p-8">
        <UploadMaterialForm
          values={values}
          onTitleChange={(value) => updateValue("title", value)}
          onDescriptionChange={(value) => updateValue("description", value)}
          onFilesSelect={handleFilesSelect}
          onFileRemove={(index) =>
            setValues((current) => ({
              ...current,
              files: current.files.filter(
                (_, fileIndex) => fileIndex !== index,
              ),
            }))
          }
          onToggleCourse={toggleCourses}
          onToggleSubject={toggleSubjects}
          onSelectType={(value) => updateValue("type", value)}
          onSelectDifficulty={(value) => updateValue("difficulty", value)}
          onSubmit={handleSubmit}
          submitDisabled={isSubmitting}
        />
      </div>
    </div>
  );
};

export default UploadMaterialPage;
