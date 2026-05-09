import { cn } from "@/shared/lib";

export type AuthFieldErrorProps = {
  message?: string;
  className?: string;
};

export const AuthFieldError = ({ message, className }: AuthFieldErrorProps) => {
  if (!message) return null;

  return <p className={cn("text-sm text-destructive", className)}>{message}</p>;
};
