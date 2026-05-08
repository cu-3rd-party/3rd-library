import { Eye, EyeOff } from "lucide-react";
import { useState } from "react";

import { AuthFieldError } from "@/common/atoms/Auth";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { cn } from "@/lib/utils";

export type AuthPasswordFieldProps = {
  id: string;
  label: string;
  value: string;
  onChange: (value: string) => void;
  autoComplete?: string;
  placeholder?: string;
  disabled?: boolean;
  error?: string;
  className?: string;
};

export const AuthPasswordField = ({
  id,
  label,
  value,
  onChange,
  autoComplete,
  placeholder,
  disabled = false,
  error,
  className,
}: AuthPasswordFieldProps) => {
  const [isPasswordVisible, setIsPasswordVisible] = useState(false);

  return (
    <div className={cn("space-y-2", className)}>
      <Label htmlFor={id}>{label}</Label>
      <div className="relative">
        <Input
          id={id}
          type={isPasswordVisible ? "text" : "password"}
          autoComplete={autoComplete}
          placeholder={placeholder}
          value={value}
          onChange={(event) => onChange(event.target.value)}
          disabled={disabled}
          aria-invalid={Boolean(error)}
          className="pr-10"
        />
        <button
          type="button"
          className="text-muted-foreground hover:text-foreground absolute inset-y-0 right-0 inline-flex items-center px-3 transition-colors disabled:cursor-not-allowed disabled:opacity-50"
          onClick={() => setIsPasswordVisible((current) => !current)}
          disabled={disabled}
          aria-label={isPasswordVisible ? "Скрыть пароль" : "Показать пароль"}
        >
          {isPasswordVisible ? (
            <EyeOff className="size-4" aria-hidden="true" />
          ) : (
            <Eye className="size-4" aria-hidden="true" />
          )}
        </button>
      </div>
      <AuthFieldError message={error} />
    </div>
  );
};
