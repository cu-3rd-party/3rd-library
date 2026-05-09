import { requestAuth } from "@/entities/session/api";

export const resendVerificationCode = async (payload: { email: string }) => {
  await requestAuth<null>("/api/auth/resend-verification-code", {
    email: payload.email,
  });
};
