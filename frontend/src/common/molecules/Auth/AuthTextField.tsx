import type { HTMLInputTypeAttribute, InputHTMLAttributes } from "react";

import { AuthFieldError } from "@/common/atoms/Auth";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { cn } from "@/lib/utils";

export type AuthTextFieldProps = {
  id: string;
  label: string;
  value: string;
  onChange: (value: string) => void;
  type?: HTMLInputTypeAttribute;
  autoComplete?: string;
  placeholder?: string;
  inputMode?: InputHTMLAttributes<HTMLInputElement>["inputMode"];
  disabled?: boolean;
  error?: string;
  className?: string;
  inputClassName?: string;
};

export const AuthTextField = ({
  id,
  label,
  value,
  onChange,
  type = "text",
  autoComplete,
  placeholder,
  inputMode,
  disabled = false,
  error,
  className,
  inputClassName,
}: AuthTextFieldProps) => (
  <div className={cn("space-y-2", className)}>
    <Label htmlFor={id}>{label}</Label>
    <Input
      id={id}
      type={type}
      autoComplete={autoComplete}
      placeholder={placeholder}
      inputMode={inputMode}
      value={value}
      onChange={(event) => onChange(event.target.value)}
      disabled={disabled}
      aria-invalid={Boolean(error)}
      className={inputClassName}
    />
    <AuthFieldError message={error} />
  </div>
);
