import { lazy } from "react";

import type { AppRoute } from "@/models/appRoute";

const LazyNotFoundPage = lazy(() => import("@/pages/Fallback/NotFoundPage"));

export const fallbackRoutes: AppRoute[] = [
  { path: `*`, element: <LazyNotFoundPage /> },
];
