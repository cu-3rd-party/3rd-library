import { FormEvent, useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";

import { AuthPageShell, LoginAuthForm } from "@/common/organisms";
import { AUTHORIZATION_PREFIX, MATERIALS_PREFIX } from "@/constants";
import { loginUser } from "@/lib/authApi";
import { getAccessToken, setAccessToken } from "@/lib/authToken";
import { loginSchema } from "@/lib/authValidation";
import { persistCurrentAuthUser } from "@/lib/currentUser";

type LoginFieldErrors = {
  email?: string;
  password?: string;
};

const LoginPage = () => {
  const navigate = useNavigate();
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [fieldErrors, setFieldErrors] = useState<LoginFieldErrors>({});
  const [errorMessage, setErrorMessage] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);

  useEffect(() => {
    const token = getAccessToken();
    if (token) {
      navigate(MATERIALS_PREFIX, { replace: true });
    }
  }, [navigate]);

  const handleSubmit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    const validationResult = loginSchema.safeParse({ email, password });
    if (!validationResult.success) {
      const flattenedErrors = validationResult.error.flatten().fieldErrors;
      setFieldErrors({
        email: flattenedErrors.email?.[0],
        password: flattenedErrors.password?.[0],
      });
      setErrorMessage("");
      return;
    }

    setFieldErrors({});
    setErrorMessage("");
    setIsSubmitting(true);

    try {
      const response = await loginUser({
        email: validationResult.data.email,
        password: validationResult.data.password,
      });

      setAccessToken(`Token ${response.user.token}`);
      persistCurrentAuthUser({
        email: response.user.email,
        username: response.user.username,
        bio: response.user.bio || "",
        image: response.user.image,
        roles: response.user.roles || ["user"],
      });
      navigate(MATERIALS_PREFIX, { replace: true });
    } catch (error) {
      console.error(error);
      setErrorMessage(
        error instanceof Error ? error.message : "Не удалось выполнить вход",
      );
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <AuthPageShell
      title="Вход"
      description="Войдите в аккаунт, чтобы работать с материалами и профилем."
    >
      <LoginAuthForm
        email={email}
        password={password}
        isSubmitting={isSubmitting}
        errorMessage={errorMessage}
        fieldErrors={fieldErrors}
        registerPath={`${AUTHORIZATION_PREFIX}/register`}
        onEmailChange={setEmail}
        onPasswordChange={setPassword}
        onSubmit={handleSubmit}
      />
    </AuthPageShell>
  );
};

export default LoginPage;
