import { useEffect } from "react";

import { useTheme } from "@/entities/settings/model";

export const ThemeApplicator = ({
  children,
}: {
  children: React.ReactNode;
}) => {
  const { theme } = useTheme();

  useEffect(() => {
    const root = window.document.documentElement;
    const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");

    const applyTheme = () => {
      root.classList.remove("light", "dark");
      root.classList.add(
        theme === "system" ? (mediaQuery.matches ? "dark" : "light") : theme,
      );
    };

    applyTheme();

    if (theme !== "system") return;

    const handleSystemThemeChange = () => applyTheme();
    mediaQuery.addEventListener("change", handleSystemThemeChange);
    return () =>
      mediaQuery.removeEventListener("change", handleSystemThemeChange);
  }, [theme]);

  return <>{children}</>;
};
