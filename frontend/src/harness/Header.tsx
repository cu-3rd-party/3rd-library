import { Moon, Sun, UploadCloud, Menu } from "lucide-react";
import { useState } from "react";
import { useLocation, useNavigate } from "react-router-dom";

import { useTheme } from "@/components/theme-provider";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from "@/components/ui/sheet";

const NAV_ITEMS = [
  { label: "Материалы", path: "/materials" },
  { label: "Авторы", path: "/authors" },
  { label: "О нас", path: "/about" },
];

export const Header = () => {
  const location = useLocation();
  const navigate = useNavigate();
  const { setTheme, theme } = useTheme();
  const [isOpen, setIsOpen] = useState(false);

  const isActive = (path: string) => {
    if (path === "/" && location.pathname !== "/") return false;
    return location.pathname.startsWith(path);
  };

  return (
    <header className="sticky top-0 z-50 border-b border-border/40 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
      <div className="container w-5/6 m-auto flex h-16 max-w-screen-2xl items-center justify-between">
        <div className="md:hidden mr-2">
          <Sheet open={isOpen} onOpenChange={setIsOpen}>
            <SheetTrigger asChild>
              <Button variant="ghost" size="icon">
                <Menu className="h-6 w-6" />
              </Button>
            </SheetTrigger>
            <SheetContent side="left" className="w-[300px] sm:w-[400px]">
              <SheetHeader>
                <SheetTitle className="text-left">Меню</SheetTitle>
              </SheetHeader>

              <div className="flex flex-col gap-4 mt-6">
                {/* Ссылки навигации */}
                <nav className="flex flex-col gap-2">
                  {NAV_ITEMS.map((item) => (
                    <Button
                      key={item.path}
                      onClick={() => {
                        navigate(item.path);
                        setIsOpen(false);
                      }}
                      variant={isActive(item.path) ? "default" : "ghost"}
                      className="justify-start text-base"
                    >
                      {item.label}
                    </Button>
                  ))}
                </nav>
              </div>
            </SheetContent>
          </Sheet>
        </div>

        <nav className="hidden md:flex items-center gap-4 text-md font-medium">
          {NAV_ITEMS.map((item) => (
            <Button
              key={item.path}
              onClick={() => navigate(item.path)}
              variant={isActive(item.path) ? "default" : "ghost"}
              className={`h-9 px-4 py-2 transition-colors text-md`}
            >
              {item.label}
            </Button>
          ))}
        </nav>

        <div className="flex flex-1 items-center justify-end space-x-2 md:space-x-4">
          <Button
            className="bg-orange-500 hover:bg-orange-600 px-3 md:px-4"
            key="upload-material"
            onClick={() => navigate("/materials/upload-material")}
          >
            <UploadCloud className="h-5 w-5 md:mr-2" />
            <span className="hidden md:inline">Опубликовать материал</span>
          </Button>

          <Button
            variant="ghost"
            size="icon"
            onClick={() => setTheme(theme === "dark" ? "light" : "dark")}
            title="Сменить тему"
          >
            <Sun className="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
            <Moon className="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
            <span className="sr-only">Переключить тему</span>
          </Button>

          <Button
            variant="ghost"
            size="icon"
            className="rounded-full"
            onClick={() => navigate("/authors/1")}
          >
            <Avatar className="h-9 w-9 md:h-10 md:w-10">
              <AvatarImage src="/pwa-144x144.png" alt="@user" />
              <AvatarFallback>CN</AvatarFallback>
            </Avatar>
          </Button>
        </div>
      </div>
    </header>
  );
};
