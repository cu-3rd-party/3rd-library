import { ApiRequestError } from "@/shared/api";

export const getModerationLoadErrorMessage = (error: unknown) => {
  if (error instanceof ApiRequestError) {
    if (error.status === 403) {
      return "Недостаточно прав для доступа к модерации.";
    }
    if (error.status === 500) {
      return "Ошибка на сервере.";
    }

    return `Не удалось загрузить модерацию (HTTP ${error.status}).`;
  }

  return "Не удалось загрузить модерацию.";
};
