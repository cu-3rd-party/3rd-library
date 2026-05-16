import {
  UploadCloud,
  Home,
  Feather,
  User,
  Info,
  ShieldCheck,
} from "lucide-react";

import { NavItemDesktop, NavItemMobile } from "@/models";

import {
  ABOUT_PREFIX,
  AUTHORS_PREFIX,
  MATERIALS_PREFIX,
} from "./routePrefixes";

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

export const NAV_ITEMS_MOBILE: NavItemMobile[] = [
  {
    label: "О проекте",
    path: `${ABOUT_PREFIX}`,
    icon: <Info className="size-5" />,
  },
  {
    label: "Авторы",
    path: `${AUTHORS_PREFIX}`,
    icon: <Feather className="size-5" />,
  },
  {
    label: "Материалы",
    path: `${MATERIALS_PREFIX}`,
    icon: <Home className="size-5" />,
  },
  {
    label: "Модерация",
    path: `${MATERIALS_PREFIX}/moderation`,
    icon: <ShieldCheck className="size-5" />,
    requiredRole: "moderator",
  },
  {
    label: "Опубликовать материал",
    path: `${MATERIALS_PREFIX}/upload-material`,
    icon: <UploadCloud className="size-5" />,
  },
  {
    label: "Профиль",
    path: `${AUTHORS_PREFIX}`,
    icon: <User className="size-5" />,
  },
];
