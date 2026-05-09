import { forwardRef } from "react";

import { cn } from "@/shared/lib";
import { Badge } from "@/shared/ui";

type MaterialBadgeProps = {
  label: string;
  className: string;
};

export const MaterialBadge = forwardRef<HTMLDivElement, MaterialBadgeProps>(
  ({ label, className, ...props }, ref) => (
    <Badge
      ref={ref}
      variant="secondary"
      className={cn(
        "rounded-md px-1 lg:px-2 py-0.5 text-xs font-normal",
        className,
      )}
      {...props}
    >
      {label}
    </Badge>
  ),
);

MaterialBadge.displayName = "MaterialBadge";
