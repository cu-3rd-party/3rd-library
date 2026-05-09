export const AUTH_ERROR_MESSAGES = {
  SERVER_UNAVAILABLE: "Сервер недоступен. Попробуйте позже.",
  INVALID_CREDENTIALS: "Неверный email или пароль.",
  EMAIL_ALREADY_REGISTERED: "Пользователь с таким email уже зарегистрирован.",
  USERNAME_ALREADY_REGISTERED:
    "Пользователь с таким именем уже зарегистрирован.",
  INVALID_VERIFICATION_CODE: "Неверный код подтверждения.",
  EXPIRED_VERIFICATION_CODE: "Срок действия кода истек. Запросите новый код.",
  TOO_MANY_REQUESTS: "Слишком много попыток. Попробуйте чуть позже.",
  FILL_REQUIRED_FIELDS: "Заполните обязательные поля.",
} as const;
