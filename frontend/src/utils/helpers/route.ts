import { NAV_ITEMS_DESKTOP } from "@/constants";

export const isPathActive = (path: string) => {
  if (path === "/" && location.pathname !== "/") return false;
  return location.pathname === path;
};

export const getAboutProjectItem = () => {
  return NAV_ITEMS_DESKTOP[NAV_ITEMS_DESKTOP.length - 1];
}