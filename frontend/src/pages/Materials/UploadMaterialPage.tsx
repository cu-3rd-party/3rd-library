import { useEffect, useMemo, useState } from "react";

import {
  UploadMaterialForm,
  type UploadMaterialFormValues,
} from "@/common/organisms/Materials/UploadMaterialForm";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { SUBMISSION_STATUS_UI } from "@/constants";
import { fetchJson, resolveApiUrl } from "@/lib/api";
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

type SubmissionsListResponse = {
  items: MaterialSubmission[];
  page: number;
  limit: number;
  total: number;
};

const resolveSubmissionsEndpoint = (submissionId?: string) => {
  const path = submissionId
    ? `/materials/submissions/${submissionId}`
    : "/materials/submissions";

  return resolveApiUrl(path);
};

const isBrowserFile = (
  file: UploadMaterialFormValues["files"][number],
): file is File => file instanceof File;

const toExistingSubmissionFiles = (
  files: UploadMaterialFormValues["files"],
): MaterialSubmissionFile[] =>
  files.filter((file): file is MaterialSubmissionFile => !isBrowserFile(file));

const buildSubmissionFormData = (params: {
  authorId: string;
  authorName: string;
  values: UploadMaterialFormValues;
}) => {
  const { authorId, authorName, values } = params;
  const formData = new FormData();

  formData.append("authorId", authorId);
  formData.append("authorName", authorName);
  formData.append("title", values.title.trim());
  formData.append("description", values.description.trim());
  values.courses.forEach((course) => formData.append("courses", course));
  values.subjects.forEach((subject) => formData.append("subjects", subject));
  formData.append("type", values.type);
  formData.append("difficulty", values.difficulty);
  values.files
    .filter(isBrowserFile)
    .forEach((file) => formData.append("files", file));

  const existingFiles = toExistingSubmissionFiles(values.files);
  if (existingFiles.length > 0) {
    formData.append("existingFiles", JSON.stringify(existingFiles));
  }

  return formData;
};

const submitSubmissionToApi = async (params: {
  submissionId?: string;
  authorId: string;
  authorName: string;
  values: UploadMaterialFormValues;
}) => {
  const { submissionId, authorId, authorName, values } = params;
  return fetchJson<MaterialSubmission>(
    resolveSubmissionsEndpoint(submissionId),
    {
      method: submissionId ? "PATCH" : "POST",
      body: buildSubmissionFormData({ authorId, authorName, values }),
    },
  );
};

const UploadMaterialPage = () => {
  const [submissions, setSubmissions] = useState<MaterialSubmission[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [isError, setIsError] = useState(false);

  const editableSubmission = useMemo(
    () => getAuthorEditableSubmission(submissions, MOCK_USER.id),
    [submissions],
  );
  const latestSubmission = useMemo(
    () => getAuthorLatestSubmission(submissions, MOCK_USER.id),
    [submissions],
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
        const payload = await fetchJson<SubmissionsListResponse>(
          resolveApiUrl(`/materials/submissions?authorId=${MOCK_USER.id}`),
          { signal: abortController.signal },
        );
        setSubmissions(payload.items);
      } catch (error) {
        if (abortController.signal.aborted) return;

        if (import.meta.env.VITE_API === "mock") {
          console.warn("[Upload] Falling back to local submissions list.");
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

    const baseInput = {
      authorId: MOCK_USER.id,
      authorName: MOCK_USER.name,
      title: values.title,
      description: values.description,
      courses: values.courses,
      subjects: values.subjects,
      difficulty: values.difficulty,
      type: values.type,
      files: values.files,
    };

    setIsSubmitting(true);

    try {
      const submitted = await submitSubmissionToApi({
        submissionId: editableSubmission?.id,
        authorId: baseInput.authorId,
        authorName: baseInput.authorName,
        values,
      });

      setSubmissions((current) => {
        const index = current.findIndex((item) => item.id === submitted.id);
        if (index === -1) return [submitted, ...current];

        return current.map((item, itemIndex) =>
          itemIndex === index ? submitted : item,
        );
      });

      setValues(emptyFormValues);
      alert("Материал отправлен на модерацию.");
    } catch (error) {
      if (import.meta.env.VITE_API === "mock") {
        console.warn("[Submissions] Mock API unavailable.");
        alert("Не удалось отправить материал в mock API.");
        return;
      }

      console.error(error);
      alert("Не удалось отправить материал. Попробуйте еще раз.");
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
