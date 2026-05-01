import { MaterialSubmissionFile } from "@/models";

const formatBytes = (value: number) => `${(value / 1024 / 1024).toFixed(2)} MB`;

const tryOpenUrl = (url: string) => {
  try {
    window.open(url, "_blank", "noopener,noreferrer");
    return true;
  } catch {
    return false;
  }
};

export const openMaterialFile = (file: MaterialSubmissionFile) => {
  if (file.url && tryOpenUrl(file.url)) {
    return;
  }

  const fallbackText = [
    `Название: ${file.name}`,
    `Тип: ${(file.extension || "unknown").toUpperCase()}`,
    `Размер: ${formatBytes(file.sizeBytes)}`,
  ].join("\n");

  const fallbackUrl = URL.createObjectURL(
    new Blob([fallbackText], {
      type: "text/plain;charset=utf-8",
    }),
  );

  tryOpenUrl(fallbackUrl);
  setTimeout(() => URL.revokeObjectURL(fallbackUrl), 1000);
};
