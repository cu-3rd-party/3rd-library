import type { ReactElement } from "react";

export interface NavItemDesktop {
  label: string;
  path: string;
}

export interface NavItemMobile {
  label: string;
  path: string;
  icon: ReactElement;
}
