import { isModerator, useCurrentUser } from "@/entities/session/lib";

export const useModerationAccess = () => {
  const currentAuthUser = useCurrentUser();

  return {
    hasModeratorAccess: isModerator(currentAuthUser),
  };
};
