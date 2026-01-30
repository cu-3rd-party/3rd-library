import type { ReactNode } from "react";
import { Header } from "@/harness/Header";

interface MainHarnessProps {
  children: ReactNode;
}

const MainHarness = ({ children }: MainHarnessProps) => {
  return (
    <div className="relative flex min-h-screen flex-col bg-background text-foreground">
      <Header />
      {/* flex-1 заставляет main занимать все доступное место,
         прижимая футер (если будет) вниз
      */}
      <main >{children}</main>
    </div>
  );
};

export default MainHarness;
