import { Navigate, Outlet } from "react-router-dom";

import { canAccessModeration, getAccessToken } from "@/entities/session/lib";
import ModerationPage from "@/pages/moderation/ModerationPage";
import { AUTHORIZATION_PREFIX, MATERIALS_PREFIX } from "@/shared/constants";

const hasCredentials = () =>
  import.meta.env.VITE_NO_AUTH === "true" || Boolean(getAccessToken()?.trim());

export const RequireAuth = () => {
  if (!hasCredentials()) {
    return <Navigate to={`${AUTHORIZATION_PREFIX}/login`} replace />;
  }

  return <Outlet />;
};

export const RequireModerator = () => {
  if (!canAccessModeration()) {
    return <Navigate to={MATERIALS_PREFIX} replace />;
  }

  return <ModerationPage />;
};
