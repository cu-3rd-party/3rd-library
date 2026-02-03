import * as React from "react";

import { cn } from "@/lib/utils";

function Textarea({ className, ...props }: React.ComponentProps<"textarea">) {
  return (
    <textarea
      data-slot="textarea"
      className={cn(
        "border-border placeholder:text-neutral-500 focus-visible:border-ring transition-colors duration-(--std-duration)",
        "aria-invalid:ring-red-500/20 dark:aria-invalid:ring-red-500/40 aria-invalid:border-red-500 flex",
        "field-sizing-content min-h-16 w-full rounded-md border bg-input px-3 py-2 text-base shadow-xs",
        "outline-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
        "dark:aria-invalid:ring-red-900/20 dark:dark:aria-invalid:ring-red-900/40 dark:aria-invalid:border-red-900",
        className,
      )}
      {...props}
    />
  );
}

export { Textarea };
