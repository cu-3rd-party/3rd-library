import { lazy } from "react";

import { ABOUT_PREFIX } from "@/shared/constants";

import { AppRoute } from "../types";

const LazyAboutPage = lazy(() => import("@/pages/about/AboutPage"));

export const aboutRoutes: AppRoute[] = [
  { path: `${ABOUT_PREFIX}`, element: <LazyAboutPage /> },
];
