import { NavigateFunction } from "react-router-dom";

import {
  persistCurrentAuthUser,
  setAccessToken,
} from "@/entities/session/lib";
import { AuthUser } from "@/entities/session/model";
import { MATERIALS_PREFIX } from "@/shared/constants";

export type RegisterFieldErrors = {
  name?: string;
  surname?: string;
  email?: string;
  password?: string;
  verificationCode?: string;
};

export const RESEND_COOLDOWN_SECONDS = 60;

export const formatCooldown = (seconds: number): string => {
  const mm = String(Math.floor(seconds / 60)).padStart(2, "0");
  const ss = String(seconds % 60).padStart(2, "0");
  return `${mm}:${ss}`;
};

export const completeAuthAndNavigate = (
  token: string,
  user: AuthUser,
  navigate: NavigateFunction,
) => {
  setAccessToken(`Token ${token}`);
  persistCurrentAuthUser({
    email: user.email,
    username: user.username,
    bio: user.bio || "",
    image: user.image,
    roles: user.roles || ["user"],
  });
  navigate(MATERIALS_PREFIX, { replace: true });
};
