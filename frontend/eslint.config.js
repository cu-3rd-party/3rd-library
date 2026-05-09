import js from "@eslint/js";
import tseslint from "typescript-eslint";
import reactPlugin from "eslint-plugin-react";
import reactHooksPlugin from "eslint-plugin-react-hooks";
import importPlugin from "eslint-plugin-import";
import { defineConfig } from "eslint/config";

export default defineConfig({
  languageOptions: {
    ecmaVersion: "latest",
    parserOptions: {
      project: "./tsconfig.json",
      tsconfigRootDir: import.meta.dirname,
    },
  },
  extends: [
    js.configs.recommended,
    ...tseslint.configs.recommended,
    reactPlugin.configs.flat.recommended,
    importPlugin.flatConfigs.recommended,
  ],
  plugins: {
    "react-hooks": reactHooksPlugin,
  },
  settings: {
    react: {
      version: "detect",
      runtime: "automatic",
    },
    "import/resolver": {
      typescript: { project: `${import.meta.dirname}/tsconfig.json` },
    },
    "import/core-modules": ["uuid"],
  },
  rules: {
    "react/react-in-jsx-scope": "off",
    "import/order": [
      "error",
      {
        groups: [
          "builtin",
          "external",
          "internal",
          "parent",
          "sibling",
          "index",
        ],
        "newlines-between": "always",
        alphabetize: {
          order: "asc",
          caseInsensitive: true,
        },
      },
    ],
    "import/newline-after-import": "off",
    "@typescript-eslint/no-explicit-any": "warn",
    "react/jsx-handler-names": "off",
    "@typescript-eslint/no-unused-vars": [
      "warn",
      {
        varsIgnorePattern: "^_",
        argsIgnorePattern: "^_",
      },
    ],
    "@typescript-eslint/naming-convention": [
      "error",
      {
        selector: "variable",
        filter: { regex: "^_", match: true },
        format: null,
      },
    ],
  },
  ignores: [
    "node_modules/**",
    "dist/**",
    "eslint.config.js",
    "manifest.ts",
    "vite.config.js",
    "workbox.config.ts",
    "public/**",
  ],
});
