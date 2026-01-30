// PWA config
export const workboxConfig = {
  // Lifecycle
  skipWaiting: true, // immediate deploy on site update
  clientsClaim: true, // immediate sw activation
  cleanupOutdatedCaches: true,

  // Precaching settings
  globDirectory: "dist/", // built files location
  globPatterns: [
    "**/*.{js,css,html}",
    "**/*.{png,jpg,jpeg,svg,webp,ico}",
    "**/*.{woff,woff2,ttf,eot}",
  ],
  swDest: "dist/sw.js",
  maximumFileSizeToCacheInBytes: 5 * 1024 * 1024, // 5 MB

  // Runtime caching strategies
  runtimeCaching: [
    {
      // External resources (CDN fonts, libraries)
      urlPattern: ({ url }: { url: URL }) =>
        url.origin !== self.location.origin,
      handler: "StaleWhileRevalidate", // serve from cache + simultaneously fetch from network to update the cache
      options: {
        cacheName: "external-resources",
        expiration: {
          maxEntries: 50, // deletes oldest when exceeded
          maxAgeSeconds: 7 * 24 * 60 * 60, // 7 days
        },
      },
    },
    {
      // API requests
      urlPattern: ({ url }: { url: URL }) => url.pathname.startsWith("/api/"), // !IMPORTANT
      handler: "NetworkFirst", // cache only if network fails
      options: {
        cacheName: "api-cache",
        networkTimeoutSeconds: 5,
        expiration: {
          maxEntries: 100,
          maxAgeSeconds: 10 * 60, // 10 minutes
        },
      },
    },
  ],
};

export default workboxConfig;
