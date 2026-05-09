import { ReactElement } from "react";

export interface NavItemMobile {
  label: string;
  path: string;
  icon: ReactElement;
  requiredRole?: "moderator";
}
