import { Avatar, AvatarFallback, AvatarImage } from "@radix-ui/react-avatar";
import { Moon, Sun, UploadCloud } from "lucide-react";
import { useNavigate } from "react-router-dom";

import { Button } from "@/components/ui/button";
import { useTheme } from "@/hooks";

export const DesktopHeader = () => {
  const navigate = useNavigate();
  const { setTheme, theme } = useTheme();

  return (
    <div className="hidden md:flex flex-1 items-center justify-end gap-4">
      <Button
        className="px-4 text-base"
        onClick={() => navigate("/materials/upload-material")}
      >
        <UploadCloud className="size-5 mr-2" />
        <span>Опубликовать материал</span>
      </Button>
      <Button
        variant="ghost"
        size="icon"
        onClick={() => setTheme(theme === "dark" ? "light" : "dark")}
        title="Сменить тему"
      >
        <Sun className="absolute size-5 rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
        <Moon className="absolute size-5 rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
        <span className="sr-only">Переключить тему</span>
      </Button>
      <Button
        variant="ghost"
        size="icon"
        className="rounded-full"
        onClick={() => navigate("/authors/1")}
      >
        <Avatar className="size-10">
          <AvatarImage src="/pwa-144x144.png" alt="@user" />
          <AvatarFallback>CN</AvatarFallback>
        </Avatar>
      </Button>
    </div>
  );
};
