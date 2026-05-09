import { useSettingsStore } from "./store";

export const useTheme = () => {
  const theme = useSettingsStore((s) => s.theme);
  const setTheme = useSettingsStore((s) => s.setTheme);
  return { theme, setTheme };
};
