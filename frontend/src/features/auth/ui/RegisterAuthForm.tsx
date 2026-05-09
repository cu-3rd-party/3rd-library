import { LoaderCircle, UserPlus } from "lucide-react";
import type { FormEvent } from "react";

import { Button } from "@/shared/ui";

import { AuthFooterLink } from "./AuthFooterLink";
import { AuthMessage } from "./AuthMessage";
import { AuthPasswordField } from "./AuthPasswordField";
import { AuthTextField } from "./AuthTextField";

type RegisterAuthFormFieldErrors = {
  name?: string;
  surname?: string;
  email?: string;
  password?: string;
  verificationCode?: string;
};

export type RegisterAuthFormProps = {
  name: string;
  surname: string;
  email: string;
  password: string;
  verificationCode: string;
  isVerificationStep: boolean;
  fieldErrors: RegisterAuthFormFieldErrors;
  infoMessage: string;
  errorMessage: string;
  isSubmitting: boolean;
  isResendingCode: boolean;
  resendCooldown: number;
  loginPath: string;
  onNameChange: (value: string) => void;
  onSurnameChange: (value: string) => void;
  onEmailChange: (value: string) => void;
  onPasswordChange: (value: string) => void;
  onVerificationCodeChange: (value: string) => void;
  onResendCode: () => void;
  onSubmit: (event: FormEvent<HTMLFormElement>) => void;
  formatCooldown: (seconds: number) => string;
};

export const RegisterAuthForm = ({
  name,
  surname,
  email,
  password,
  verificationCode,
  isVerificationStep,
  fieldErrors,
  infoMessage,
  errorMessage,
  isSubmitting,
  isResendingCode,
  resendCooldown,
  loginPath,
  onNameChange,
  onSurnameChange,
  onEmailChange,
  onPasswordChange,
  onVerificationCodeChange,
  onResendCode,
  onSubmit,
  formatCooldown,
}: RegisterAuthFormProps) => (
  <>
    <form className="space-y-4" onSubmit={onSubmit}>
      <AuthTextField
        id="register-name"
        label="Имя"
        autoComplete="given-name"
        placeholder="Иван"
        value={name}
        onChange={onNameChange}
        disabled={isSubmitting || isVerificationStep}
        error={fieldErrors.name}
      />

      <AuthTextField
        id="register-surname"
        label="Фамилия"
        autoComplete="family-name"
        placeholder="Иванов"
        value={surname}
        onChange={onSurnameChange}
        disabled={isSubmitting || isVerificationStep}
        error={fieldErrors.surname}
      />

      <AuthTextField
        id="register-email"
        label="Email"
        type="email"
        autoComplete="email"
        placeholder="name@edu.centraluniversity.ru"
        value={email}
        onChange={onEmailChange}
        disabled={isSubmitting || isVerificationStep}
        error={fieldErrors.email}
      />

      <AuthPasswordField
        id="register-password"
        label="Пароль"
        autoComplete="new-password"
        placeholder="Минимум 8 символов"
        value={password}
        onChange={onPasswordChange}
        disabled={isSubmitting || isVerificationStep}
        error={fieldErrors.password}
      />

      {isVerificationStep ? (
        <div className="space-y-2">
          <AuthTextField
            id="register-verification-code"
            label="Код из письма"
            autoComplete="one-time-code"
            inputMode="numeric"
            placeholder="Введите код подтверждения"
            value={verificationCode}
            onChange={onVerificationCodeChange}
            disabled={isSubmitting}
            error={fieldErrors.verificationCode}
          />
          <Button
            type="button"
            variant="outline"
            className="w-full"
            disabled={isSubmitting || isResendingCode || resendCooldown > 0}
            onClick={onResendCode}
          >
            {isResendingCode ? (
              <>
                <LoaderCircle className="size-4 animate-spin" />
                Отправляем код...
              </>
            ) : resendCooldown > 0 ? (
              `Отправить код повторно (${formatCooldown(resendCooldown)})`
            ) : (
              "Отправить код повторно"
            )}
          </Button>
        </div>
      ) : null}

      {infoMessage ? (
        <AuthMessage message={infoMessage} variant="info" />
      ) : null}

      {errorMessage ? (
        <AuthMessage message={errorMessage} variant="error" />
      ) : null}

      <Button type="submit" className="w-full" disabled={isSubmitting}>
        {isSubmitting ? (
          <>
            <LoaderCircle className="size-4 animate-spin" />
            {isVerificationStep
              ? "Подтверждаем email..."
              : "Создаем аккаунт..."}
          </>
        ) : (
          <>
            <UserPlus className="size-4" />
            {isVerificationStep ? "Подтвердить email" : "Зарегистрироваться"}
          </>
        )}
      </Button>
    </form>

    <AuthFooterLink
      promptText="Уже есть аккаунт?"
      linkText="Войти"
      to={loginPath}
    />
  </>
);
