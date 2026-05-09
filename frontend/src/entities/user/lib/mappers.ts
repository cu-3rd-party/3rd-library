import { LibraryUserPublicProfile, RealWorldProfile } from "../api";
import { User } from "../model/types";

export const mapLibraryUserToUser = (user: LibraryUserPublicProfile): User => ({
  id: user.id,
  name: user.name,
  bio: user.bio || "",
  image: user.image || null,
  verified: user.is_email_verified,
  isEmailVerified: user.is_email_verified,
  materialsCount: user.materials_count,
});

export const mapProfileToUser = (profile: RealWorldProfile): User => ({
  id: profile.username,
  name: profile.username,
  bio: profile.bio || "",
  image: profile.image,
  verified: true,
  isEmailVerified: true,
});
