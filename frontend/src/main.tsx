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

if ("serviceWorker" in navigator && import.meta.env.PROD) {
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
}
