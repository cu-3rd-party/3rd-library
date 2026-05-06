import { z } from "zod";

const EDU_EMAIL_DOMAIN = "@edu.centraluniversity.ru";

const eduEmailSchema = z
  .string()
  .trim()
  .min(1, "Введите email")
  .email("Введите корректный email")
  .refine((email) => email.toLowerCase().endsWith(EDU_EMAIL_DOMAIN), {
    message: `Почта должна заканчиваться на ${EDU_EMAIL_DOMAIN}`,
  });

export const loginSchema = z.object({
  email: eduEmailSchema,
  password: z.string().min(1, "Введите пароль"),
});

export const registerSchema = z.object({
  username: z.string().trim().min(1, "Введите имя пользователя"),
  email: eduEmailSchema,
  password: z.string().min(8, "Пароль должен содержать минимум 8 символов"),
});
