import { LoaderCircle, LogIn } from "lucide-react";
import type { FormEvent } from "react";

import { AuthMessage } from "@/common/atoms/Auth";
import {
  AuthFooterLink,
  AuthPasswordField,
  AuthTextField,
} from "@/common/molecules/Auth";
import { Button } from "@/components/ui/button";

type LoginAuthFormFieldErrors = {
  email?: string;
  password?: string;
};

export type LoginAuthFormProps = {
  email: string;
  password: string;
  isSubmitting: boolean;
  errorMessage: string;
  fieldErrors: LoginAuthFormFieldErrors;
  registerPath: string;
  onEmailChange: (value: string) => void;
  onPasswordChange: (value: string) => void;
  onSubmit: (event: FormEvent<HTMLFormElement>) => void;
};

export const LoginAuthForm = ({
  email,
  password,
  isSubmitting,
  errorMessage,
  fieldErrors,
  registerPath,
  onEmailChange,
  onPasswordChange,
  onSubmit,
}: LoginAuthFormProps) => (
  <>
    <form className="space-y-4" onSubmit={onSubmit}>
      <AuthTextField
        id="login-email"
        label="Email"
        type="email"
        autoComplete="email"
        placeholder="name@edu.centraluniversity.ru"
        value={email}
        onChange={onEmailChange}
        disabled={isSubmitting}
        error={fieldErrors.email}
      />

      <AuthPasswordField
        id="login-password"
        label="Пароль"
        autoComplete="current-password"
        placeholder="Введите пароль"
        value={password}
        onChange={onPasswordChange}
        disabled={isSubmitting}
        error={fieldErrors.password}
      />

      {errorMessage ? (
        <AuthMessage message={errorMessage} variant="error" />
      ) : null}

      <Button type="submit" className="w-full" disabled={isSubmitting}>
        {isSubmitting ? (
          <>
            <LoaderCircle className="size-4 animate-spin" />
            Входим...
          </>
        ) : (
          <>
            <LogIn className="size-4" />
            Войти
          </>
        )}
      </Button>
    </form>

    <AuthFooterLink
      promptText="Нет аккаунта?"
      linkText="Зарегистрироваться"
      to={registerPath}
    />
  </>
);
