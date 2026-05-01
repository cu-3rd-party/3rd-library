import { lazy } from "react";

import { ABOUT_PREFIX } from "@/constants";
import type { AppRoute } from "@/models";

const LazyAboutPage = lazy(() => import("@/pages/About/AboutPage"));

export const aboutRoutes: AppRoute[] = [
  { path: `${ABOUT_PREFIX}`, element: <LazyAboutPage /> },
];
