export type RawStoredAuthUser = {
  email?: unknown;
  username?: unknown;
  bio?: unknown;
  image?: unknown;
  roles?: unknown;
};

export type StoredAuthUser = {
  email: string;
  username: string;
  bio: string;
  image: string | null;
  roles: string[];
};

export type AuthUser = {
  email: string;
  token: string;
  username: string;
  bio: string;
  image: string | null;
  roles: string[];
};
