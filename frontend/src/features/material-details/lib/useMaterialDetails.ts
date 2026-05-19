import { useEffect, useState } from "react";

import { MOCK_SUBMISSIONS } from "@/app/mocks/data/submission";
import {
  LibraryMaterialDetailsResponse,
  RealWorldArticleResponse,
  RealWorldAttachmentsResponse,
} from "@/entities/material/api";
import {
  mapArticleToMaterialDetails,
  mapAttachmentToMaterialFile,
  mapLibraryMaterialToMaterial,
} from "@/entities/material/lib";
import { MaterialDetailsResponse } from "../model";
import { fetchJsonWithAuth } from "@/entities/session/api";
import { mapLibraryMaterialFileToSubmissionFile } from "@/entities/submission/lib";
import { RealWorldProfileResponse } from "@/entities/user/api";
import { ApiRequestError, resolveApiUrl } from "@/shared/api";

const getFallbackMaterial = (materialId: string): MaterialDetailsResponse | null => {
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

export const useMaterialDetails = (materialId: string) => {
  const [materialDetails, setMaterialDetails] = useState<MaterialDetailsResponse | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [isError, setIsError] = useState(false);

  useEffect(() => {
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
          const materialPayload = await fetchJsonWithAuth<LibraryMaterialDetailsResponse>(
            resolveApiUrl(`/api/materials/${encodeURIComponent(materialId)}`),
            { signal: abortController.signal },
          );

          const mappedMaterial = mapLibraryMaterialToMaterial(materialPayload);
          setMaterialDetails({
            ...mappedMaterial,
            authorImage: materialPayload.author_image || null,
            files: materialPayload.files.map(mapLibraryMaterialFileToSubmissionFile),
            submittedAt: materialPayload.submitted_at || undefined,
            publishedAt: materialPayload.published_at || undefined,
          });
          return;
        } catch (error) {
          if (!(error instanceof ApiRequestError) || error.status !== 404) throw error;
        }

        const articlePath = `/api/articles/${encodeURIComponent(materialId)}`;
        const [articlePayload, attachmentsPayload] = await Promise.all([
          fetchJsonWithAuth<RealWorldArticleResponse>(
            resolveApiUrl(articlePath),
            { signal: abortController.signal },
          ),
          fetchJsonWithAuth<RealWorldAttachmentsResponse>(
            resolveApiUrl(`${articlePath}/attachments`),
            { signal: abortController.signal },
          ).catch((error: unknown) => {
            if (!abortController.signal.aborted)
              console.warn("[MaterialDetails] Failed to load attachments.", error);
            return { attachments: [] };
          }),
        ]);

        const mappedMaterial = mapArticleToMaterialDetails(articlePayload.article);
        const encodedAuthorId = encodeURIComponent(mappedMaterial.authorId);

        const authorProfilePayload = await fetchJsonWithAuth<RealWorldProfileResponse>(
          resolveApiUrl(`/api/profiles/${encodedAuthorId}`),
          { signal: abortController.signal },
        ).catch((error: unknown) => {
          if (!abortController.signal.aborted)
            console.warn("[MaterialDetails] Failed to load author profile.", error);
          return null;
        });

        setMaterialDetails({
          ...mappedMaterial,
          authorImage:
            authorProfilePayload?.profile.image ?? articlePayload.article.author.image,
          files: attachmentsPayload.attachments.map((attachment) =>
            mapAttachmentToMaterialFile(materialId, attachment),
          ),
        });
      } catch (error) {
        if (abortController.signal.aborted) return;

        if (import.meta.env.VITE_API === "mock") {
          const fallbackMaterial = getFallbackMaterial(materialId);
          if (fallbackMaterial) {
            console.warn("[MaterialDetails] Falling back to local mock details.");
            setMaterialDetails(fallbackMaterial);
            setIsError(false);
            return;
          }
        }

        console.error(error);
        setIsError(true);
      } finally {
        if (!abortController.signal.aborted) setIsLoading(false);
      }
    };

    fetchMaterial();
    return () => abortController.abort();
  }, [materialId]);

  return { materialDetails, isLoading, isError };
};
