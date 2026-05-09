import z from "zod";

import { eduEmailSchema } from "./eduEmailSchema";

export const loginSchema = z.object({
  email: eduEmailSchema,
  password: z.string().min(1, "Введите пароль"),
});
