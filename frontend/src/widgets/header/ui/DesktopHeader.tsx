
import { LogOut, User } from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import { useNavigate } from "react-router-dom";

import { fetchJsonWithAuth } from "@/entities/session/api";
import {
  clearCurrentSession,
  getCurrentAuthUser,
  persistCurrentAuthUser,
  resolveCurrentProfilePath,
  subscribeToCurrentAuthUser,
} from "@/entities/session/lib";
import { StoredAuthUser } from "@/entities/session/model";
import { ThemeToggleButton } from "@/entities/settings/ui";
import { ApiRequestError, resolveApiUrl } from "@/shared/api";
import { AUTHORIZATION_PREFIX } from "@/shared/constants";
import { isPathActive } from "@/shared/lib";
import {
  Avatar,
  AvatarFallback,
  AvatarImage,
  Button,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/shared/ui";

import { getAboutProjectItem } from "../model/navItems";

type CurrentUserResponse = {
  user: {
    email: string;
    username: string;
    bio: string;
    image: string | null;
  };
};

type LibraryCurrentUserResponse = {
  id: string;
  name: string;
  email: string;
  bio: string;
  image?: string | null;
  roles?: string[];
};

export const DesktopHeader = () => {
  const navigate = useNavigate();
  const [currentAuthUser, setCurrentAuthUser] = useState(() =>
    getCurrentAuthUser(),
  );
  const profilePath = useMemo(
    () =>
      currentAuthUser?.username
        ? `/authors/${encodeURIComponent(currentAuthUser.username)}`
        : resolveCurrentProfilePath(),
    [currentAuthUser?.username],
  );
  const headerAvatarFallback = currentAuthUser?.username
    ? currentAuthUser.username.slice(0, 2).toUpperCase()
    : "U";
  const handleLogout = () => {
    clearCurrentSession();
    navigate(`${AUTHORIZATION_PREFIX}/login`, { replace: true });
  };

  useEffect(() => {
    const unsubscribe = subscribeToCurrentAuthUser(setCurrentAuthUser);
    const abortController = new AbortController();

    const syncCurrentUser = async () => {
      try {
        let syncedUser: StoredAuthUser;
        try {
          const response = await fetchJsonWithAuth<LibraryCurrentUserResponse>(
            resolveApiUrl("/api/users/me"),
            {
              signal: abortController.signal,
            },
          );
          syncedUser = {
            email: response.email,
            username: response.name,
            bio: response.bio || "",
            image:
              response.image !== undefined
                ? response.image || null
                : getCurrentAuthUser()?.image || null,
            roles: response.roles || ["user"],
          };
        } catch (error) {
          if (!(error instanceof ApiRequestError) || error.status !== 404) {
            throw error;
          }

          const response = await fetchJsonWithAuth<CurrentUserResponse>(
            resolveApiUrl("/api/user"),
            {
              signal: abortController.signal,
            },
          );
          syncedUser = {
            email: response.user.email,
            username: response.user.username,
            bio: response.user.bio || "",
            image: response.user.image,
            roles: getCurrentAuthUser()?.roles || ["user"],
          };
        }
        const storedUser = getCurrentAuthUser();

        if (
          !storedUser ||
          storedUser.email !== syncedUser.email ||
          storedUser.username !== syncedUser.username ||
          storedUser.bio !== syncedUser.bio ||
          storedUser.image !== syncedUser.image ||
          storedUser.roles.join(",") !== syncedUser.roles.join(",")
        ) {
          persistCurrentAuthUser(syncedUser);
        }
      } catch (error) {
        if (abortController.signal.aborted) return;
        console.debug("[Header] Skip current user sync:", error);
      }
    };

    void syncCurrentUser();

    return () => {
      abortController.abort();
      unsubscribe();
    };
  }, []);

  const aboutProjectItem = useMemo(() => getAboutProjectItem(), []);

  return (
    <div className="hidden md:flex flex-1 items-center justify-end gap-4">
      <Button
        key={aboutProjectItem.path}
        onClick={() => navigate(aboutProjectItem.path)}
        variant={
          isPathActive(aboutProjectItem.path) ? "navActive" : "navInactive"
        }
        className="h-9 px-4 py-2 transition-colors text-base lg:text-lg"
      >
        {aboutProjectItem.label}
      </Button>
      <ThemeToggleButton />

      <DropdownMenu modal={false}>
        <DropdownMenuTrigger asChild>
          <Button variant="ghost" size="icon" className="rounded-full">
            <Avatar className="size-10">
              {currentAuthUser?.image && (
                <AvatarImage
                  src={currentAuthUser.image}
                  alt={currentAuthUser.username || "user"}
                />
              )}
              <AvatarFallback>{headerAvatarFallback}</AvatarFallback>
            </Avatar>
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end" className="w-44">
          <DropdownMenuItem onSelect={() => navigate(profilePath)}>
            <User className="size-4" />
            Профиль
          </DropdownMenuItem>
          <DropdownMenuSeparator />
          <DropdownMenuItem variant="destructive" onSelect={handleLogout}>
            <LogOut className="size-4" />
            Выйти
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  );
};
