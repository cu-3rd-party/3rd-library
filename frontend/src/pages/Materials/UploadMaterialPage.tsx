import { useEffect, useMemo, useState } from "react";

import {
  UploadMaterialForm,
  type UploadMaterialFormValues,
} from "@/common/organisms/Materials/UploadMaterialForm";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { SUBMISSION_STATUS_UI } from "@/constants";
import { MOCK_USER } from "@/mocks";
import { Course, Subject } from "@/models";
import {
  getAuthorEditableSubmission,
  getAuthorLatestSubmission,
  getSubmissionFiles,
  useMaterialSubmissionStore,
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

const UploadMaterialPage = () => {
  const submissions = useMaterialSubmissionStore((state) => state.submissions);
  const submitSubmission = useMaterialSubmissionStore(
    (state) => state.submitSubmission,
  );

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

  const handleSubmit = () => {
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

    submitSubmission({
      id: editableSubmission?.id,
      authorId: MOCK_USER.id,
      authorName: MOCK_USER.name,
      title: values.title,
      description: values.description,
      courses: values.courses,
      subjects: values.subjects,
      difficulty: values.difficulty,
      type: values.type,
      files: values.files,
    });

    setValues(emptyFormValues);
    alert("Материал отправлен на модерацию.");
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
        />
      </div>
    </div>
  );
};

export default UploadMaterialPage;
