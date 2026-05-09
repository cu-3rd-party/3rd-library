import { useMemo } from "react";
import { useNavigate } from "react-router-dom";

import { isModerator, useCurrentUser } from "@/entities/session/lib";
import { isPathActive } from "@/shared/lib/router";
import { Button } from "@/shared/ui/kit/button";

import { NAV_ITEMS_DESKTOP } from "../model/navItems";

import { DesktopHeader } from "./DesktopHeader";
import { MobileHeader } from "./MobileHeader";

export const Header = () => {
  const navigate = useNavigate();
  const user = useCurrentUser();
  const hasModeratorAccess = isModerator(user);

  const desktopNavItems = useMemo(
    () =>
      NAV_ITEMS_DESKTOP.filter(
        (item) =>
          (item.requiredRole !== "moderator" || hasModeratorAccess) &&
          item.type !== "special",
      ),
    [hasModeratorAccess],
  );

  return (
    <header className="sticky top-0 z-50 border-b border-border/40 bg-background">
      <div className="container px-4 lg:px-0 lg:w-5/6 m-auto flex h-12 lg:h-16 max-w-screen-2xl items-center justify-between">
        <nav className="hidden md:flex items-center gap-4 font-medium">
          {desktopNavItems.map((item) => (
            <Button
              key={item.path}
              onClick={() => navigate(item.path)}
              variant={isPathActive(item.path) ? "navActive" : "navInactive"}
              className="h-9 px-4 py-2 transition-colors text-base lg:text-lg"
            >
              {item.label}
            </Button>
          ))}
        </nav>
        <DesktopHeader />
        <MobileHeader />
      </div>
    </header>
  );
};
