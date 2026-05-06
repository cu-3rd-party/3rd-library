import { Eye, EyeOff, LoaderCircle, UserPlus } from "lucide-react";
import { FormEvent, useEffect, useState } from "react";
import { Link, useNavigate } from "react-router-dom";

import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { AUTHORIZATION_PREFIX, MATERIALS_PREFIX } from "@/constants";
import {
  registerUser,
  resendVerificationCode,
  verifyEmailCode,
} from "@/lib/authApi";
import { registerSchema } from "@/lib/authValidation";
import { persistCurrentAuthUser } from "@/lib/currentUser";

type RegisterFieldErrors = {
  username?: string;
  email?: string;
  password?: string;
  verificationCode?: string;
};

const RESEND_COOLDOWN_SECONDS = 60;

const RegisterPage = () => {
  const navigate = useNavigate();
  const [username, setUsername] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [isPasswordVisible, setIsPasswordVisible] = useState(false);
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
    const token = globalThis.localStorage?.getItem("accessToken");
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

  const completeAuth = (token: string, user: { email: string; username: string; bio: string; image: string | null; roles?: string[] }) => {
    globalThis.localStorage?.setItem("accessToken", `Token ${token}`);
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

        completeAuth(response.user.token, response.user);
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
      username,
      email,
      password,
    });
    if (!validationResult.success) {
      const flattenedErrors = validationResult.error.flatten().fieldErrors;
      setFieldErrors({
        username: flattenedErrors.username?.[0],
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
        username: validationResult.data.username,
        email: validationResult.data.email,
        password: validationResult.data.password,
      });

      if (result.status === "authorized") {
        completeAuth(result.response.user.token, result.response.user);
        return;
      }

      const formattedChannel = result.channel === "email_code" ? "по email" : "";
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
      setErrorMessage("Не удалось определить email для повторной отправки кода.");
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
        error instanceof Error && /too many requests/i.test(error.message);
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
          error instanceof Error ? error.message || fallbackMessage : fallbackMessage,
        );
      }
    } finally {
      setIsResendingCode(false);
    }
  };

  return (
    <div className="relative min-h-screen overflow-hidden bg-background px-4 py-10">
      <div className="pointer-events-none absolute inset-0">
        <div className="absolute -left-24 top-0 h-72 w-72 rounded-full bg-primary/20 blur-3xl" />
        <div className="absolute -right-24 bottom-0 h-72 w-72 rounded-full bg-secondary/30 blur-3xl" />
      </div>

      <div className="relative mx-auto flex min-h-[calc(100vh-5rem)] w-full max-w-md items-center">
        <Card className="w-full border-border bg-card/95 backdrop-blur">
          <CardHeader className="space-y-2">
            <CardTitle className="text-2xl font-bold tracking-tight">
              Регистрация
            </CardTitle>
            <p className="text-sm text-muted-foreground">
              Создайте аккаунт, чтобы публиковать материалы и работать с
              профилем.
            </p>
          </CardHeader>
          <CardContent className="space-y-5">
            <form className="space-y-4" onSubmit={handleSubmit}>
              <div className="space-y-2">
                <Label htmlFor="register-username">Имя пользователя</Label>
                <Input
                  id="register-username"
                  type="text"
                  autoComplete="username"
                  placeholder="например, daniil"
                  value={username}
                  onChange={(event) => setUsername(event.target.value)}
                  disabled={isSubmitting || isVerificationStep}
                  aria-invalid={Boolean(fieldErrors.username)}
                />
                {fieldErrors.username ? (
                  <p className="text-sm text-destructive">
                    {fieldErrors.username}
                  </p>
                ) : null}
              </div>

              <div className="space-y-2">
                <Label htmlFor="register-email">Email</Label>
                <Input
                  id="register-email"
                  type="email"
                  autoComplete="email"
                  placeholder="name@example.com"
                  value={email}
                  onChange={(event) => {
                    setEmail(event.target.value);
                    if (!isVerificationStep) return;

                    setVerificationEmail(event.target.value.trim());
                  }}
                  disabled={isSubmitting || isVerificationStep}
                  aria-invalid={Boolean(fieldErrors.email)}
                />
                {fieldErrors.email ? (
                  <p className="text-sm text-destructive">
                    {fieldErrors.email}
                  </p>
                ) : null}
              </div>

              <div className="space-y-2">
                <Label htmlFor="register-password">Пароль</Label>
                <div className="relative">
                  <Input
                    id="register-password"
                    type={isPasswordVisible ? "text" : "password"}
                    autoComplete="new-password"
                    placeholder="Минимум 8 символов"
                    value={password}
                    onChange={(event) => setPassword(event.target.value)}
                    disabled={isSubmitting || isVerificationStep}
                    aria-invalid={Boolean(fieldErrors.password)}
                    className="pr-10"
                  />
                  <button
                    type="button"
                    className="text-muted-foreground hover:text-foreground absolute inset-y-0 right-0 inline-flex items-center px-3 transition-colors disabled:cursor-not-allowed disabled:opacity-50"
                    onClick={() => setIsPasswordVisible((current) => !current)}
                    disabled={isSubmitting || isVerificationStep}
                    aria-label={
                      isPasswordVisible ? "Скрыть пароль" : "Показать пароль"
                    }
                  >
                    {isPasswordVisible ? (
                      <EyeOff className="size-4" aria-hidden="true" />
                    ) : (
                      <Eye className="size-4" aria-hidden="true" />
                    )}
                  </button>
                </div>
                {fieldErrors.password ? (
                  <p className="text-sm text-destructive">
                    {fieldErrors.password}
                  </p>
                ) : null}
              </div>

              {isVerificationStep ? (
                <div className="space-y-2">
                  <Label htmlFor="register-verification-code">
                    Код из письма
                  </Label>
                  <Input
                    id="register-verification-code"
                    type="text"
                    autoComplete="one-time-code"
                    inputMode="numeric"
                    placeholder="Введите код подтверждения"
                    value={verificationCode}
                    onChange={(event) => setVerificationCode(event.target.value)}
                    disabled={isSubmitting}
                    aria-invalid={Boolean(fieldErrors.verificationCode)}
                  />
                  {fieldErrors.verificationCode ? (
                    <p className="text-sm text-destructive">
                      {fieldErrors.verificationCode}
                    </p>
                  ) : null}
                  <Button
                    type="button"
                    variant="outline"
                    className="w-full"
                    disabled={
                      isSubmitting || isResendingCode || resendCooldown > 0
                    }
                    onClick={handleResendCode}
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
                <p className="rounded-md border border-primary/20 bg-primary/10 px-3 py-2 text-sm text-foreground">
                  {infoMessage}
                </p>
              ) : null}

              {errorMessage ? (
                <p className="rounded-md border border-destructive/30 bg-destructive/10 px-3 py-2 text-sm text-destructive">
                  {errorMessage}
                </p>
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
                    {isVerificationStep
                      ? "Подтвердить email"
                      : "Зарегистрироваться"}
                  </>
                )}
              </Button>
            </form>

            <p className="text-center text-sm text-muted-foreground">
              Уже есть аккаунт?{" "}
              <Link
                className="font-medium text-foreground underline-offset-4 hover:underline"
                to={`${AUTHORIZATION_PREFIX}/login`}
              >
                Войти
              </Link>
            </p>
          </CardContent>
        </Card>
      </div>
    </div>
  );
};

export default RegisterPage;
