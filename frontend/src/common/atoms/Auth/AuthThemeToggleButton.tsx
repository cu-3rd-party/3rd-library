import { Moon, Sun } from "lucide-react";

import { Button } from "@/components/ui/button";
import { useTheme } from "@/hooks";
import { cn } from "@/lib/utils";

export type AuthThemeToggleButtonProps = {
  className?: string;
  title?: string;
};

export const AuthThemeToggleButton = ({
  className,
  title = "Сменить тему",
}: AuthThemeToggleButtonProps) => {
  const { setTheme, theme } = useTheme();

  return (
    <Button
      type="button"
      variant="ghost"
      size="icon"
      className={cn(className)}
      onClick={() => setTheme(theme === "dark" ? "light" : "dark")}
      title={title}
    >
      <Sun className="absolute size-5 rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
      <Moon className="absolute size-5 rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
      <span className="sr-only">Переключить тему</span>
    </Button>
  );
};
