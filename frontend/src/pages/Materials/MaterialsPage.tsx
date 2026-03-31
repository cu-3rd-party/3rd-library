import { useMemo } from "react";

import { MaterialsSection } from "@/common/organisms";
import { getPublishedMaterials, useMaterialSubmissionStore } from "@/store";

const MaterialsPage = () => {
  const submissions = useMaterialSubmissionStore((state) => state.submissions);
  const materials = useMemo(
    () => getPublishedMaterials(submissions),
    [submissions],
  );

  return (
    <div className="w-full px-4 py-0 md:py-6 xl:w-11/12 mx-auto max-w-screen-2xl">
      <MaterialsSection materials={materials} />
    </div>
  );
};

export default MaterialsPage;
