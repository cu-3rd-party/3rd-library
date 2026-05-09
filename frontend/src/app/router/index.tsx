import { Suspense } from "react";
import { Routes, Route, Outlet, Navigate } from "react-router-dom";

import { useTheme } from "@/entities/settings/model";
import { MATERIALS_PREFIX } from "@/shared/constants";

import { MainLayout } from "../layouts";

import { RequireAuth } from "./guards";
import { aboutRoutes } from "./routes/about";
import { authorizationRoutes } from "./routes/authorization";
import { authorsRoutes } from "./routes/authors";
import { fallbackRoutes } from "./routes/fallback";
import { materialsRoutes } from "./routes/materials";
import { moderationRoutes } from "./routes/moderation";
import { AppRoute } from "./types";

const PageLoader = () => <div className="p-10 text-center">Загрузка...</div>;

export const AppRouter = () => {
  useTheme();

  const withHarnessRoutes: AppRoute[] = [
    ...aboutRoutes,
    ...authorsRoutes,
    ...materialsRoutes,
    ...moderationRoutes,
  ];

  const plainRoutes = [...authorizationRoutes, ...fallbackRoutes];

  const HarnessLayout = () => (
    <MainLayout>
      <Suspense fallback={<PageLoader />}>
        <Outlet />
      </Suspense>
    </MainLayout>
  );

  return (
    <Routes>
      <Route element={<RequireAuth />}>
        <Route element={<HarnessLayout />}>
          <Route
            path="/"
            element={<Navigate to={`${MATERIALS_PREFIX}`} replace />}
          />
          {withHarnessRoutes.map(({ path, element }) => (
            <Route key={path} element={element} path={path} />
          ))}
        </Route>
      </Route>
      {plainRoutes.map(({ path, element }) => (
        <Route key={path} element={element} path={path} />
      ))}
    </Routes>
  );
};
