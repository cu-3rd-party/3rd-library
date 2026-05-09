import { lazy } from "react";

import { AppRoute } from "../types";

const LazyNotFoundPage = lazy(() => import("@/pages/not-found/NotFoundPage"));

export const fallbackRoutes: AppRoute[] = [
  { path: `*`, element: <LazyNotFoundPage /> },
];
