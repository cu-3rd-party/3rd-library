import { lazy } from "react";

import { AUTHORIZATION_PREFIX } from "@/constants";
import type { AppRoute } from "@/models";
import LoginPage from "@/pages/Authorization/LoginPage";

const LazyRegisterPage = lazy(
  () => import("@/pages/Authorization/RegisterPage"),
);

// login page is the first one unauth user sees
export const authorizationRoutes: AppRoute[] = [
  { path: `${AUTHORIZATION_PREFIX}`, element: <LoginPage /> },
  { path: `${AUTHORIZATION_PREFIX}`, element: <LazyRegisterPage /> },
];
