import { create } from "zustand";
import { persist } from "zustand/middleware";

import { Theme } from "./types";

type SettingsStore = {
  theme: Theme;
  displayMaterialTags: boolean;
  setTheme: (theme: Theme) => void;
  setDisplayMateialTags: (displayMateialTags: boolean) => void;
};

export const useSettingsStore = create<SettingsStore>()(
  persist(
    (set) => ({
      theme: "system",
      displayMaterialTags: true,

      setTheme: (theme) => set({ theme }),
      setDisplayMateialTags: (displayMaterialTags) =>
        set({ displayMaterialTags }),
    }),
    {
      name: "settings-store",
    },
  ),
);
