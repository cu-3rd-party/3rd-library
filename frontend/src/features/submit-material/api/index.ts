import {
  RealWorldArticleResponse,
  RealWorldAttachment,
  RealWorldAttachmentsResponse,
} from "@/entities/material/api";
import { mapAttachmentToMaterialFile } from "@/entities/material/lib";
import { fetchJsonWithAuth, fetchWithAuth } from "@/entities/session/api";
import { LibrarySubmission } from "@/entities/submission/api";
import { mapLibrarySubmissionToMaterialSubmission } from "@/entities/submission/lib";
import { MaterialSubmission, MaterialSubmissionFile } from "@/entities/submission/model";
import { ApiRequestError, resolveApiUrl } from "@/shared/api";

import { UploadMaterialFormValues } from "../ui";
import {
  createApiSubmission,
  isBrowserFile,
  isStoredSubmissionFile,
  toArticleTags,
} from "../lib/helpers";

type AttachmentResponse = { attachment: RealWorldAttachment };

export const uploadAttachment = (articleSlug: string, file: File) => {
  const formData = new FormData();
  formData.append("file", file);
  return fetchJsonWithAuth<AttachmentResponse>(
    resolveApiUrl(`/api/articles/${encodeURIComponent(articleSlug)}/attachments`),
    { method: "POST", body: formData },
  );
};

export const deleteArticle = async (articleSlug: string) => {
  const response = await fetchWithAuth(
    resolveApiUrl(`/api/articles/${encodeURIComponent(articleSlug)}`),
    { method: "DELETE" },
  );
  if (!response.ok && response.status !== 404)
    throw new Error(`Failed to rollback article: ${response.status}`);
};

export const createMaterialSubmission = async (
  values: UploadMaterialFormValues,
): Promise<LibrarySubmission> => {
  const browserFiles = values.files.filter(isBrowserFile);
  const binaryFiles = await Promise.all(
    browserFiles.map(async (file) => Array.from(new Uint8Array(await file.arrayBuffer()))),
  );
  return fetchJsonWithAuth<LibrarySubmission>(resolveApiUrl("/api/materials/submissions"), {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      title: values.title.trim(),
      description: values.description.trim() || values.title.trim(),
      courses: values.courses,
      subjects: values.subjects,
      type: values.type,
      difficulty: values.difficulty,
      files: binaryFiles,
    }),
  });
};

export const updateMaterialSubmission = (
  id: string,
  values: UploadMaterialFormValues,
): Promise<LibrarySubmission> => {
  const formData = new FormData();
  formData.append("title", values.title.trim());
  formData.append("description", values.description.trim());
  formData.append("type", values.type);
  formData.append("difficulty", values.difficulty);
  values.courses.forEach((c) => formData.append("courses", c));
  values.subjects.forEach((s) => formData.append("subjects", s));
  values.files.forEach((file) => {
    if (isBrowserFile(file)) formData.append("files", file);
    else if (isStoredSubmissionFile(file) && file.id) formData.append("keepFileIds", file.id);
  });
  return fetchJsonWithAuth<LibrarySubmission>(
    resolveApiUrl(`/api/materials/submissions/${encodeURIComponent(id)}`),
    { method: "PATCH", body: formData },
  );
};

export const createViaArticleFlow = async (
  values: UploadMaterialFormValues,
  onArticleCreated: (slug: string) => void,
): Promise<MaterialSubmission> => {
  const browserFiles = values.files.filter(isBrowserFile);

  const { article } = await fetchJsonWithAuth<RealWorldArticleResponse>(
    resolveApiUrl("/api/articles"),
    {
      method: "POST",
      headers: { "Content-Type": "application/json" },
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
  onArticleCreated(article.slug);

  await Promise.all(browserFiles.map((file) => uploadAttachment(article.slug, file)));

  const { attachments } = await fetchJsonWithAuth<RealWorldAttachmentsResponse>(
    resolveApiUrl(`/api/articles/${encodeURIComponent(article.slug)}/attachments`),
  );

  const files: MaterialSubmissionFile[] = attachments.map((a) =>
    mapAttachmentToMaterialFile(article.slug, a),
  );
  return createApiSubmission(article, files);
};

export type PerformSubmitResult = { kind: "updated" | "created"; submission: MaterialSubmission };

export const performSubmit = async (
  editingSubmissionId: string | null,
  values: UploadMaterialFormValues,
): Promise<PerformSubmitResult> => {
  if (editingSubmissionId) {
    const raw = await updateMaterialSubmission(editingSubmissionId, values);
    return { kind: "updated", submission: mapLibrarySubmissionToMaterialSubmission(raw) };
  }

  try {
    const raw = await createMaterialSubmission(values);
    return { kind: "created", submission: mapLibrarySubmissionToMaterialSubmission(raw) };
  } catch (error) {
    if (!(error instanceof ApiRequestError) || error.status !== 404) throw error;
  }

  let createdArticleSlug: string | null = null;
  try {
    const submission = await createViaArticleFlow(values, (slug) => {
      createdArticleSlug = slug;
    });
    return { kind: "created", submission };
  } catch (error) {
    if (createdArticleSlug) {
      await deleteArticle(createdArticleSlug).catch((e: unknown) => {
        console.warn(`[Upload] Failed to rollback article ${createdArticleSlug}.`, e);
      });
    }
    throw error;
  }
};
