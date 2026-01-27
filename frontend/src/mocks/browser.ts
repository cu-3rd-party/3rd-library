import type { SharedOptions } from "msw";
import { setupWorker } from "msw/browser";


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

export const worker = setupWorker();  // !IMPORTANT

export async function enableMocking() {
  if (import.meta.env.VITE_API === "mock") {
    await worker.start({
      onUnhandledRequest,
    });
  }
}
