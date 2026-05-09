import { AuthUserResponse } from "@/entities/session/api";

export type AuthUserPayload = {
  email: string;
  password: string;
};

export type RegisterUserPayload = AuthUserPayload & {
  name: string;
  surname: string;
};

export type RegisterUserResult =
  | {
      status: "authorized";
      response: AuthUserResponse;
    }
  | {
      status: "verification_required";
      email: string;
      channel?: string | null;
    };
