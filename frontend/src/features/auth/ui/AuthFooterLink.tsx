import { Link } from "react-router-dom";

import { cn } from "@/shared/lib";

export type AuthFooterLinkProps = {
  promptText: string;
  linkText: string;
  to: string;
  className?: string;
};

export const AuthFooterLink = ({
  promptText,
  linkText,
  to,
  className,
}: AuthFooterLinkProps) => (
  <p className={cn("text-center text-sm text-muted-foreground", className)}>
    {promptText}{" "}
    <Link
      className="font-medium text-foreground underline-offset-4 hover:underline"
      to={to}
    >
      {linkText}
    </Link>
  </p>
);
