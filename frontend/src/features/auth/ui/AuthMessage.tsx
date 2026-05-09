import { cn } from "@/shared/lib";

type AuthMessageVariant = "info" | "error";

export type AuthMessageProps = {
  message: string;
  variant: AuthMessageVariant;
  className?: string;
};

const variantClassName: Record<AuthMessageVariant, string> = {
  info: "border-primary/20 bg-primary/10 text-foreground",
  error: "border-destructive/30 bg-destructive/10 text-destructive",
};

export const AuthMessage = ({
  message,
  variant,
  className,
}: AuthMessageProps) => (
  <p
    className={cn(
      "rounded-md border px-3 py-2 text-sm",
      variantClassName[variant],
      className,
    )}
  >
    {message}
  </p>
);
