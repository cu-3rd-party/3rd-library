import { NewAuthResponse, requestAuth } from "@/entities/session/api";
import { mapNewAuthResponse } from "@/entities/session/lib";

export const verifyEmailCode = async (payload: {
  email: string;
  code: string;
}) => {
  const response = await requestAuth<NewAuthResponse>(
    "/api/auth/verify-email",
    {
      email: payload.email,
      code: payload.code,
    },
  );

  return mapNewAuthResponse(response);
};
