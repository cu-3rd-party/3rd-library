import { z } from "zod";

const EDU_EMAIL_DOMAIN = "@edu.centraluniversity.ru";

export const eduEmailSchema = z
  .string()
  .trim()
  .min(1, "Введите email")
  .email("Введите корректный email")
  .refine((email) => email.toLowerCase().endsWith(EDU_EMAIL_DOMAIN), {
    message: `Почта должна заканчиваться на ${EDU_EMAIL_DOMAIN}`,
  });
