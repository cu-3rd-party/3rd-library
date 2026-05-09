import { ReactElement } from "react";

export interface AppRoute {
  path: string;
  element: ReactElement;
  protected?: boolean;
}
