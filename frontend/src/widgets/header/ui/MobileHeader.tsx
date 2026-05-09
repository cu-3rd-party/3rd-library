import { LogOut } from "lucide-react";
import { useNavigate } from "react-router-dom";

import { clearCurrentSession } from "@/entities/session/lib";
import { ThemeToggleButton } from "@/entities/settings/ui";
import { AUTHORIZATION_PREFIX } from "@/shared/constants";
import { Button } from "@/shared/ui";

export const MobileHeader = () => {
  const navigate = useNavigate();
  const handleLogout = () => {
    clearCurrentSession();
    navigate(`${AUTHORIZATION_PREFIX}/login`, { replace: true });
  };

  return (
    <div className="flex md:hidden flex-1 items-center justify-end gap-6">
      <ThemeToggleButton />
      <Button
        variant="ghost"
        size="icon-lg"
        onClick={handleLogout}
        title="Выход"
      >
        <LogOut className="absolute size-5" />
        <span className="sr-only">Выход</span>
      </Button>
    </div>
  );
};
