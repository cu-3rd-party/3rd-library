import { MATERIALS_PREFIX } from "@/shared/constants";

import { RequireModerator } from "../guards";
import { AppRoute } from "../types";

export const moderationRoutes: AppRoute[] = [
  {
    path: `${MATERIALS_PREFIX}/moderation`,
    element: <RequireModerator />,
  },
];
