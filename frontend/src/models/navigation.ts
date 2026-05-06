import type { ReactElement } from "react";

export interface NavItemDesktop {
  label: string;
  path: string;
  requiredRole?: "moderator";
}

export interface NavItemMobile {
  label: string;
  path: string;
  icon: ReactElement;
  requiredRole?: "moderator";
}
