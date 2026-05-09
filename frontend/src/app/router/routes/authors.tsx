import { lazy } from "react";

import { AUTHORS_PREFIX } from "@/shared/constants";

import { AppRoute } from "../types";

const LazyAuthorsPage = lazy(() => import("@/pages/authors/AuthorsPage"));
const LazyProfilePage = lazy(() => import("@/pages/authors/ProfilePage"));

export const authorsRoutes: AppRoute[] = [
  { path: `${AUTHORS_PREFIX}`, element: <LazyAuthorsPage /> },
  { path: `${AUTHORS_PREFIX}/:id`, element: <LazyProfilePage /> },
];
