import { lazy } from "react";

import { AUTHORS_PREFIX } from "@/constants";
import type { AppRoute } from "@/models";

const LazyAuthorsPage = lazy(() => import("@/pages/Authors/AuthorsPage"));

const LazyProfilePage = lazy(() => import("@/pages/Authors/ProfilePage"));

export const authorsRoutes: AppRoute[] = [
  { path: `${AUTHORS_PREFIX}`, element: <LazyAuthorsPage /> },
  { path: `${AUTHORS_PREFIX}/:id`, element: <LazyProfilePage /> },
];
