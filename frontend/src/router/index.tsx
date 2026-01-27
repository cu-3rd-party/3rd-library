import { Routes, Route } from "react-router-dom";

import { MainPage } from "@/pages/Main";

const PageLoader = () => <div>Загрузка...</div>;

export const AppRouter = () => {
  return (
    <Routes>
      <Route element={<MainPage />} path="/" />
    </Routes>
  );
};
