import { useEffect, useMemo, useRef, useState } from "react";

import { getAuthorEditableSubmission } from "@/entities/submission/lib";
import { MaterialSubmission } from "@/entities/submission/model";
import { Course, Subject } from "@/entities/material/model";

import { performSubmit } from "../api";
import { UploadMaterialFormValues } from "../ui";
import {
  createFormValues,
  emptyFormValues,
  getSubmitErrorMessage,
  getUpdateErrorMessage,
  resolveCurrentUserId,
} from "./helpers";
import { fetchUserSubmissions } from "./fetchSubmissions";

export const useSubmitMaterial = () => {
  const [currentUserId, setCurrentUserId] = useState(resolveCurrentUserId);
  const [submissions, setSubmissions] = useState<MaterialSubmission[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [isError, setIsError] = useState(false);
  const [editingSubmissionId, setEditingSubmissionId] = useState<string | null>(null);
  const [values, setValues] = useState<UploadMaterialFormValues>(emptyFormValues);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const formSectionRef = useRef<HTMLDivElement>(null);

  const editableSubmission = useMemo(
    () => getAuthorEditableSubmission(submissions, currentUserId),
    [currentUserId, submissions],
  );

  const authorSubmissions = useMemo(
    () =>
      submissions
        .filter((s) => s.material.authorId === currentUserId)
        .sort((a, b) => new Date(b.updatedAt).getTime() - new Date(a.updatedAt).getTime()),
    [currentUserId, submissions],
  );

  const editingSubmission = useMemo(
    () =>
      editingSubmissionId
        ? (authorSubmissions.find((s) => s.id === editingSubmissionId) ?? null)
        : null,
    [authorSubmissions, editingSubmissionId],
  );

  useEffect(() => {
    const ctrl = new AbortController();

    fetchUserSubmissions(ctrl.signal)
      .then(({ userId, submissions: fetched }) => {
        if (ctrl.signal.aborted) return;
        setCurrentUserId(userId);
        setSubmissions(fetched);
      })
      .catch((error: unknown) => {
        if (ctrl.signal.aborted) return;
        console.error(error);
        setIsError(true);
      })
      .finally(() => {
        if (!ctrl.signal.aborted) setIsLoading(false);
      });

    return () => ctrl.abort();
  }, []);

  useEffect(() => {
    setValues(createFormValues(editableSubmission));
  }, [editableSubmission]);

  useEffect(() => {
    if (editingSubmissionId && !editingSubmission) setEditingSubmissionId(null);
  }, [editingSubmission, editingSubmissionId]);

  const updateValue = <K extends keyof UploadMaterialFormValues>(
    key: K,
    value: UploadMaterialFormValues[K],
  ) => setValues((current) => ({ ...current, [key]: value }));

  const toggleCourses = (item: Course) =>
    setValues((current) => ({
      ...current,
      courses: current.courses.includes(item)
        ? current.courses.filter((c) => c !== item)
        : [...current.courses, item].sort(),
    }));

  const toggleSubjects = (item: Subject) =>
    setValues((current) => ({
      ...current,
      subjects: current.subjects.includes(item)
        ? current.subjects.filter((s) => s !== item)
        : [...current.subjects, item].sort(),
    }));

  const handleFilesSelect = (selectedFiles: File[]) =>
    setValues((current) => ({ ...current, files: [...current.files, ...selectedFiles] }));

  const handleRemoveFile = (index: number) =>
    setValues((current) => ({
      ...current,
      files: current.files.filter((_, i) => i !== index),
    }));

  const handleStartEdit = (submission: MaterialSubmission) => {
    if (submission.status !== "rejected") return;
    setEditingSubmissionId(submission.id);
    setValues(createFormValues(submission));
    globalThis.requestAnimationFrame(() => {
      formSectionRef.current?.scrollIntoView({ behavior: "smooth", block: "start" });
    });
  };

  const handleCancelEdit = () => {
    setEditingSubmissionId(null);
    setValues(createFormValues(editableSubmission));
  };

  const handleSubmit = async () => {
    if (
      !values.title.trim() ||
      !values.courses.length ||
      !values.subjects.length ||
      !values.files.length
    ) {
      alert("Заполните название, курсы, предметы и прикрепите хотя бы один файл.");
      return;
    }

    setIsSubmitting(true);
    try {
      const { kind, submission } = await performSubmit(editingSubmissionId, values);
      if (kind === "updated") {
        setSubmissions((current) => current.map((s) => (s.id === submission.id ? submission : s)));
        setEditingSubmissionId(null);
        alert("Заявка обновлена и повторно отправлена на модерацию.");
      } else {
        setSubmissions((current) => [submission, ...current.filter((s) => s.id !== submission.id)]);
        alert("Материал успешно отправлен на модерацию.");
      }
      setValues(emptyFormValues);
    } catch (error) {
      if (import.meta.env.VITE_API === "mock") {
        console.warn("[Submissions] Mock API unavailable.");
        alert("Не удалось отправить материал в mock API.");
        return;
      }
      console.error(error);
      alert(editingSubmissionId ? getUpdateErrorMessage(error) : getSubmitErrorMessage(error));
    } finally {
      setIsSubmitting(false);
    }
  };

  return {
    isLoading,
    isError,
    authorSubmissions,
    editingSubmission,
    isEditingSubmission: Boolean(editingSubmission),
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
  };
};
