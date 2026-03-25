import { create } from "zustand";
import { persist } from "zustand/middleware";

import { Theme } from "@/models";

type SettingsStore = {
  theme: Theme;
  setTheme: (theme: Theme) => void;
};

export const useSettingsStore = create<SettingsStore>()(
  persist(
    (set) => ({
      theme: "system",
      setTheme: (theme) => set({ theme }),
    }),
    {
      name: "settings-store",
    },
  ),
);
