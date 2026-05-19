import { FormEvent, useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";

import { getAccessToken } from "@/entities/session/lib";
import { MATERIALS_PREFIX } from "@/shared/constants";

import {
  registerUser,
  resendVerificationCode,
  verifyEmailCode,
} from "../api";
import { registerSchema } from "../model";
import {
  completeAuthAndNavigate,
  formatCooldown,
  RegisterFieldErrors,
  RESEND_COOLDOWN_SECONDS,
} from "./helpers";

export const useRegister = () => {
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
    if (getAccessToken()) navigate(MATERIALS_PREFIX, { replace: true });
  }, [navigate]);

  useEffect(() => {
    if (resendCooldown <= 0) return;
    const timer = globalThis.setTimeout(
      () => setResendCooldown((s) => Math.max(s - 1, 0)),
      1000,
    );
    return () => globalThis.clearTimeout(timer);
  }, [resendCooldown]);

  const handleEmailChange = (value: string) => {
    setEmail(value);
    if (isVerificationStep) setVerificationEmail(value.trim());
  };

  const handleSubmit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    if (isVerificationStep) {
      const code = verificationCode.trim();
      if (!code) {
        setFieldErrors((c) => ({ ...c, verificationCode: "Введите код из письма." }));
        return;
      }
      setFieldErrors({}); setErrorMessage(""); setIsSubmitting(true);
      try {
        const response = await verifyEmailCode({ email: verificationEmail || email.trim(), code });
        completeAuthAndNavigate(response.user.token, response.user, navigate);
      } catch (error) {
        console.error(error);
        setErrorMessage(error instanceof Error ? error.message : "Не удалось подтвердить email");
      } finally {
        setIsSubmitting(false);
      }
      return;
    }

    const result = registerSchema.safeParse({ name, surname, email, password });
    if (!result.success) {
      const fe = result.error.flatten().fieldErrors;
      setFieldErrors({ name: fe.name?.[0], surname: fe.surname?.[0], email: fe.email?.[0], password: fe.password?.[0] });
      setInfoMessage(""); setErrorMessage("");
      return;
    }

    setFieldErrors({}); setInfoMessage(""); setErrorMessage(""); setIsSubmitting(true);
    try {
      const reg = await registerUser(result.data);
      if (reg.status === "authorized") {
        completeAuthAndNavigate(reg.response.user.token, reg.response.user, navigate);
        return;
      }
      setIsVerificationStep(true);
      setVerificationEmail(reg.email);
      setVerificationCode("");
      setResendCooldown(RESEND_COOLDOWN_SECONDS);
      setInfoMessage(
        `Код подтверждения отправлен ${reg.channel === "email_code" ? "по email" : ""} на ${reg.email}. Введите его в поле ниже.`,
      );
    } catch (error) {
      console.error(error);
      setErrorMessage(error instanceof Error ? error.message : "Не удалось выполнить регистрацию");
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleResendCode = async () => {
    if (resendCooldown > 0) {
      setErrorMessage(`Подождите ${formatCooldown(resendCooldown)} перед повторной отправкой кода.`);
      return;
    }
    const targetEmail = (verificationEmail || email).trim();
    if (!targetEmail) { setErrorMessage("Не удалось определить email для повторной отправки кода."); return; }

    setErrorMessage(""); setIsResendingCode(true);
    try {
      await resendVerificationCode({ email: targetEmail });
      setResendCooldown(RESEND_COOLDOWN_SECONDS);
      setInfoMessage(`Новый код отправлен на ${targetEmail}. Если письма нет, проверьте папку «Спам».`);
    } catch (error) {
      const isRateLimited = error instanceof Error && /too many requests|слишком много/i.test(error.message);
      if (isRateLimited) {
        setResendCooldown((s) => (s > 0 ? s : RESEND_COOLDOWN_SECONDS));
        setErrorMessage("Код уже отправлялся недавно. Повторить отправку можно через минуту.");
      } else {
        console.error(error);
        const fallback = "Не удалось отправить код повторно. Попробуйте снова чуть позже.";
        setErrorMessage(error instanceof Error ? error.message || fallback : fallback);
      }
    } finally {
      setIsResendingCode(false);
    }
  };

  return {
    name, surname, email, password, verificationCode,
    isVerificationStep, fieldErrors, infoMessage, errorMessage,
    isSubmitting, isResendingCode, resendCooldown,
    setName, setSurname, setPassword, setVerificationCode,
    handleEmailChange, handleSubmit, handleResendCode, formatCooldown,
  };
};
