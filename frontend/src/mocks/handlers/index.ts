import { authHandlers } from "./auth";
import { materialsHandlers } from "./materials";
import { submissionsHandlers } from "./submissions";
import { usersHandlers } from "./users";

export const handlers = [
  ...authHandlers,
  ...usersHandlers,
  ...materialsHandlers,
  ...submissionsHandlers,
];
