import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import { VitePWA } from "vite-plugin-pwa";
import basicSsl from "@vitejs/plugin-basic-ssl";
import { lingui } from "@lingui/vite-plugin";
import tsconfigPaths from "vite-tsconfig-paths";
import manifest from "./manifest";
import tailwindcss from "@tailwindcss/vite";

const isDevHost = process.env.npm_lifecycle_event === "dev:host";
const DEV_ENABLE_HTTPS = isDevHost;
const DEV_ENABLE_PWA = false;

export default defineConfig({
  plugins: [
    react(),
    tsconfigPaths(),
    lingui(),
    tailwindcss(),
    DEV_ENABLE_HTTPS && basicSsl(),
    VitePWA({
      registerType: "autoUpdate",
      includeAssets: ["favicon.ico"],
      manifest: manifest,
      workbox: {
        globPatterns: ["**/*.{js,css,html,ico,png,svg}"],
      },
      devOptions: {
        enabled: DEV_ENABLE_PWA,
      },
    }),
  ].filter(Boolean),
  server: {
    port: 5173,
    host: "localhost",
    https: DEV_ENABLE_HTTPS,
    proxy: {
      "/api/auth": {
        target: "http://localhost:8080",
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api\/auth/, "/auth"),
      },
      "/api/users": {
        target: "http://localhost:8080",
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api\/users/, "/users"),
      },
      "/api/user": {
        target: "http://localhost:8080",
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api\/user$/, "/users/me"),
      },
      "/api/materials": {
        target: "http://localhost:8080",
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api\/materials/, "/materials"),
      },
      "/api/articles": {
        target: "http://localhost:8080",
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api\/articles/, "/materials"),
      },
      "/api/moderation": {
        target: "http://localhost:8080",
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api\/moderation/, "/moderation"),
      },
      "/api": {
        target: "http://localhost:8080",
        changeOrigin: true,
      },
    },
  },
  build: {
    target: "esnext",
  },
});
