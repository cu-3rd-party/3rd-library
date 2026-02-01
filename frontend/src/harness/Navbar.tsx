import {
  Moon,
  Sun,
  UploadCloud,
  Home,
  Feather,
  User,
  Info,
  LogOut,
} from "lucide-react";
import { ReactNode } from "react";
import { useLocation, useNavigate } from "react-router-dom";

import { useTheme } from "@/components/theme-provider";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import { Tabs, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { ABOUT_PREFIX, AUTHORS_PREFIX, MATERIALS_PREFIX } from "@/constants";

import { DesktopHeader } from "./DesktopHeader";
import { MobileHeader } from "./MobileHeader";

const NAV_ITEMS = [
  { label: "Материалы", path: `${MATERIALS_PREFIX}` },
  { label: "Авторы", path: `${AUTHORS_PREFIX}` },
  { label: "О проекте", path: `${ABOUT_PREFIX}` },
];

const NAV_ITEMS_MOBILE = [
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
    label: "Опубликовать материал",
    path: `${MATERIALS_PREFIX}/upload-material`,
    icon: <UploadCloud className="size-5" />,
  },
  {
    label: "Профиль",
    path: `${AUTHORS_PREFIX}/1`,
    icon: <User className="size-5" />,
  }, // change
];

interface HeaderProps {
  children: ReactNode;
}

export const Header = ({ children }: HeaderProps) => {
  const location = useLocation();
  const navigate = useNavigate();
  const { setTheme, theme } = useTheme();

  const isActive = (path: string) => {
    if (path === "/" && location.pathname !== "/") return false;
    return location.pathname === path;
  };

  return (
    <div className="flex flex-col min-h-screen">
      <header className="sticky top-0 z-50 border-b border-border/40 bg-background">
        <div className="container px-4 lg:px-0 lg:w-5/6 m-auto flex h-12 lg:h-16 max-w-screen-2xl items-center justify-between">
          <nav className="hidden lg:flex items-center gap-4 text-md font-medium">
            {NAV_ITEMS.map((item) => (
              <Button
                key={item.path}
                onClick={() => navigate(item.path)}
                variant={
                  isActive(item.path) ? "primaryActive" : "primaryInactive"
                }
                className={`h-9 px-4 py-2 transition-colors text-md`}
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
      <nav className="lg:hidden sticky bottom-0 z-50 border-t border-border/40 bg-background">
        <Tabs
          value={
            NAV_ITEMS_MOBILE.find((item) => isActive(item.path))?.label ||
            NAV_ITEMS_MOBILE[0].label
          }
          className="flex w-full justify-center items-center h-12"
        >
          <TabsList className="w-full h-full">
            {NAV_ITEMS_MOBILE.map((item) => (
              <TabsTrigger
                key={item.path}
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
