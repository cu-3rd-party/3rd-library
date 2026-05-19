import { useEffect, useState } from "react";

import {
  getCurrentAuthUser,
  persistCurrentAuthUser,
} from "@/entities/session/lib";
import { StoredAuthUser } from "@/entities/session/model";

import { fetchUserProfile } from "../api";
import { UserProfileResponse } from "../model";
import {
  decodeRouteUserId,
  getFallbackProfile,
  isSameAuthUser,
  resolvePfpUrl,
} from "./helpers";

export const useProfile = (routeId: string | undefined) => {
  const routeUserId = decodeRouteUserId(routeId);

  const [imageError, setImageError] = useState(false);
  const [profile, setProfile] = useState<UserProfileResponse | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [isError, setIsError] = useState(false);
  const [currentAuthUser, setCurrentAuthUser] = useState<StoredAuthUser | null>(
    () => getCurrentAuthUser(),
  );

  useEffect(() => {
    if (!routeUserId) {
      setIsLoading(false);
      setIsError(true);
      return;
    }

    const ctrl = new AbortController();

    fetchUserProfile(routeUserId, ctrl.signal, currentAuthUser)
      .then(({ profile: nextProfile, resolvedEmail }) => {
        if (ctrl.signal.aborted) return;

        const currentUsername = currentAuthUser?.username || "";
        const isCurrentUserProfile =
          currentUsername.length > 0 &&
          (routeUserId === currentUsername ||
            nextProfile.user.name === currentUsername ||
            nextProfile.user.id === currentUsername);

        if (isCurrentUserProfile) {
          const synced: StoredAuthUser = {
            email: resolvedEmail,
            username: nextProfile.user.name,
            bio: nextProfile.user.bio,
            image: nextProfile.user.image || null,
            roles: currentAuthUser?.roles || ["user"],
          };
          if (!isSameAuthUser(currentAuthUser, synced)) {
            persistCurrentAuthUser(synced);
            setCurrentAuthUser(synced);
          }
        }

        setProfile(nextProfile);
      })
      .catch((error: unknown) => {
        if (ctrl.signal.aborted) return;

        if (import.meta.env.VITE_API === "mock") {
          const fallback = getFallbackProfile(routeUserId);
          if (fallback) {
            console.warn("[Profile] Falling back to local mock profile.");
            setProfile(fallback);
            return;
          }
          if (currentAuthUser?.username === routeUserId) {
            setProfile({
              user: {
                id: currentAuthUser.username,
                name: currentAuthUser.username,
                bio: currentAuthUser.bio,
                image: currentAuthUser.image,
                isEmailVerified: true,
                verified: true,
              },
              materials: { items: [], page: 1, limit: 20, total: 0 },
            });
            return;
          }
        }

        console.error(error);
        setIsError(true);
      })
      .finally(() => {
        if (!ctrl.signal.aborted) setIsLoading(false);
      });

    return () => ctrl.abort();
  }, [currentAuthUser, routeUserId]);

  useEffect(() => {
    setImageError(false);
  }, [profile?.user.id, profile?.user.image]);

  const isOwnProfile =
    Boolean(currentAuthUser?.username) &&
    (routeUserId === currentAuthUser?.username ||
      profile?.user.id === currentAuthUser?.username);

  return {
    profile,
    setProfile,
    isLoading,
    isError,
    imageError,
    setImageError,
    isOwnProfile,
    avatarSrc: profile ? resolvePfpUrl(profile.user.image) : null,
    currentAuthUser,
    setCurrentAuthUser,
  };
};
