import { LoaderCircle, UserPlus } from "lucide-react";
import { FormEvent, useEffect, useState } from "react";
import { Link, useNavigate } from "react-router-dom";

import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { AUTHORIZATION_PREFIX, MATERIALS_PREFIX } from "@/constants";
import { registerUser } from "@/lib/authApi";
import { registerSchema } from "@/lib/authValidation";
import { persistCurrentAuthUser } from "@/lib/currentUser";

type RegisterFieldErrors = {
  username?: string;
  email?: string;
  password?: string;
};

const RegisterPage = () => {
  const navigate = useNavigate();
  const [username, setUsername] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [fieldErrors, setFieldErrors] = useState<RegisterFieldErrors>({});
  const [errorMessage, setErrorMessage] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);

  useEffect(() => {
    const token = globalThis.localStorage?.getItem("accessToken");
    if (token) {
      navigate(MATERIALS_PREFIX, { replace: true });
    }
  }, [navigate]);

  const handleSubmit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();

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
      setErrorMessage("");
      return;
    }

    setFieldErrors({});
    setErrorMessage("");
    setIsSubmitting(true);

    try {
      const response = await registerUser({
        username: validationResult.data.username,
        email: validationResult.data.email,
        password: validationResult.data.password,
      });

      globalThis.localStorage?.setItem(
        "accessToken",
        `Token ${response.user.token}`,
      );
      persistCurrentAuthUser({
        email: response.user.email,
        username: response.user.username,
        bio: response.user.bio || "",
        image: response.user.image,
      });
      navigate(MATERIALS_PREFIX, { replace: true });
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
                  disabled={isSubmitting}
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
                  onChange={(event) => setEmail(event.target.value)}
                  disabled={isSubmitting}
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
                <Input
                  id="register-password"
                  type="password"
                  autoComplete="new-password"
                  placeholder="Минимум 8 символов"
                  value={password}
                  onChange={(event) => setPassword(event.target.value)}
                  disabled={isSubmitting}
                  aria-invalid={Boolean(fieldErrors.password)}
                />
                {fieldErrors.password ? (
                  <p className="text-sm text-destructive">
                    {fieldErrors.password}
                  </p>
                ) : null}
              </div>

              {errorMessage ? (
                <p className="rounded-md border border-destructive/30 bg-destructive/10 px-3 py-2 text-sm text-destructive">
                  {errorMessage}
                </p>
              ) : null}

              <Button type="submit" className="w-full" disabled={isSubmitting}>
                {isSubmitting ? (
                  <>
                    <LoaderCircle className="size-4 animate-spin" />
                    Создаем аккаунт...
                  </>
                ) : (
                  <>
                    <UserPlus className="size-4" />
                    Зарегистрироваться
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
