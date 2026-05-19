import { useNavigate, useParams } from "react-router-dom";

import {
  MaterialDetailsFiles,
  MaterialDetailsHeader,
  MaterialDetailsMeta,
  useMaterialDetails,
} from "@/features/material-details";
import { MATERIALS_PREFIX } from "@/shared/constants";
import { Button } from "@/shared/ui";

const MaterialDetailsPage = () => {
  const { id } = useParams();
  const navigate = useNavigate();
  const { materialDetails, isLoading, isError } = useMaterialDetails(id || "");

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
      <MaterialDetailsHeader materialDetails={materialDetails} />
      <MaterialDetailsMeta materialDetails={materialDetails} />
      <MaterialDetailsFiles
        files={materialDetails.files}
        submittedAt={materialDetails.submittedAt}
        pubDate={materialDetails.pubDate}
      />
    </div>
  );
};

export default MaterialDetailsPage;
