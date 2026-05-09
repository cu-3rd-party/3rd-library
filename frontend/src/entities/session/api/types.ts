import { AuthUser } from "../model";

export type AuthUserResponse = {
  user: AuthUser;
};

export type LegacyAuthUserResponse = AuthUserResponse;

export type NewAuthResponse = {
  user: {
    id: string;
    name: string;
    email: string;
    bio: string;
    image?: string | null;
    is_email_verified: boolean;
    can_submit_materials: boolean;
    roles: string[];
  };
  tokens: {
    access_token: string;
    refresh_token: string;
    expires_in: number;
  };
};

export type NewRegisterResponse = {
  user: {
    id: string;
    name: string;
    email: string;
    bio: string;
    image?: string | null;
    is_email_verified: boolean;
    can_submit_materials: boolean;
    roles: string[];
  };
  verification_required?: boolean;
  verification_channel?: string | null;
};

export type NewCurrentUserResponse = {
  id: string;
  name: string;
  email: string;
  bio: string;
  image?: string | null;
  is_email_verified: boolean;
  can_submit_materials: boolean;
  roles: string[];
};

export type AuthApiError = {
  errors?: Record<string, string[]>;
};
