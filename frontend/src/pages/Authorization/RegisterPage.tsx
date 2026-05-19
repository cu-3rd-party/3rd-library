import { CircleHelp } from "lucide-react";

import { RegisterAuthForm, useRegister } from "@/features/auth";
import { AUTHORIZATION_PREFIX } from "@/shared/constants";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/shared/ui";

import { AuthLayout } from "./AuthLayout";

const RegisterPage = () => {
  const {
    name, surname, email, password, verificationCode,
    isVerificationStep, fieldErrors, infoMessage, errorMessage,
    isSubmitting, isResendingCode, resendCooldown,
    setName, setSurname, setPassword, setVerificationCode,
    handleEmailChange, handleSubmit, handleResendCode, formatCooldown,
  } = useRegister();

  return (
    <AuthLayout
      title="Регистрация"
      description="Создайте аккаунт, чтобы публиковать материалы и работать с профилем."
      headerRight={
        <TooltipProvider delayDuration={200}>
          <Tooltip>
            <TooltipTrigger asChild>
              <button
                type="button"
                aria-label="Зачем нужны студенческая почта и настоящее имя"
                className="text-muted-foreground/70 hover:text-foreground inline-flex h-8 w-8 items-center justify-center rounded-md transition-colors"
              >
                <CircleHelp className="size-4" />
              </button>
            </TooltipTrigger>
            <TooltipContent side="left" sideOffset={8} className="max-w-64">
              Для регистрации требуется указать настоящее имя и фамилию, а также
              email в домене @edu.centraluniversity.ru. Это необходимо для
              подтверждения вашего статуса студента Центрального Университета и
              обеспечения доступа к материалам, предназначенным только для
              студентов.
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>
      }
    >
      <RegisterAuthForm
        name={name}
        surname={surname}
        email={email}
        password={password}
        verificationCode={verificationCode}
        isVerificationStep={isVerificationStep}
        fieldErrors={fieldErrors}
        infoMessage={infoMessage}
        errorMessage={errorMessage}
        isSubmitting={isSubmitting}
        isResendingCode={isResendingCode}
        resendCooldown={resendCooldown}
        loginPath={`${AUTHORIZATION_PREFIX}/login`}
        onNameChange={setName}
        onSurnameChange={setSurname}
        onEmailChange={handleEmailChange}
        onPasswordChange={setPassword}
        onVerificationCodeChange={setVerificationCode}
        onResendCode={handleResendCode}
        onSubmit={handleSubmit}
        formatCooldown={formatCooldown}
      />
    </AuthLayout>
  );
};

export default RegisterPage;
