import { authHandlers } from "./auth";
import { materialsHandlers } from "./material";
import { submissionsHandlers } from "./submission";
import { usersHandlers } from "./user";

export const handlers = [
  ...authHandlers,
  ...usersHandlers,
  ...materialsHandlers,
  ...submissionsHandlers,
];
