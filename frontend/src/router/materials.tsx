import { lazy } from "react";

import { MATERIALS_PREFIX } from "@/constants";
import type { AppRoute } from "@/models";
import MaterialsPage from "@/pages/Materials/MaterialsPage";

const UploadMaterialPage = lazy(
  () => import("@/pages/Materials/UploadMaterialPage"),
);

// materials page is the first one auth user sees
export const materialsRoutes: AppRoute[] = [
  { path: `${MATERIALS_PREFIX}`, element: <MaterialsPage /> },
  {
    path: `${MATERIALS_PREFIX}/upload-material`,
    element: <UploadMaterialPage />,
  },
];
