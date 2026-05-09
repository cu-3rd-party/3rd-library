import { lazy } from "react";

import MaterialsPage from "@/pages/materials/MaterialsPage";
import { MATERIALS_PREFIX } from "@/shared/constants";

import { AppRoute } from "../types";

const UploadMaterialPage = lazy(
  () => import("@/pages/materials/UploadMaterialPage"),
);

const MaterialDetailsPage = lazy(
  () => import("@/pages/materials/MaterialDetailsPage"),
);

// materials page is the first one auth user sees
export const materialsRoutes: AppRoute[] = [
  { path: `${MATERIALS_PREFIX}`, element: <MaterialsPage /> },
  {
    path: `${MATERIALS_PREFIX}/upload-material`,
    element: <UploadMaterialPage />,
  },
  {
    path: `${MATERIALS_PREFIX}/:id`,
    element: <MaterialDetailsPage />,
  },
];
