import { Suspense } from "react";
import { Routes, Route, Outlet, Navigate } from "react-router-dom";

import MainHarness from "@/harness";
import { AuthorsPage } from "@/pages/Authors/AuthorsPage.tsx";
import { ProfilePage } from "@/pages/Authors/ProfilePage.tsx";
import { MaterialDetailPage } from "@/pages/Materials/MaterialDetailPage.tsx";
import { MaterialsPage } from "@/pages/Materials/MaterialsPage.tsx";
import { UploadMaterialPage } from "@/pages/Materials/UploadMaterialPage.tsx";

const AboutPage = () => <div>О нас</div>;
const PageLoader = () => <div className="p-10 text-center">Загрузка...</div>;

export const AppRouter = () => {
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
        <Route path="/" element={<Navigate to="/materials" replace />} />

        <Route path="/materials" element={<MaterialsPage />} />
        <Route path="/materials/:id" element={<MaterialDetailPage />} />
        <Route
          path="/materials/upload-material"
          element={<UploadMaterialPage />}
        />
        <Route path="/authors/:id" element={<ProfilePage />} />
        <Route path="/authors" element={<AuthorsPage />} />
        <Route path="/about" element={<AboutPage />} />
      </Route>

      <Route path="/login" element={<div>Страница логина</div>} />

      <Route path="*" element={<div>404 Not Found</div>} />
    </Routes>
  );
};
