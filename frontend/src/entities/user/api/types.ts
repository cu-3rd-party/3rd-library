import { LibraryPaginatedMaterialsResponse } from "@/entities/material/api";

export type RealWorldProfile = {
  username: string;
  bio: string;
  image: string | null;
  following: boolean;
};

export type RealWorldProfileResponse = {
  profile: RealWorldProfile;
};

export type LibraryUserPublicProfile = {
  id: string;
  name: string;
  bio: string;
  image?: string | null;
  is_email_verified: boolean;
  materials_count: number;
};

export type LibraryUsersResponse = {
  items: LibraryUserPublicProfile[];
  page: number;
  limit: number;
  total: number;
};

export type LibraryCurrentUserResponse = {
  id: string;
  name: string;
  email: string;
  bio: string;
  image?: string | null;
  is_email_verified: boolean;
  can_submit_materials: boolean;
  roles: string[];
};

export type LibraryUserWithMaterialsResponse = {
  user: LibraryUserPublicProfile;
  materials: LibraryPaginatedMaterialsResponse;
};
