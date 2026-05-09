import {
  ABOUT_PREFIX,
  AUTHORS_PREFIX,
  MATERIALS_PREFIX,
} from "@/shared/constants";

import { NavItemDesktop } from "./types";

export const NAV_ITEMS_DESKTOP: NavItemDesktop[] = [
  { label: "Материалы", path: `${MATERIALS_PREFIX}` },
  { label: "Авторы", path: `${AUTHORS_PREFIX}` },
  { label: "Загрузить", path: `${MATERIALS_PREFIX}/upload-material` },
  {
    label: "Модерация",
    path: `${MATERIALS_PREFIX}/moderation`,
    requiredRole: "moderator",
  },
  {
    label: "О проекте",
    path: `${ABOUT_PREFIX}`,
    type: "special",
  },
];

export const getAboutProjectItem = () => {
  return NAV_ITEMS_DESKTOP[NAV_ITEMS_DESKTOP.length - 1];
};
