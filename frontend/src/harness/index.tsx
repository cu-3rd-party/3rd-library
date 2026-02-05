import type { ReactNode } from "react";

import { Navbar } from "@/harness/Navbar";

interface MainHarnessProps {
  children: ReactNode;
}

const MainHarness = ({ children }: MainHarnessProps) => {
  return (
    <div className="relative flex min-h-screen flex-col bg-background text-foreground">
      <Navbar>{children}</Navbar>
      {/* flex-1 заставляет main занимать все доступное место,
         прижимая футер (если будет) вниз
      */}
    </div>
  );
};

export default MainHarness;
