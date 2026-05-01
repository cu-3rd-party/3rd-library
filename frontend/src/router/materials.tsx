import { lazy } from "react";

import { MATERIALS_PREFIX } from "@/constants";
import type { AppRoute } from "@/models";
import MaterialsPage from "@/pages/Materials/MaterialsPage";

const UploadMaterialPage = lazy(
  () => import("@/pages/Materials/UploadMaterialPage"),
);
const ModerationPage = lazy(() => import("@/pages/Materials/ModerationPage"));
const MaterialDetailsPage = lazy(
  () => import("@/pages/Materials/MaterialDetailsPage"),
);

// materials page is the first one auth user sees
export const materialsRoutes: AppRoute[] = [
  { path: `${MATERIALS_PREFIX}`, element: <MaterialsPage /> },
  {
    path: `${MATERIALS_PREFIX}/upload-material`,
    element: <UploadMaterialPage />,
  },
  {
    path: `${MATERIALS_PREFIX}/moderation`,
    element: <ModerationPage />,
  },
  {
    path: `${MATERIALS_PREFIX}/:id`,
    element: <MaterialDetailsPage />,
  },
];
