import { ReactNode } from "react";
import { useLocation, useNavigate } from "react-router-dom";

import { Button } from "@/components/ui/button";
import { Tabs, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { NAV_ITEMS_DESKTOP, NAV_ITEMS_MOBILE } from "@/constants";
import { resolveCurrentProfilePath } from "@/lib/currentUser";

import { DesktopHeader } from "./DesktopHeader";
import { MobileHeader } from "./MobileHeader";

interface NavbarProps {
  children: ReactNode;
}

export const Navbar = ({ children }: NavbarProps) => {
  const location = useLocation();
  const navigate = useNavigate();
  const profilePath = resolveCurrentProfilePath();
  const mobileNavItems = NAV_ITEMS_MOBILE.map((item) =>
    item.label === "Профиль" ? { ...item, path: profilePath } : item,
  );

  const isActive = (path: string) => {
    if (path === "/" && location.pathname !== "/") return false;
    return location.pathname === path;
  };

  return (
    <div className="flex flex-col min-h-screen">
      <header className="sticky top-0 z-50 border-b border-border/40 bg-background">
        <div className="container px-4 lg:px-0 lg:w-5/6 m-auto flex h-12 lg:h-16 max-w-screen-2xl items-center justify-between">
          <nav className="hidden md:flex items-center gap-4 font-medium">
            {NAV_ITEMS_DESKTOP.map((item) => (
              <Button
                key={item.path}
                onClick={() => navigate(item.path)}
                variant={isActive(item.path) ? "navActive" : "navInactive"}
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
      <main className="flex-1"> {children} </main>
      <nav className="md:hidden sticky bottom-0 z-50 border-t border-border/40 bg-background">
        <Tabs
          value={
            mobileNavItems.find((item) => isActive(item.path))?.label ||
            mobileNavItems[0].label
          }
          className="flex w-full justify-center items-center h-12"
        >
          <TabsList className="w-full h-full">
            {mobileNavItems.map((item) => (
              <TabsTrigger
                key={`${item.label}-${item.path}`}
                value={item.label}
                onClick={() => {
                  navigate(item.path);
                }}
                className="justify-center text-base"
              >
                {item.icon}
              </TabsTrigger>
            ))}
          </TabsList>
        </Tabs>
      </nav>
    </div>
  );
};
