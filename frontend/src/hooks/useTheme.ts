import { useEffect } from "react";

import { useSettingsStore } from "@/store";

export const useTheme = () => {
  const { theme, setTheme } = useSettingsStore();

  useEffect(() => {
    const root = window.document.documentElement;
    const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");

    const applyTheme = () => {
      root.classList.remove("light", "dark");

      if (theme === "system") {
        root.classList.add(mediaQuery.matches ? "dark" : "light");
        return;
      }

      root.classList.add(theme);
    };

    applyTheme();

    if (theme !== "system") return;

    const handleSystemThemeChange = () => {
      applyTheme();
    };

    if (typeof mediaQuery.addEventListener === "function") {
      mediaQuery.addEventListener("change", handleSystemThemeChange);
      return () => {
        mediaQuery.removeEventListener("change", handleSystemThemeChange);
      };
    }

    mediaQuery.addListener(handleSystemThemeChange);
    return () => {
      mediaQuery.removeListener(handleSystemThemeChange);
    };
  }, [theme]);

  return { theme, setTheme };
};
