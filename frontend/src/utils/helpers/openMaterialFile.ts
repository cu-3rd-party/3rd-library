import { fetchWithAuth } from "@/lib/api";
import { MaterialSubmissionFile } from "@/models";

const BLOB_URL_REVOKE_DELAY_MS = 60_000;
const MAX_DOWNLOAD_URL_DEPTH = 3;
const OPEN_TARGET = "_blank";
const OPEN_WINDOW_FEATURES = "noopener,noreferrer";

const formatBytes = (value: number) => `${(value / 1024 / 1024).toFixed(2)} MB`;

const extractExtensionFromName = (fileName: string) => {
  const fileNameParts = fileName.split(".");
  return fileNameParts.length > 1
    ? (fileNameParts[fileNameParts.length - 1] || "").toLowerCase()
    : "";
};

const isPdfFile = (file: MaterialSubmissionFile) => {
  const extension = (
    file.extension || extractExtensionFromName(file.name)
  ).toLowerCase();
  return extension === "pdf";
};

const resolveUrl = (url: string) => {
  try {
    return new URL(url, globalThis.location.origin).toString();
  } catch {
    return null;
  }
};

const tryOpenUrl = (url: string) => {
  try {
    window.open(url, OPEN_TARGET, OPEN_WINDOW_FEATURES);
    return true;
  } catch {
    return false;
  }
};

const tryDownloadBlob = (blob: Blob, fileName: string) => {
  try {
    const blobUrl = URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.href = blobUrl;
    link.download = fileName;
    link.rel = "noopener noreferrer";
    document.body.appendChild(link);
    link.click();
    link.remove();
    setTimeout(() => URL.revokeObjectURL(blobUrl), BLOB_URL_REVOKE_DELAY_MS);
    return true;
  } catch {
    return false;
  }
};

const tryOpenPdfBlob = (blob: Blob) => {
  try {
    const pdfBlob = blob.type.toLowerCase().includes("pdf")
      ? blob
      : new Blob([blob], { type: "application/pdf" });
    const blobUrl = URL.createObjectURL(pdfBlob);
    const opened = tryOpenUrl(blobUrl);
    if (!opened) {
      URL.revokeObjectURL(blobUrl);
      return false;
    }
    setTimeout(() => URL.revokeObjectURL(blobUrl), BLOB_URL_REVOKE_DELAY_MS);
    return true;
  } catch {
    return false;
  }
};

const toDownloadUrlFromPayload = (payload: unknown) => {
  if (!payload || typeof payload !== "object") return null;

  const candidateUrls = [
    (payload as { url?: unknown }).url,
    (payload as { downloadUrl?: unknown }).downloadUrl,
    (payload as { fileUrl?: unknown }).fileUrl,
    (payload as { attachment?: { url?: unknown } }).attachment?.url,
    (payload as { attachment?: { downloadUrl?: unknown } }).attachment
      ?.downloadUrl,
    (payload as { file?: { url?: unknown } }).file?.url,
  ];

  const nextUrl = candidateUrls.find(
    (candidate): candidate is string =>
      typeof candidate === "string" && candidate.trim().length > 0,
  );

  return nextUrl ? resolveUrl(nextUrl) : null;
};

const fetchRemoteFileBlob = async (
  sourceUrl: string,
  depth = 0,
): Promise<Blob | null> => {
  const resolvedUrl = resolveUrl(sourceUrl);
  if (!resolvedUrl) return null;

  const response = await fetchWithAuth(resolvedUrl, {
    method: "GET",
    cache: "no-store",
  });

  if (!response.ok) return null;

  const contentType = (
    response.headers.get("content-type") || ""
  ).toLowerCase();
  if (
    contentType.includes("application/json") &&
    depth < MAX_DOWNLOAD_URL_DEPTH
  ) {
    const payload = (await response.json()) as unknown;
    const nestedDownloadUrl = toDownloadUrlFromPayload(payload);
    if (!nestedDownloadUrl) return null;
    return fetchRemoteFileBlob(nestedDownloadUrl, depth + 1);
  }

  return response.blob();
};

const openMaterialFileFallback = (file: MaterialSubmissionFile) => {
  if (file.url) {
    const resolvedUrl = resolveUrl(file.url);
    if (resolvedUrl) {
      if (isPdfFile(file)) {
        if (tryOpenUrl(resolvedUrl)) return;
      } else {
        const anchor = document.createElement("a");
        anchor.href = resolvedUrl;
        anchor.download = file.name;
        anchor.rel = "noopener noreferrer";
        document.body.appendChild(anchor);
        anchor.click();
        anchor.remove();
        return;
      }
    }
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

export const openMaterialFile = (file: MaterialSubmissionFile) => {
  if (!file.url) {
    openMaterialFileFallback(file);
    return;
  }
  const sourceUrl = file.url;

  void (async () => {
    try {
      const blob = await fetchRemoteFileBlob(sourceUrl);
      if (!blob || blob.size === 0) {
        openMaterialFileFallback(file);
        return;
      }

      if (isPdfFile(file)) {
        if (!tryOpenPdfBlob(blob)) {
          openMaterialFileFallback(file);
        }
        return;
      }

      if (!tryDownloadBlob(blob, file.name)) {
        openMaterialFileFallback(file);
      }
    } catch {
      openMaterialFileFallback(file);
    }
  })();
};
