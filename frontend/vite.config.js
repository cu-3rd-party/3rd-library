import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import { VitePWA } from 'vite-plugin-pwa'
import basicSsl from '@vitejs/plugin-basic-ssl'
import { lingui } from '@lingui/vite-plugin'
import tsconfigPaths from 'vite-tsconfig-paths'
import manifest from './manifest'

const isDevHost = process.env.npm_lifecycle_event === 'dev:host'
const DEV_ENABLE_HTTPS = isDevHost
const DEV_ENABLE_PWA = false

export default defineConfig({
  plugins: [
    react(),
    tsconfigPaths(),
    lingui(),
    DEV_ENABLE_HTTPS && basicSsl(),
    VitePWA({
      registerType: 'autoUpdate',
      includeAssets: ['favicon.ico'],
      manifest: manifest,
      workbox: {
        globPatterns: ['**/*.{js,css,html,ico,png,svg}'],
      },
      devOptions: {
        enabled: DEV_ENABLE_PWA,
      },
    }),
  ].filter(Boolean),
  server: {
    port: 3000,
    host: 'localhost',
    https: DEV_ENABLE_HTTPS,
  },
  build: {
    target: 'esnext',
  },
})
