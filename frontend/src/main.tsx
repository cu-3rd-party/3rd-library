import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { BrowserRouter } from "react-router-dom";

import { enableMocking } from "@/mocks/browser";
import { AppRouter } from "@/router";
import "./index.css";

const container = document.querySelector("#root") as HTMLElement;

enableMocking().then(() => {
  createRoot(container).render(
    <StrictMode>
      <BrowserRouter>
        <AppRouter />
      </BrowserRouter>
    </StrictMode>,
  );
});

const isLocalhostHost =
  globalThis.location.hostname === "localhost" ||
  globalThis.location.hostname === "127.0.0.1" ||
  globalThis.location.hostname === "::1";

const shouldRegisterServiceWorker = import.meta.env.PROD && !isLocalhostHost;

if ("serviceWorker" in navigator) {
  if (shouldRegisterServiceWorker) {
    window.addEventListener("load", () => {
      navigator.serviceWorker
        .register("/sw.js", { scope: "/" })
        .then((reg) => {
          console.log("Service Worker registered");
          setInterval(
            () => {
              reg.update().then(() => console.log("Updated"));
            },
            6 * 60 * 60 * 1000,
          );
        })
        .catch((error) => console.error("SW registration failed:", error));
    });
  } else {
    navigator.serviceWorker
      .getRegistrations()
      .then((registrations) =>
        Promise.all(
          registrations.map((registration) => registration.unregister()),
        ),
      )
      .then(() => {
        if (isLocalhostHost) {
          console.log("Service Worker disabled on localhost");
        }
      })
      .catch((error) =>
        console.warn("Failed to cleanup service workers", error),
      );
  }
}
