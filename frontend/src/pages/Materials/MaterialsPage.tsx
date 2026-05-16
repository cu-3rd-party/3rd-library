import { useEffect, useState } from "react";

import { MaterialsSection } from "@/common/organisms";
import { ApiRequestError, fetchJson, resolveApiUrl } from "@/lib/api";
import {
  LibraryPaginatedMaterialsResponse,
  mapArticleToMaterial,
  mapLibraryMaterialToMaterial,
  RealWorldArticlesResponse,
} from "@/lib/materialsApi";
import { MOCK_SUBMISSIONS } from "@/mocks/mockData";
import { Material } from "@/models/material";

const getFallbackPublishedMaterials = () =>
  MOCK_SUBMISSIONS.filter((submission) => submission.status === "approved").map(
    (submission) => submission.material,
  );

const MaterialsPage = () => {
  const [materials, setMaterials] = useState<Material[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [isError, setIsError] = useState(false);

  useEffect(() => {
    const abortController = new AbortController();

    const fetchMaterials = async () => {
      setIsLoading(true);
      setIsError(false);

      try {
        try {
          const payload = await fetchJson<LibraryPaginatedMaterialsResponse>(
            resolveApiUrl("/api/materials?limit=100"),
            {
              signal: abortController.signal,
            },
          );
          setMaterials(payload.items.map(mapLibraryMaterialToMaterial));
          return;
        } catch (error) {
          if (!(error instanceof ApiRequestError) || error.status !== 404) {
            throw error;
          }
        }

        const payload = await fetchJson<RealWorldArticlesResponse>(
          resolveApiUrl("/api/articles"),
          {
            signal: abortController.signal,
          },
        );
        setMaterials(payload.articles.map(mapArticleToMaterial));
      } catch (error) {
        if (abortController.signal.aborted) return;

        if (import.meta.env.VITE_API === "mock") {
          console.warn("[Materials] Falling back to local mock list.");
          setMaterials(getFallbackPublishedMaterials());
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

    fetchMaterials();

    return () => abortController.abort();
  }, []);

  return (
    <div className="w-full px-4 py-4 md:py-6 xl:w-11/12 mx-auto max-w-screen-2xl">
      {isLoading ? (
        <div className="col-span-full text-center py-20 text-muted-foreground bg-card rounded-xl border border-dashed border-border/50">
          Загружаем материалы...
        </div>
      ) : isError ? (
        <div className="col-span-full text-center py-20 text-destructive bg-card rounded-xl border border-dashed border-border/50">
          Не удалось загрузить материалы
        </div>
      ) : (
        <MaterialsSection materials={materials} />
      )}
    </div>
  );
};

export default MaterialsPage;
