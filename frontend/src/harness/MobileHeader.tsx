import { LogOut, Moon, Sun } from "lucide-react";
import { useNavigate } from "react-router-dom";

import { Button } from "@/components/ui/button";
import { AUTHORIZATION_PREFIX } from "@/constants";
import { useTheme } from "@/hooks";
import { clearCurrentSession } from "@/lib/currentUser";

export const MobileHeader = () => {
  const navigate = useNavigate();
  const { setTheme, theme } = useTheme();
  const handleLogout = () => {
    clearCurrentSession();
    navigate(`${AUTHORIZATION_PREFIX}/login`, { replace: true });
  };

  return (
    <div className="flex md:hidden flex-1 items-center justify-end gap-6">
      <Button
        variant="ghost"
        size="icon-lg"
        onClick={() => setTheme(theme === "dark" ? "light" : "dark")}
        title="Сменить тему"
      >
        <Sun className="absolute size-5 rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
        <Moon className="absolute size-5 rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
        <span className="sr-only">Переключить тему</span>
      </Button>
      <Button
        variant="ghost"
        size="icon-lg"
        onClick={handleLogout}
        title="Выход"
      >
        <LogOut className="absolute size-5" />
        <span className="sr-only">Выход</span>
      </Button>
    </div>
  );
};
