import { useEffect, useState } from "react";

import { getCurrentAuthUser, subscribeToCurrentAuthUser } from "./currentUser";

export const useCurrentUser = () => {
  const [user, setUser] = useState(getCurrentAuthUser);

  useEffect(() => subscribeToCurrentAuthUser(setUser), []);

  return user;
};
