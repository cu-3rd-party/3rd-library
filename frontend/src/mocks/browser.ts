import type { SharedOptions } from "msw";
import { setupWorker } from "msw/browser";

import { handlers } from "./handlers";

// Handler for unmatched requests
const onUnhandledRequest: SharedOptions["onUnhandledRequest"] = (
  req,
  print,
) => {
  const excludedExtensions = [
    ".woff2",
    ".css",
    ".tsx",
    ".ts",
    ".js",
    ".png",
    ".otf",
    ".ttf",
    ".woff",
    ".webmanifest",
  ];

  const { pathname } = new URL(req.url);
  if (excludedExtensions.some((ext) => pathname.endsWith(ext))) {
    return;
  }

  print.warning();
};

export const worker = setupWorker(...handlers); // !IMPORTANT

export async function enableMocking() {
  if (import.meta.env.VITE_API !== "mock") return;

  if (!globalThis.isSecureContext) {
    console.warn(
      "[MSW] Mocking skipped: Service Worker requires secure context. Use http://localhost or HTTPS.",
    );
    return;
  }

  try {
    await worker.start({
      onUnhandledRequest,
    });
  } catch (error) {
    console.warn("[MSW] Failed to start worker. Falling back to in-app mocks.");
    console.debug(error);
  }
}
