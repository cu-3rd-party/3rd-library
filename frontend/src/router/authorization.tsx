import { lazy } from "react";
import { Navigate } from "react-router-dom";

import { AUTHORIZATION_PREFIX } from "@/constants";
import type { AppRoute } from "@/models";
import LoginPage from "@/pages/Authorization/LoginPage";

const LazyRegisterPage = lazy(
  () => import("@/pages/Authorization/RegisterPage"),
);

export const authorizationRoutes: AppRoute[] = [
  {
    path: `${AUTHORIZATION_PREFIX}`,
    element: <Navigate to={`${AUTHORIZATION_PREFIX}/login`} replace />,
  },
  { path: `${AUTHORIZATION_PREFIX}/login`, element: <LoginPage /> },
  { path: `${AUTHORIZATION_PREFIX}/register`, element: <LazyRegisterPage /> },
];
