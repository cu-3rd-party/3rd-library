import { fetchWithAuth } from "@/entities/session/api";
import { resolveApiUrl } from "@/shared/api";

type PfpResponse = { pfp: { pfpId: string } };

export const uploadProfilePicture = async (file: File): Promise<string> => {
  const formData = new FormData();
  formData.append("file", file);

  const response = await fetchWithAuth(resolveApiUrl("/api/user/pfp"), {
    method: "POST",
    body: formData,
  });

  if (!response.ok) {
    throw new Error(`Не удалось загрузить аватар: ${response.status}`);
  }

  const data = (await response.json()) as PfpResponse;
  return data.pfp.pfpId;
};
