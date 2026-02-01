import { Suspense } from "react";
import { Routes, Route, Outlet, Navigate } from "react-router-dom";

import { MATERIALS_PREFIX } from "@/constants";
import MainHarness from "@/harness";
import { AppRoute } from "@/models";

import { aboutRoutes } from "./about";
import { authorizationRoutes } from "./authorization";
import { authorsRoutes } from "./authors";
import { fallbackRoutes } from "./fallback";
import { materialsRoutes } from "./materials";

const PageLoader = () => <div className="p-10 text-center">Загрузка...</div>;

export const AppRouter = () => {
  const withHarnessRoutes: AppRoute[] = [
    ...aboutRoutes,
    ...authorsRoutes,
    ...materialsRoutes,
  ];

  const plainRoutes = [...authorizationRoutes, ...fallbackRoutes];

  const HarnessLayout = () => (
    <MainHarness>
      <Suspense fallback={<PageLoader />}>
        <Outlet />
      </Suspense>
    </MainHarness>
  );

  return (
    <Routes>
      <Route element={<HarnessLayout />}>
        <Route
          path="/"
          element={<Navigate to={`${MATERIALS_PREFIX}`} replace />}
        />
        {withHarnessRoutes.map(({ path, element }) => (
          <Route key={path} element={element} path={path} />
        ))}
      </Route>
      {plainRoutes.map(({ path, element }) => (
        <Route key={path} element={element} path={path} />
      ))}
    </Routes>
  );
};
