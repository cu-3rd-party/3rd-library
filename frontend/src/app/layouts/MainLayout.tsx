import { ReactNode } from "react";

import { BottomNav } from "@/widgets/bottom-nav/ui";
import { Header } from "@/widgets/header/ui";

export const MainLayout = ({ children }: { children: ReactNode }) => (
  <div className="flex flex-col min-h-screen bg-background text-foreground">
    <Header />
    <main className="flex-1">{children}</main>
    <BottomNav />
  </div>
);
