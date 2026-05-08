import { CircleHelp } from "lucide-react";
import { FormEvent, useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";

import { AuthPageShell, RegisterAuthForm } from "@/common/organisms";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { AUTHORIZATION_PREFIX, MATERIALS_PREFIX } from "@/constants";
import {
  registerUser,
  resendVerificationCode,
  verifyEmailCode,
} from "@/lib/authApi";
import { getAccessToken, setAccessToken } from "@/lib/authToken";
import { registerSchema } from "@/lib/authValidation";
import { persistCurrentAuthUser } from "@/lib/currentUser";

type RegisterFieldErrors = {
  name?: string;
  surname?: string;
  email?: string;
  password?: string;
  verificationCode?: string;
};

const RESEND_COOLDOWN_SECONDS = 60;

const RegisterPage = () => {
  const navigate = useNavigate();
  const [name, setName] = useState("");
  const [surname, setSurname] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [verificationCode, setVerificationCode] = useState("");
  const [verificationEmail, setVerificationEmail] = useState("");
  const [isVerificationStep, setIsVerificationStep] = useState(false);
  const [fieldErrors, setFieldErrors] = useState<RegisterFieldErrors>({});
  const [infoMessage, setInfoMessage] = useState("");
  const [errorMessage, setErrorMessage] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isResendingCode, setIsResendingCode] = useState(false);
  const [resendCooldown, setResendCooldown] = useState(0);

  useEffect(() => {
    const token = getAccessToken();
    if (token) {
      navigate(MATERIALS_PREFIX, { replace: true });
    }
  }, [navigate]);

  useEffect(() => {
    if (resendCooldown <= 0) return;

    const timer = globalThis.setTimeout(() => {
      setResendCooldown((seconds) => Math.max(seconds - 1, 0));
    }, 1000);

    return () => globalThis.clearTimeout(timer);
  }, [resendCooldown]);

  const formatCooldown = (seconds: number) => {
    const mm = String(Math.floor(seconds / 60)).padStart(2, "0");
    const ss = String(seconds % 60).padStart(2, "0");
    return `${mm}:${ss}`;
  };

const completeAuth = (
    token: string,
    user: { email: string; username: string; bio: string; image: string | null; roles?: string[] },
    refreshToken?: string,
  ) => {
    globalThis.localStorage?.setItem("accessToken", `Token ${token}`);
    if (refreshToken) {
      globalThis.localStorage?.setItem("refreshToken", refreshToken);
    }
    persistCurrentAuthUser({
      email: user.email,
      username: user.username,
      bio: user.bio || "",
      image: user.image,
      roles: user.roles || ["user"],
    });
    navigate(MATERIALS_PREFIX, { replace: true });
  };

  const handleSubmit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    if (isVerificationStep) {
      const nextCode = verificationCode.trim();
      if (!nextCode) {
        setFieldErrors((current) => ({
          ...current,
          verificationCode: "Введите код из письма.",
        }));
        return;
      }

      setFieldErrors({});
      setErrorMessage("");
      setIsSubmitting(true);

      try {
        const response = await verifyEmailCode({
          email: verificationEmail || email.trim(),
          code: nextCode,
        });

        completeAuth(response.tokens.accessToken, response.user, response.tokens.refreshToken);
      } catch (error) {
        console.error(error);
        setErrorMessage(
          error instanceof Error
            ? error.message
            : "Не удалось подтвердить email",
        );
      } finally {
        setIsSubmitting(false);
      }

      return;
    }

    const validationResult = registerSchema.safeParse({
      name,
      surname,
      email,
      password,
    });
    if (!validationResult.success) {
      const flattenedErrors = validationResult.error.flatten().fieldErrors;
      setFieldErrors({
        name: flattenedErrors.name?.[0],
        surname: flattenedErrors.surname?.[0],
        email: flattenedErrors.email?.[0],
        password: flattenedErrors.password?.[0],
      });
      setInfoMessage("");
      setErrorMessage("");
      return;
    }

    setFieldErrors({});
    setInfoMessage("");
    setErrorMessage("");
    setIsSubmitting(true);

    try {
      const result = await registerUser({
        name: validationResult.data.name,
        surname: validationResult.data.surname,
        email: validationResult.data.email,
        password: validationResult.data.password,
      });

      if (result.status === "authorized") {
        completeAuth(result.response.user.token, result.response.user);
        return;
      }

      const formattedChannel =
        result.channel === "email_code" ? "по email" : "";
      setIsVerificationStep(true);
      setVerificationEmail(result.email);
      setVerificationCode("");
      setResendCooldown(RESEND_COOLDOWN_SECONDS);
      setInfoMessage(
        `Код подтверждения отправлен ${formattedChannel} на ${result.email}. Введите его в поле ниже.`,
      );
    } catch (error) {
      console.error(error);
      setErrorMessage(
        error instanceof Error
          ? error.message
          : "Не удалось выполнить регистрацию",
      );
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleResendCode = async () => {
    if (resendCooldown > 0) {
      setErrorMessage(
        `Подождите ${formatCooldown(resendCooldown)} перед повторной отправкой кода.`,
      );
      return;
    }

    const targetEmail = (verificationEmail || email).trim();
    if (!targetEmail) {
      setErrorMessage(
        "Не удалось определить email для повторной отправки кода.",
      );
      return;
    }

    setErrorMessage("");
    setIsResendingCode(true);

    try {
      await resendVerificationCode({ email: targetEmail });
      setResendCooldown(RESEND_COOLDOWN_SECONDS);
      setInfoMessage(
        `Новый код отправлен на ${targetEmail}. Если письма нет, проверьте папку «Спам».`,
      );
    } catch (error) {
      const isRateLimited =
        error instanceof Error &&
        /too many requests|слишком много/i.test(error.message);
      if (isRateLimited) {
        setResendCooldown((seconds) =>
          seconds > 0 ? seconds : RESEND_COOLDOWN_SECONDS,
        );
        setErrorMessage(
          "Код уже отправлялся недавно. Повторить отправку можно через минуту.",
        );
      } else {
        console.error(error);
        const fallbackMessage =
          "Не удалось отправить код повторно. Попробуйте снова чуть позже.";
        setErrorMessage(
          error instanceof Error
            ? error.message || fallbackMessage
            : fallbackMessage,
        );
      }
    } finally {
      setIsResendingCode(false);
    }
  };

  return (
    <AuthPageShell
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
        onEmailChange={(value) => {
          setEmail(value);
          if (!isVerificationStep) return;

          setVerificationEmail(value.trim());
        }}
        onPasswordChange={setPassword}
        onVerificationCodeChange={setVerificationCode}
        onResendCode={handleResendCode}
        onSubmit={handleSubmit}
        formatCooldown={formatCooldown}
      />
    </AuthPageShell>
  );
};

export default RegisterPage;
