import {
  SubmissionHistoryList,
  UploadMaterialForm,
  useSubmitMaterial,
} from "@/features/submit-material";
import { Button } from "@/shared/ui";

const UploadMaterialPage = () => {
  const {
    isLoading,
    isError,
    authorSubmissions,
    editingSubmission,
    isEditingSubmission,
    formSectionRef,
    values,
    isSubmitting,
    updateValue,
    toggleCourses,
    toggleSubjects,
    handleFilesSelect,
    handleRemoveFile,
    handleStartEdit,
    handleCancelEdit,
    handleSubmit,
  } = useSubmitMaterial();

  return (
    <div className="w-full px-4 py-6 xl:w-11/12 mx-auto max-w-screen-2xl space-y-6 lg:space-y-8">
      <div className="space-y-2">
        <h1 className="text-2xl lg:text-3xl font-bold tracking-tight">
          Загрузка материалов
        </h1>
        <p className="text-sm lg:text-base text-muted-foreground max-w-3xl">
          Заявка попадет к модераторам. Если ее вернут на доработку, форма
          сохранит заполненные данные и покажет комментарий модератора.
        </p>
      </div>

      {isLoading && (
        <div className="rounded-xl border border-border bg-card px-6 py-10 text-center text-muted-foreground">
          Загружаем ваши заявки...
        </div>
      )}
      {isError && (
        <div className="rounded-xl border border-border bg-card px-6 py-10 text-center text-destructive">
          Не удалось загрузить ваши заявки
        </div>
      )}

      {authorSubmissions.length > 0 && (
        <SubmissionHistoryList
          submissions={authorSubmissions}
          editingSubmissionId={editingSubmission?.id ?? null}
          isSubmitting={isSubmitting}
          onEdit={handleStartEdit}
        />
      )}

      <div
        ref={formSectionRef}
        className="rounded-xl border border-border bg-card p-4 lg:p-8"
      >
        {isEditingSubmission && editingSubmission && (
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
              onClick={handleCancelEdit}
              disabled={isSubmitting}
            >
              Отменить
            </Button>
          </div>
        )}

        <UploadMaterialForm
          values={values}
          onTitleChange={(value) => updateValue("title", value)}
          onDescriptionChange={(value) => updateValue("description", value)}
          onFilesSelect={handleFilesSelect}
          onFileRemove={handleRemoveFile}
          onToggleCourse={toggleCourses}
          onToggleSubject={toggleSubjects}
          onSelectType={(value) => updateValue("type", value)}
          onSelectDifficulty={(value) => updateValue("difficulty", value)}
          onSubmit={handleSubmit}
          submitLabel={isEditingSubmission ? "Обновить заявку" : "Отправить материал"}
          submitDisabled={isSubmitting}
        />
      </div>
    </div>
  );
};

export default UploadMaterialPage;
