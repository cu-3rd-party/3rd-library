import { useEffect, useMemo, useRef, useState } from "react";

import {
  UploadMaterialForm,
  type UploadMaterialFormValues,
} from "@/common/organisms/Materials/UploadMaterialForm";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
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
  mapLibrarySubmissionToMaterialSubmission,
  LibraryCurrentUserResponse,
  LibraryPaginatedSubmissionsResponse,
  LibrarySubmission,
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

const isStoredSubmissionFile = (
  file: UploadMaterialFormValues["files"][number],
): file is MaterialSubmissionFile => !isBrowserFile(file);

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

const getUpdateSubmitErrorMessage = (error: unknown) => {
  if (error instanceof ApiRequestError) {
    if (error.status === 403) {
      return "Эту заявку нельзя обновить в текущем статусе.";
    }

    if (error.status === 400 || error.status === 422) {
      const details =
        typeof error.payload === "string" ? error.payload.trim() : "";

      if (details) {
        return `Не удалось обновить заявку: ${details}.`;
      }

      return "Сервер не принял изменения. Проверьте данные и повторите попытку.";
    }
  }

  return "Не удалось обновить заявку. Попробуйте еще раз.";
};

const createSubmissionUpdateFormData = (values: UploadMaterialFormValues) => {
  const formData = new FormData();

  formData.append("title", values.title.trim());
  formData.append("description", values.description.trim());
  formData.append("type", values.type);
  formData.append("difficulty", values.difficulty);

  values.courses.forEach((course) => {
    formData.append("courses", course);
  });
  values.subjects.forEach((subject) => {
    formData.append("subjects", subject);
  });

  values.files.forEach((file) => {
    if (isBrowserFile(file)) {
      formData.append("files", file);
      return;
    }

    if (isStoredSubmissionFile(file) && file.id) {
      formData.append("keepFileIds", file.id);
    }
  });

  return formData;
};

const formatSubmissionDate = (value: string) => {
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) return "Дата не указана";

  return new Intl.DateTimeFormat("ru-RU", {
    day: "2-digit",
    month: "2-digit",
    year: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  }).format(date);
};

const UploadMaterialPage = () => {
  const [currentUserId, setCurrentUserId] = useState(resolveCurrentUserId);
  const [submissions, setSubmissions] = useState<MaterialSubmission[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [isError, setIsError] = useState(false);
  const formSectionRef = useRef<HTMLDivElement>(null);

  const editableSubmission = useMemo(
    () => getAuthorEditableSubmission(submissions, currentUserId),
    [currentUserId, submissions],
  );
  const authorSubmissions = useMemo(
    () =>
      submissions
        .filter((submission) => submission.material.authorId === currentUserId)
        .sort(
          (first, second) =>
            new Date(second.updatedAt).getTime() -
            new Date(first.updatedAt).getTime(),
        ),
    [currentUserId, submissions],
  );
  const [editingSubmissionId, setEditingSubmissionId] = useState<string | null>(
    null,
  );
  const editingSubmission = useMemo(
    () =>
      editingSubmissionId
        ? authorSubmissions.find(
            (submission) => submission.id === editingSubmissionId,
          ) || null
        : null,
    [authorSubmissions, editingSubmissionId],
  );
  const isEditingSubmission = Boolean(editingSubmission);

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
        try {
          const currentUserPayload = await fetchJson<LibraryCurrentUserResponse>(
            resolveApiUrl("/api/users/me"),
            { signal: abortController.signal },
          );
          const nextCurrentUserId = currentUserPayload.id;
          setCurrentUserId(nextCurrentUserId);

          try {
            const submissionsPayload =
              await fetchJson<LibraryPaginatedSubmissionsResponse>(
                resolveApiUrl("/api/materials/submissions?limit=100"),
                { signal: abortController.signal },
              );

            const mergedSubmissions = submissionsPayload.items
              .map(mapLibrarySubmissionToMaterialSubmission)
              .sort(
                (first, second) =>
                  new Date(second.updatedAt).getTime() -
                  new Date(first.updatedAt).getTime(),
              );

            setSubmissions(mergedSubmissions);
            return;
          } catch (error) {
            if (error instanceof ApiRequestError && error.status === 500) {
              console.warn(
                "[Upload] Backend submissions endpoint returned 500. Falling back to empty list.",
                error,
              );
              setSubmissions([]);
              setIsError(false);
              return;
            }
            throw error;
          }
        } catch (error) {
          if (!(error instanceof ApiRequestError) || error.status !== 404) {
            throw error;
          }
        }

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

  useEffect(() => {
    if (!editingSubmissionId) return;
    if (editingSubmission) return;
    setEditingSubmissionId(null);
  }, [editingSubmission, editingSubmissionId]);

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

  const handleStartSubmissionEdit = (submission: MaterialSubmission) => {
    if (submission.status !== "rejected") return;

    setEditingSubmissionId(submission.id);
    setValues(createFormValues(submission));

    globalThis.requestAnimationFrame(() => {
      formSectionRef.current?.scrollIntoView({
        behavior: "smooth",
        block: "start",
      });
    });
  };

  const handleCancelSubmissionEdit = () => {
    setEditingSubmissionId(null);
    setValues(createFormValues(editableSubmission));
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
      if (editingSubmissionId) {
        const payload = createSubmissionUpdateFormData(values);
        const updatedPayload = await fetchJson<LibrarySubmission>(
          resolveApiUrl(
            `/api/materials/submissions/${encodeURIComponent(editingSubmissionId)}`,
          ),
          {
            method: "PATCH",
            body: payload,
          },
        );

        const updatedSubmission =
          mapLibrarySubmissionToMaterialSubmission(updatedPayload);
        setSubmissions((current) =>
          current.map((submission) =>
            submission.id === updatedSubmission.id
              ? updatedSubmission
              : submission,
          ),
        );

        setEditingSubmissionId(null);
        setValues(emptyFormValues);
        alert("Заявка обновлена и повторно отправлена на модерацию.");
        return;
      }

      const browserFiles = values.files.filter(isBrowserFile);
      try {
        const binaryFiles = await Promise.all(
          browserFiles.map(async (file) =>
            Array.from(new Uint8Array(await file.arrayBuffer())),
          ),
        );

        const submissionPayload = await fetchJson<LibrarySubmission>(
          resolveApiUrl("/api/materials/submissions"),
          {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify({
              title: values.title.trim(),
              description: values.description.trim() || values.title.trim(),
              courses: values.courses,
              subjects: values.subjects,
              type: values.type,
              difficulty: values.difficulty,
              files: binaryFiles,
            }),
          },
        );

        const submitted = mapLibrarySubmissionToMaterialSubmission(
          submissionPayload,
        );
        setSubmissions((current) => {
          const nextSubmissions = current.filter(
            (item) => item.id !== submitted.id,
          );
          return [submitted, ...nextSubmissions];
        });

        setValues(emptyFormValues);
        alert("Материал успешно отправлен на модерацию.");
        return;
      } catch (error) {
        if (!(error instanceof ApiRequestError) || error.status !== 404) {
          throw error;
        }
      }

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
      if (editingSubmissionId) {
        alert(getUpdateSubmitErrorMessage(error));
        return;
      }

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

      {authorSubmissions.length > 0 && (
        <Card className="border-border">
          <Accordion type="single" collapsible>
            <AccordionItem value="history" className="border-b-0">
              <CardHeader className="pb-2">
                <AccordionTrigger className="py-0 hover:no-underline">
                  <div className="space-y-1 text-left">
                    <CardTitle className="text-lg">
                      История заявок ({authorSubmissions.length})
                    </CardTitle>
                    <p className="text-sm text-muted-foreground">
                      Здесь отображаются все отправленные материалы и их
                      текущий статус.
                    </p>
                  </div>
                </AccordionTrigger>
              </CardHeader>

              <AccordionContent>
                <CardContent className="space-y-3 pt-0">
                  {authorSubmissions.map((submission) => {
                    const statusUi = SUBMISSION_STATUS_UI[submission.status];
                    const updatedAt = formatSubmissionDate(
                      submission.updatedAt || submission.createdAt,
                    );
                    const isEditingCurrent =
                      editingSubmissionId === submission.id;

                    return (
                      <div
                        key={`${submission.id}-${submission.updatedAt}`}
                        className="rounded-lg border border-border bg-card/70 p-4 space-y-2"
                      >
                        <div className="flex flex-col gap-2 sm:flex-row sm:items-start sm:justify-between">
                          <div className="min-w-0 space-y-1">
                            <p className="break-words font-medium text-foreground">
                              {submission.material.title}
                            </p>
                            <p className="text-xs text-muted-foreground">
                              Обновлено: {updatedAt}
                            </p>
                          </div>
                          <Badge
                            variant="secondary"
                            className={statusUi.badgeClassName}
                          >
                            {statusUi.label}
                          </Badge>
                        </div>

                        {submission.status === "rejected" ? (
                          <div className="rounded-lg border border-red-500/20 bg-destructive/5 p-3 space-y-3">
                            <div>
                              <p className="text-sm font-medium text-foreground">
                                Комментарий модератора
                              </p>
                              <p className="mt-1 text-sm text-muted-foreground break-words">
                                {submission.moderatorComment ||
                                  "Комментарий отсутствует"}
                              </p>
                            </div>
                            <Button
                              type="button"
                              variant={isEditingCurrent ? "secondary" : "outline"}
                              size="sm"
                              onClick={() => handleStartSubmissionEdit(submission)}
                              disabled={isSubmitting}
                            >
                              {isEditingCurrent
                                ? "Редактирование открыто"
                                : "Обновить заявку"}
                            </Button>
                          </div>
                        ) : (
                          <p className="text-sm text-muted-foreground">
                            {submission.status === "pending_review"
                              ? "Заявка находится на модерации."
                              : submission.status === "approved"
                                ? "Материал опубликован."
                                : "Черновик доступен для доработки."}
                          </p>
                        )}
                      </div>
                    );
                  })}
                </CardContent>
              </AccordionContent>
            </AccordionItem>
          </Accordion>
        </Card>
      )}

      <div
        ref={formSectionRef}
        className="rounded-xl border border-border bg-card p-4 lg:p-8"
      >
        {isEditingSubmission && editingSubmission ? (
          <div className="mb-4 rounded-lg border border-orange-500/30 bg-orange-500/10 p-3 flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
            <div className="min-w-0">
              <p className="text-sm font-semibold text-foreground">
                Редактирование отклоненной заявки
              </p>
              <p className="text-sm text-muted-foreground break-words">
                {editingSubmission.material.title}
              </p>
            </div>
            <Button
              type="button"
              variant="ghost"
              size="sm"
              onClick={handleCancelSubmissionEdit}
              disabled={isSubmitting}
            >
              Отменить
            </Button>
          </div>
        ) : null}

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
          submitLabel={
            isEditingSubmission ? "Обновить заявку" : "Отправить материал"
          }
          submitDisabled={isSubmitting}
        />
      </div>
    </div>
  );
};

export default UploadMaterialPage;
