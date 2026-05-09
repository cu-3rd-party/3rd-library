import { lazy } from "react";
import { Navigate } from "react-router-dom";

import LoginPage from "@/pages/authorization/LoginPage";
import { AUTHORIZATION_PREFIX } from "@/shared/constants";

import { AppRoute } from "../types";

const LazyRegisterPage = lazy(
  () => import("@/pages/authorization/RegisterPage"),
);

export const authorizationRoutes: AppRoute[] = [
  {
    path: `${AUTHORIZATION_PREFIX}`,
    element: <Navigate to={`${AUTHORIZATION_PREFIX}/login`} replace />,
  },
  { path: `${AUTHORIZATION_PREFIX}/login`, element: <LoginPage /> },
  { path: `${AUTHORIZATION_PREFIX}/register`, element: <LazyRegisterPage /> },
];
