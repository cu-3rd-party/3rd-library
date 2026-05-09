import { useMemo } from "react";
import { useNavigate } from "react-router-dom";

import {
  isModerator,
  resolveCurrentProfilePath,
  useCurrentUser,
} from "@/entities/session/lib";
import { isPathActive } from "@/shared/lib/router";
import { Tabs, TabsList, TabsTrigger } from "@/shared/ui";

import { NAV_ITEMS_MOBILE } from "../model";

export const BottomNav = () => {
  const navigate = useNavigate();
  const user = useCurrentUser();
  const hasModeratorAccess = isModerator(user);
  const profilePath = resolveCurrentProfilePath();

  const mobileNavItems = useMemo(
    () =>
      NAV_ITEMS_MOBILE.map((item) =>
        item.label === "Профиль" ? { ...item, path: profilePath } : item,
      ).filter(
        (item) => item.requiredRole !== "moderator" || hasModeratorAccess,
      ),
    [hasModeratorAccess, profilePath],
  );

  return (
    <nav className="md:hidden sticky bottom-0 z-50 border-t border-border/40 bg-background">
      <Tabs
        value={
          mobileNavItems.find((item) => isPathActive(item.path))?.label ||
          mobileNavItems[0]?.label ||
          ""
        }
        className="flex w-full justify-center items-center h-12"
      >
        <TabsList className="w-full h-full">
          {mobileNavItems.map((item) => (
            <TabsTrigger
              key={`${item.label}-${item.path}`}
              value={item.label}
              onClick={() => navigate(item.path)}
              className="justify-center text-base"
            >
              {item.icon}
            </TabsTrigger>
          ))}
        </TabsList>
      </Tabs>
    </nav>
  );
};
