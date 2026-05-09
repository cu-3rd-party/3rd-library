import { Moon, Sun } from "lucide-react";

import { Button } from "@/shared/ui/kit/button";

import { useTheme } from "../model";

type ThemeToggleButtonProps = {
  size?: "icon" | "icon-lg";
  className?: string;
  title?: string;
};

export const ThemeToggleButton = ({
  size = "icon",
  className,
  title = "Сменить тему",
}: ThemeToggleButtonProps) => {
  const { theme, setTheme } = useTheme();

  return (
    <Button
      type="button"
      variant="ghost"
      size={size}
      className={className}
      onClick={() => setTheme(theme === "dark" ? "light" : "dark")}
      title={title}
    >
      <Sun className="absolute size-5 rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
      <Moon className="absolute size-5 rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
      <span className="sr-only">Переключить тему</span>
    </Button>
  );
};
