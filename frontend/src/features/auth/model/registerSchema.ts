import z from "zod";

import { eduEmailSchema } from "./eduEmailSchema";

export const registerSchema = z.object({
  name: z.string().trim().min(1, "Введите имя"),
  surname: z.string().trim().min(1, "Введите фамилию"),
  email: eduEmailSchema,
  password: z.string().min(8, "Пароль должен содержать минимум 8 символов"),
});
